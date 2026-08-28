//! **OFF-D PR-2 review probes (ordinal 82)** — claims-to-falsify rows
//! for `shell` sealed/opened. Each row carries the claim it attacks.
//!
//! Not part of the PR under review; lives on the probe branch only.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{Body, FaceKey, ShellError};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

const FIT_TOL: f64 = 1e-6;

fn boxy(w: f64, d: f64, h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(w, 0.0), 0.0),
        ProfileVertex::new(p2(w, d), 0.0),
        ProfileVertex::new(p2(0.0, d), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("rectangle profile");
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .expect("rectangle extrudes")
        .body
}

fn prism(pts: &[(f64, f64)], h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(
        pts.iter()
            .map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("polygon profile");
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .expect("polygon extrudes")
        .body
}

fn vessel(r: f64, h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(r, 0.0), 0.0),
        ProfileVertex::new(p2(r, h), 0.0),
        ProfileVertex::new(p2(0.0, h), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("meridian profile");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("meridian revolves")
    .body
}

fn plane_face_at(body: &Body<f64>, z: f64) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (origin.z - z).abs() < 1e-9 && normal.x.abs() < 1e-9 && normal.y.abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .unwrap_or_else(|| panic!("no z = {z} cap"))
}

/// A face whose plane has the given outward axis-aligned normal
/// component dominant and origin coordinate near `c`.
fn plane_face_x(body: &Body<f64>, x: f64) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (origin.x - x).abs() < 1e-9 && normal.y.abs() < 1e-9 && normal.z.abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .unwrap_or_else(|| panic!("no x = {x} wall"))
}

// ---------------------------------------------------------------------
// Claim 1 — the evidence discipline: every-margin-positive-but-no-room
// bodies must fail LOUD, never return a silently self-intersecting
// "valid" body. Planes have NO margin at all, so plane-walled fixtures
// are the pure form of the honesty gap the PR names.
// ---------------------------------------------------------------------

/// A box shelled far past half its smallest dimension: every plane
/// face offsets (no margin exists), the cavity is inside-out.
#[test]
fn probe_overthick_box_fails_loud() {
    let r = topo::shell(&boxy(2.0, 3.0, 4.0), 1.9, FIT_TOL, band(), Tol::witness());
    match r {
        Err(e) => println!("[probe] overthick box: LOUD: {e}"),
        Ok(body) => panic!(
            "overthick box returned Ok — silent wrong answer; shells = {}, volume = {:?}",
            body.shells().count(),
            topo::mass_properties(&body, Tol::witness()).map(|p| p.volume)
        ),
    }
}

/// A slab shelled past half its thickness (t = 0.6 > h/2 = 0.5): the
/// PR's own named gap fixture. Every per-face margin is positive.
#[test]
fn probe_overhalf_slab_fails_loud() {
    let r = topo::shell(&boxy(4.0, 4.0, 1.0), 0.6, FIT_TOL, band(), Tol::witness());
    match r {
        Err(e) => println!("[probe] over-half slab: LOUD: {e}"),
        Ok(body) => panic!(
            "over-half slab returned Ok — silent wrong answer; shells = {}, volume = {:?}",
            body.shells().count(),
            topo::mass_properties(&body, Tol::witness()).map(|p| p.volume)
        ),
    }
}

/// Exactly half the thickness: the cavity's top and bottom coincide.
#[test]
fn probe_exact_half_slab_fails_loud() {
    let r = topo::shell(&boxy(4.0, 4.0, 1.0), 0.5, FIT_TOL, band(), Tol::witness());
    match r {
        Err(e) => println!("[probe] exact-half slab: LOUD: {e}"),
        Ok(body) => panic!(
            "exact-half slab returned Ok — degenerate cavity; shells = {}, volume = {:?}",
            body.shells().count(),
            topo::mass_properties(&body, Tol::witness()).map(|p| p.volume)
        ),
    }
}

/// An L-prism whose legs are 1.0 wide, shelled at t = 0.6: the inward
/// offset of the L self-intersects at the reflex corner. Every single
/// face's margin is positive (planes have none).
#[test]
fn probe_lshape_colliding_cavity_fails_loud() {
    let l = prism(
        &[
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.0, 3.0),
        ],
        2.0,
    );
    let r = topo::shell(&l, 0.6, FIT_TOL, band(), Tol::witness());
    match r {
        Err(e) => println!("[probe] L-shape: LOUD: {e}"),
        Ok(body) => panic!(
            "L-shape returned Ok — colliding cavity; shells = {}, volume = {:?}",
            body.shells().count(),
            topo::mass_properties(&body, Tol::witness()).map(|p| p.volume)
        ),
    }
}

/// The dumbbell: two 2x2 blobs joined by a 0.4-wide neck, shelled at
/// t = 0.3. The neck's two cavity walls CROSS while every face's own
/// loop stays simple and consistently wound — the hardest instance of
/// the class: no per-face margin fails, no per-face winding inverts.
#[test]
fn probe_dumbbell_neck_collision_fails_loud() {
    let db = prism(
        &[
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 0.8),
            (3.0, 0.8),
            (3.0, 0.0),
            (5.0, 0.0),
            (5.0, 2.0),
            (3.0, 2.0),
            (3.0, 1.2),
            (2.0, 1.2),
            (2.0, 2.0),
            (0.0, 2.0),
        ],
        2.0,
    );
    let r = topo::shell(&db, 0.3, FIT_TOL, band(), Tol::witness());
    // **MAJ-1, closed (ordinal 82 -> fix pass).** At `259fde04` this
    // returned Ok, tier-3 VALIDATED, and reported volume 11.76 against
    // a true erosion volume of 11.312: the cavity's neck walls
    // (y = 0.8 + 0.3 = 1.1 and y = 1.2 - 0.3 = 0.9) had CROSSED. No
    // per-face margin could see it — a plane's reach is unbounded — and
    // tier 3 could not either, because every per-face loop stays simple
    // and consistently wound while the walls march through each other.
    //
    // The closed-form planar clearance gate is what sees it now: the
    // neck's two facing walls are 0.4 apart and two 0.3 walls need 0.6.
    let e = r.expect_err("the neck is thinner than two walls");
    assert!(
        matches!(e, ShellError::WallClearance { .. }),
        "expected the wall-clearance gate, got {e}"
    );
    println!("[probe] MAJ-1: dumbbell refuses LOUD: {e}");
}

/// Shelling an ALREADY-HOLLOW body: the operand has two shells, the
/// verb's cavity clone offsets both and inserts BOTH as voids beside
/// the operand's existing void — nested/overlapping voids unless
/// something refuses. One solid, so `NotOneSolid` does not gate it.
#[test]
fn probe_shell_of_a_hollow_fails_loud() {
    let hollow = topo::shell(&boxy(2.0, 3.0, 4.0), 0.25, FIT_TOL, band(), Tol::witness())
        .expect("the first shell is the PR's own green row");
    let r = topo::shell(&hollow, 0.05, FIT_TOL, band(), Tol::witness());
    // **MAJ-2, closed (ordinal 82 -> fix pass).** At `259fde04` this
    // returned Ok with FOUR shells, tier-3 valid, volume 4.362: the
    // verb offset the operand's VOID shell too and inserted both
    // cavity-clone shells as new voids beside the existing one, with
    // `Carried { Positive }` asserted for a shell that was never in
    // material. `NotOneSolid` did not gate it — a hollow body is ONE
    // solid with two shells.
    //
    // The operand gate refuses it now. The ratified semantics for when
    // this is answered rather than refused is "thicken EVERY boundary"
    // (issue #1056) — offsetting only the outer shell is explicitly
    // not the answer.
    let e = r.expect_err("a hollow operand has no single boundary to erode");
    assert!(
        matches!(e, ShellError::OperandAlreadyHollow { shells: 2 }),
        "expected the already-hollow gate naming two shells, got {e}"
    );
    println!("[probe] MAJ-2: shell-of-hollow refuses LOUD: {e}");
}

// ---------------------------------------------------------------------
// Claim 2 — the opened arm: census accounting, and designations the
// acceptance lacks (adjacent pair; a revolved body's cap).
// ---------------------------------------------------------------------

/// Census on the opened box: V − E + F − R = 2(S − G) must hold with
/// the numbers the rim surgery predicts (16, 24, 11, 1, 1; genus 0).
#[test]
fn probe_opened_box_census() {
    let (w, d, h, t) = (2.0, 3.0, 4.0, 0.25);
    let body = boxy(w, d, h);
    let top = plane_face_at(&body, h);
    let cup = topo::shell_open(&body, t, &[top], FIT_TOL, band(), Tol::witness())
        .expect("the PR's own green fixture");
    let v = cup.vertices().count() as i64;
    let e = cup.edges().count() as i64;
    let f = cup.faces().count() as i64;
    let r: i64 = cup.faces().map(|(_, fc)| fc.rings.len() as i64).sum();
    let s = cup.shells().count() as i64;
    println!("[probe] cup census: V={v} E={e} F={f} R={r} S={s}");
    assert_eq!(
        (v, e, f, r, s),
        (16, 24, 11, 1, 1),
        "the rim surgery's census"
    );
    assert_eq!(v - e + f - r, 2 * s, "Euler–Poincaré at genus 0");

    // And the tube (two opposite rims): genus 1.
    let bottom = plane_face_at(&body, 0.0);
    let tube = topo::shell_open(&body, t, &[top, bottom], FIT_TOL, band(), Tol::witness())
        .expect("the PR's own green fixture");
    let v = tube.vertices().count() as i64;
    let e = tube.edges().count() as i64;
    let f = tube.faces().count() as i64;
    let r: i64 = tube.faces().map(|(_, fc)| fc.rings.len() as i64).sum();
    let s = tube.shells().count() as i64;
    println!("[probe] tube census: V={v} E={e} F={f} R={r} S={s}");
    assert_eq!(v - e + f - r, 2 * (s - 1), "Euler–Poincaré at genus 1");
}

/// TWO ADJACENT faces designated open — the designation the acceptance
/// lacks. Every gate passes (both resolve, distinct, remainder is 4
/// connected faces). Either the rim surgery handles the shared edge or
/// it must refuse TYPED — an invalid or wrong-volume Ok is the miss.
#[test]
fn probe_adjacent_two_face_opening() {
    let (w, d, h, t) = (2.0, 3.0, 4.0, 0.25);
    let body = boxy(w, d, h);
    let top = plane_face_at(&body, h);
    let side = plane_face_x(&body, w);
    match topo::shell_open(&body, t, &[top, side], FIT_TOL, band(), Tol::witness()) {
        Err(e) => println!("[probe] adjacent pair: typed refusal: {e}"),
        Ok(open) => {
            assert_eq!(
                topo::validate_geometric(&open, Tol::witness()),
                Ok(()),
                "adjacent-pair opening returned an INVALID body"
            );
            let props = topo::mass_properties(&open, Tol::witness()).expect("props");
            // Cavity runs to the top AND to the x = w side.
            let want = w * d * h - (w - t) * (d - 2.0 * t) * (h - t);
            assert!(
                (props.volume - want).abs() <= 1e-12,
                "adjacent-pair volume: got {}, want {want}",
                props.volume
            );
            println!(
                "[probe] adjacent pair: Ok and coherent (volume {})",
                props.volume
            );
        }
    }
}

/// The opened arm on a REVOLVED body: the vessel's planar cap over
/// cylinder walls (plane x cylinder IS routable). The acceptance only
/// ever opens boxes; this is the first curved-walled cup.
///
/// **Extended after #1082**, whose transferable lesson this row IS: as
/// first written it checked tier 3, the shell count and the volume —
/// every one of which the wrong body satisfied — and never the rings,
/// the genus or the mesh, so it blessed a rim carrying its own cavity
/// counterpart's boundary as a ring for a whole milestone. A probe
/// that checks only the quantities that are right is not evidence
/// about the quantities that are wrong. All three are now here.
#[test]
fn probe_opened_vessel_cup() {
    let (r, h, t) = (1.0, 2.0, 0.2);
    let v = vessel(r, h);
    // A full revolve splits each planar cap into TWO faces on one
    // plane chart (4 planar faces total). Designate the whole top cap:
    // both of its faces.
    // **The cap coordinate is `y`, not `z`.** This fixture revolves an
    // xy-sketch meridian about `dir = (0, 1)`, so the body's axis is
    // `y` and BOTH cap planes sit at `origin.z == 0`. Keying the
    // selection on `z` (as this row first did) selects all four planar
    // faces, designates both caps, and builds a two-ended TUBE — whose
    // volume is `pi*(r^2*h - (r-t)^2*h)`, not the cup's. The verb was
    // right and the selector was wrong; corrected here rather than
    // loosening the volume the row checks.
    let mut caps: Vec<(FaceKey, f64)> = v
        .faces()
        .filter_map(|(k, f)| match v.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, .. }) => Some((k, origin.y)),
            _ => None,
        })
        .collect();
    println!("[probe] vessel planar faces: {}", caps.len());
    caps.sort_by(|a, b| b.1.total_cmp(&a.1));
    let ymax = caps[0].1;
    let top: Vec<FaceKey> = caps
        .iter()
        .filter(|(_, y)| (*y - ymax).abs() < 1e-9)
        .map(|(k, _)| *k)
        .collect();
    // The refusal arm is an ASSERT, not a print. As a print it made
    // this row unfalsifiable in the one direction that matters: a
    // regression that turned the revolved cup into a typed refusal
    // would have read as a green probe. The verb builds this rim, so
    // anything else reds here.
    match topo::shell_open(&v, t, &top, FIT_TOL, band(), Tol::witness()) {
        Err(e) => panic!(
            "the revolved vessel cup must BUILD ({} top faces designated); the verb \
             refused with {e}",
            top.len()
        ),
        Ok(cup) => {
            assert_eq!(
                topo::validate_geometric(&cup, Tol::witness()),
                Ok(()),
                "the vessel cup must validate"
            );
            assert_eq!(cup.shells().count(), 1, "one shell after the rim fuses");
            // THE RINGS: one, and on the mouth plane — the rim is the
            // annulus between the wall's two radii, not a copy of the
            // cavity cap's own boundary laid over the designated face.
            let rings: usize = cup.faces().map(|(_, f)| f.rings.len()).sum();
            let mouth: Vec<FaceKey> = cup
                .faces()
                .filter(|(_, f)| {
                    matches!(cup.get_surface(f.surface),
                        Some(geom::Surface::Plane { origin, .. }) if (origin.y - h).abs() < 1e-12)
                })
                .map(|(k, _)| k)
                .collect();
            assert_eq!(mouth.len(), 1, "the mouth chart is ONE rim face");
            assert_eq!(
                cup.get_face(mouth[0]).expect("the rim").rings.len(),
                1,
                "the rim carries exactly one ring"
            );
            assert_eq!(rings, 1, "and that is the body's only ring");
            // THE GENUS: `topo::shell`'s own docs say a cup is 0.
            let (v, e, f) = (
                cup.vertices().count() as i64,
                cup.edges().count() as i64,
                cup.faces().count() as i64,
            );
            let chi = v - e + f - rings as i64;
            assert!(chi % 2 == 0, "v - e + f - r = {chi} is ODD");
            assert_eq!(
                cup.shells().count() as i64 - chi / 2,
                0,
                "one opening gives a cup, which is genus 0"
            );
            // THE MESH: the consumer that discovered #1082, run here.
            for delta in [1e-2, 1e-3] {
                mesh::tessellate(&cup, delta, Tol::witness()).unwrap_or_else(|err| {
                    panic!("the vessel cup must triangulate at delta = {delta}, got {err:?}")
                });
            }
            let props = topo::mass_properties(&cup, Tol::witness()).expect("props");
            let want = core::f64::consts::PI * (r * r * h - (r - t) * (r - t) * (h - t));
            assert!(
                (props.volume - want).abs() <= 1e-9 + props.volume_pad,
                "vessel cup volume: got {} (pad {}), want {want}",
                props.volume,
                props.volume_pad
            );
            println!(
                "[probe] vessel cup: Ok and coherent (volume {}, rings {rings})",
                props.volume
            );
        }
    }
}

/// `OpenFaceStale` — the one designation gate the acceptance never
/// fires. A key minted past the operand's face count cannot resolve.
#[test]
fn probe_stale_designation_refuses_typed() {
    let body = boxy(2.0, 3.0, 4.0);
    let big = prism(
        &[
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 0.8),
            (3.0, 0.8),
            (3.0, 0.0),
            (5.0, 0.0),
            (5.0, 2.0),
            (3.0, 2.0),
            (3.0, 1.2),
            (2.0, 1.2),
            (2.0, 2.0),
            (0.0, 2.0),
        ],
        2.0,
    );
    let foreign = big
        .faces()
        .map(|(k, _)| k)
        .last()
        .expect("the prism has more faces than the box");
    assert!(body.get_face(foreign).is_none(), "the key must not resolve");
    let e = topo::shell_open(&body, 0.25, &[foreign], FIT_TOL, band(), Tol::witness())
        .expect_err("a stale designation must refuse");
    assert!(
        matches!(e, topo::ShellError::OpenFaceStale { .. }),
        "expected OpenFaceStale, got {e}"
    );
}

// ---------------------------------------------------------------------
// Claim 3 — the chart-group door: partial-group refusal and the
// bit-untouched-on-Err contract, checked on whole-body Debug.
// ---------------------------------------------------------------------

/// Replacing only ONE of the two faces wearing the vessel's cylinder
/// chart must still refuse `SharedSurfaceKey`, and the body must be
/// bit-untouched across the Err.
#[test]
fn probe_partial_group_refuses_and_leaves_body_untouched() {
    let v = vessel(1.0, 2.0);
    let cyl: Vec<FaceKey> = v
        .faces()
        .filter(|(_, f)| {
            matches!(
                v.get_surface(f.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert!(
        cyl.len() >= 2,
        "the full revolve wears one cylinder on two faces"
    );

    let mut work = v.clone();
    let before = format!("{work:?}");
    let e = topo::replace_faces_offset(&mut work, &cyl[..1], -0.2, FIT_TOL, band(), Tol::witness())
        .expect_err("a partial group must refuse");
    assert!(
        matches!(e, topo::ReplaceFaceError::SharedSurfaceKey { .. }),
        "expected SharedSurfaceKey for the partial group, got {e}"
    );
    assert_eq!(
        before,
        format!("{work:?}"),
        "body moved across a partial-group Err"
    );

    // A mixed group refuses too, untouched.
    let cap = v
        .faces()
        .find(|(_, f)| matches!(v.get_surface(f.surface), Some(geom::Surface::Plane { .. })))
        .map(|(k, _)| k)
        .unwrap();
    let mixed = vec![cyl[0], cap];
    let e = topo::replace_faces_offset(&mut work, &mixed, -0.2, FIT_TOL, band(), Tol::witness())
        .expect_err("a mixed group must refuse");
    assert!(
        matches!(e, topo::ReplaceFaceError::GroupChartsDiffer { .. }),
        "expected GroupChartsDiffer, got {e}"
    );
    assert_eq!(
        before,
        format!("{work:?}"),
        "body moved across a mixed-group Err"
    );

    // The empty group.
    let e = topo::replace_faces_offset(&mut work, &[], -0.2, FIT_TOL, band(), Tol::witness())
        .expect_err("an empty group must refuse");
    assert!(matches!(e, topo::ReplaceFaceError::EmptyGroup), "got {e}");
    assert_eq!(
        before,
        format!("{work:?}"),
        "body moved across an empty-group Err"
    );
}

/// A LATE Err path through the group door (the C5 refusal, decided
/// after the mint and the boundary plan): whole-body Debug still
/// untouched — the decided-then-mutated clone discipline.
#[test]
fn probe_late_err_leaves_body_untouched() {
    // A partial revolve's torus wall: replacing a CAP meets plane x
    // torus, which has no route arm — an Err decided deep in the plan.
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(-0.3, 0.0), 1.0),
        ProfileVertex::new(p2(0.3, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("disc profile");
    let elbow = revolve(
        &profile,
        RevolveAxis {
            origin: p2(1.2, 0.0),
            dir: Vec2::new(0.0, -1.0),
        },
        Revolution::Partial(-0.5 * core::f64::consts::PI),
        Tol::witness(),
    )
    .expect("the elbow revolves")
    .body;
    let cap = elbow
        .faces()
        .find(|(_, f)| {
            matches!(
                elbow.get_surface(f.surface),
                Some(geom::Surface::Plane { .. })
            )
        })
        .map(|(k, _)| k)
        .unwrap();
    let mut work = elbow.clone();
    let before = format!("{work:?}");
    let e = topo::replace_face_offset(&mut work, cap, -0.05, FIT_TOL, band(), Tol::witness())
        .expect_err("plane x torus has no route arm");
    assert!(
        matches!(e, topo::ReplaceFaceError::NeighborPairUnroutable { .. }),
        "expected the C5 refusal, got {e}"
    );
    assert_eq!(before, format!("{work:?}"), "body moved across a late Err");
}
