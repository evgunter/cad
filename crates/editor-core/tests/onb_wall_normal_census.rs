//! **Wall-normal `z` census over the Band 4 model corpus**, and the
//! `Datum::FaceFrame` half of the byte-movement question.
//!
//! `Vec3::orthonormal_basis` hulls the frame at `Interval` when the
//! normal's `z` encloses zero. The narrowing that canonicalises the
//! zero at `f64` moves the stored `u_ref` of every planar face whose
//! normal has `z = -0.0` — and a `Datum::FaceFrame` stores its spin
//! RELATIVE to the carrier's `u_ref`, so a moved `u_ref` rotates a
//! saved sketch with the document's bytes unchanged. This counts the
//! faces and names the frames.
//!
//! `#[ignore]`d: asserts nothing, gates nothing, prints. The corpus is
//! `corpus::documents()` — the registry itself, so a document added
//! there is censused here without editing this file.
//!
//! ```text
//! cargo test -p editor-core --test all \
//!     -- --ignored --nocapture onb_wall_normal_census
//! ```

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;

use editor_core::{
    CancelToken, Datum, EvalOptions, Node, NodeResult, ValuePayload, all_faces, evaluate,
};
use geom::Surface;
use geom_core::Tol;

/// The four classes the ruling asks for, over one document.
#[derive(Default, Clone, Copy)]
struct ZClasses {
    pos_zero: usize,
    neg_zero: usize,
    tiny: usize,
    other: usize,
}

impl ZClasses {
    fn add(&mut self, z: f64) {
        if z == 0.0 {
            if z.is_sign_negative() {
                self.neg_zero += 1;
            } else {
                self.pos_zero += 1;
            }
        } else if z.abs() < 1e-12 {
            self.tiny += 1;
        } else {
            self.other += 1;
        }
    }

    fn merge(&mut self, o: ZClasses) {
        self.pos_zero += o.pos_zero;
        self.neg_zero += o.neg_zero;
        self.tiny += o.tiny;
        self.other += o.other;
    }

    fn planes(&self) -> usize {
        self.pos_zero + self.neg_zero + self.tiny + self.other
    }
}

fn eval(doc: &editor_core::ProfileDoc) -> editor_core::Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// **Table 2 (editor-core row)** — every planar face of every body the
/// corpus evaluates to, by the class of the stored normal's `z`.
#[test]
#[ignore = "wall-normal census instrument; run explicitly"]
fn wall_normal_z_census_over_the_band4_corpus() {
    println!("| document | bodies | planes | z = +0.0 | z = -0.0 | 0 < |z| < 1e-12 | other |");
    println!("| --- | --- | --- | --- | --- | --- | --- |");
    let mut total = ZClasses::default();
    let (mut docs, mut bodies_total) = (0usize, 0usize);
    for doc in corpus::documents() {
        let ev = eval(&doc.doc);
        let mut c = ZClasses::default();
        let mut bodies = 0usize;
        for result in ev.nodes.values() {
            let NodeResult::Ok(v) = result else { continue };
            let ValuePayload::Body(b) = &v.payload else {
                continue;
            };
            bodies += 1;
            for (_, surface) in b.surfaces() {
                if let Surface::Plane { normal, .. } = surface {
                    c.add(normal.z);
                }
            }
        }
        println!(
            "| {} | {bodies} | {} | {} | {} | {} | {} |",
            doc.name,
            c.planes(),
            c.pos_zero,
            c.neg_zero,
            c.tiny,
            c.other
        );
        total.merge(c);
        bodies_total += bodies;
        docs += 1;
    }
    println!(
        "| **Band 4 corpus ({docs} documents)** | {bodies_total} | {} | {} | {} | {} | {} |",
        total.planes(),
        total.pos_zero,
        total.neg_zero,
        total.tiny,
        total.other
    );
}

/// **Table 3 (`FaceFrame` half)** — every `Datum::FaceFrame` in the
/// corpus, with the `z` class of the planar faces of the body it reads
/// its face out of.
///
/// A frame can only sit on a face of `at`'s body, so a body with no
/// `n.z = -0.0` plane cannot carry a frame on one, whatever face the
/// name resolves to. The per-body counts are therefore the complete
/// answer for this corpus, and they need no name resolution to be it.
#[test]
#[ignore = "FaceFrame instrument; run explicitly"]
fn face_frames_and_the_faces_they_could_sit_on() {
    println!(
        "| document | frame node | at | named faces on `at` | `at` planes | of those, z = -0.0 |"
    );
    println!("| --- | --- | --- | --- | --- | --- |");
    let mut frames = 0usize;
    let mut on_neg_zero_bodies = 0usize;
    for doc in corpus::documents() {
        let ev = eval(&doc.doc);
        for id in doc.doc.order() {
            let Some(Node::Datum(Datum::FaceFrame { at, .. })) = doc.doc.node(*id) else {
                continue;
            };
            frames += 1;
            let mut c = ZClasses::default();
            let named = ev.value(*at).map_or(0, |_| all_faces(&ev, *at).len());
            if let Some(ValuePayload::Body(b)) = ev.value(*at).map(|v| &v.payload) {
                for (_, surface) in b.surfaces() {
                    if let Surface::Plane { normal, .. } = surface {
                        c.add(normal.z);
                    }
                }
            }
            if c.neg_zero > 0 {
                on_neg_zero_bodies += 1;
            }
            println!(
                "| {} | {} | {} | {named} | {} | {} |",
                doc.name,
                id.0,
                at.0,
                c.planes(),
                c.neg_zero
            );
        }
    }
    if frames == 0 {
        println!("| **(none)** | — | — | — | — | — |");
    }
    println!(
        "Datum::FaceFrame nodes in the corpus: {frames}; \
         of those, on a body carrying any n.z = -0.0 plane: {on_neg_zero_bodies}"
    );
}
