//! M2 PR 7 e2e review — public-API falsification of the exact mass
//! properties: shapes BEYOND the implementer's acceptance set, each
//! with a first-principles closed form derived independently in the
//! comment above it (never copied from the props module), plus
//! orientation attacks (negative extrusion distance, negative revolve
//! angle) and a 1e6-scaled body. (The mesh-volume cross-checks for
//! these shapes live in the stl review suite, which links `mesh`.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use core::f64::consts::{FRAC_PI_2, PI};
use profile::RawLoop;

use profile::{ProfileLoop, ProfileVertex};
use sweep::{Extrusion, Revolution, extrude, revolve};
use topo::{Body, mass_properties, validate, validate_closed, validate_geometric};

use geom_core::Tol;
use revolve_common::{axis_y, p2, validated};

fn v(x: f64, y: f64, b: f64) -> ProfileVertex<f64> {
    ProfileVertex::new(p2(x, y), b)
}

fn check(body: &Body<f64>, what: &str, volume: f64, area: f64) {
    assert_eq!(validate(body), Ok(()), "{what}: tier 1");
    assert_eq!(validate_closed(body), Ok(()), "{what}: tier 2");
    assert_eq!(
        validate_geometric(body, Tol::witness()),
        Ok(()),
        "{what}: tier 3 (+V)"
    );
    let props = mass_properties(body, Tol::witness()).expect("mass properties must compute");
    let rel = |got: f64, want: f64| (got - want).abs() / want.abs().max(1.0);
    assert!(
        rel(props.volume, volume) <= 1e-12,
        "{what} volume: got {}, derived {} (rel {:.3e})",
        props.volume,
        volume,
        rel(props.volume, volume)
    );
    assert!(
        rel(props.surface_area, area) <= 1e-12,
        "{what} area: got {}, derived {} (rel {:.3e})",
        props.surface_area,
        area,
        rel(props.surface_area, area)
    );
}

/// Frustum shell: revolve [(1,0),(2,0),(1,1)] about y. Outer radius
/// r(y) = 2 − y, inner bore r = 1.
/// V = π∫₀¹((2−y)² − 1)dy = π(7/3 − 1) = 4π/3.
/// A = bottom annulus π(4−1) + cone slant π(2+1)·√2 + bore 2π·1·1
///   = (5 + 3√2)π.
#[test]
fn frustum_with_bore_matches_independent_closed_forms() {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(1.0, 1.0)]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap();
    check(
        &t.body,
        "frustum",
        4.0 * PI / 3.0,
        (5.0 + 3.0 * core::f64::consts::SQRT_2) * PI,
    );
}

/// Cup (inner cylinder wall with interior OUTSIDE the surface —
/// `s_f = −1` — plus an interior-facing plane ring): revolve
/// [(0,0),(2,0),(2,2),(1.5,2),(1.5,0.5),(0,0.5)].
/// V = π·4·0.5 + π(4 − 2.25)·1.5 = 4.625π.
/// A = 4π (bottom) + 8π (outer) + 1.75π (top ring) + 4.5π (inner
/// wall) + 2.25π (cavity floor) = 20.5π.
#[test]
fn cup_inner_walls_match_independent_closed_forms() {
    let lp = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(2.0, 0.0),
        p2(2.0, 2.0),
        p2(1.5, 2.0),
        p2(1.5, 0.5),
        p2(0.0, 0.5),
    ]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap();
    check(&t.body, "cup", 4.625 * PI, 20.5 * PI);
}

/// Quarter donut wedge (torus `s_f` via rim adjacency, on wedge faces
/// with real rims + meridian minor circles): the donut profile (circle
/// center (2,0), r = 1/2, two bulge-1 arcs) revolved θ = π/2.
/// V = (θ/2π)·2π²Rr² = π²/4.
/// A = (θ/2π)·4π²Rr + 2·πr² = π² + π/2.
#[test]
fn quarter_donut_wedge_matches_independent_closed_forms() {
    let lp = ProfileLoop::new(vec![v(2.0, -0.5, 1.0), v(2.0, 0.5, 1.0)]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap();
    check(&t.body, "quarter donut", PI * PI / 4.0, PI * PI + PI / 2.0);
}

/// Dome wedge (sphere patch with a POLE and no rim at the top):
/// quarter-disc profile [(0,0),(1,0),arc to (0,1)] revolved θ = π/2.
/// V = (θ/2π)·(2π/3) = π/6.
/// A = (θ/2π)·2π (spherical) + 2·(π/4) (meridian quarter-discs)
///   + (θ/2)·1² (floor sector) = π/2 + π/2 + π/4 = 5π/4.
#[test]
fn dome_wedge_matches_independent_closed_forms() {
    let b = (PI / 8.0).tan(); // quarter arc (1,0) → (0,1) about origin
    let lp = ProfileLoop::new(vec![v(0.0, 0.0, 0.0), v(1.0, 0.0, b), v(0.0, 1.0, 0.0)]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap();
    check(&t.body, "dome wedge", PI / 6.0, 5.0 * PI / 4.0);
}

/// Pac-man prism (a MAJOR arc, 3π/2, as profile boundary): unit-circle
/// arc from (1,0) CCW the long way to (0,−1) (bulge tan(3π/8)), closed
/// through the center. Region = the 3π/2 pie sector (the two straight
/// edges are radii), area ½r²θ = 3π/4.
/// V = (3π/4)·1;
/// A = 2·(3π/4) + (3π/2)·1 + 1 + 1 = 3π + 2.
/// (First derivation here said 3π/4 + 1/2 — a reviewer arithmetic
/// slip, caught by the kernel value and re-derived by segment
/// decomposition: sector = chord triangle (−1/2 signed) + major
/// segment ½(θ − sin θ) = 3π/4 + 1/2 − 1/2.)
#[test]
fn major_arc_prism_matches_independent_closed_forms() {
    let b = (3.0 * PI / 8.0).tan();
    let lp = ProfileLoop::new(vec![v(0.0, 0.0, 0.0), v(1.0, 0.0, b), v(0.0, -1.0, 0.0)]);
    let t = extrude(
        &validated(vec![lp]),
        Extrusion::Distance(1.0),
        Tol::witness(),
    )
    .unwrap();
    check(&t.body, "pac-man prism", 0.75 * PI, 3.0 * PI + 2.0);
}

/// Two-hole plate (multi-ring planar caps): [−3,3]² with a round hole
/// (r = 1 at (−1.5, 0)) and a square hole [0.5,2.5]×[−1,1], height 1.
/// V = 36 − π − 4 = 32 − π;
/// A = 2(32 − π) + 24 + 2π + 8 = 96.
#[test]
fn two_hole_plate_matches_independent_closed_forms() {
    let outer = ProfileLoop::polygon([p2(-3.0, -3.0), p2(3.0, -3.0), p2(3.0, 3.0), p2(-3.0, 3.0)]);
    let round = ProfileLoop::new(vec![v(-0.5, 0.0, 1.0), v(-2.5, 0.0, 1.0)]);
    let square = ProfileLoop::polygon([p2(0.5, -1.0), p2(2.5, -1.0), p2(2.5, 1.0), p2(0.5, 1.0)]);
    let t = extrude(
        &validated(vec![outer, round, square]),
        Extrusion::Distance(1.0),
        Tol::witness(),
    )
    .unwrap();
    check(&t.body, "two-hole plate", 32.0 - PI, 96.0);
}

/// Orientation attack: NEGATIVE extrusion distance must still produce
/// a positive-volume, tier-3-valid body (never a silently inverted
/// shell).
#[test]
fn negative_extrusion_distance_is_positively_oriented() {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(0.0, 1.0)]);
    let t = extrude(
        &validated(vec![lp]),
        Extrusion::Distance(-1.5),
        Tol::witness(),
    )
    .unwrap();
    check(&t.body, "negative extrude", 3.0, 2.0 * 2.0 + 9.0);
}

/// Orientation attack: NEGATIVE revolve angle (θ = −π/2 sweeps the
/// other way) must still produce a positive-volume tier-3-valid wedge
/// with the θ-scaled closed forms.
#[test]
fn negative_revolve_angle_is_positively_oriented() {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(-FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap();
    check(&t.body, "negative wedge", 3.0 * PI / 4.0, 3.0 * PI + 2.0);
}

/// Scale attack: a large-scale washer — the V/A margin of the +V
/// check is a length and scales linearly; the closed forms must hold
/// to the same relative accuracy with no overflow or misclassification.
/// The scale tracks the run's ε (s = ε·1e14): f64 carries ~2e-16
/// relative noise, so a FIXED 1e6 scale at ε = 1e-12 is outside the
/// absolute-ε envelope and revolve correctly REFUSES to certify it
/// (typed `ResidualExceeded` — executed at the 1e-12 row; the honest
/// D4 posture, not a defect).
#[test]
fn megascale_washer_matches_and_validates() {
    let s = (geom_core::Tol::witness().get().eps * 1e14).max(1.0);
    let lp = ProfileLoop::polygon([p2(s, 0.0), p2(2.0 * s, 0.0), p2(2.0 * s, s), p2(s, s)]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap();
    check(
        &t.body,
        "megascale washer",
        3.0 * PI * s * s * s,
        12.0 * PI * s * s,
    );
}

/// PUBLIC-OP escape from the iso-rectangle inventory: split a washer
/// wall (cylinder face) with a diagonal chord `mef` — the new boundary
/// line is not a meridian, so the closed form must refuse TYPED
/// (`NotIsoRectangle` under the face) — never a silent number, never a
/// quadrature fallback — and tier 3 must refuse the body.
///
/// **Re-expressed at PCURVE P-1b, with a reachability loss stated
/// rather than papered over.** The tier-3 half used to read
/// `VolumeUncomputable`. It cannot any more, and the reason is
/// structural, not a re-baseline: check 7 runs only `if
/// errors.is_empty()` (cascade discipline — a volume computed from an
/// already-broken body is noise), and this fixture now trips an
/// earlier check. A straight chord between two points of a CYLINDER is
/// a secant: it lies in neither adjacent surface, so it has no chart
/// image and no intrinsic citation — no legal at-rest description
/// exists for it at all — and the `line_between` the row hands `mef`
/// therefore stays at the scaffolding door, which U2's transience
/// fence names at check 2.
///
/// So the defect this row plants is now caught EARLIER and by name,
/// which is what the row asserts. The closed form's typed refusal —
/// the actual subject — is untouched and still read directly from
/// `mass_properties`. The tier-3 `VolumeUncomputable` lane keeps its
/// own pins on bodies that ARE at rest and merely uncomputable
/// (`m5_pr12_fix_pass`, `step-import`'s `nurbs_import` / `freecad` /
/// `wild`), so nothing lost coverage — this fixture stopped being an
/// example of it.
#[test]
fn diagonal_chord_split_refuses_typed_not_silent() {
    use topo::{FaceSurface, LoopBoundary, MassPropsError, MefSite, ValidationError};
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    let t = revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap();
    let mut body = t.body;
    let wall = t.walls[0][1].expect("outer wall face");
    let outer = body.get_face(wall).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("wall loop must be a cycle");
    };
    let cycle = body.loop_cycle(first).unwrap();
    assert!(cycle.len() >= 4, "wall loop has 4 sides");
    let (he1, he2) = (cycle[0], cycle[2]);
    let point_of = |body: &topo::Body<f64>, he| {
        let v = body.get_half_edge(he).unwrap().start;
        *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
    };
    let (a, b) = (point_of(&body, he1), point_of(&body, he2));
    let split = body
        .mef(
            MefSite::Chords { he1, he2 },
            topo::EdgeCurveSpec::line_between(a, b),
            FaceSurface::Inherit,
            Tol::witness(),
        )
        .expect("diagonal chord split through the public operator");
    match mass_properties(&body, Tol::witness()) {
        Err(MassPropsError::Face { source, .. }) => {
            assert!(
                matches!(source, geom_brep::PropsError::NotIsoRectangle { .. }),
                "expected NotIsoRectangle, got {source:?}"
            );
        }
        other => panic!("expected typed refusal, got {other:?}"),
    }
    let errs = validate_geometric(&body, Tol::witness()).unwrap_err();
    assert_eq!(
        errs,
        vec![ValidationError::ScaffoldAtRest { edge: split.edge }],
        "tier 3 must name the secant chord that has no at-rest \
         description, and nothing else; got {errs:?}"
    );
}

/// Grooved and bumped washers — the torus faces where the solid lies
/// OUTSIDE the tube (`s_f = −1`, groove) and INSIDE it (`s_f = +1`,
/// bump), both through public ops on the [1,3]×[0,2] washer profile
/// with a semicircular arc (r = 1/2 centered (3,1)) in the outer wall.
/// Pappus (V = 2π∫∫x dA): rect term 2π·8 = 16π; half-disc term
///   2π·(π/8)(3 ∓ 2/(3π)) = 3π²/4 ∓ π/6 (− groove, + bump).
/// A: bore 4π + outer bands 6π + annuli 16π + half-tube
///   2πr[Rπ ∓ 2r] = 3π² ∓ π ⇒ A = 25π + 3π² (groove),
///   27π + 3π² (bump).
/// (Executed lesson pinned here: bulge +1 on the (3,0.5)→(3,1.5) arc
/// is the CCW arc bulging AWAY from the material — the bump; the
/// reviewer's first sign reading was wrong and the kernel's value
/// exposed it.)
#[test]
fn grooved_and_bumped_washer_torus_both_interior_sides_match() {
    let profile = |bulge: f64| {
        vec![
            v(1.0, 0.0, 0.0),
            v(3.0, 0.0, 0.0),
            v(3.0, 0.5, bulge),
            v(3.0, 1.5, 0.0),
            v(3.0, 2.0, 0.0),
            v(1.0, 2.0, 0.0),
        ]
    };
    let groove = revolve(
        &validated(vec![ProfileLoop::new(profile(-1.0))]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap();
    check(
        &groove.body,
        "grooved washer",
        16.0 * PI - 3.0 * PI * PI / 4.0 + PI / 6.0,
        25.0 * PI + 3.0 * PI * PI,
    );
    let bump = revolve(
        &validated(vec![ProfileLoop::new(profile(1.0))]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap();
    check(
        &bump.body,
        "bumped washer",
        16.0 * PI + 3.0 * PI * PI / 4.0 + PI / 6.0,
        27.0 * PI + 3.0 * PI * PI,
    );
}

/// Pappus magnitude cross-check on the review shapes (independent line
/// integral, not the divergence closed forms).
#[test]
fn pappus_cross_checks_review_revolves() {
    use revolve_common::full_pappus_y;
    let frustum = revolve(
        &validated(vec![ProfileLoop::polygon([
            p2(1.0, 0.0),
            p2(2.0, 0.0),
            p2(1.0, 1.0),
        ])]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap();
    let v = mass_properties(&frustum.body, Tol::witness())
        .unwrap()
        .volume;
    let pappus = full_pappus_y(&frustum);
    assert!(
        (v - pappus).abs() < 1e-3 * v,
        "frustum Pappus: exact {v} vs line integral {pappus}"
    );
}
