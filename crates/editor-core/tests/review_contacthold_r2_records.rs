//! **Reviewer probe (CONTACTHOLD at-rest-cell-pairs, R2)**: what the
//! corpus re-bless moved. Prints, per corpus document, the evaluation
//! digest with every contact list left out, and each value's records as
//! a sorted list of unordered cell pairs with their citations.
//!
//! Written against the two methods both the seven-list and the one-list
//! `ContactRecords` expose (`cell_pairs`, `cites`), so the same file
//! compiles on main before the change and on the change: the outputs of
//! the two trees are diffed by hand. It asserts nothing; `--nocapture`
//! reads it.
#![allow(clippy::expect_used)]

use crate::corpus;
use crate::fixture::digest::feed_body;
use editor_core::{BooleanValue, Evaluation, SplitSide, ValuePayload};

fn records(c: &topo::ContactRecords) -> Vec<String> {
    let mut out: Vec<String> = c
        .cell_pairs()
        .zip(c.cites())
        .map(|((x, y), cites)| {
            let (x, y) = if x <= y { (x, y) } else { (y, x) };
            format!("{x:?}~{y:?} <- {:?}", cites.iter().collect::<Vec<_>>())
        })
        .collect();
    out.sort();
    out
}

fn digest_without_contacts(ev: &Evaluation<f64>) -> (u64, Vec<String>) {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut lines = Vec::new();
    let mut feed = |bytes: &[u8]| {
        for b in bytes {
            h ^= u64::from(*b);
            h = h.wrapping_mul(0x1000_0000_01b3);
        }
    };
    for id in &ev.order {
        feed(format!("#{id:?}").as_bytes());
        let Some(value) = ev.value(*id) else { continue };
        for (name, entry) in value.name_table.iter() {
            feed(format!("{name:?}={entry:?}").as_bytes());
        }
        feed(value.payload.kind_name().as_bytes());
        let at_rest = records(&value.contacts);
        if !at_rest.is_empty() {
            lines.push(format!("  {id:?} value.contacts {at_rest:?}"));
        }
        match &value.payload {
            ValuePayload::Body(body) => feed_body(&mut feed, body),
            ValuePayload::Boolean(BooleanValue::Body {
                body,
                kind,
                contacts,
            }) => {
                feed(format!("{kind:?}").as_bytes());
                let r = records(contacts);
                lines.push(format!("  {id:?} boolean.contacts {r:?}"));
                feed_body(&mut feed, body);
            }
            ValuePayload::Boolean(BooleanValue::Empty) => feed(b"empty"),
            ValuePayload::Split { above, below } => {
                for (role, side) in [("above", above), ("below", below)] {
                    feed(role.as_bytes());
                    match side {
                        SplitSide::Body(body) => feed_body(&mut feed, body),
                        SplitSide::Empty => feed(b"empty-side"),
                    }
                }
            }
            _ => {}
        }
    }
    (h, lines)
}

#[test]
#[ignore = "reviewer differential: run with --ignored --nocapture"]
fn corpus_outcomes_with_records_normalised() {
    for doc in corpus::documents() {
        let ev = corpus::eval::<f64>(&doc.doc);
        let failures = corpus::failures(&ev);
        let (h, lines) = digest_without_contacts(&ev);
        println!("R2DIFF {}: {h:#018x} failures={}", doc.name, failures.len());
        for l in lines {
            println!("R2DIFF {}{l}", doc.name);
        }
    }
}
