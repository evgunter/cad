//! R1 review scratch: dumps every corpus document's whole name table
//! to `$R1_DUMP` so the head and the merge base can be diffed name for
//! name (C4). Not a pin; it asserts nothing.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;

#[test]
fn r1_dump_every_corpus_name() {
    let Ok(path) = std::env::var("R1_DUMP") else {
        return;
    };
    let mut out = String::new();
    for d in corpus::documents() {
        let ev = corpus::eval::<f64>(&d.doc);
        for id in &ev.order {
            if let Some(v) = ev.value(*id) {
                let mut rows: Vec<String> = v
                    .name_table
                    .iter()
                    .map(|(n, e)| format!("{}\t{id:?}\t{n:?}\t{e:?}", d.name))
                    .collect();
                rows.sort();
                for r in rows {
                    out.push_str(&r);
                    out.push('\n');
                }
            }
        }
    }
    std::fs::write(path, out).unwrap();
}
