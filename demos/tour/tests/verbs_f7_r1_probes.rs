//! **F7 pole-exemption R1 review probes (ordinal 104, PR #1131)** —
//! the revolve-side rows: my own fixtures, not the PR's.
//!
//! Not part of the PR under review; lives on the probe branch only.
//!
//! - T1: a plain cylinder revolve (axis-touching caps, NO ledges) is
//!   the pure pole-split body — it must now pass the F7 gate.
//! - T2/T3: the teapot's sharp vessel and its shipped cup carry the
//!   OTHER #1031 defect (half B); with the exemption in force they
//!   must STILL refuse `NonMaximalFaces`, and the probe dumps the
//!   refused pair (edge, faces, endpoint valences, plane) to compare
//!   against issue #1031's F7DUMP.
//! - T4: the authorized merge run, reproduced: `merge_coplanar_faces`
//!   on the cup refuses `MergedFaceRoleAmbiguous`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::authoring::{p2, validated};
use pncad::geom::Surface;
use pncad::geom_core::{Point2, Tol, Vec2};
use pncad::prelude::{Open, Start};
use pncad::profile::{ProfileLoop, SketchPlane};
use pncad::sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use pncad::topo::{Body, BooleanError, EdgeKey, FaceKey};

const FIT_TOL: f64 = 1e-6;

fn band(tol: Tol) -> pncad::geom_core::Band {
    pncad::geom_core::Band::linear(tol).expect("band")
}

fn revolved(lp: ProfileLoop<f64>, tol: Tol) -> Body<f64> {
    revolve(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("meridian validates"),
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("meridian revolves")
    .body
}

/// A brick nowhere near any revolve here, so the pair-scoped operand
/// gate has nothing to say and the F7 gate's verdict is the signal.
fn distant_brick(tol: Tol) -> Body<f64> {
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(50.0, 50.0))
        .line_to(Point2::new(51.0, 50.0), tol)
        .expect("side")
        .line_to(Point2::new(51.0, 51.0), tol)
        .expect("side")
        .line_to(Point2::new(50.0, 51.0), tol)
        .expect("side")
        .line_to(Start, tol)
        .expect("close")
        .into();
    extrude(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("footprint validates"),
        Extrusion::Distance(1.0),
        tol,
    )
    .expect("footprint extrudes")
    .body
}

/// Every same-surface-key PLANAR adjacent pair in `body`, dumped the
/// way #1031's F7DUMP is: edge, faces, endpoint valences, and whether
/// either endpoint is a valence-2 vertex both of whose edges separate
/// the same pair (the exemption's own predicate, recomputed here
/// independently of `reduce.rs`).
fn dump_planar_same_key_pairs(tag: &str, body: &Body<f64>) -> Vec<(EdgeKey, bool)> {
    let mut rows = Vec::new();
    for (ek, edge) in body.edges() {
        let face_of = |he| {
            let parent = body.get_half_edge(he)?.parent_loop;
            Some(body.get_loop(parent)?.face)
        };
        let (Some(f1), Some(f2)) = (face_of(edge.he_plus), face_of(edge.he_minus)) else {
            continue;
        };
        if f1 == f2 {
            continue;
        }
        let (k1, k2) = (
            body.get_face(f1).map(|f| f.surface),
            body.get_face(f2).map(|f| f.surface),
        );
        if k1.is_none() || k1 != k2 {
            continue;
        }
        let surf = k1.and_then(|k| body.get_surface(k));
        let Some(Surface::Plane { origin, normal, .. }) = surf else {
            continue;
        };
        let pair = |ek2: EdgeKey| -> Option<(FaceKey, FaceKey)> {
            let e = body.get_edge(ek2)?;
            let (x, y) = (face_of(e.he_plus)?, face_of(e.he_minus)?);
            Some(if x <= y { (x, y) } else { (y, x) })
        };
        let want = pair(ek).unwrap();
        let mut exempt = false;
        for he in [edge.he_plus, edge.he_minus] {
            let start = body.get_half_edge(he).unwrap().start;
            let orbit = body.vertex_orbit(he).unwrap();
            let ok = orbit.len() == 2
                && orbit.iter().all(|h| {
                    body.get_half_edge(*h)
                        .and_then(|x| pair(x.edge))
                        .is_some_and(|p| p == want)
                });
            println!(
                "[{tag}] edge {ek:?} pair {f1:?}/{f2:?} endpoint {start:?} \
                 valence {} same-pair-valence-2: {ok}",
                orbit.len()
            );
            exempt |= ok;
        }
        println!(
            "[{tag}] edge {ek:?} plane o=({}, {}, {}) n=({}, {}, {}) exempt={exempt}",
            origin.x, origin.y, origin.z, normal.x, normal.y, normal.z
        );
        rows.push((ek, exempt));
    }
    rows
}

/// **T1: the pure pole-split revolve passes the gate.** A cylinder
/// profile touching the axis at both caps: the only planar same-key
/// pairs are the two caps' meridian pairs, every such edge has a
/// valence-2 pole endpoint, and the disjoint union now succeeds where
/// the old rule refused the operand outright.
#[test]
fn t1_axis_touching_cylinder_now_passes_the_f7_gate() {
    let tol = Tol::witness();
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(0.12, 0.0), tol)
        .expect("base")
        .line_to(Point2::new(0.12, 0.3), tol)
        .expect("wall")
        .line_to(Point2::new(0.0, 0.3), tol)
        .expect("top")
        .line_to(Start, tol)
        .expect("axis")
        .into();
    let cyl = revolved(lp, tol);
    let rows = dump_planar_same_key_pairs("t1", &cyl);
    assert!(
        !rows.is_empty() && rows.iter().all(|(_, exempt)| *exempt),
        "every planar same-key edge of the pure revolve is pole-exempt: {rows:?}"
    );
    let result = pncad::topo::union(&cyl, &distant_brick(tol), tol);
    match &result {
        Ok(_) => println!("[t1] union(cyl, distant) = Ok"),
        Err(e) => {
            println!("[t1] union(cyl, distant) refused: {e:?}");
            assert!(
                !matches!(e, BooleanError::NonMaximalFaces { .. }),
                "the pole-split cylinder must no longer refuse F7 — got {e:?}"
            );
        }
    }
}

/// **T5 (the deviation's load-bearing premise, attacked): a one-face
/// axis-touching cap IS reachable from the two-band revolve output,
/// by two PUBLIC euler ops.** The PR argues "every single-face route
/// removes a pole meridian: one → valence 1, banned; both → an
/// isolated interior vertex, `MergedFaceRoleAmbiguous`" — and
/// concludes "(b) has nothing to mint and (a) has nothing to repair
/// ... a shape nothing can build". But tier 2 binds the FINISHED
/// body, not the construction (box_with_hole's own recipe passes
/// through struts), and the missing route is `kef` (merge the cap
/// pair by killing ONE meridian; the other becomes a strut into the
/// pole) followed by `kev` (kill the strut AND the pole vertex).
/// The end state — a disk cap bounded only by its rim, no interior
/// vertex — is exactly the cap every EXTRUSION in this repo already
/// ships (the lily foot's caps), so it is a legal, buildable form.
/// This probe performs the two calls on a revolve's own output and
/// validates the result closed.
#[test]
fn t5_kef_kev_reaches_the_one_face_cap_the_pr_calls_unbuildable() {
    let tol = Tol::witness();
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(0.12, 0.0), tol)
        .expect("base")
        .line_to(Point2::new(0.12, 0.3), tol)
        .expect("wall")
        .line_to(Point2::new(0.0, 0.3), tol)
        .expect("top")
        .line_to(Start, tol)
        .expect("axis")
        .into();
    let mut cyl = revolved(lp, tol);
    // One cap's meridian pair, from the dump: pick the first planar
    // same-key edge and kill it with kef (merging the two half-disks).
    let rows = dump_planar_same_key_pairs("t5-before", &cyl);
    let (first_edge, _) = rows[0];
    let he = cyl.get_edge(first_edge).expect("edge resolves").he_plus;
    cyl.kef(he).expect("kef merges the cap pair");
    // The pole is now a strut tip: valence 1. Find it and kill the
    // strut with kev (whose killed vertex is the FAR end of the he we
    // pass, so pass the half-edge pointing rim -> pole).
    let pole_he = {
        let mut found = None;
        for (_, v) in cyl.vertices() {
            let Some(em) = v.emanating else { continue };
            let Some(orbit) = cyl.vertex_orbit(em) else {
                continue;
            };
            if orbit.len() == 1 {
                // `em` starts at the pole; its mate points rim->pole.
                let edge = cyl.get_half_edge(em).expect("he resolves").edge;
                let e = cyl.get_edge(edge).expect("edge resolves");
                let mate = if e.he_plus == em {
                    e.he_minus
                } else {
                    e.he_plus
                };
                found = Some(mate);
                break;
            }
        }
        found.expect("after kef, one cap meridian is a strut into the pole")
    };
    cyl.kev(pole_he).expect("kev kills the strut and the pole");
    assert_eq!(
        pncad::topo::validate_closed(&cyl),
        Ok(()),
        "the one-face cap body is tier-2 legal — the form the PR calls \
         structurally banned"
    );
    let rows_after = dump_planar_same_key_pairs("t5-after", &cyl);
    println!(
        "[t5] planar same-key pairs: before = {}, after = {} (the repaired cap \
         is ONE face)",
        rows.len(),
        rows_after.len()
    );
    let result = pncad::topo::union(&cyl, &distant_brick(tol), tol);
    match &result {
        Ok(_) => println!("[t5] union(repaired, distant) = Ok"),
        Err(e) => println!("[t5] union(repaired, distant) refused: {e:?}"),
    }
}

/// The teapot's SHARP vessel, byte-for-byte the scene's meridian
/// (constants copied from `demos/tour/src/teapot.rs`; a probe cannot
/// import a binary's modules).
fn sharp_vessel(tol: Tol) -> Body<f64> {
    const R_FOOT: f64 = 3.0 / 64.0;
    const R_BELLY: f64 = 5.0 / 64.0;
    const R_NECK: f64 = R_FOOT;
    const Y_FOOT: f64 = 1.0 / 64.0;
    const Y_SHOULDER: f64 = 6.0 / 64.0;
    const Y_MOUTH: f64 = 8.0 / 64.0;
    let lp: ProfileLoop<f64> = Open
        .at(Point2::new(0.0, 0.0))
        .line_to(Point2::new(R_FOOT, 0.0), tol)
        .expect("base")
        .line_to(Point2::new(R_FOOT, Y_FOOT), tol)
        .expect("foot")
        .line_to(Point2::new(R_BELLY, Y_FOOT), tol)
        .expect("lower shoulder")
        .line_to(Point2::new(R_BELLY, Y_SHOULDER), tol)
        .expect("belly")
        .line_to(Point2::new(R_NECK, Y_SHOULDER), tol)
        .expect("upper shoulder")
        .line_to(Point2::new(R_NECK, Y_MOUTH), tol)
        .expect("neck")
        .line_to(Point2::new(0.0, Y_MOUTH), tol)
        .expect("mouth")
        .line_to(Start, tol)
        .expect("axis")
        .into();
    revolved(lp, tol)
}

/// Every planar face of `body` whose plane origin sits at station `y`.
fn plane_chart_at(body: &Body<f64>, y: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-12)
        })
        .map(|(k, _)| k)
        .collect()
}

/// **T2: the sharp vessel — where does the F7 gate stand on it now?**
/// An instrument first: dumps every planar same-key pair with the
/// exemption's own predicate recomputed, then asserts the gate's
/// verdict matches the dump (refuses iff some pair edge is not
/// exempt).
#[test]
fn t2_sharp_vessel_f7_verdict_matches_its_own_structure() {
    let tol = Tol::witness();
    let sharp = sharp_vessel(tol);
    let rows = dump_planar_same_key_pairs("t2", &sharp);
    let any_non_exempt = rows.iter().any(|(_, exempt)| !exempt);
    let result = pncad::topo::union(&sharp, &distant_brick(tol), tol);
    match &result {
        Ok(_) => {
            println!("[t2] union(sharp, distant) = Ok");
            assert!(
                !any_non_exempt,
                "gate admitted a body whose dump shows a non-exempt planar pair"
            );
        }
        Err(e) => {
            println!("[t2] union(sharp, distant) refused: {e:?}");
            if matches!(e, BooleanError::NonMaximalFaces { .. }) {
                assert!(
                    any_non_exempt,
                    "gate refused F7 but the dump shows every planar pair exempt"
                );
            }
        }
    }
}

/// **T3: the shipped cup (shell_open of the sharp vessel at its
/// mouth), the body #1031's half-B dump names.** Same instrument as
/// T2; the expectation from the PR record is a `NonMaximalFaces`
/// refusal surviving the exemption, on a valence-4 pair.
#[test]
fn t3_teapot_cup_still_refuses_f7_on_its_valence4_pair() {
    let tol = Tol::witness();
    const Y_MOUTH: f64 = 8.0 / 64.0;
    const WALL: f64 = 1.0 / 128.0;
    let sharp = sharp_vessel(tol);
    let mouth = plane_chart_at(&sharp, Y_MOUTH);
    let cup = pncad::topo::shell_open(&sharp, WALL, &mouth, FIT_TOL, band(tol), tol)
        .expect("the pot opens at its mouth");
    let rows = dump_planar_same_key_pairs("t3", &cup);
    let any_non_exempt = rows.iter().any(|(_, exempt)| !exempt);
    let result = pncad::topo::union(&cup, &distant_brick(tol), tol);
    match &result {
        Ok(_) => println!("[t3] union(cup, distant) = Ok"),
        Err(e) => println!("[t3] union(cup, distant) refused: {e:?}"),
    }
    match &result {
        Err(BooleanError::NonMaximalFaces { .. }) => assert!(
            any_non_exempt,
            "gate refused F7 but the dump shows every planar pair exempt"
        ),
        _ => assert!(
            !any_non_exempt,
            "dump shows a non-exempt planar pair but the gate did not refuse F7"
        ),
    }
}

/// **T4: the authorized merge run, reproduced.** `merge_coplanar_faces`
/// on the cup: the PR pins `Err(MergedFaceRoleAmbiguous {{ face: 4v1 }})`
/// as the measurement that half B's repair door is shut.
#[test]
fn t4_merge_door_on_the_cup_reproduces_the_authorized_run() {
    let tol = Tol::witness();
    const Y_MOUTH: f64 = 8.0 / 64.0;
    const WALL: f64 = 1.0 / 128.0;
    let sharp = sharp_vessel(tol);
    let mouth = plane_chart_at(&sharp, Y_MOUTH);
    let mut cup = pncad::topo::shell_open(&sharp, WALL, &mouth, FIT_TOL, band(tol), tol)
        .expect("the pot opens at its mouth");
    let merged = cup.merge_coplanar_faces(tol);
    println!("[t4] merge_coplanar_faces(cup) = {merged:?}");
    assert!(
        matches!(
            merged,
            Err(pncad::topo::MergeCoplanarError::MergedFaceRoleAmbiguous { .. })
        ),
        "the PR's authorized run says the merge door refuses on the cup"
    );
}
