//! Review probes for PR 4982 (a union fold step mints the face–face row
//! of a given tangency). Each probe prints what the union answered; the
//! asserting rows are the ones whose expectation is independent of the
//! PR (volume, solids, ff count by geometry).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, failure, run};
use crate::fixture::{self, insert, len, union_over};
use editor_core::{
    BooleanValue, ExtrudeSide, LoopProgram, Node, ProfileDoc, ProfileProgram, RecipeNodeId,
    ValuePayload,
};
use geom_core::Tol;

fn rod(
    doc: ProfileDoc,
    (cx, cy): (f64, f64),
    r: f64,
    z0: f64,
    h: f64,
    phase: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, plane) = insert(
        doc,
        fixture::frame([0.0, 0.0, z0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            frame: plane.into(),
            loops: vec![LoopProgram::circle_split(cx, cy, r, 2, phase).expect("a finite circle")],
            ids: Vec::new(),
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(h),
            side: ExtrudeSide::Along,
        },
    )
}

#[derive(Debug)]
struct Outcome {
    ff: usize,
    ee: usize,
    ef: usize,
    solids: usize,
    volume: Option<f64>,
    ff_cites_pass_tangent: Vec<bool>,
}

fn outcome(doc: &ProfileDoc, members: &[RecipeNodeId]) -> Result<Outcome, String> {
    let (doc, u) = union_over(doc.clone(), members);
    let ev = run(&doc);
    if let Some(e) = failure(&ev, u) {
        return Err(format!("{e:?}"));
    }
    let value = ev.value(u).expect("the union evaluated");
    let ValuePayload::Boolean(BooleanValue::Body { body, contacts, .. }) = &value.payload else {
        return Err("not a body".into());
    };
    let mut o = Outcome {
        ff: 0,
        ee: 0,
        ef: 0,
        solids: body.solids().count(),
        volume: topo::mass_properties(body, Tol::witness())
            .ok()
            .map(|m| m.volume),
        ff_cites_pass_tangent: Vec::new(),
    };
    for c in contacts.rows() {
        match c.record.kind() {
            topo::ContactKind::FaceFace(..) => {
                o.ff += 1;
                let ok = c.cites.iter().all(|b| match b {
                    topo::Backing::Decided(k) => {
                        value.coincidences.get(k as usize).is_some_and(|row| {
                            row.relation == topo::Relation::SurfaceTangent
                                && row.site == topo::DecisionSite::TangentWitness
                        })
                    }
                    _ => false,
                }) && c.cites.iter().count() > 0;
                o.ff_cites_pass_tangent.push(ok);
            }
            topo::ContactKind::EdgeEdge(..) => o.ee += 1,
            topo::ContactKind::EdgeFace(..) => o.ef += 1,
            _ => {}
        }
    }
    Ok(o)
}

fn names(n: &[(&str, RecipeNodeId)], order: &[usize]) -> (String, Vec<RecipeNodeId>) {
    (
        order.iter().map(|&i| n[i].0).collect::<Vec<_>>().join(","),
        order.iter().map(|&i| n[i].1).collect(),
    )
}

fn sweep(label: &str, doc: &ProfileDoc, n: &[(&str, RecipeNodeId)], orders: &[&[usize]]) {
    for order in orders {
        let (what, members) = names(n, order);
        eprintln!("PROBE {label} [{what}] => {:?}", outcome(doc, &members));
    }
}

fn base() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("rev4982", Tol::witness());
    let (doc, plate) = block(doc, (-2.0, 2.0), (-2.0, 2.0), 0.0, 1.0);
    let (doc, r) = rod(doc, (0.0, 2.5), 0.5, 0.0, 1.0, 0.0);
    (doc, plate, r)
}

#[test]
fn probe_two_members() {
    let (doc, p, r) = base();
    sweep("two", &doc, &[("P", p), ("R", r)], &[&[0, 1], &[1, 0]]);
}

#[test]
fn probe_three_and_four_far() {
    let (doc, p, r) = base();
    let (doc, b) = block(doc, (5.0, 6.0), (5.0, 6.0), 0.0, 1.0);
    let (doc, b2) = block(doc, (-6.0, -5.0), (5.0, 6.0), 0.0, 1.0);
    let n = [("P", p), ("R", r), ("B", b), ("B2", b2)];
    sweep(
        "far",
        &doc,
        &n,
        &[
            &[1, 0, 2],
            &[0, 2, 1],
            &[2, 1, 0],
            &[0, 1, 2, 3],
            &[2, 3, 0, 1],
            &[1, 2, 3, 0],
            &[0, 2, 3, 1],
            &[2, 0, 3, 1],
        ],
    );
}

/// D sits flush on the plate's y = 2 face away from the line (a Rest
/// pair with P), D2 crosses that face top to bottom, splitting it.
#[test]
fn probe_dividers() {
    let (doc, p, r) = base();
    let (doc, d) = block(doc, (1.0, 1.8), (2.0, 2.6), 0.0, 1.0);
    let (doc, d2) = block(doc, (1.0, 1.5), (1.5, 2.5), -0.5, 2.0);
    let n = [("P", p), ("R", r), ("D", d), ("D2", d2)];
    sweep(
        "divide",
        &doc,
        &n,
        &[
            &[0, 2, 1],
            &[2, 0, 1],
            &[1, 2, 0],
            &[0, 1, 2],
            &[0, 3, 1],
            &[3, 1, 0],
            &[1, 0, 3],
            &[3, 0, 1],
            &[0, 2, 3, 1],
        ],
    );
}

/// Q covers the line's upper half (z 0.5..1), Q2 swallows the rod.
/// Volume oracle with Q: 16 + π/4 + 3.4 − 0.5 − π/8.
#[test]
fn probe_removers() {
    let (doc, p, r) = base();
    let (doc, q) = block(doc, (-1.0, 1.0), (1.5, 3.2), 0.5, 1.0);
    let (doc, q2) = block(doc, (-1.0, 1.0), (1.5, 3.2), -0.5, 2.0);
    eprintln!(
        "PROBE remove oracle Q volume = {}",
        16.0 + std::f64::consts::FRAC_PI_4 + 3.4 - 0.5 - std::f64::consts::FRAC_PI_8
    );
    let n = [("P", p), ("R", r), ("Q", q), ("Q2", q2)];
    sweep(
        "remove",
        &doc,
        &n,
        &[
            &[0, 2, 1],
            &[2, 0, 1],
            &[1, 2, 0],
            &[0, 1, 2],
            &[0, 3, 1],
            &[1, 0, 3],
        ],
    );
}

/// Two touches: R on y = 2, R2 on x = 2 (vertices off the ruling); and
/// a rod between two plates (one wall face each side).
#[test]
fn probe_two_touches() {
    let (doc, p, r) = base();
    let (doc, r2) = rod(doc, (2.5, 0.0), 0.5, 0.0, 1.0, std::f64::consts::FRAC_PI_2);
    let (doc, p2) = block(doc, (-2.0, 2.0), (3.0, 7.0), 0.0, 1.0);
    let n = [("P", p), ("R", r), ("R2", r2), ("P2", p2)];
    sweep(
        "twotouch",
        &doc,
        &n,
        &[
            &[0, 1, 2],
            &[1, 2, 0],
            &[1, 0, 2],
            &[2, 1, 0],
            &[0, 3, 1],
            &[0, 1, 3],
            &[1, 0, 3],
            &[3, 1, 0],
        ],
    );
}

/// Two flush plates whose shared y = 2 face merges, the rod tangent to
/// the merged face across the seam's one side (claim 4's path: two pass
/// verdicts on one merged face).
#[test]
fn probe_merged_face_two_verdicts() {
    let doc = ProfileDoc::empty_derived("rev4982m", Tol::witness());
    let (doc, p1) = block(doc, (-2.0, 0.0), (-2.0, 2.0), 0.0, 1.0);
    let (doc, p2) = block(doc, (0.0, 2.0), (-2.0, 2.0), 0.0, 1.0);
    let (doc, r) = rod(doc, (0.25, 2.5), 0.5, 0.0, 1.0, 0.0);
    let n = [("P1", p1), ("P2", p2), ("R", r)];
    sweep(
        "merged",
        &doc,
        &n,
        &[&[0, 1, 2], &[1, 0, 2], &[2, 0, 1], &[0, 2, 1]],
    );
}

/// A tilted slab crossing the rod and the plate across the line's upper
/// part: the rod wall's pieces carry ellipse trims at the step that
/// reads the line, which the pass (reading the whole rod) never met.
#[test]
fn probe_tilted_slab() {
    let (doc, p, r) = base();
    let (doc, prof) = crate::fixture::on_frame(
        doc,
        [-1.0, 2.0, 0.7],
        [1.0, 0.0, 0.0],
        [0.0, 0.8, 0.6],
        vec![vec![(0.0, -1.0), (2.0, -1.0), (2.0, 1.5), (0.0, 1.5)]],
    );
    let (doc, s) = insert(
        doc,
        Node::Extrude {
            profile: prof.into(),
            distance: len(0.2),
            side: ExtrudeSide::Along,
        },
    );
    let n = [("P", p), ("R", r), ("S", s)];
    sweep(
        "tilt",
        &doc,
        &n,
        &[&[0, 1, 2], &[0, 2, 1], &[2, 0, 1], &[1, 2, 0], &[2, 1, 0]],
    );
}

/// Claim 2: a rod whose axis leans toward the plate face by θ = 1e-10
/// (tangent within the band at the pass's reach), and a long flush
/// neighbour N whose y = 2 face merges with the plate's before the rod
/// is folded: the step reads the line at the merged face's reach.
#[test]
fn probe_reach_flip() {
    let doc = ProfileDoc::empty_derived("rev4982r", Tol::witness());
    let (doc, p) = block(doc, (-2.0, 2.0), (-2.0, 2.0), 0.0, 1.0);
    let (doc, n) = block(doc, (2.0, 300.0), (-2.0, 2.0), 0.0, 1.0);
    let th: f64 = 1e-10;
    let (doc, plane) = insert(
        doc,
        fixture::frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, th.cos(), th.sin()]),
    );
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            frame: plane.into(),
            loops: vec![LoopProgram::circle_split(0.0, 2.5, 0.5, 2, 0.0).expect("a finite circle")],
            ids: Vec::new(),
        }),
    );
    let (doc, r) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let names = [("P", p), ("N", n), ("R", r)];
    sweep(
        "reachflip",
        &doc,
        &names,
        &[
            &[0, 2],
            &[0, 1],
            &[0, 1, 2],
            &[1, 0, 2],
            &[2, 0, 1],
            &[0, 2, 1],
        ],
    );
}
