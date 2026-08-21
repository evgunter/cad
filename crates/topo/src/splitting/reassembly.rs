//! The **reassembly oracle** (test-only): split, then re-glue the two
//! sides and verify the surgery is undone — census, per-shell
//! Euler–Poincaré, and mass properties equal to the reference.
//!
//! Runs at the pre-carve seam (the finish product before the two
//! bodies are carved): cross-BODY combination is PR 5's ratified
//! boundary (`kfmrh`'s `CrossSolid` refusal — the boolean combine owns
//! it), so the oracle glues the two sides while they still share one
//! solid — which is exactly the surgery whose invertibility the oracle
//! certifies. Reported as a deviation in the PR writeup.
//!
//! The reference for census equality is the operand **with the same
//! crossing vertices inserted** (reduction's `split_edge` residue):
//! inverting an edge split needs a `join_edge` op (kill a degree-2
//! vertex, re-certify the merged span) that does not exist and is out
//! of PR 3 scope. That crossing-mode reference is the ONLY mode this
//! oracle implements: a pristine-operand comparison for sections that
//! run entirely through existing vertices/edges is not built (future
//! work if such fixtures join the net).
//!
//! Scope, plainly: `carve` is OUTSIDE this oracle's net — the oracle
//! stops at the pre-carve seam, so the carve step is never round-trip
//! certified here. Compensating coverage: the PR 3 review's
//! referential-integrity audit of the carved bodies, plus the
//! acceptance tests' per-body censuses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point3, Tol, Vec3};

use super::SplitPlane;
use super::split_scratch;
use crate::body::Body;
use crate::entity::{FaceKey, LoopBoundary};
use crate::euler::{FaceSurface, MefSite, MevSite};
use crate::euler_ring::MekrSite;
use crate::props::mass_properties;
use crate::validate::{validate, validate_closed};
use geom_brep::EdgeCurveSpec;

/// A geometric quad prism (profile in x–y, extruded +z; the
/// tests/common builder's minimal in-crate copy): planar Newell
/// surfaces, certified chord-line carriers.
pub(crate) fn quad_prism(profile: &[(f64, f64); 4], height: f64, tol: Tol) -> Body<f64> {
    let c = |&(x, y): &(f64, f64), z: f64| Point3::new(x, y, z);
    let bot: Vec<Point3<f64>> = profile.iter().map(|p| c(p, 0.0)).collect();
    let top: Vec<Point3<f64>> = profile.iter().map(|p| c(p, height)).collect();
    let n = 4;
    let line = EdgeCurveSpec::line_between;
    let plane = |corners: &[Point3<f64>]| {
        geom_brep::newell_plane(corners, Band::linear(tol).unwrap()).unwrap()
    };
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(bot[0]).unwrap();
    let mut chain = Vec::new();
    chain.push(
        body.mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            bot[1],
            line(bot[0], bot[1]),
            tol,
        )
        .unwrap(),
    );
    for i in 2..n {
        let at = chain[i - 2].he_minus;
        chain.push(
            body.mev(
                MevSite::Fan { he1: at, he2: at },
                bot[i],
                line(bot[i - 1], bot[i]),
                tol,
            )
            .unwrap(),
        );
    }
    let bottom: Vec<_> = core::iter::once(seed.vertex)
        .chain(chain.iter().map(|m| m.vertex))
        .collect();
    let he_last = body
        .find_half_edge(seed.face, bottom[n - 1], bottom[n - 2])
        .unwrap();
    let rev: Vec<Point3<f64>> = core::iter::once(bot[0])
        .chain(bot[1..].iter().rev().copied())
        .collect();
    let f_bottom = body
        .mef(
            MefSite::Chords {
                he1: he_last,
                he2: chain[0].he_plus,
            },
            line(bot[n - 1], bot[0]),
            FaceSurface::New(plane(&rev)),
            tol,
        )
        .unwrap();
    let mut struts = Vec::new();
    for i in 0..n {
        let at = if i == 0 {
            chain[0].he_plus
        } else if i < n - 1 {
            chain[i].he_plus
        } else {
            f_bottom.he_plus
        };
        struts.push(
            body.mev(
                MevSite::Fan { he1: at, he2: at },
                top[i],
                line(bot[i], top[i]),
                tol,
            )
            .unwrap(),
        );
    }
    let mut first_side_he_plus = None;
    for i in 0..n {
        let j = (i + 1) % n;
        let he2 = if i < n - 1 {
            struts[j].he_minus
        } else {
            first_side_he_plus.unwrap()
        };
        let f = body
            .mef(
                MefSite::Chords {
                    he1: struts[i].he_minus,
                    he2,
                },
                line(top[i], top[j]),
                FaceSurface::New(plane(&[bot[i], bot[j], top[j], top[i]])),
                tol,
            )
            .unwrap();
        if i == 0 {
            first_side_he_plus = Some(f.he_plus);
        }
    }
    body.set_face_surface(seed.face, FaceSurface::New(plane(&top)))
        .unwrap();
    body
}

/// Re-glues one section pair: `kfmrh(below_face, above_face)` (same
/// solid, cross-shell fusion), then the loopglue zip — per coincident
/// vertex pair a scaffolding `mekr`/`mef` + `kev`, per doubled edge a
/// `kef` — the ch. 12 machinery's ch. 14 call site.
fn reglue_pair<T: geom_core::Decide>(
    body: &mut Body<T>,
    below_face: FaceKey,
    above_face: FaceKey,
    tol: Tol,
) {
    let fused = body.kfmrh(below_face, above_face).unwrap();
    let ring = fused.ring; // the above loop, now a ring of below_face
    let outer = body.get_face(below_face).unwrap().outer;

    // Align the two antiparallel coincident cycles: for the outer's
    // anchor vertex, find the ring half-edge STARTING at the
    // bit-identical point whose mate-cycle direction is antiparallel:
    // outer he b_i→b_{i+1} pairs with ring he a_{i+1}→a_i.
    let LoopBoundary::Cycle { first: outer_first } = body.get_loop(outer).unwrap().boundary else {
        panic!("section outer loop is a cycle");
    };
    let LoopBoundary::Cycle { first: ring_first } = body.get_loop(ring).unwrap().boundary else {
        panic!("section ring loop is a cycle");
    };
    let outer_cycle = body.loop_cycle(outer_first).unwrap();
    let ring_cycle = body.loop_cycle(ring_first).unwrap();
    let n = outer_cycle.len();
    assert_eq!(ring_cycle.len(), n, "coincident cycles have equal length");
    let point_of = |body: &Body<T>, he| {
        let v = body.get_half_edge(he).unwrap().start;
        *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
    };
    let bits = |p: Point3<T>| format!("{p:?}");

    // `ob[j]` = outer half starting at b_j (b_j → b_{j+1});
    // `rs[j]` = ring half starting at a_j (a_j → a_{j-1}) — the
    // antiparallel coincident cycle, aligned by bit-equal points.
    let ob: Vec<_> = outer_cycle;
    let mut rs = Vec::with_capacity(n);
    for &b_he in &ob {
        let b_bits = bits(point_of(body, b_he));
        let matched = ring_cycle
            .iter()
            .copied()
            .find(|&rhe| bits(point_of(body, rhe)) == b_bits)
            .expect("each below corner has a coincident above copy");
        rs.push(matched);
    }

    // The zip, derived from OUR op semantics (see the trace in the PR
    // writeup): pair 0 via mekr (kills the ring loop) + kev; pairs
    // n−1 … 1 via mef + kev + kef(rs[j+1 mod n]); final kef(rs[1]).
    let self_loop = |body: &Body<T>, he| EdgeCurveSpec::self_loop_circle_at(point_of(body, he));
    let n0 = body
        .mekr(
            MekrSite::Cycles {
                target: ob[0],
                ring: rs[0],
            },
            self_loop(body, ob[0]),
            tol,
        )
        .unwrap();
    body.kev(n0.he_plus).unwrap();
    for j in (1..n).rev() {
        let nj = body
            .mef(
                MefSite::Chords {
                    he1: ob[j],
                    he2: rs[j],
                },
                self_loop(body, ob[j]),
                FaceSurface::Inherit,
                tol,
            )
            .unwrap();
        body.kev(nj.he_plus).unwrap();
        body.kef(rs[(j + 1) % n]).unwrap();
    }
    body.kef(rs[1 % n]).unwrap();
}

/// The generic-plane oracle on a cube: split at y = 0.5, re-glue at
/// the pre-carve seam, merge the same-key coplanar pairs, and compare
/// against the operand-with-crossings reference.
#[test]
fn reassembly_oracle_generic_cube() {
    let tol = Tol::witness();
    let operand = quad_prism(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 1.0, tol);
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.5, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
    };
    let (red, completed, _fragments) = split_scratch(&operand, &plane).unwrap();
    assert_eq!(completed.len(), 1);
    let mut body = red.body;

    // Finish through promotion + distribution (the pre-carve state):
    // mirror split_finish's promotion inline via the public pieces.
    let section = completed[0];
    let face_data = body.get_face(section.face).unwrap();
    let ring = if face_data.outer == section.above_loop {
        section.below_loop
    } else {
        section.above_loop
    };
    let promoted = body.mfkrh(ring, FaceSurface::Inherit).unwrap();
    let (above_face, below_face) = if ring == section.above_loop {
        (promoted.face, section.face)
    } else {
        (section.face, promoted.face)
    };
    body.clear_null_face_pair(section.face);
    let shells: Vec<_> = body.shells().map(|(k, _)| k).collect();
    for shell in shells {
        body.movefac(shell).unwrap();
    }
    assert_eq!(body.shells().count(), 2);
    assert_eq!(validate(&body), Ok(()));

    // Re-glue and compare.
    reglue_pair(&mut body, below_face, above_face, tol);
    assert_eq!(validate(&body), Ok(()));
    body.merge_coplanar_faces(tol).unwrap();
    assert_eq!(validate_closed(&body), Ok(()));

    // Reference: operand + the same crossing insertions only.
    let reference = {
        let band = geom_core::Band::linear(tol).unwrap();
        let mut b = operand.clone();
        let (mut sides, mut on) = super::classify::classify_vertices(&b, &plane, band).unwrap();
        super::classify::insert_crossings(&mut b, &plane, &mut sides, &mut on, tol).unwrap();
        b
    };
    assert_eq!(
        body.arena_counts(),
        reference.arena_counts(),
        "census equality"
    );
    let m1 = mass_properties(&body, tol).unwrap();
    let m0 = mass_properties(&operand, tol).unwrap();
    let close = |a: f64, b: f64| (a - b).abs() <= 1e-12 * b.abs().max(1.0);
    assert!(
        close(m1.volume, m0.volume),
        "volume restored: {} vs {}",
        m1.volume,
        m0.volume
    );
    assert!(close(m1.surface_area, m0.surface_area), "area restored");
}
