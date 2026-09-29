//! Review probe (origin-graft-r2): dump every corpus document's name
//! tables, slot versions stripped, to `$GRAFT_R2_DUMP/<doc>.txt`, so two
//! commits can be diffed name-for-name and slot-index-for-slot-index.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;

fn strip_versions(s: &str) -> String {
    // `Key(12v5)` -> `Key(12)`: drop `v<digits>` that follows a digit.
    let b: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == 'v' && i > 0 && b[i - 1].is_ascii_digit() && i + 1 < b.len() && b[i + 1].is_ascii_digit() {
            let mut j = i + 1;
            while j < b.len() && b[j].is_ascii_digit() {
                j += 1;
            }
            i = j;
            continue;
        }
        out.push(b[i]);
        i += 1;
    }
    out
}

#[test]
fn graft_r2_dump_corpus_names() {
    let Ok(dir) = std::env::var("GRAFT_R2_DUMP") else {
        return;
    };
    std::fs::create_dir_all(&dir).unwrap();
    for d in corpus::documents() {
        let ev = corpus::eval::<f64>(&d.doc);
        let mut full = String::new();
        for id in &ev.order {
            full.push_str(&format!("#{id:?}\n"));
            match ev.value(*id) {
                Some(v) => {
                    for (n, e) in v.name_table.iter() {
                        full.push_str(&format!("{n:?}={e:?};\n"));
                    }
                }
                None => full.push_str("NO VALUE\n"),
            }
        }
        std::fs::write(format!("{dir}/{}.full.txt", d.name), &full).unwrap();
        std::fs::write(format!("{dir}/{}.stripped.txt", d.name), strip_versions(&full)).unwrap();
    }
}
