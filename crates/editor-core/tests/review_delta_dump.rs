//! Delta review: placed geometry of pattern documents, ids masked, for
//! a diff against main.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::corpus;
use editor_core::{NodeResult, ValuePayload};

fn pts(b: &topo::Body<f64>) -> Vec<[u64; 3]> {
    let mut v: Vec<[u64; 3]> = b
        .points()
        .map(|(_, p)| p.to_array().map(f64::to_bits))
        .collect();
    v.sort_unstable();
    v
}

#[test]
#[ignore = "a diff tool: DELTA_DUMP=<path> on this tree and on main, then diff the two files"]
fn delta_dump_placed_geometry() {
    let out = std::env::var("DELTA_DUMP").expect("DELTA_DUMP path");
    let mut lines = Vec::new();
    for d in corpus::documents() {
        if !["heat_sink", "kitchen_sink", "die_fillet"].contains(&d.name) {
            continue;
        }
        let ev = corpus::eval::<f64>(&d.doc);
        let mut nodes: Vec<(String, Vec<Vec<[u64; 3]>>)> = Vec::new();
        for id in d.doc.ids() {
            let row = match ev.result(id) {
                Some(NodeResult::Ok(v)) => match &v.payload {
                    ValuePayload::Body(b) => {
                        ("body".to_owned(), vec![pts(&crate::fixture::in_space(b))])
                    }
                    ValuePayload::Instances(ms) => (
                        format!("family{}", ms.len()),
                        ms.iter()
                            .map(|m| {
                                m.as_ref()
                                    .map(|b| pts(&crate::fixture::in_space(b)))
                                    .unwrap_or_default()
                            })
                            .collect(),
                    ),
                    other => (other.kind_name().to_string(), vec![]),
                },
                Some(NodeResult::Failed(_)) => ("failed".into(), vec![]),
                _ => ("other".into(), vec![]),
            };
            nodes.push(row);
        }
        // ids masked: the multiset of node rows, sorted.
        nodes.sort();
        for (kind, members) in nodes {
            let mut h = std::collections::hash_map::DefaultHasher::new();
            std::hash::Hash::hash(&members, &mut h);
            lines.push(format!(
                "{} {kind} members={} pts={:?} h={:016x}",
                d.name,
                members.len(),
                members.iter().map(Vec::len).collect::<Vec<_>>(),
                std::hash::Hasher::finish(&h)
            ));
        }
    }
    std::fs::write(out, lines.join("\n")).unwrap();
}
