//! Delta-review probe for PR 4441: split the seat7/seat8 digest feed
//! into what it reads, so main and head can be compared channel by
//! channel. Prints `PROBE` lines and writes each document's curve
//! sources to a file.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use editor_core::{BooleanValue, SplitSide, ValuePayload};
use topo::Body;

fn fnv(h: &mut u64, bytes: &[u8]) {
    for b in bytes {
        *h ^= u64::from(*b);
        *h = h.wrapping_mul(0x1000_0000_01b3);
    }
}

fn bodies(ev: &editor_core::Evaluation<f64>) -> Vec<std::sync::Arc<Body<f64>>> {
    let mut out = Vec::new();
    for id in &ev.order {
        let Some(value) = ev.value(*id) else { continue };
        match &value.payload {
            ValuePayload::Body(b) => out.push(std::sync::Arc::clone(b)),
            ValuePayload::Boolean(BooleanValue::Body { body, .. }) => out.push(std::sync::Arc::clone(body)),
            ValuePayload::Split { above, below } => {
                for side in [above, below] {
                    if let SplitSide::Body(b) = side {
                        out.push(std::sync::Arc::clone(b));
                    }
                }
            }
            _ => {}
        }
    }
    out
}

#[test]
fn probe_pin_channels() {
    let dir = std::env::var("PROBE_OUT").unwrap_or_else(|_| "/tmp".into());
    for name in ["cut_cylinder", "boss_union", "die", "corner_table", "kitchen_sink"] {
        let doc = corpus::documents()
            .into_iter()
            .find(|d| d.name == name)
            .expect("registered");
        let ev = corpus::eval::<f64>(&doc.doc);
        let (mut geo, mut other_src, mut curve_src) = (
            0xcbf2_9ce4_8422_2325u64,
            0xcbf2_9ce4_8422_2325u64,
            0xcbf2_9ce4_8422_2325u64,
        );
        let mut dump = String::new();
        for body in bodies(&ev) {
            for (key, p) in body.points() {
                for c in p.to_array() {
                    fnv(&mut geo, &c.to_bits().to_be_bytes());
                }
                fnv(&mut other_src, format!("{key:?}<-{:?}", body.point_source(key)).as_bytes());
            }
            for (key, curve) in body.curves() {
                fnv(&mut geo, format!("{key:?}{curve:?}").as_bytes());
                let s = format!("{key:?}<-{:?}", body.curve_source(key));
                fnv(&mut curve_src, s.as_bytes());
                dump += &s;
                dump.push('\n');
            }
            for (key, surface) in body.surfaces() {
                fnv(&mut geo, format!("{key:?}{surface:?}").as_bytes());
                fnv(&mut other_src, format!("{key:?}<-{:?}", body.surface_source(key)).as_bytes());
            }
            for (key, face) in body.faces() {
                fnv(&mut geo, format!("{key:?}{:?}", face.surface).as_bytes());
            }
            for (key, edge) in body.edges() {
                fnv(&mut geo, format!("{key:?}{:?}", edge.curve).as_bytes());
            }
        }
        println!(
            "PROBE {name}: geometry+topology {geo:#018x} point/surface sources {other_src:#018x} \
             curve sources {curve_src:#018x}"
        );
        std::fs::write(format!("{dir}/{name}.curvesrc.txt"), dump).unwrap();
    }
}
