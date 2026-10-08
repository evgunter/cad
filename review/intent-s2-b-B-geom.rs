//! Reviewer B: id-free body digests of every corpus document, by node position.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use editor_core::{BooleanValue, ValuePayload};

fn fnv(h: &mut u64, x: u64) {
    for b in x.to_le_bytes() {
        *h ^= u64::from(b);
        *h = h.wrapping_mul(0x100_0000_01b3);
    }
}

#[test]
fn review_b_id_free_bodies() {
    let mut out = String::new();
    for doc in crate::corpus::documents() {
        let ev = crate::corpus::eval::<f64>(&doc.doc);
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        let mut bodies = 0u64;
        for (pos, id) in doc.doc.ids().iter().enumerate() {
            let body = match ev.value(*id).map(|v| &v.payload) {
                Some(ValuePayload::Body(b)) => b,
                Some(ValuePayload::Boolean(BooleanValue::Body { body, .. })) => body,
                _ => continue,
            };
            bodies += 1;
            fnv(&mut h, pos as u64);
            let mut pts: Vec<[u64; 3]> = body
                .points()
                .map(|(_, p)| p.to_array().map(f64::to_bits))
                .collect();
            pts.sort_unstable();
            fnv(&mut h, pts.len() as u64);
            for c in pts.into_iter().flatten() {
                fnv(&mut h, c);
            }
        }
        out.push_str(&format!("{} bodies={bodies} {h:016x}\n", doc.name));
    }
    std::fs::write(std::env::var("REVIEW_B_OUT").unwrap(), out).unwrap();
}
