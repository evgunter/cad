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

use geom_core::{Band, Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, FaceKey, ReplaceFaceError};

mod common;
use common::approx::{prism, twisted_loft};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

const FIT_TOL: f64 = 1e-6;

fn revolved_by(points: &[(f64, f64)], rev: Revolution<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(
        points
            .iter()
            .map(|(r, y)| ProfileVertex::new(p2(*r, *y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("probe polygon is a valid profile");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
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

fn dump(body: &Body<f64>) -> String {
    format!("{body:?}")
}

/// **Opening nappe, the pass.** On the apex-below cone the window is
/// `v ∈ [0.5, 1.0]` and `d = -0.05` shifts it toward the apex by
/// `0.0375` — nowhere near zero, so the predicate must pass and the
/// door must fall through to the C5 gate (`cone × cylinder` unrouted).
/// A predicate whose mirror-nappe derivation broke the primary form
/// would refuse HERE.
#[test]
fn opening_nappe_small_d_passes_the_apex_predicate() {
    for d in [-0.05_f64, 0.05] {
        let mut body = cone_up_tube();
        let face = cone_face(&body);
        let e = topo::replace_face_offset(&mut body, face, d, FIT_TOL, band(), Tol::witness())
            .expect_err("cone x cylinder has no route arm");
        assert!(
            matches!(
                e,
                ReplaceFaceError::NeighborPairUnroutable {
                    kind: geom_brep::SurfaceKind::Cone,
                    other_kind: geom_brep::SurfaceKind::Cylinder,
                    ..
                }
            ),
            "d = {d}: the apex predicate must pass on the opening nappe and \
             the C5 gate must be what refuses; got {e}"
        );
    }
}

/// **Opening nappe, the crossing.** `cot α = 0.75`, window inf `0.5`:
/// `d = -1.0` shifts the inf to `-0.25`, across the apex — the refusal
/// must be `ApexWindow`, NOT the C5 refusal that the same face draws at
/// small `d`, which pins the predicate ahead of the route gate on THIS
/// nappe too (the suite pins it only on the mirror one).
#[test]
fn opening_nappe_apex_crossing_refuses_before_the_route_gate() {
    let mut body = cone_up_tube();
    let face = cone_face(&body);
    let e = topo::replace_face_offset(&mut body, face, -1.0, FIT_TOL, band(), Tol::witness())
        .expect_err("the shifted window crosses the apex");
    assert!(
        matches!(e, ReplaceFaceError::ApexWindow { face: f, .. } if f == face),
        "expected ApexWindow ahead of the route gate, got {e}"
    );
}

/// **The sign is monotone the right way.** A large `d` AWAY from the
/// apex (`+5.0` on the opening nappe, window landing at `[4.25, 4.75]`)
/// must not trip the predicate — a `|shift|`-shaped bug would refuse
/// here. The C5 gate is again the expected stop.
#[test]
fn a_large_d_away_from_the_apex_is_not_an_apex_crossing() {
    let mut body = cone_up_tube();
    let face = cone_face(&body);
    let e = topo::replace_face_offset(&mut body, face, 5.0, FIT_TOL, band(), Tol::witness())
        .expect_err("cone x cylinder still has no route arm");
    assert!(
        !matches!(e, ReplaceFaceError::ApexWindow { .. }),
        "a shift away from the apex must not read as a crossing; got {e}"
    );
}

/// **The routed cone pair is not shadowed, and the lane table is held
/// to its own words.** The frustum's rims are `cone × plane`, which
/// the C5 table routes, so `NeighborPairUnroutable` must NOT fire.
///
/// The honest downstream refusal is structural: under the mint's
/// `v ↦ v + d·cot α` contract every rim moves axially by `−d·sin α`,
/// so the planar caps' own seam carriers cannot hold the moved
/// vertices, and the re-anchor gate (`offset_reanchor_on_carrier`) is
/// what should say so — the seam and rim transports themselves agree
/// wherever the lane table ("a generator translates by `d·n`; a
/// parallel's `v` shifts by `d·cot α`") is implemented as the mint
/// derives it. A `VertexDisagreement` here instead means the door's
/// own two cone transports put the SAME vertex in two places — the
/// transport lanes disagreeing with the mint they serve, not a
/// property of the fixture.
///
/// Opening-nappe fixture: the generator arm's `copysign` is the
/// identity, so any disagreement is the parallel arm's alone (the
/// apex slide `−axis·d/sin α` that `offset_surface` derives and
/// `transport_curve`'s parallel arm does not apply).
#[test]
fn the_routed_opening_cone_reaches_past_c5_and_refuses_at_the_caps() {
    let mut body = frustum_opening();
    let face = cone_face(&body);
    let before = dump(&body);
    let e = topo::replace_face_offset(&mut body, face, 0.01, FIT_TOL, band(), Tol::witness())
        .expect_err("the caps cannot follow the cone's moved rims");
    assert!(
        !matches!(e, ReplaceFaceError::NeighborPairUnroutable { .. }),
        "cone x plane routes; the C5 gate must not shadow it, got {e}"
    );
    assert_eq!(dump(&body), before, "the body is bit-untouched on Err");
    assert!(
        matches!(e, ReplaceFaceError::ReanchorOffCarrier { .. }),
        "expected the honest cap refusal (the transports agreeing with the \
         mint, the caps unable to follow); got {e}"
    );
}

/// The same routed configuration on the MIRROR nappe — the nappe the
/// suite's own cone lives on. Here the generator arm's `copysign`
/// negates the mint's continuous-extension normal field
/// (`geom_brep::offset`'s complete-locus fine print), so a coherent
/// transport additionally requires the generator arm to follow the
/// mint across the apex.
#[test]
fn the_routed_mirror_cone_reaches_past_c5_and_refuses_at_the_caps() {
    let mut body = frustum_mirror();
    let face = cone_face(&body);
    let before = dump(&body);
    let e = topo::replace_face_offset(&mut body, face, 0.01, FIT_TOL, band(), Tol::witness())
        .expect_err("the caps cannot follow the cone's moved rims");
    assert!(
        !matches!(e, ReplaceFaceError::NeighborPairUnroutable { .. }),
        "cone x plane routes; the C5 gate must not shadow it, got {e}"
    );
    assert_eq!(dump(&body), before, "the body is bit-untouched on Err");
    assert!(
        matches!(e, ReplaceFaceError::ReanchorOffCarrier { .. }),
        "expected the honest cap refusal (the transports agreeing with the \
         mint, the caps unable to follow); got {e}"
    );
}

/// **Whole-body bit-identity on every `Err` path the suite planted —
/// and the ones it didn't.** The acceptance rows compare circle radii
/// or the face list; this row compares the full `Debug` of the body
/// (every arena, every point, every cached curve), so a partial
/// mutation that leaks through any refusal shows here.
#[test]
fn every_err_path_leaves_the_body_bit_untouched() {
    // The radius floor (the suite's fixture, the stronger assert).
    let tube = revolved(&[(0.4, 0.0), (0.8, 0.0), (0.8, 0.6), (0.4, 0.6)]);
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
        (cone_up_tube(), cone_face(&cone_up_tube()), -0.05),
        (cone_up_tube(), cone_face(&cone_up_tube()), -1.0),
    ];
    for (mut body, face, d) in cases {
        let before = dump(&body);
        let e = topo::replace_face_offset(&mut body, face, d, FIT_TOL, band(), Tol::witness())
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
        let e = topo::replace_face_offset(&mut body, wall, d, FIT_TOL, band(), Tol::witness())
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
/// vertices tangentially, which takes every rim arc's endpoint OFF its
/// (unchanged) circle carrier by `≈ d²/2r ≫ ε`. The door must refuse
/// typed — through the circle-inversion re-anchor gate or the mapped
/// lane's scope refusal — and leave the body bit-untouched. A silent
/// green here would be a body whose rim arcs no longer end on their
/// carriers.
#[test]
fn a_side_wall_replacement_refuses_typed_at_the_rim_arcs() {
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
    let before = dump(&body);
    let e = topo::replace_face_offset(&mut body, side, 0.05, FIT_TOL, band(), Tol::witness())
        .expect_err("the rim arcs cannot follow a tangential wall move");
    assert!(
        matches!(
            e,
            ReplaceFaceError::ReanchorOffCarrier { .. }
                | ReplaceFaceError::CarrierLaneUnsupported { .. }
                | ReplaceFaceError::VertexDisagreement { .. }
                | ReplaceFaceError::Op { .. }
        ),
        "expected a typed refusal from the re-anchor/attach family, got {e}"
    );
    assert_eq!(dump(&body), before, "the body is bit-untouched on Err");
}

/// **Which leg of the fitted obstruction fires, and on a CURVED fit
/// too.** The spec's acceptance named "an Approx replacement on a
/// curved fit"; the suite's prism walls are PLANAR splines. The
/// twisted loft's saddle walls are genuinely curved, so this row runs
/// the door there: the fit door must still run (a fit refusal would
/// surface as `Fit`, not `FittedBoundaryUnsupported`) and the refusal
/// must still be the structural one. Both rows also pin WHICH leg the
/// loop walk hits, which the suite left as bookkeeping.
#[test]
fn the_fitted_obstruction_holds_on_a_curved_fit() {
    for (name, mut body) in [
        ("planar prism", prism()),
        ("twisted loft", twisted_loft(0.3)),
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
        let e = topo::replace_face_offset(&mut body, wall, 5e-10, FIT_TOL, band(), Tol::witness())
            .expect_err("the fitted boundary refuses");
        let ReplaceFaceError::FittedBoundaryUnsupported { what, .. } = e else {
            panic!("{name}: expected the structural refusal, got {e}");
        };
        assert!(
            [
                "a seam shared with another bounded chart",
                "an iso-curve of a neighbour's chart",
                "a periodic seam",
                "a mapped rim (a v-row is not an `IsoCurve`, which is u-const by definition)",
                "an intrinsic intersection with an untouched neighbour",
            ]
            .contains(&what),
            "{name}: an undocumented leg: {what}"
        );
    }
}
