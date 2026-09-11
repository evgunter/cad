//! **The index seam's picture is bit-for-bit the fresh one, edit after
//! edit** — the differential the pick-index memo is built under.
//!
//! The seam ([`InlineIndexer`]) is where the previous generation's
//! index lives, so it is where any reuse across edits happens. Whatever
//! it keeps between builds, the picture it answers has ONE definition:
//! the index the plain door ([`PickIndex::build`]) builds from the same
//! landed run, whose meshes are `mesh::tessellate` of each root body.
//! Every row here opens a document, indexes it through the seam, then
//! runs a sequence of edits — change a parameter, change another,
//! revert the first — and after every landing asserts that the seam's
//! meshes are byte-identical to the plain door's (the D9 goldens'
//! digest, over every position, patch and boundary) and that a fixed
//! set of rays picks the same faces on both.
//!
//! The corpus is `editor-core`'s (`crate::corpus`, every parametric
//! document through its own bump edit), plus the tour's gallery ring.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use std::collections::BTreeMap;
use std::sync::Arc;

use bvh::{Aabb, Ray};
use editor_core::{Dimension, DocEdit, Expr, ProfileDoc, RecipeNodeId, SlotId, unparse};
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::mesh::Mesh;
use viewer::evalseam::{IndexRequest, IndexService, InlineIndexer};
use viewer::pickindex::PickIndex;
use viewer::scene::DisplayTolerance;
use viewer::session::{DocSession, SessionOp};

use crate::common;
use crate::corpus;

/// Coarse on purpose: the rows are about reuse across edits, not about
/// mesh density, and the corpus has million-triangle documents at the
/// application's δ.
fn delta() -> DisplayTolerance {
    DisplayTolerance::new(2.0e-3).expect("a positive delta")
}

fn fnv(h: &mut u64, x: u64) {
    for b in x.to_le_bytes() {
        *h ^= u64::from(b);
        *h = h.wrapping_mul(0x0100_0000_01b3);
    }
}

fn fnv_str(h: &mut u64, s: &str) {
    fnv(h, s.len() as u64);
    for b in s.bytes() {
        fnv(h, u64::from(b));
    }
}

/// Every byte of a mesh value, the way `d9_mesh_goldens` digests one:
/// positions by bit pattern, patches (face key, triangles), boundaries
/// (edge key, polyline ids, endpoint vertex keys).
fn digest(m: &Mesh) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    fnv(&mut h, m.positions.len() as u64);
    for p in &m.positions {
        fnv(&mut h, p.x.to_bits());
        fnv(&mut h, p.y.to_bits());
        fnv(&mut h, p.z.to_bits());
    }
    fnv(&mut h, m.patches.len() as u64);
    for q in &m.patches {
        fnv_str(&mut h, &format!("{:?}", q.face));
        fnv(&mut h, q.triangles.len() as u64);
        for t in &q.triangles {
            fnv(&mut h, u64::from(t[0]));
            fnv(&mut h, u64::from(t[1]));
            fnv(&mut h, u64::from(t[2]));
        }
    }
    fnv(&mut h, m.boundaries.len() as u64);
    for b in &m.boundaries {
        fnv_str(&mut h, &format!("{:?}", b.edge));
        fnv(&mut h, b.points.len() as u64);
        for id in &b.points {
            fnv(&mut h, u64::from(*id));
        }
        fnv_str(&mut h, &format!("{:?}", b.start_vertex));
        fnv_str(&mut h, &format!("{:?}", b.end_vertex));
    }
    h
}

/// One edit as the session spells it.
#[derive(Clone, Debug)]
struct Edit {
    node: RecipeNodeId,
    slot: SlotId,
    text: String,
}

impl Edit {
    fn op(&self) -> SessionOp {
        SessionOp::SetSlotExpression {
            node: self.node,
            slot: self.slot,
            text: self.text.clone(),
        }
    }
}

/// The corpus document's own bump edit — every parametric corpus
/// document carries one — and the text that reverts it.
fn bump_of(c: &corpus::CorpusDoc) -> Option<(Edit, Edit)> {
    let DocEdit::SetParam { node, slot, expr } = c.bump.clone() else {
        return None;
    };
    let original = c.doc.node(node)?.expr(slot)?;
    Some((
        Edit {
            node,
            slot,
            text: unparse(&expr),
        },
        Edit {
            node,
            slot,
            text: unparse(original),
        },
    ))
}

/// A second parameter to change: the first literal length slot on a
/// node other than `not`, scaled — "change another", when the document
/// has another to change.
fn another_length_slot(doc: &ProfileDoc, not: RecipeNodeId) -> Option<Edit> {
    for &node in doc.order() {
        if node == not {
            continue;
        }
        let n = doc.node(node)?;
        for slot in n.slots() {
            let Some(expr) = n.expr(slot) else { continue };
            if expr.dim() != Dimension::Length {
                continue;
            }
            let Some(value) = expr.literal_value() else {
                continue;
            };
            if value == 0.0 {
                continue;
            }
            let scaled = Expr::literal(value * 1.015_625, Dimension::Length).ok()?;
            return Some(Edit {
                node,
                slot,
                text: unparse(&scaled),
            });
        }
    }
    None
}

/// The seam's answer for the session's landed run at `at`: the index,
/// or the refusal (a failed or poisoned root is an ordinary editing
/// state).
fn seam_index_at(
    seam: &mut InlineIndexer,
    session: &DocSession,
    at: DisplayTolerance,
) -> Result<PickIndex, viewer::pickindex::PickIndexError> {
    let (doc, _) = session.landed_pair().expect("a landed pair");
    let generation = session
        .landed_generation()
        .expect("a landed evaluation has a generation");
    seam.submit(IndexRequest {
        generation,
        delta: at,
        doc: doc.clone(),
        evaluation: Arc::clone(session.evaluation_arc().expect("a landed run")),
        tol: session.tol(),
    });
    let done = seam.poll().expect("the inline seam answers inside poll");
    assert_eq!(done.generation, generation);
    done.index
}

fn seam_index(
    seam: &mut InlineIndexer,
    session: &DocSession,
) -> Result<PickIndex, viewer::pickindex::PickIndexError> {
    seam_index_at(seam, session, delta())
}

/// The plain door's answer for the same run: the definition of the
/// picture.
fn fresh_index(session: &DocSession) -> Result<PickIndex, viewer::pickindex::PickIndexError> {
    let (doc, eval) = session.landed_pair().expect("a landed pair");
    let generation = session
        .landed_generation()
        .expect("a landed evaluation has a generation");
    PickIndex::build(doc, eval, generation, delta(), session.tol())
}

/// A fixed set of rays for the picture: the six axis rays through the
/// bounding box's centre and the eight corner-to-centre diagonals.
fn rays_for(index: &PickIndex) -> Vec<Ray> {
    let mut lo = Point3::new(f64::MAX, f64::MAX, f64::MAX);
    let mut hi = Point3::new(f64::MIN, f64::MIN, f64::MIN);
    for part in index.parts() {
        for p in &part.mesh().positions {
            lo = Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
            hi = Point3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
        }
    }
    let c = Point3::new(
        (lo.x + hi.x) * 0.5,
        (lo.y + hi.y) * 0.5,
        (lo.z + hi.z) * 0.5,
    );
    let ext = (hi - lo).norm().max(1e-3);
    let mut rays = Vec::new();
    for dir in [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
    ] {
        rays.push(Ray {
            origin: c - dir * (2.0 * ext),
            dir,
        });
    }
    let aabb = Aabb {
        min_x: lo.x,
        min_y: lo.y,
        min_z: lo.z,
        max_x: hi.x,
        max_y: hi.y,
        max_z: hi.z,
    };
    for corner in common::corners(&aabb) {
        let toward = c - corner;
        if toward.norm() > 0.0 {
            rays.push(Ray {
                origin: corner - toward,
                dir: toward,
            });
        }
    }
    rays
}

/// A pick's answer as comparable bits.
fn hits(index: &PickIndex, session: &DocSession, rays: &[Ray]) -> Vec<String> {
    let (_, eval) = session.landed_pair().expect("a landed pair");
    rays.iter()
        .map(|ray| match index.pick(eval, ray) {
            Ok(Some(hit)) => format!(
                "{:?}/{}/{}/{:016x}/{:016x}/{:016x}/{:016x}",
                hit.node,
                hit.body,
                hit.name,
                hit.t.to_bits(),
                hit.point.x.to_bits(),
                hit.point.y.to_bits(),
                hit.point.z.to_bits()
            ),
            Ok(None) => "miss".to_owned(),
            Err(e) => format!("refused: {e}"),
        })
        .collect()
}

/// The seam's answer against the plain door's: the same refusal, or
/// the same picture part by part. Answers the picture's face count.
fn assert_same_answer(
    name: &str,
    step: &str,
    seam: &Result<PickIndex, viewer::pickindex::PickIndexError>,
    fresh: &Result<PickIndex, viewer::pickindex::PickIndexError>,
    session: &DocSession,
) -> usize {
    match (seam, fresh) {
        (Ok(seam), Ok(fresh)) => {
            assert_same_picture(name, step, seam, fresh, session);
            faces_of(seam)
        }
        (Err(a), Err(b)) => {
            assert_eq!(
                format!("{a:?}"),
                format!("{b:?}"),
                "{name} after {step}: the seam refuses differently from the plain door"
            );
            0
        }
        (Ok(_), Err(e)) => {
            panic!("{name} after {step}: the plain door refuses ({e:?}) and the seam does not")
        }
        (Err(e), Ok(_)) => {
            panic!("{name} after {step}: the seam refuses ({e:?}) and the plain door does not")
        }
    }
}

/// The seam's picture against the plain door's, part by part.
fn assert_same_picture(
    name: &str,
    step: &str,
    seam: &PickIndex,
    fresh: &PickIndex,
    session: &DocSession,
) {
    assert_eq!(
        seam.parts().len(),
        fresh.parts().len(),
        "{name} after {step}: the seam draws a different number of bodies"
    );
    for (a, b) in seam.parts().iter().zip(fresh.parts()) {
        assert_eq!(
            (a.node(), a.body()),
            (b.node(), b.body()),
            "{name} after {step}: part order"
        );
        assert_eq!(
            digest(a.mesh()),
            digest(b.mesh()),
            "{name} after {step}: node {:?} body {} — the seam's mesh is not the fresh tessellation",
            a.node(),
            a.body()
        );
    }
    let rays = rays_for(fresh);
    assert_eq!(
        hits(seam, session, &rays),
        hits(fresh, session, &rays),
        "{name} after {step}: the seam's index answers different picks"
    );
}

/// Open → index; then each edit → land → index, asserting the seam's
/// picture is the plain door's after every landing. Answers the
/// per-step face counts of the picture, for the memo rows to read.
fn drive(name: &str, doc: ProfileDoc, edits: &[(&str, Edit)], tol: Tol) -> Vec<(String, usize)> {
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    assert!(session.evaluation().is_some(), "{name}: the document lands");
    let mut seam = InlineIndexer::new();
    let mut steps = Vec::new();
    let index = seam_index(&mut seam, &session);
    let fresh = fresh_index(&session);
    assert!(
        index.is_ok(),
        "{name}: the document indexes as opened: {index:?}"
    );
    let faces = assert_same_answer(name, "open", &index, &fresh, &session);
    steps.push(("open".to_owned(), faces));
    for (step, edit) in edits {
        let outcome = session.perform(edit.op());
        assert!(
            outcome.refusal.is_none(),
            "{name}: edit {step} refused: {:?}",
            outcome.refusal
        );
        session.pump();
        let index = seam_index(&mut seam, &session);
        let fresh = fresh_index(&session);
        let faces = assert_same_answer(name, step, &index, &fresh, &session);
        steps.push(((*step).to_owned(), faces));
    }
    steps
}

fn faces_of(index: &PickIndex) -> usize {
    index.parts().iter().map(|p| p.mesh().patches.len()).sum()
}

/// The edit sequence for a document with a bump: the bump, another
/// length slot where one exists, then the bump reverted.
fn sequence(doc: &ProfileDoc, bump: Edit, revert: Edit) -> Vec<(&'static str, Edit)> {
    let mut edits = vec![("the first edit", bump.clone())];
    if let Some(other) = another_length_slot(doc, bump.node) {
        edits.push(("the second edit", other));
    }
    edits.push(("the revert", revert));
    edits
}

#[test]
fn every_parametric_corpus_document_indexes_the_same_through_the_seam_across_edits() {
    let tol = Tol::witness();
    let mut seen = BTreeMap::new();
    for c in corpus::documents() {
        let Some((bump, revert)) = bump_of(&c) else {
            continue;
        };
        let edits = sequence(&c.doc, bump, revert);
        let steps = drive(c.name, c.doc.clone(), &edits, tol);
        seen.insert(c.name, steps);
    }
    assert!(
        seen.len() >= 8,
        "the corpus carries at least eight parametric documents; saw {:?}",
        seen.keys().collect::<Vec<_>>()
    );
}

#[test]
fn the_gallery_ring_indexes_the_same_through_the_seam_across_edits() {
    let tol = Tol::witness();
    let text = common::gallery_ring_at(tol);
    let loaded = pncad::document::load(&text, tol).expect("the gallery ring loads");
    let doc = loaded.snapshot;
    let (node, slot, expr) = first_length_slot(&doc);
    let original = doc
        .node(node)
        .expect("a node")
        .expr(slot)
        .expect("its slot");
    let bump = Edit {
        node,
        slot,
        text: unparse(&expr),
    };
    let revert = Edit {
        node,
        slot,
        text: unparse(original),
    };
    let edits = sequence(&doc, bump, revert);
    drive("gallery_ring", doc, &edits, tol);
}

/// The last extrude distance or revolve angle in the document, scaled
/// — the gallery ring's own bump.
fn first_length_slot(doc: &ProfileDoc) -> (RecipeNodeId, SlotId, Expr) {
    let env = doc.param_env::<f64>();
    for &node in doc.order().iter().rev() {
        match doc.node(node).expect("a node") {
            editor_core::Node::Extrude { distance, .. } => {
                let value = editor_core::eval(distance, &env).expect("a literal distance");
                let expr =
                    Expr::literal(value * 1.03125, Dimension::Length).expect("a length literal");
                return (node, SlotId::Distance, expr);
            }
            editor_core::Node::Revolve { angle, .. } => {
                let value = editor_core::eval(angle, &env).expect("a literal angle");
                let expr =
                    Expr::literal(value * 0.96875, Dimension::Angle).expect("an angle literal");
                return (node, SlotId::RevolveAngle, expr);
            }
            _ => {}
        }
    }
    panic!("no extrude or revolve in the document")
}
