//! **The axis-order tie census over the Band 4 model corpus**, and the
//! `Datum::FaceFrame` half of the frame-movement question.
//!
//! `Vec3::orthonormal_basis` crosses the normal with the world axis of
//! its smallest-magnitude component. The construction's one
//! discontinuity is the set where the two smallest magnitudes TIE — a
//! set every axis-aligned normal sits exactly on, with two components
//! at zero. At `f64` and at a point enclosure that tie DECIDES; only an
//! enclosure of positive width across it hulls. This counts the faces
//! on the tie set, and names the frames that read one.
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
use geom_core::{Tol, Vec3};

/// The axis order's decision, at `f64`: which world axis the normal is
/// crossed with, and whether the two smallest magnitudes TIE exactly.
///
/// The tie set is the construction's discontinuity, so an exact tie is
/// the class that matters: at `f64` and at a point enclosure it DECIDES
/// (the tie-break keys on the value, not on a zero's sign bit), and it
/// is the only class an enclosure of positive width can fail to decide.
/// Every axis-aligned normal is on it — two components exactly zero.
#[derive(Default, Clone, Copy)]
struct TieClasses {
    exact_tie: usize,
    separated: usize,
    by_axis: [usize; 3],
}

impl TieClasses {
    fn add(&mut self, n: Vec3<f64>) {
        let (ax, ay, az) = (n.x.abs(), n.y.abs(), n.z.abs());
        let k = if az <= ay && az <= ax {
            2
        } else if ay <= ax {
            1
        } else {
            0
        };
        self.by_axis[k] += 1;
        let mut m = [ax, ay, az];
        m.sort_by(f64::total_cmp);
        if m[0] == m[1] {
            self.exact_tie += 1;
        } else {
            self.separated += 1;
        }
    }

    fn merge(&mut self, o: TieClasses) {
        self.exact_tie += o.exact_tie;
        self.separated += o.separated;
        for k in 0..3 {
            self.by_axis[k] += o.by_axis[k];
        }
    }

    fn planes(&self) -> usize {
        self.exact_tie + self.separated
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

/// Every planar face of every body the corpus evaluates to, by whether
/// its normal sits on the axis order's tie set and by which axis wins.
#[test]
#[ignore = "wall-normal census instrument; run explicitly"]
fn axis_tie_census_over_the_band4_corpus() {
    println!(
        "| document | bodies | planes | on an exact tie | separated | k = x | k = y | k = z |"
    );
    println!("| --- | --- | --- | --- | --- | --- | --- | --- |");
    let mut total = TieClasses::default();
    let (mut docs, mut bodies_total) = (0usize, 0usize);
    for doc in corpus::documents() {
        let ev = eval(&doc.doc);
        let mut c = TieClasses::default();
        let mut bodies = 0usize;
        for result in ev.nodes.values() {
            let NodeResult::Ok(v) = result else { continue };
            let ValuePayload::Body(b) = &v.payload else {
                continue;
            };
            bodies += 1;
            for (_, surface) in b.surfaces() {
                if let Surface::Plane { normal, .. } = surface {
                    c.add(*normal);
                }
            }
        }
        println!(
            "| {} | {bodies} | {} | {} | {} | {} | {} | {} |",
            doc.name,
            c.planes(),
            c.exact_tie,
            c.separated,
            c.by_axis[0],
            c.by_axis[1],
            c.by_axis[2]
        );
        total.merge(c);
        bodies_total += bodies;
        docs += 1;
    }
    println!(
        "| **Band 4 corpus ({docs} documents)** | {bodies_total} | {} | {} | {} | {} | {} | {} |",
        total.planes(),
        total.exact_tie,
        total.separated,
        total.by_axis[0],
        total.by_axis[1],
        total.by_axis[2]
    );
}

/// Every `Datum::FaceFrame` in the corpus, with the tie class of the
/// planar faces of the body it reads its face out of.
///
/// A frame stores its spin RELATIVE to the carrier's stored `u_ref`, so
/// a changed constructor rotates a saved sketch with the document's
/// bytes unchanged. A frame can only sit on a face of `at`'s body, so
/// the per-body counts are the complete answer for this corpus and they
/// need no name resolution to be it.
#[test]
#[ignore = "FaceFrame instrument; run explicitly"]
fn face_frames_and_the_faces_they_could_sit_on() {
    println!(
        "| document | frame node | at | named faces on `at` | `at` planes | of those, on a tie |"
    );
    println!("| --- | --- | --- | --- | --- | --- |");
    let mut frames = 0usize;
    let mut on_tie_bodies = 0usize;
    for doc in corpus::documents() {
        let ev = eval(&doc.doc);
        for id in doc.doc.order() {
            let Some(Node::Datum(Datum::FaceFrame { at, .. })) = doc.doc.node(*id) else {
                continue;
            };
            frames += 1;
            let mut c = TieClasses::default();
            let named = ev.value(*at).map_or(0, |_| all_faces(&ev, *at).len());
            if let Some(ValuePayload::Body(b)) = ev.value(*at).map(|v| &v.payload) {
                for (_, surface) in b.surfaces() {
                    if let Surface::Plane { normal, .. } = surface {
                        c.add(*normal);
                    }
                }
            }
            if c.exact_tie > 0 {
                on_tie_bodies += 1;
            }
            println!(
                "| {} | {} | {} | {named} | {} | {} |",
                doc.name,
                id.0,
                at.0,
                c.planes(),
                c.exact_tie
            );
        }
    }
    if frames == 0 {
        println!("| **(none)** | — | — | — | — | — |");
    }
    println!(
        "Datum::FaceFrame nodes in the corpus: {frames}; \
         of those, on a body carrying any plane on the tie set: {on_tie_bodies}"
    );
}
