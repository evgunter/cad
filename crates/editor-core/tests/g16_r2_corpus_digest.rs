//! REVIEW PROBE (lib-g16-r2): per-document name-table digests over the
//! whole corpus registry, printed for cross-tree comparison.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

/// FNV-1a 64 over the tables' deterministic Debug encoding — copied
/// from `m4_pr3_names_ci.rs::digest` so the number is comparable.
fn digest(ev: &editor_core::Evaluation<f64>) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut feed = |s: &str| {
        for b in s.bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x1000_0000_01b3);
        }
    };
    for id in &ev.order {
        feed(&format!("#{id:?}"));
        if let Some(v) = ev.value(*id) {
            for (n, e) in v.name_table.iter() {
                feed(&format!("{n:?}={e:?};"));
            }
        }
    }
    h
}

#[test]
fn g16_r2_per_document_name_digests() {
    for d in corpus::documents() {
        let ev = corpus::eval::<f64>(&d.doc);
        println!("G16R2 {} {:016x}", d.name, digest(&ev));
    }
}
