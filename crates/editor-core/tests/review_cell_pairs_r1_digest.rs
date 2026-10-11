//! Reviewer probe for PR 4598 (R1): the corpus digests the PR re-blessed,
//! recomputed with each boolean's contact records fed as a SORTED list
//! of unordered cell pairs with their citations instead of the records'
//! `Debug`. Run on the PR's merge base and on its head, the two printed
//! columns must agree: the outcome under each re-blessed digest did not
//! move, only the records' spelling did.

use crate::corpus;
use crate::fixture::digest::feed_body;
use editor_core::{BooleanValue, Evaluation, ValuePayload};

/// The records as sorted unordered pairs, each with its citations.
fn canon(contacts: &topo::ContactRecords) -> String {
    let mut rows: Vec<String> = contacts
        .cell_pairs()
        .zip(contacts.cites())
        .map(|((x, y), cites)| {
            let (x, y) = if x <= y { (x, y) } else { (y, x) };
            format!("{x:?}{y:?}{:?}", cites.iter().collect::<Vec<_>>())
        })
        .collect();
    rows.sort();
    format!("{}:{}", rows.len(), rows.join(";"))
}

fn canon_digest(ev: &Evaluation<f64>) -> (u64, Vec<String>) {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut said = Vec::new();
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
        match &value.payload {
            ValuePayload::Body(body) => feed_body(&mut feed, body),
            ValuePayload::Boolean(BooleanValue::Body {
                body,
                kind,
                contacts,
            }) => {
                let c = canon(contacts);
                said.push(c.clone());
                feed(format!("{kind:?}{c}").as_bytes());
                feed_body(&mut feed, body);
            }
            ValuePayload::Boolean(BooleanValue::Empty) => feed(b"empty"),
            ValuePayload::Split { above, below } => {
                for (role, side) in [("above", above), ("below", below)] {
                    feed(role.as_bytes());
                    match side {
                        editor_core::SplitSide::Body(body) => feed_body(&mut feed, body),
                        editor_core::SplitSide::Empty => feed(b"empty-side"),
                    }
                }
            }
            _ => {}
        }
    }
    (h, said)
}

#[test]
fn review_r1_canonical_corpus_digests() {
    for name in [
        "crossing_slots",
        "heat_sink",
        "kiss_carry",
        "die",
        "corner_table",
        "boss_union",
        "kitchen_sink",
        "part_select",
    ] {
        let doc = corpus::documents()
            .into_iter()
            .find(|d| d.name == name)
            .expect("the document is registered");
        let ev = corpus::eval::<f64>(&doc.doc);
        assert!(corpus::failures(&ev).is_empty(), "{name}");
        let (h, said) = canon_digest(&ev);
        println!("R1CANON {name} {h:#018x} {said:?}");
    }
}
