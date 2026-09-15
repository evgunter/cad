//! **The frame-seam census over the Band 4 model corpus**, and the
//! `Datum::FaceFrame` half of the frame-movement question.
//!
//! `Vec3::orthonormal_basis` crosses the normal with the world axis its
//! own components choose — `e_z` when `|n.z| ≤ max(|n.x|, |n.y|)/2` —
//! and the equality is its one discontinuity. At `f64` and at a point
//! enclosure the comparison decides there anyway; only an enclosure of
//! positive width across it hulls. The classification lives in
//! `test_utils::seam_census`, which the three corpus instruments share;
//! read its docs for what a count of normals on the seam does and does
//! not measure (it bounds the exposure: a blend mints its own `u_ref`
//! and never reads this constructor).
//!
//! [`no_corpus_face_sits_on_the_frame_seam`] ASSERTS the zero this
//! corpus measures — the number that decided the comparison — and the
//! two instruments below it print the full tables and are `#[ignore]`d.
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
use test_utils::seam_census::SeamClasses;

fn eval(doc: &editor_core::ProfileDoc) -> editor_core::Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// **The measurement that decided the comparison, asserted.** Not one
/// of this corpus's planar faces sits on the seam
/// `|n.z| = max(|n.x|, |n.y|)/2`, so no stored frame in it can be
/// hulled by an undecided axis choice — while 578 of the same 600 sit
/// on the tie set of the three-component order the spec first named,
/// which is why that rule is not the one.
///
/// The row is asserted rather than printed because it is the ground the
/// construction stands on: a document added to the registry with a face
/// on the seam is a fact the kernel's designers need to hear, and an
/// `#[ignore]`d printer would never tell them. The instruments below
/// print the per-document tables.
#[test]
fn no_corpus_face_sits_on_the_frame_seam() {
    let mut total = SeamClasses::default();
    let mut docs = 0usize;
    for doc in corpus::documents() {
        let ev = eval(&doc.doc);
        let mut c = SeamClasses::default();
        for result in ev.nodes.values() {
            let NodeResult::Ok(v) = result else { continue };
            let ValuePayload::Body(b) = &v.payload else {
                continue;
            };
            for (_, surface) in b.surfaces() {
                if let Surface::Plane { normal, .. } = surface {
                    c.add((normal.x, normal.y, normal.z));
                }
            }
        }
        assert_eq!(
            c.on_seam,
            0,
            "{}: {} of its {} planar faces are on the frame seam",
            doc.name,
            c.on_seam,
            c.planes()
        );
        total.merge(c);
        docs += 1;
    }
    println!(
        "Band 4 corpus ({docs} documents): {} planar faces, {} on the seam, \
         {} on the three-component order's tie set",
        total.planes(),
        total.on_seam,
        total.three_way_tie
    );
    // Anti-vacuity: the corpus is 600 planar faces over 28 documents
    // at the default ε, and a registry that stopped evaluating would
    // otherwise pass this row in silence. The floor sits well below
    // that because which documents build is ε-dependent; the zero
    // above is not.
    assert!(
        docs >= 28 && total.planes() >= 400,
        "the corpus shrank: {docs} documents, {} planar faces",
        total.planes()
    );
    // The contrast, measured rather than recited.
    assert!(
        total.three_way_tie * 2 > total.planes(),
        "the three-component order's tie set no longer covers most of the corpus: \
         {} of {}",
        total.three_way_tie,
        total.planes()
    );
}

/// Every planar face of every body the corpus evaluates to, by whether
/// its normal sits on the axis order's tie set and by which axis wins.
#[test]
#[ignore = "wall-normal census instrument; run explicitly"]
fn axis_tie_census_over_the_band4_corpus() {
    println!(
        "| document | bodies | planes | on the seam | off it | e_z arm | e_y arm | on a three-way tie |"
    );
    println!("| --- | --- | --- | --- | --- | --- | --- | --- |");
    let mut total = SeamClasses::default();
    let (mut docs, mut bodies_total) = (0usize, 0usize);
    for doc in corpus::documents() {
        let ev = eval(&doc.doc);
        let mut c = SeamClasses::default();
        let mut bodies = 0usize;
        for result in ev.nodes.values() {
            let NodeResult::Ok(v) = result else { continue };
            let ValuePayload::Body(b) = &v.payload else {
                continue;
            };
            bodies += 1;
            for (_, surface) in b.surfaces() {
                if let Surface::Plane { normal, .. } = surface {
                    c.add((normal.x, normal.y, normal.z));
                }
            }
        }
        println!(
            "| {} | {bodies} | {} | {} | {} | {} | {} | {} |",
            doc.name,
            c.planes(),
            c.on_seam,
            c.off_seam,
            c.e_z_arm,
            c.e_y_arm,
            c.three_way_tie
        );
        total.merge(c);
        bodies_total += bodies;
        docs += 1;
    }
    println!(
        "| **Band 4 corpus ({docs} documents)** | {bodies_total} | {} | {} | {} | {} | {} | {} |",
        total.planes(),
        total.on_seam,
        total.off_seam,
        total.e_z_arm,
        total.e_y_arm,
        total.three_way_tie
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
    println!("| --- | --- | --- | --- | --- | --- | --- |");
    let mut frames = 0usize;
    let mut on_tie_bodies = 0usize;
    for doc in corpus::documents() {
        let ev = eval(&doc.doc);
        for id in doc.doc.order() {
            let Some(Node::Datum(Datum::FaceFrame { at, .. })) = doc.doc.node(*id) else {
                continue;
            };
            frames += 1;
            let mut c = SeamClasses::default();
            let named = ev.value(*at).map_or(0, |_| all_faces(&ev, *at).len());
            if let Some(ValuePayload::Body(b)) = ev.value(*at).map(|v| &v.payload) {
                for (_, surface) in b.surfaces() {
                    if let Surface::Plane { normal, .. } = surface {
                        c.add((normal.x, normal.y, normal.z));
                    }
                }
            }
            if c.on_seam > 0 {
                on_tie_bodies += 1;
            }
            println!(
                "| {} | {} | {} | {named} | {} | {} |",
                doc.name,
                id.0,
                at.0,
                c.planes(),
                c.on_seam
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
