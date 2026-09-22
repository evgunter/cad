//! R2 review scratch: the shell corpus documents dumped at `f64`, as
//! text, so a base-vs-head diff is a byte comparison. Not committed to
//! main; written to the path in `R2_D9_OUT`.

use editor_core::eval::{NodeResult, ValuePayload};

use crate::corpus;
use crate::fixture::digest::feed_body;

fn dump_doc(name: &str, doc: &editor_core::ProfileDoc, out: &mut String) {
    let ev = corpus::eval::<f64>(doc);
    out.push_str(&format!("== {name}\n"));
    out.push_str(&format!("outcome {:?}\n", ev.outcome));
    for id in &ev.order {
        let r = ev.nodes.get(id);
        match r {
            None => out.push_str(&format!("{id:?} MISSING\n")),
            Some(NodeResult::Failed(e)) => out.push_str(&format!("{id:?} FAILED {e:?}\n")),
            Some(NodeResult::Poisoned { through }) => {
                out.push_str(&format!("{id:?} POISONED {through:?}\n"));
            }
            Some(NodeResult::Ok(v)) => {
                out.push_str(&format!("{id:?} OK {}\n", v.payload.kind_name()));
                out.push_str(&format!("  names {:?}\n", v.name_table));
                if let ValuePayload::Body(b) = &v.payload {
                    let mut bytes: Vec<u8> = Vec::new();
                    feed_body(&mut |s: &[u8]| bytes.extend_from_slice(s), b);
                    for chunk in bytes.chunks(48) {
                        out.push_str("  ");
                        for byte in chunk {
                            out.push_str(&format!("{byte:02x}"));
                        }
                        out.push('\n');
                    }
                }
            }
        }
    }
}

#[test]
fn r2_d9_shell_corpus_dump() {
    let Ok(path) = std::env::var("R2_D9_OUT") else {
        return;
    };
    let mut out = String::new();
    dump_doc("cup", &corpus::cup::document().doc, &mut out);
    dump_doc("vessel", &corpus::vessel::document().doc, &mut out);
    std::fs::write(&path, out.as_bytes()).expect("the dump is written");
    println!("wrote {} bytes to {path}", out.len());
}
