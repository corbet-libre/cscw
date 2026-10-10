use super::*;
use payments::{Action, MemoryJournal, Private};
use std::{sync::Arc, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

fn instruction(
    id: u8,
    operation: &str,
    parent: Option<[u8; 32]>,
    terms: serde_json::Value,
) -> Instruction {
    Instruction {
        kind: Action::Execute,
        id: [id; 32],
        parent,
        adapter: "fixture-stripe".into(),
        operation: operation.into(),
        binding: Binding {
            scope: "scope".into(),
            subject: "member".into(),
            action: "deposit".into(),
        },
        expires_at: 100,
        terms: Private::new(serde_json::to_vec(&terms).unwrap()),
        evidence: Private::new(vec![]),
    }
}
fn permit(request: &Instruction, mode: cost::Mode) -> cost::DispatchPermit<String> {
    let fee = cost::Amount {
        unit: cost::Unit {
            code: "fixture-fee".into(),
            scale: 0,
        },
        atoms: 0,
    };
    cost::Plan::new(
        vec![cost::Offer {
            candidate: cost::Candidate::new(request.adapter.clone(), [request.operation.clone()]),
            quote: cost::Quote {
                price: cost::Price::Exact(fee.clone()),
                source: "inert fixture".into(),
                valid_until: 100,
                binding: cost::binding(request, mode),
            },
        }],
        cost::Config::new([request.adapter.clone()], [request.operation.clone()]),
        cost::CostOrder::Configured,
        fee,
        1,
        mode,
        0,
    )
    .unwrap()
    .start()
    .begin(1)
    .unwrap()
}
async fn server(replies: Vec<String>) -> (String, tokio::task::JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}/", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        let mut requests = Vec::new();
        for reply in replies {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut bytes = Vec::new();
            let mut buf = [0; 4096];
            loop {
                let n = stream.read(&mut buf).await.unwrap();
                bytes.extend_from_slice(&buf[..n]);
                if let Some(end) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
                    let head = String::from_utf8_lossy(&bytes[..end]).to_lowercase();
                    let len = head
                        .lines()
                        .find_map(|l| l.strip_prefix("content-length: "))
                        .map_or(0, |v| v.parse::<usize>().unwrap());
                    if bytes.len() >= end + 4 + len {
                        break;
                    }
                }
            }
            requests.push(String::from_utf8(bytes).unwrap());
            stream
                .write_all(
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        reply.len(),
                        reply
                    )
                    .as_bytes(),
                )
                .await
                .unwrap();
        }
        requests
    });
    (url, task)
}
fn payments(url: String, mode: cost::Mode) -> Payments<MemoryJournal> {
    let mut payments = Payments::new(MemoryJournal::default(), [9; 32]).unwrap();
    let client = stripe::ClientBuilder::new("sk_test_fixture")
        .url(url)
        .build()
        .unwrap();
    payments
        .register(Arc::new(
            cpmt::stripe::Stripe::from_client(
                "fixture-stripe".into(),
                client,
                Duration::from_secs(2),
                8192,
                mode,
            )
            .unwrap(),
        ))
        .unwrap();
    payments
}
#[tokio::test]
async fn real_sdk_hold_capture_and_refund_preserve_domain_meaning_and_private_results() {
    let (url,calls)=server(vec![
        r#"{"id":"pi_fixture","object":"payment_intent","status":"requires_capture","amount":1200,"amount_capturable":1200,"currency":"chf","private_extension":"full"}"#.into(),
        r#"{"id":"pi_fixture","object":"payment_intent","status":"succeeded","amount":1200,"amount_received":1200,"currency":"chf"}"#.into(),
        r#"{"id":"re_fixture","object":"refund","status":"succeeded","amount":1200,"currency":"chf"}"#.into(),
    ]).await;
    let payments = payments(url, cost::Mode::Production);
    let mut requests = [
        instruction(
            1,
            "stripe.intent.create",
            None,
            serde_json::json!({"amount":1200,"currency":"chf","capture_method":"manual"}),
        ),
        instruction(
            2,
            "stripe.intent.capture",
            Some([1; 32]),
            serde_json::json!({}),
        ),
        instruction(
            3,
            "stripe.refund.create",
            Some([2; 32]),
            serde_json::json!({}),
        ),
    ];
    for (request, fact) in requests
        .iter()
        .zip([Fact::Held, Fact::Collected, Fact::Returned])
    {
        let result = execute(
            &payments,
            request,
            permit(request, cost::Mode::Production),
            1,
            details::Disclosure::Complete,
        )
        .await
        .unwrap();
        assert_eq!(result.fact(), fact);
        assert!(result.evidence(&request.binding, 1, 50).is_ok());
        let mut other = request.binding.clone();
        other.subject.push('x');
        assert!(result.evidence(&other, 1, 50).is_err());
        assert!(!result.payment().upstream.is_empty());
        assert!(result.into_payment().outcome.satisfies_terms());
    }
    let replay = execute(
        &payments,
        &requests[0],
        permit(&requests[0], cost::Mode::Production),
        1,
        details::Disclosure::Complete,
    )
    .await
    .unwrap();
    assert!(replay.payment().upstream.is_empty());
    requests[0].terms = Private::new(b"different terms".to_vec());
    assert!(
        execute(
            &payments,
            &requests[0],
            permit(&requests[0], cost::Mode::Production),
            1,
            details::Disclosure::Minimal
        )
        .await
        .is_err()
    );
    let calls = calls.await.unwrap();
    assert_eq!(calls.len(), 3);
    assert!(calls[1].starts_with("POST /v1/payment_intents/pi_fixture/capture "));
    assert!(calls[2].contains("payment_intent=pi_fixture"));
}
#[tokio::test]
async fn sandbox_and_unproven_amounts_do_not_mint_deposit_proofs() {
    for (mode, amount) in [(cost::Mode::Sandbox, 1200), (cost::Mode::Production, 100)] {
        let (url,calls)=server(vec![format!(r#"{{"id":"pi_fixture","object":"payment_intent","status":"requires_capture","amount_capturable":{amount},"currency":"chf"}}"#)]).await;
        let payments = payments(url, mode);
        let request = instruction(
            1,
            "stripe.intent.create",
            None,
            serde_json::json!({"amount":1200,"currency":"chf"}),
        );
        let result = execute(
            &payments,
            &request,
            permit(&request, mode),
            1,
            details::Disclosure::Minimal,
        )
        .await
        .unwrap();
        assert!(result.evidence(&request.binding, 1, 50).is_err());
        assert_eq!(
            result.fact(),
            if amount == 1200 {
                Fact::Held
            } else {
                Fact::Unqualified
            }
        );
        assert!(result.payment().upstream.is_empty());
        assert_eq!(calls.await.unwrap().len(), 1);
    }
}
#[test]
fn dummy_transformations_preserve_all_external_meanings_without_granting_proof() {
    for (state, expected) in [
        (State::Authorized, Fact::Held),
        (State::Collected, Fact::Collected),
        (State::Released, Fact::Released),
        (State::Refunded, Fact::Returned),
        (State::Pending, Fact::Pending),
        (State::RequiresAction, Fact::Pending),
        (State::Unknown, Fact::Unknown),
        (State::Failed, Fact::Refused),
        (State::Settled, Fact::Unqualified),
        (State::Verified, Fact::Unqualified),
        (State::Active, Fact::Unqualified),
        (State::Cancelled, Fact::Unqualified),
    ] {
        assert_eq!(simulate_fact(state, true), Simulated { fact: expected });
    }
    assert_eq!(
        simulate_fact(State::Authorized, false).fact,
        Fact::Unqualified
    );
}
