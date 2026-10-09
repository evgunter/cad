//! **OFF-D PR-1 review probes** (`replace_face_offset`, PR #1043,
//! frozen head `34ee2537`). Runs the implementer did not run:
//!
//! - the apex-window predicate on the OPENING nappe (the acceptance
//!   suite's only cone is the mirror-nappe form), both the pass and the
//!   crossing, plus a large away-from-apex `d` (the sign is monotone
//!   the right way);
//! - a cone whose rim pair IS routed (`cone × plane`): the C5 gate must
//!   not shadow it, and the honest refusal downstream is named;
//! - whole-body `Debug` bit-identity on every `Err` path (the suite
//!   compares circle radii / face lists only);
//! - a partial-revolve side wall: the re-anchor lane on carriers the
//!   suite never touches (rim arcs, revolved-point mapped rims);
//! - which leg of the fitted-boundary obstruction actually fires, on
//!   the planar prism AND on a genuinely curved (twisted) loft.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, FaceKey, ReplaceFaceError};

use crate::common;
use crate::common::shell_operands::tube;
use common::approx::{prism, twisted_loft};

fn revolved_by(points: &[(f64, f64)], rev: Revolution<f64>) -> Body<f64> {
    let lp = bulge_loop(
        points
            .iter()
            .map(|(r, y)| (Point2::new(*r, *y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("probe polygon is a valid profile");
    revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        rev,
        Tol::witness(),
    )
    .expect("probe polygon revolves")
    .body
}

fn revolved(points: &[(f64, f64)]) -> Body<f64> {
    revolved_by(points, Revolution::Full)
}

/// A tube whose outer wall is a cone BELOW (apex under the body, so the
/// face sweeps `v > 0` — the OPENING nappe) and a cylinder above. The
/// acceptance suite's cone is the mirror form (apex above, `v < 0`);
/// this is the other one.
fn cone_up_tube() -> Body<f64> {
    revolved(&[(0.4, 0.0), (0.8, 0.3), (0.8, 0.6), (0.4, 0.6)])
}

/// A frustum with PLANAR caps: the cone's rim pair is `cone × plane`,
/// which the C5 table routes — the one cone configuration where the
/// route gate must NOT fire. Mirror-nappe form (apex above the body).
fn frustum_mirror() -> Body<f64> {
    revolved(&[(0.2, 0.0), (0.6, 0.0), (0.4, 0.6), (0.2, 0.6)])
}

/// The same routed configuration on the OPENING nappe (apex below the
/// body): the generator arm's `copysign` is the identity here, so this
/// fixture isolates the parallel arm.
fn frustum_opening() -> Body<f64> {
    revolved(&[(0.2, 0.0), (0.4, 0.0), (0.6, 0.6), (0.2, 0.6)])
}

/// Every corner of `face` lies on the moved cone and on an untouched
/// plane of the body: the moved cone's rims stayed on their caps.
fn assert_rims_on_caps(body: &Body<f64>, face: FaceKey) {
    let cone = moved_cone(body, face);
    let caps: Vec<(geom_core::Point3<f64>, geom_core::Vec3<f64>)> = body
        .faces()
        .filter_map(|(_, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. }) => Some((*origin, *normal)),
            _ => None,
        })
        .collect();
    let mut corners = 0;
    for (he, _) in body.half_edges() {
        if body.face_of_half_edge(he) != Some(face) {
            continue;
        }
        let p = body.half_edge_start_point(he).expect("a corner");
        assert!(
            caps.iter().any(|(o, n)| (p - *o).dot(*n).abs() < 1e-9),
            "a corner of the moved cone at {p:?} is on no cap"
        );
        assert!(on_cone(&cone, p), "a corner at {p:?} is off the moved cone");
        corners += 1;
    }
    assert!(corners > 0, "the moved cone has corners");
}

/// The surface `face` wears after a move.
fn moved_cone(body: &Body<f64>, face: FaceKey) -> geom::Surface<f64> {
    body.get_face(face)
        .and_then(|f| body.get_surface(f.surface))
        .cloned()
        .expect("the moved face has a surface")
}

/// `p` lies on the cone (`apex`, `axis`, `α`): its distance from the
/// axis is its axial height from the apex times `tan α`.
fn on_cone(cone: &geom::Surface<f64>, p: geom_core::Point3<f64>) -> bool {
    let geom::Surface::Cone {
        apex,
        axis,
        half_angle,
        ..
    } = *cone
    else {
        panic!("the moved face is a cone")
    };
    let w = p - apex;
    let h = w.dot(axis);
    ((w - axis * h).norm() - h.abs() * half_angle.tan()).abs() < 1e-9
}

/// The moved cone's rims: each circle of `face`'s boundary lies on the
/// moved cone (its height) and on an untouched cylinder (its radius).
fn assert_rims_on_cylinders(body: &Body<f64>, face: FaceKey) {
    let cone = moved_cone(body, face);
    let cylinders: Vec<f64> = body
        .faces()
        .filter_map(|(_, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Cylinder { radius, .. }) => Some(*radius),
            _ => None,
        })
        .collect();
    let mut rims = 0;
    for (he, h) in body.half_edges() {
        if body.face_of_half_edge(he) != Some(face) {
            continue;
        }
        let curve = body
            .get_curve_geom(body.get_edge(h.edge).expect("a live edge").curve)
            .and_then(topo::CurveGeom::certified)
            .expect("a certified rim");
        if let geom::Curve3::Circle { radius, .. } = *curve.carrier() {
            assert!(
                cylinders.iter().any(|c| (c - radius).abs() < 1e-12),
                "a rim of radius {radius} is not on an untouched cylinder {cylinders:?}"
            );
            let (t0, t1) = curve.params();
            for t in [t0, 0.5 * (t0 + t1), t1] {
                let p = curve.carrier().eval(t);
                assert!(
                    on_cone(&cone, p),
                    "a rim point at {p:?} is off the moved cone"
                );
            }
            rims += 1;
        }
    }
    assert_eq!(rims, 2, "the moved cone keeps both rims");
}

fn cone_face(body: &Body<f64>) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cone { .. })
            )
        })
        .map(|(k, _)| k)
        .expect("the fixture has a cone face")
}

/// The body's whole `Debug`, for an equality between two builds.
/// NOT `common::bitdump::dump`, which writes a curated bit-faithful
/// subset for a file diff.
fn dump(body: &Body<f64>) -> String {
    format!("{body:?}")
}

/// **Opening nappe, the pass.** On the apex-below cone the window is
/// `v ∈ [0.5, 1.0]` and `d = -0.05` shifts it toward the apex by
/// `0.0375` — nowhere near zero, so the predicate must pass and the
/// door must fall THROUGH it. A predicate whose mirror-nappe derivation
/// broke the primary form would refuse HERE.
///
/// **This row does not pin an ordering** — on this fixture the C5 gate
/// cannot fire at all (both of the cone's neighbours are cylinders, and
/// that pair is routed), so nothing here says the apex predicate runs
/// FIRST; the crossing rows do. What it pins is the pass itself: the
/// predicate lets a small `d` through, and the door goes on to move
/// the cone between its two coaxial cylinders, each rim re-derived as
/// that pair's coaxial section — a circle of the untouched cylinder's
/// own radius.
#[test]
fn opening_nappe_small_d_passes_the_apex_predicate() {
    for d in [-0.05_f64, 0.05] {
        let mut body = cone_up_tube();
        let face = cone_face(&body);
        topo::replace_face_offset(&mut body, face, d, Tol::witness())
            .unwrap_or_else(|e| panic!("d = {d}: the cone moves between its cylinders, got {e}"));
        assert_rims_on_cylinders(&body, face);
    }
}

/// **Opening nappe, the crossing.** `cot α = 0.75`, window inf `0.5`:
/// `d = -1.0` shifts the inf to `-0.25`, across the apex, and the
/// refusal must be `ApexWindow` on THIS nappe as it is on the mirror
/// one — the mirror-nappe derivation of the predicate applied to the
/// primary form is what this fixture isolates.
///
/// **This row no longer pins an ORDERING**, and its old name said it
/// did. On `cone_up_tube` the C5 gate cannot fire at any `d`: both of
/// the cone's neighbours are cylinders and that pair is routed, so
/// "before the route gate" names a race with nothing in the other lane.
/// What still carries the apex predicate's PLACE in the sequence is the
/// mirror-nappe crossing row in the acceptance suite, whose fixture has
/// an unrouted neighbour.
#[test]
fn opening_nappe_apex_crossing_refuses_typed() {
    let mut body = cone_up_tube();
    let face = cone_face(&body);
    let e = topo::replace_face_offset(&mut body, face, -1.0, Tol::witness())
        .expect_err("the shifted window crosses the apex");
    assert!(
        matches!(e, ReplaceFaceError::ApexWindow { face: f, .. } if f == face),
        "expected ApexWindow ahead of the route gate, got {e}"
    );
}

/// **The sign is monotone the right way.** A large `d` AWAY from the
/// apex (`+5.0` on the opening nappe, window landing at `[4.25, 4.75]`)
/// must not trip the predicate — a `|shift|`-shaped bug would refuse
/// here. It builds: the moved cone's meridian is the old one moved `5`
/// along its normal, cut by the two untouched cylinders, and the body
/// is that meridian revolved (Pappus).
#[test]
fn a_large_d_away_from_the_apex_is_not_an_apex_crossing() {
    let mut body = cone_up_tube();
    let face = cone_face(&body);
    topo::replace_face_offset(&mut body, face, 5.0, Tol::witness())
        .unwrap_or_else(|e| panic!("a shift away from the apex is no crossing; got {e}"));
    let geom::Surface::Cone {
        apex,
        axis,
        half_angle,
        ..
    } = moved_cone(&body, face)
    else {
        panic!("the moved face is a cone")
    };
    // The meridian height at radius `r` on the moved cone (the revolve
    // axis is `+y`, through the origin).
    let y_at = |r: f64| apex.y + axis.y.signum() * r / half_angle.tan();
    let (y1, y2) = (y_at(0.4), y_at(0.8));
    // The moved meridian line stands `|d|` off the old one, through
    // `(0.4, 0)` and `(0.8, 0.3)`.
    let (dx, dy) = (0.4, 0.3);
    let dist = ((0.4 - 0.4) * dy - (y1 - 0.0) * dx).abs() / (dx * dx + dy * dy).sqrt();
    assert!(
        (dist - 5.0).abs() < 1e-9,
        "the moved meridian is 5 off the old one: {dist}"
    );
    assert!((y2 - y1 - 0.3).abs() < 1e-9, "and parallel to it");
    // Pappus over the meridian polygon `(0.4, y1) (0.8, y2) (0.8, 0.6)
    // (0.4, 0.6)`.
    let poly = [(0.4, y1), (0.8, y2), (0.8, 0.6), (0.4, 0.6)];
    let mut twice_area_moment = 0.0;
    for i in 0..4 {
        let (xa, ya) = poly[i];
        let (xb, yb) = poly[(i + 1) % 4];
        twice_area_moment += (xa * yb - xb * ya) * (xa + xb);
    }
    let pappus = (core::f64::consts::PI * twice_area_moment / 3.0).abs();
    let volume = topo::mass_properties(&body, Tol::witness())
        .expect("the moved body has properties")
        .volume;
    assert!(
        (volume - pappus).abs() < 1e-9 * pappus,
        "the body is its meridian revolved: {volume} vs {pappus}"
    );
}

/// **The routed cone pair is not shadowed, and the caps hold the moved
/// rims.** The frustum's rims are `cone × plane`, which the C5 table
/// routes, so `NeighborPairUnroutable` must NOT fire. Each rim is then
/// the moved cone's section with its untouched cap — the axis-normal
/// circle — and each corner where the cone's seam meets it, so the
/// rims stay on their caps rather than moving axially with the cone.
///
/// Opening-nappe fixture: the generator arm's `copysign` is the
/// identity.
#[test]
fn the_routed_opening_cone_reaches_past_c5_and_its_rims_stay_on_the_caps() {
    let mut body = frustum_opening();
    let face = cone_face(&body);
    topo::replace_face_offset(&mut body, face, 0.01, Tol::witness())
        .expect("the cone moves between its caps");
    assert_rims_on_caps(&body, face);
}

/// The same routed configuration on the MIRROR nappe — the nappe the
/// suite's own cone lives on. Here the generator arm's `copysign`
/// negates the mint's continuous-extension normal field
/// (`geom_brep::offset`'s complete-locus fine print), so the seam's
/// transport additionally has to follow the mint across the apex.
#[test]
fn the_routed_mirror_cone_reaches_past_c5_and_its_rims_stay_on_the_caps() {
    let mut body = frustum_mirror();
    let face = cone_face(&body);
    topo::replace_face_offset(&mut body, face, 0.01, Tol::witness())
        .expect("the cone moves between its caps");
    assert_rims_on_caps(&body, face);
}

/// **Whole-body bit-identity on every `Err` path the suite planted —
/// and the ones it didn't.** The acceptance rows compare circle radii
/// or the face list; this row compares the full `Debug` of the body
/// (every arena, every point, every cached curve), so a partial
/// mutation that leaks through any refusal shows here.
#[test]
fn every_err_path_leaves_the_body_bit_untouched() {
    // The radius floor (the suite's fixture, the stronger assert).
    let tube = tube(0.4, 0.8, 0.6);
    let inner = tube
        .faces()
        .find(|(_, f)| {
            matches!(
                tube.get_surface(f.surface),
                Some(geom::Surface::Cylinder { radius, .. }) if (radius - 0.4).abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .unwrap();
    let cases: Vec<(Body<f64>, FaceKey, f64)> = vec![
        (tube.clone(), inner, -0.5),
        (cone_up_tube(), cone_face(&cone_up_tube()), -1.0),
    ];
    for (mut body, face, d) in cases {
        let before = dump(&body);
        let e = topo::replace_face_offset(&mut body, face, d, Tol::witness())
            .expect_err("a planted red");
        assert_eq!(
            dump(&body),
            before,
            "d = {d} ({e}): the body must be bit-untouched on Err"
        );
    }
    // The fitted boundary, both signs — the refusal the PR says fires
    // AFTER the fit door ran but BEFORE any mutation.
    for d in [5e-10_f64, -5e-10] {
        let mut body = prism();
        let wall = body
            .faces()
            .find(|(_, f)| {
                matches!(
                    body.get_surface(f.surface),
                    Some(geom::Surface::Nurbs(n)) if !n.is_placeholder()
                )
            })
            .map(|(k, _)| k)
            .unwrap();
        let before = dump(&body);
        let e = topo::replace_face_offset(&mut body, wall, d, Tol::witness())
            .expect_err("the fitted boundary refuses");
        assert!(
            matches!(e, ReplaceFaceError::FittedBoundaryUnsupported { .. }),
            "got {e}"
        );
        assert_eq!(
            dump(&body),
            before,
            "d = {d}: bit-untouched through the fit-then-refuse path"
        );
    }
}

/// **The re-anchor lanes the suite never touches.** A quarter-revolve
/// annulus has two planar SIDE walls; replacing one moves its four
/// corners tangentially. Its edges with the cylinders are their
/// sections with the moved plane (a ruling each), each corner the root
/// of the moved plane along the cap's rim arc it stands on, and every
/// arc is re-anchored at that root: the arcs end on their carriers at
/// the moved wall.
#[test]
fn a_side_wall_replacement_re_anchors_the_rim_arcs() {
    // NOT `common::shell_operands::tube`: its meridian turned a quarter, a wedge.
    let mut body = revolved_by(
        &[(0.4, 0.0), (0.8, 0.0), (0.8, 0.6), (0.4, 0.6)],
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
    );
    // A side wall: a plane whose normal is horizontal (the caps' are
    // vertical), i.e. a plane containing the revolve axis.
    let side = body
        .faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { normal, .. }) if normal.y.abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .expect("a partial revolve has planar side walls");
    let Some(geom::Surface::Plane { origin, normal, .. }) = body
        .get_face(side)
        .and_then(|f| body.get_surface(f.surface))
        .cloned()
    else {
        unreachable!("the side wall is a plane")
    };
    topo::replace_face_offset(&mut body, side, 0.05, Tol::witness())
        .expect("the side wall moves and the rim arcs follow it");
    for (he, _) in body.half_edges() {
        if body.face_of_half_edge(he) == Some(side) {
            let p = body.half_edge_start_point(he).expect("a corner");
            assert!(
                ((p - origin).dot(normal) - 0.05).abs() < 1e-9,
                "a corner of the moved wall is on the moved plane"
            );
        }
    }
}

/// **Which leg of the fitted obstruction fires, and on a CURVED fit
/// too.** The spec's acceptance named "an Approx replacement on a
/// curved fit"; the suite's prism walls are PLANAR splines. The
/// twisted loft's saddle walls are genuinely curved, so this row runs
/// the door there: the fit door must still run (a fit refusal would
/// surface as `Fit`, not `FittedBoundaryUnsupported`) and the refusal
/// must still be the structural one. Both rows also pin WHICH leg the
/// loop walk hits, which the suite left as bookkeeping.
///
/// **Re-expressed at PCURVE P-1b, and the leg list retired with it.**
/// Two of the five legs this row enumerated no longer exist: U2
/// collapsed `IsoCurve`/`Seam`/`MappedCurve` into one conventional
/// form, so "an iso-curve of a neighbour's chart" and "a periodic
/// seam" merged into "a chart image of a neighbour's chart", and the
/// "a mapped rim (a v-row is not an `IsoCurve`)" refusal was retired
/// outright — a rim is a chart image like any other, and a u-const one
/// takes the exact-row lane whatever minted it (P-1b item 4). Rather
/// than swap five strings for four, the row now pins the leg EXACTLY,
/// per fixture: membership in a list of five could never distinguish
/// a door that fired for the wrong reason from one that fired for the
/// right one, which is the whole thing this row exists to check.
///
/// **The curved fixture's outcome depends on ε, and on every ε row CI
/// gates it is the structural arm.** The offset door's fit target is
/// the run's ε_precision — the door takes the tolerance WITNESS and
/// derives no number of its own — and a genuinely curved base cannot
/// always reach it: below [`CURVED_FIT_REACH`] the twisted loft's
/// saddle wall stalls, so the door refuses at the FIT and the boundary
/// re-description is never attempted. That is D4's blessed
/// ε-tightening consequence, not a defect. [`CURVED_FIT_REACH`] sits
/// below every CI row, so the `Fit` arm below is reached only by a run
/// configured tighter than CI's (`CAD_TOLERANCE_EPS=1e-14` reaches
/// it), and on CI the arm's job is to red if the fit starts refusing
/// where it reaches today. What the row does NOT allow is the fit
/// refusing on the PLANAR fixture: a planar spline's offset is a planar
/// spline, which the interpolation reproduces exactly at any ε, so a
/// fit refusal there would be a real defect and reds.
/// The tightest ε at which the twisted loft's saddle wall still
/// certifies its offset fit at this row's `d = 5e-10`, measured on this
/// fixture: it certifies at 1e-12 (1 refinement round, sup bound
/// 1.13e-13) and at 1e-13 (2 rounds, 5.4e-14), and stalls at 1e-14 with
/// an achieved bound of 1.29e-11. Every ε row CI gates is therefore on
/// the structural arm. The constant is what turns the `Fit` arm below
/// from an or-pin into a claim — at any ε this loose, a fit refusal is
/// a regression rather than ε-tightening, and reds.
const CURVED_FIT_REACH: f64 = 1e-13;

#[test]
fn the_fitted_obstruction_holds_on_a_curved_fit() {
    // Both fixtures' spline walls are bounded by rims described in a
    // NEIGHBOUR's chart (the cap plane they lie in), so both land on
    // the same leg — and the row says so by name rather than by
    // membership.
    for (name, mut body, leg, curved) in [
        (
            "planar prism",
            prism(),
            "a curve drawn on a neighbour's surface",
            false,
        ),
        (
            "twisted loft",
            twisted_loft(0.3),
            "a curve drawn on a neighbour's surface",
            true,
        ),
    ] {
        let wall = body
            .faces()
            .find(|(_, f)| {
                matches!(
                    body.get_surface(f.surface),
                    Some(geom::Surface::Nurbs(n)) if !n.is_placeholder()
                )
            })
            .map(|(k, _)| k)
            .unwrap_or_else(|| panic!("{name}: no spline wall"));
        let e = topo::replace_face_offset(&mut body, wall, 5e-10, Tol::witness())
            .expect_err("the fitted boundary refuses");
        match e {
            ReplaceFaceError::FittedBoundaryUnsupported { what, .. } => {
                assert_eq!(what, leg, "{name}: the wrong leg of the fitted door");
            }
            // The fit could not reach this run's ε, so the structural
            // door was never reached. Legitimate only where the base is
            // genuinely curved AND the run's ε is tighter than what the
            // engine reaches on it — both, so this arm cannot absorb a
            // fit-engine regression at the epsilons where the fit does
            // reach today.
            ReplaceFaceError::Fit { error, .. } => {
                assert!(
                    curved,
                    "{name}: a PLANAR spline's offset is exactly fittable at every ε, so a fit \
                     refusal here is a defect rather than ε-tightening: {error}"
                );
                assert!(
                    Tol::witness().eps() < CURVED_FIT_REACH,
                    "{name}: the curved fit refused at ε = {:e}, where it reaches today \
                     (measured: it certifies at ε ≥ {CURVED_FIT_REACH:e} and stalls at 1e-14). \
                     That is a fit-engine regression, not ε-tightening: {error}",
                    Tol::witness().eps()
                );
            }
            other => panic!("{name}: expected the structural refusal or the fit's, got {other}"),
        }
    }
}

/// **An offset as deep as the walls are tall collapses their seams, and
/// says so.** The prism's walls are 1 m tall: moving the top cap 1 m
/// down puts each vertical seam's moved end on its other end, and 1.5 m
/// carries it past. Both are the move's length — a user's input — so
/// the refusal names the offset and asks for a shorter one rather than
/// reporting a kernel defect.
#[test]
fn an_offset_as_deep_as_the_walls_collapses_their_seams() {
    for d in [-1.0_f64, -1.5] {
        let mut body = prism();
        let top = body
            .faces()
            .find(|(_, f)| {
                matches!(
                    body.get_surface(f.surface),
                    Some(geom::Surface::Plane { origin, .. }) if origin.z > 0.5
                )
            })
            .map(|(k, _)| k)
            .unwrap();
        let e = topo::replace_face_offset(&mut body, top, d, Tol::witness())
            .expect_err("a seam of no length refuses");
        assert!(
            matches!(e, ReplaceFaceError::ReanchorCollapse { offset, .. } if offset == d),
            "{d}: {e:?}"
        );
        let text = e.to_string();
        assert!(
            text.contains(&format!("{d:?} m")) && !text.contains("kernel defect"),
            "{d}: {text}"
        );
    }
}

/// **The apex window reads the rims as derived.** A frustum with a tiny
/// top radius: its rims are sections with the held caps, so moving the
/// cone inward by `d = −0.39` slides the apex between the caps and
/// would land the far rim on the other nappe, though the old window
/// shifted by `d·cot α` still clears it. The door refuses `ApexWindow`
/// on the window it actually derived, on both nappes.
#[test]
fn a_derived_rim_past_the_moved_apex_refuses_the_apex_window() {
    for (what, pts) in [
        (
            "opening",
            [(0.01, 0.0), (0.4, 0.0), (0.6, 0.6), (0.01, 0.6)],
        ),
        ("mirror", [(0.01, 0.0), (0.6, 0.0), (0.4, 0.6), (0.01, 0.6)]),
    ] {
        let mut body = revolved(&pts);
        let face = cone_face(&body);
        let before = dump(&body);
        let e = topo::replace_face_offset(&mut body, face, -0.39, Tol::witness())
            .expect_err("the moved apex lies between the caps");
        let ReplaceFaceError::ApexWindow {
            v_min,
            v_max,
            shift,
            ..
        } = e
        else {
            panic!("{what}: expected the apex window, got {e}");
        };
        let (near, sense) = if v_min > 0.0 {
            (v_min, 1.0)
        } else {
            (v_max, -1.0)
        };
        assert!(
            (near + shift) * sense <= 0.0,
            "{what}: the derived window's near end reaches the moved apex \
             (v [{v_min}, {v_max}], shift {shift})"
        );
        assert_eq!(
            dump(&body),
            before,
            "{what}: the body is bit-untouched on Err"
        );
    }
}

/// **A plane moved beside OBLIQUE planes is solved, not transported.**
/// One side face of a hexagonal prism, moved inward: its two side
/// neighbours meet it at 120°, so they are not carried onto themselves
/// by the move, and each edge with them is the two planes' line and
/// each corner the moved plane's root along the edge the neighbours
/// share. Every corner of the moved face then lies on the moved plane
/// and on every other plane it meets. A rigid transport would leave it
/// `|d|·cot 60°` off a neighbour.
#[test]
fn a_hex_prisms_side_moves_with_its_corners_on_its_oblique_neighbours() {
    let tol = Tol::witness();
    let hex: Vec<_> = (0..6)
        .map(|i| {
            let a = core::f64::consts::TAU * f64::from(i) / 6.0;
            (geom_core::Point2::new(a.cos(), a.sin()), 0.0)
        })
        .collect();
    let profile = profile::Profile::new(
        profile::SketchPlane::xy(),
        vec![profile::test_support::bulge_loop(hex)],
    )
    .validate(tol)
    .expect("the hexagon validates");
    let mut body = sweep::extrude(
        &profile,
        sweep::Extrusion::Distance {
            depth: 1.0,
            side: sweep::ExtrudeSide::Along,
        },
        tol,
    )
    .expect("the hexagonal prism extrudes")
    .body;
    let planes: Vec<(FaceKey, geom_core::Point3<f64>, geom_core::Vec3<f64>)> = body
        .faces()
        .filter_map(|(k, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. }) => Some((k, *origin, *normal)),
            _ => None,
        })
        .collect();
    let &(side, origin, normal) = planes
        .iter()
        .find(|(_, _, n)| n.z.abs() < 1e-9)
        .expect("a side face");
    let d = -0.05;
    topo::replace_face_offset(&mut body, side, d, tol).expect("the side face moves");
    for (he, _) in body.half_edges() {
        if body.face_of_half_edge(he) != Some(side) {
            continue;
        }
        let p = body.half_edge_start_point(he).expect("a corner");
        assert!(
            ((p - origin).dot(normal) - d).abs() < 1e-9,
            "a corner of the moved face is on the moved plane"
        );
        let on_others = planes
            .iter()
            .filter(|(k, _, _)| *k != side)
            .filter(|(_, o, n)| (p - *o).dot(*n).abs() < 1e-9)
            .count();
        assert_eq!(
            on_others, 2,
            "a corner at {p:?} lies on its oblique neighbour and its cap"
        );
    }
}
