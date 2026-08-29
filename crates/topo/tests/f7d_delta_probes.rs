//! DELTA review probes (ordinal 104 verification pass, PR #1131's
//! rebuilt mechanism): attacks on `merge_faces::
//! redundant_subdivision_vertex` and the kef→kev straight-seam repair.
//!
//! **ADOPTED** from the delta review's `verbs/f7d-probes`,
//! authorship-preserving. They were written as review-lane probes;
//! they ship because they are the mechanism's differential rows —
//! D1 makes the merge-side comparison RED-CAPABLE, where the
//! shipped `verbs_f7_collinear_seam` row only printed it.
//! Each probe names the trigger clause it attacks.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::prism_z;
use geom_core::{Point3, Tol};
use topo::{Body, MefSite, MevSite, validate_closed, validate_geometric};

/// A prism whose top face is split by a chord from (0,0) to (2,2)
/// through an interior vertex at `mid` (the PR's own split_top shape,
/// with the mid point free).
fn split_top_at(mid: Point3<f64>) -> Body<f64> {
    let p = prism_z::<f64>(&[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)], 0.0, 1.0);
    let mut b = p.body;
    let tol = Tol::witness();
    let he1 = he_at(&b, p.top_face, 0.0, 0.0);
    let strut = b
        .mev_line(MevSite::Fan { he1, he2: he1 }, mid, tol)
        .unwrap();
    let he2 = he_at(&b, p.top_face, 2.0, 2.0);
    b.mef_chord(
        MefSite::Chords {
            he1: strut.he_minus,
            he2,
        },
        tol,
    )
    .unwrap();
    assert_eq!(validate_closed(&b), Ok(()), "fixture is tier-2 legal");
    b
}

fn he_at(b: &Body<f64>, face: topo::FaceKey, x: f64, y: f64) -> topo::HalfEdgeKey {
    let outer = b.get_face(face).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = b.get_loop(outer).unwrap().boundary else {
        panic!("outer loop is a cycle")
    };
    b.loop_cycle(first)
        .unwrap()
        .into_iter()
        .find(|&he| {
            let v = b.get_half_edge(he).unwrap().start;
            let pt = b.get_point(b.get_vertex(v).unwrap().point).unwrap();
            (pt.x - x).abs() < 1e-12 && (pt.y - y).abs() < 1e-12
        })
        .expect("a half-edge starting there")
}

fn census(b: &Body<f64>) -> (usize, usize, usize) {
    (b.faces().count(), b.vertices().count(), b.edges().count())
}

/// D1 — the corpus differential, ASSERTED (the shipped
/// `verbs_f7_collinear_seam` row prints it): collinear ⇒ the repair
/// commits with the vertex gone and tier 2 + 3 green; bent ⇒ typed
/// refusal, body untouched.
#[test]
fn d1_collinear_merges_bent_refuses_asserted() {
    let tol = Tol::witness();

    // Collinear: mid ON the (0,0)-(2,2) diagonal.
    let mut b = split_top_at(Point3::new(1.0, 1.0, 1.0));
    let (f0, v0, e0) = census(&b);
    let out = b.merge_coplanar_faces(tol).expect("collinear seam repairs");
    assert_eq!(out.groups.len(), 1);
    assert!(out.groups[0].rings_made.is_empty(), "kev path, not kemr");
    assert_eq!(census(&b), (f0 - 1, v0 - 1, e0 - 2), "kef + kev arithmetic");
    assert_eq!(validate_closed(&b), Ok(()), "tier 2 after repair");
    assert_eq!(validate_geometric(&b, tol), Ok(()), "tier 3 after repair");

    // Bent: R1's P2 point, off the diagonal.
    let mut b = split_top_at(Point3::new(0.9, 0.6, 1.0));
    let before = census(&b);
    let err = b
        .merge_coplanar_faces(tol)
        .expect_err("a bent seam must not merge");
    println!("[d1] bent-seam refusal = {err:?}");
    assert_eq!(census(&b), before, "refusal leaves the body untouched");
}

/// D2 — the meter's three arms at the ε edge (ε = 1e-9, K = 10; the
/// perpendicular offset of the mid vertex from the diagonal is d/√2 and
/// the collinear margin is ~0.707·d):
/// well inside `zero` ⇒ repairs; inside the ambiguity band ⇒ the typed
/// `Escalated` refusal (the indeterminate arm is honest, not a guess);
/// beyond `escalate` ⇒ the bent-path refusal.
#[test]
fn d2_near_collinear_band_arms() {
    let tol = Tol::witness();

    // (a) d = 1e-10: margin ~7e-11 < ε — decidedly collinear in-band.
    let mut b = split_top_at(Point3::new(1.0, 1.0 + 1e-10, 1.0));
    let out = b.merge_coplanar_faces(tol);
    println!(
        "[d2a] d=1e-10 => {:?}",
        out.as_ref().map(|o| o.groups.len())
    );
    let out = out.expect("in-band deviation merges (locus change < eps)");
    assert_eq!(out.groups.len(), 1);
    assert_eq!(
        validate_geometric(&b, tol),
        Ok(()),
        "tier 3 after in-band repair"
    );

    // (b) d = 5e-9: margin ~3.5e-9 in (ε, 10ε) — the ambiguity band.
    let mut b = split_top_at(Point3::new(1.0, 1.0 + 5e-9, 1.0));
    let before = census(&b);
    let err = b
        .merge_coplanar_faces(tol)
        .expect_err("the ambiguity band must escalate typed, never guess");
    println!("[d2b] d=5e-9 => {err:?}");
    assert!(
        matches!(err, topo::MergeCoplanarError::Escalated { .. }),
        "expected the typed escalation — got {err:?}"
    );
    assert_eq!(census(&b), before, "escalation leaves the body untouched");

    // (c) d = 1e-3: decidedly bent.
    let mut b = split_top_at(Point3::new(1.0, 1.0 + 1e-3, 1.0));
    let before = census(&b);
    let err = b.merge_coplanar_faces(tol).expect_err("bent refuses");
    println!("[d2c] d=1e-3 => {err:?}");
    assert_eq!(census(&b), before);
}

/// D3 — a vertex the trigger must NOT license: the pole of a FOUR-sector
/// disc. Two of its four spokes are collinear+opposed pairs, but the
/// vertex carries two more edges — removal would be wrong. The
/// valence-2 clause must keep the trigger false and the merge must
/// refuse whole with the body untouched.
#[test]
fn d3_four_sector_pole_is_not_licensed() {
    let p = prism_z::<f64>(&[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)], 0.0, 1.0);
    let mut b = p.body;
    let tol = Tol::witness();
    let mid = Point3::new(1.0, 1.0, 1.0);
    // First diagonal through the centre: (0,0) -> M -> (2,2).
    let he1 = he_at(&b, p.top_face, 0.0, 0.0);
    let strut = b
        .mev_line(MevSite::Fan { he1, he2: he1 }, mid, tol)
        .unwrap();
    let he2 = he_at(&b, p.top_face, 2.0, 2.0);
    b.mef_chord(
        MefSite::Chords {
            he1: strut.he_minus,
            he2,
        },
        tol,
    )
    .unwrap();
    // Second diagonal, both halves into M: (2,0) -> M and M -> (0,2).
    // Each cut lives in whichever fragment holds both endpoints.
    let fragment_with = |b: &Body<f64>, x: f64, y: f64| {
        b.faces()
            .map(|(fk, _)| fk)
            .find(|&fk| cycle_has_corner(b, fk, 1.0, 1.0) && cycle_has_corner(b, fk, x, y))
    };
    let fk = fragment_with(&b, 2.0, 0.0).expect("a fragment holds both M and (2,0)");
    let he_corner = he_at(&b, fk, 2.0, 0.0);
    let m_in_face = he_at(&b, fk, 1.0, 1.0);
    b.mef_chord(
        MefSite::Chords {
            he1: he_corner,
            he2: m_in_face,
        },
        tol,
    )
    .unwrap();
    // And M -> (0,2) in the fragment holding both.
    let fk2 = fragment_with(&b, 0.0, 2.0).expect("a fragment holds both M and (0,2)");
    let he_m2 = he_at(&b, fk2, 1.0, 1.0);
    let he_c2 = he_at(&b, fk2, 0.0, 2.0);
    b.mef_chord(
        MefSite::Chords {
            he1: he_c2,
            he2: he_m2,
        },
        tol,
    )
    .unwrap();
    assert_eq!(
        validate_closed(&b),
        Ok(()),
        "four-sector fixture is tier-2 legal"
    );
    // The pole M is valence 4 with two collinear+opposed spoke pairs.
    let before = census(&b);
    let res = b.merge_coplanar_faces(tol);
    println!("[d3] four-sector merge => {res:?}");
    let err = res.expect_err("a valence-4 pole must not be repaired away");
    println!("[d3] refusal = {err:?}");
    assert_eq!(census(&b), before, "refusal leaves the body untouched");
}

fn cycle_has_corner(b: &Body<f64>, fk: topo::FaceKey, x: f64, y: f64) -> bool {
    let outer = b.get_face(fk).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = b.get_loop(outer).unwrap().boundary else {
        return false;
    };
    let Some(cycle) = b.loop_cycle(first) else {
        return false;
    };
    cycle.iter().any(|&he| {
        let v = b.get_half_edge(he).unwrap().start;
        let pt = b.get_point(b.get_vertex(v).unwrap().point).unwrap();
        (pt.x - x).abs() < 1e-12 && (pt.y - y).abs() < 1e-12
    })
}

/// D4 — the OPPOSED clause's guard, attacked with a zero-width bigon:
/// two edges between the same two vertices along one line. Departures
/// at both junction vertices are parallel and SAME-signed, so the
/// opposed decide must answer Positive and the trigger must stay
/// false — removal here would delete a boundary vertex of a
/// (degenerate) face. Expect: no kev repair; whatever the merge does,
/// it must not commit a body that lost the junction vertices silently.
#[test]
fn d4_bigon_same_direction_is_not_opposed() {
    let p = prism_z::<f64>(&[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)], 0.0, 1.0);
    let mut b = p.body;
    let tol = Tol::witness();
    let mid = Point3::new(1.0, 1.0, 1.0);
    let he1 = he_at(&b, p.top_face, 0.0, 0.0);
    let strut = b
        .mev_line(MevSite::Fan { he1, he2: he1 }, mid, tol)
        .unwrap();
    // Close a bigon: a second edge M -> (0,0) beside the strut.
    let he_back = he_at(&b, p.top_face, 0.0, 0.0);
    let res = b.mef_chord(
        MefSite::Chords {
            he1: strut.he_minus,
            he2: he_back,
        },
        tol,
    );
    println!("[d4] bigon mef => {:?}", res.as_ref().map(|_| "ok"));
    let Ok(_) = res else {
        println!("[d4] the bigon is not constructible through mef_chord — attack void");
        return;
    };
    let closed = validate_closed(&b);
    println!("[d4] bigon tier2 = {closed:?}");
    if closed.is_err() {
        println!("[d4] bigon is not tier-2 legal — the merge's input gate excludes it");
        return;
    }
    let before = census(&b);
    let out = b.merge_coplanar_faces(tol);
    println!("[d4] bigon merge => {out:?}");
    match out {
        Ok(o) => {
            // If it merged, the repair path must not have run (no kev):
            // the junction vertices are boundary, not interior.
            assert_eq!(
                b.vertices().count(),
                before.1,
                "no vertex may vanish from a same-direction bigon"
            );
            assert_eq!(validate_closed(&b), Ok(()));
            println!("[d4] merged without vertex loss: {o:?}");
        }
        Err(e) => {
            assert_eq!(census(&b), before, "refusal leaves the body untouched");
            println!("[d4] refused: {e:?}");
        }
    }
}
