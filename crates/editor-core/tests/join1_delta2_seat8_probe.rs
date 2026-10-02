//! JOIN-1 delta-2 review probe: every body-valued node of the seat8
//! split documents, read at tiers 2/3′, the certificate, volume, face
//! count and as an operand. Prints one line per node so trees diff.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use corpus::eval;
use editor_core::{BooleanValue, SplitSide, ValuePayload};
use geom_core::Tol;
use topo::Body;

fn line(b: &Body<f64>, contacts: Option<&topo::ContactRecords>) -> String {
    let t = Tol::witness();
    let t2 = topo::validate_closed(b).is_ok();
    let t3 = match contacts {
        Some(c) => format!("{:?}", topo::validate_pseudomanifold(b, c, t).is_ok()),
        None => "n/a".into(),
    };
    let cert = topo::validate_geometric_certificate(b, t).is_ok();
    let v = topo::mass_properties(b, t).map(|m| m.volume);
    let far = sweep::test_support::brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), t);
    let op = match topo::union(b, &far, t) {
        Ok(_) => "ok".to_string(),
        Err(e) => format!("REFUSED {:?}", e.kind()),
    };
    format!(
        "faces={} edges={} t2={t2} t3p={t3} cert={cert} v={:?} operand={op}",
        b.faces().count(),
        b.edges().count(),
        v.map(|v| format!("{v:.12}"))
    )
}

#[test]
#[ignore = "review probe"]
fn d2_seat8_bodies() {
    for name in ["cut_cylinder", "part_select", "kitchen_sink"] {
        let doc = corpus::documents()
            .into_iter()
            .find(|d| d.name == name)
            .expect("registered");
        let ev = eval::<f64>(&doc.doc);
        for (i, id) in ev.order.iter().enumerate() {
            let s = match ev.value(*id).map(|v| &v.payload) {
                Some(ValuePayload::Body(b)) => format!("Body {}", line(b, None)),
                Some(ValuePayload::Boolean(BooleanValue::Body { body, contacts, .. })) => {
                    format!("Boolean {}", line(body, Some(contacts)))
                }
                Some(ValuePayload::Split { above, below }) => {
                    let side = |s: &SplitSide<f64>| match s {
                        SplitSide::Empty => "empty".to_string(),
                        SplitSide::Body(b) => line(b, None),
                    };
                    format!("Split above[{}] below[{}]", side(above), side(below))
                }
                Some(_) => continue,
                None => "FAILED/none".into(),
            };
            println!("SEAT8 {name} #{i} {s}");
        }
    }
}
