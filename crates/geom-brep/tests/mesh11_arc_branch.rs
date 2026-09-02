//! **The branch predicate: each traversed ARC on one chart branch**
//! (issue 1571). `props::require_iso_rectangle` certifies each edge's
//! CARRIER and the face's rim structure; `require_one_chart_branch`
//! certifies the traversed ARC, and these rows pin both sides of the
//! seam between them on every kind.
//!
//! The rows that matter most are the ones asserting the two doors
//! DISAGREE on one face. That is not a defect: the extent derivations
//! fold a pole into the face's extent and measure a pole-crossing arc
//! exactly (`cert1_sphere_polar.rs`), so the flux lane must keep
//! admitting it, while a lane reading one chart coordinate per edge
//! cannot read that edge at all. Every offset in a band row comes from
//! the run's OWN `Band`, never from an ε literal — this file is on
//! CI's `eps ∈ {default, 1e-6, 1e-12}` matrix.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::Surface;
use geom_brep::props::{
    LoopEdge, PropsError, curved_face, require_iso_rectangle, require_one_chart_branch,
};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};

fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}
fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}
fn edge(carrier: Curve3<f64>, a: f64, b: f64, start: u32, end: u32) -> LoopEdge<f64> {
    let (t0, t1, forward) = if a < b { (a, b, true) } else { (b, a, false) };
    LoopEdge::hand_built(carrier, t0, t1, forward, start, end)
}

// ---------------------------------------------------------------------
// Sphere — R = 10 mm about +Z at the origin (cert1_sphere_polar's)
// ---------------------------------------------------------------------

const RS: f64 = 0.010;

fn sphere() -> Surface<f64> {
    Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: RS,
        axis: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// The rim at latitude `v`, `u0 → u1`.
fn rim(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, RS * v.sin()),
            axis: v3(0.0, 0.0, 1.0),
            radius: RS * v.cos(),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        a,
        b,
    )
}

/// The meridian great circle whose plane contains the axis at azimuth
/// `u`; its parameter IS the latitude on the `u` side, so `t = π/2` is
/// the north pole and `t ∈ (π/2, 3π/2)` descends the `u + π` side.
fn great(u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, 0.0),
            axis: v3(u.sin(), -u.cos(), 0.0),
            radius: RS,
            u_ref: v3(u.cos(), u.sin(), 0.0),
        },
        t0,
        t1,
        a,
        b,
    )
}

/// **The half-cap, on which the two doors DISAGREE — and both are
/// right** (issue 1571's witness at the predicate). The domain is
/// `[0, π] × [b, π/2]`: one rim half-circle at latitude `b` and ONE
/// meridian great-circle arc from `(0, b)` over the north pole down to
/// `(π, b)`. Its carrier is a certified meridian great circle and its
/// rims are at the extremes, so the shape door admits it; its extent
/// folds the pole in, so the closed form measures it EXACTLY
/// (`cert1_sphere_polar::a_pole_crossing_meridian_arc_measures_the_
/// half_cap_exactly` is the same face, split). The traversed arc lies
/// on TWO chart meridians, so the branch door refuses it, naming the
/// edge.
#[test]
fn a_pole_crossing_meridian_arc_is_not_one_chart_branch() {
    let b = 0.5;
    let face = vec![
        rim(b, 0.0, core::f64::consts::PI, 0, 1),
        great(0.0, core::f64::consts::PI - b, b, 1, 0),
    ];
    assert_eq!(
        require_iso_rectangle(&sphere(), &face, band()),
        Ok(()),
        "the shape door certifies the carrier and the rim structure"
    );
    let exact = RS * RS * core::f64::consts::PI * (1.0 - b.sin());
    let fc = curved_face(&sphere(), &face, 1.0, band()).expect("the flux lane measures it");
    assert!(
        (fc.area - exact).abs() / exact < 1e-12,
        "the closed form is exact on this face: {} vs {exact}",
        fc.area
    );
    assert_eq!(
        require_one_chart_branch(&sphere(), &face, band()),
        Err(PropsError::NotOneChartBranch {
            edge: 1,
            what: "a sphere meridian arc whose stored span contains a pole — the chart \
                   singularity, where u jumps by π",
        }),
        "the traversed arc runs over the pole, so it is not one chart branch"
    );
}

/// **The quiet side: an arc ENDING at a pole is admitted.** The
/// revolved cap — a full rim at latitude `b` and a meridian seam from
/// the rim to the north pole — is the shape every sphere cap in the
/// inventory has, and its span ends exactly at the pole. Both doors
/// admit it.
#[test]
fn an_arc_ending_exactly_at_a_pole_is_admitted() {
    let b = 0.5;
    let cap = vec![
        rim(b, 0.0, core::f64::consts::PI, 0, 1),
        great(core::f64::consts::PI, b, core::f64::consts::FRAC_PI_2, 1, 2),
        great(0.0, core::f64::consts::FRAC_PI_2, b, 2, 0),
    ];
    assert_eq!(require_iso_rectangle(&sphere(), &cap, band()), Ok(()));
    assert_eq!(
        require_one_chart_branch(&sphere(), &cap, band()),
        Ok(()),
        "a span that ENDS at the pole never leaves its branch"
    );
}

/// **The band's own quiet side, at every ε.** The same half-cap split
/// by a vertex a hair off the pole: the sub-arc's span end sits inside
/// the run's own band of the pole, so the membership margin is `Zero`
/// or indeterminate — and the branch door admits there, exactly as the
/// extent fold folds there (`cert1_sphere_polar::a_split_vertex_a_hair
/// _off_the_pole_still_certifies` is the same offsets on the same
/// face). A door that refused an in-band margin would refuse a cap
/// whose area is not in doubt.
#[test]
fn a_split_vertex_a_hair_off_the_pole_is_admitted() {
    let b = 0.5;
    // Offsets in RADIANS whose point deviation at R is inside the
    // run's own band: the arc's lever is R, so an angular offset d
    // moves the pole R·d.
    let mid_band = 0.5 * (band().zero() / RS);
    for d in [mid_band, 0.25 * mid_band] {
        let split = core::f64::consts::FRAC_PI_2 - d;
        let face = vec![
            rim(b, 0.0, core::f64::consts::PI, 0, 1),
            great(0.0, core::f64::consts::PI - b, split, 1, 2),
            great(0.0, split, b, 2, 0),
        ];
        assert_eq!(
            require_one_chart_branch(&sphere(), &face, band()),
            Ok(()),
            "at an offset of {d} rad ({} m at R) the pole is at a span end, not inside it",
            d * RS
        );
    }
}

/// **The refusal floor: it begins where the pole clears the band.**
/// The same split, pushed until the pole sits a decisive distance
/// inside the sub-arc's span — ten escalation widths at the arc's own
/// lever. Below the coincidence threshold the row above admits; here
/// the honest answer is the typed refusal.
#[test]
fn the_branch_refusal_begins_when_the_pole_clears_the_band() {
    let b = 0.5;
    let d = 10.0 * band().escalate() / RS;
    let split = core::f64::consts::FRAC_PI_2 - d;
    let face = vec![
        rim(b, 0.0, core::f64::consts::PI, 0, 1),
        great(0.0, core::f64::consts::PI - b, split, 1, 2),
        great(0.0, split, b, 2, 0),
    ];
    // Edge 1 spans `[split, π − b]`, which contains the pole at π/2.
    assert!(
        matches!(
            require_one_chart_branch(&sphere(), &face, band()),
            Err(PropsError::NotOneChartBranch { edge: 1, .. })
        ),
        "a pole {} m inside the span is decisively inside it",
        d * RS
    );
    // The rim-bearing shape door and the closed form are unmoved.
    assert_eq!(require_iso_rectangle(&sphere(), &face, band()), Ok(()));
}

/// **A rim is never measured against a pole.** An equatorial rim is a
/// great circle too — same centre, same radius — and the whole point
/// of splitting on `props_circle_axis_class` is that the pole-
/// membership arithmetic is a MERIDIAN's. A full equatorial rim
/// (span 2π, which contains every direction) must be admitted.
#[test]
fn an_equatorial_rim_is_not_a_meridian_and_is_admitted() {
    let band_face = vec![
        rim(0.0, 0.0, core::f64::consts::TAU, 0, 0),
        rim(1.0, core::f64::consts::TAU, 0.0, 1, 1),
        great(0.0, 0.0, 1.0, 0, 1),
        great(0.0, 1.0, 0.0, 1, 0),
    ];
    assert_eq!(
        require_one_chart_branch(&sphere(), &band_face, band()),
        Ok(()),
        "a rim's v is constant over its whole circle, at any span"
    );
}

// ---------------------------------------------------------------------
// Cone — apex at the origin about +Z, half-angle π/4
// ---------------------------------------------------------------------

fn cone() -> Surface<f64> {
    Surface::Cone {
        apex: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        half_angle: core::f64::consts::FRAC_PI_4,
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// The cone rim at signed slant length `v`, `u0 → u1`.
fn cone_rim(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    let s = core::f64::consts::FRAC_1_SQRT_2;
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, v * s),
            axis: v3(0.0, 0.0, 1.0),
            radius: (v * s).abs(),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        a,
        b,
    )
}

/// The generator through the apex whose positive half sits at azimuth
/// `u`, from signed slant `v0` to `v1` — the line parameter IS the
/// slant length, so a span with `v0 < 0 < v1` runs through the apex.
fn generator(u: f64, v0: f64, v1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    let s = core::f64::consts::FRAC_1_SQRT_2;
    edge(
        Curve3::Line {
            origin: p3(0.0, 0.0, 0.0),
            dir: v3(u.cos() * s, u.sin() * s, s),
        },
        v0,
        v1,
        a,
        b,
    )
}

/// **The cone's apex is the same defect, and the shape door admits it**
/// (this unit's class-sweep finding). A "bow tie" across both nappes —
/// rims at slant ∓1 joined by two generators that run THROUGH the apex
/// — passes `require_iso_rectangle` on every edge: the carriers are
/// certified generators through the apex and coaxial rims, and both
/// rims sit at the extremes of `min_max`. The traversed segments leave
/// the branch at the apex, where `u` jumps to the mirror nappe, and
/// the branch door is what says so. (`NappeSpanning` is the FLUX arm's
/// refusal and is not on the shape door's path — `mesh` cites the
/// shape door, not `curved_face`.)
#[test]
fn an_apex_crossing_generator_is_not_one_chart_branch() {
    let bow = vec![
        cone_rim(-1.0, 0.0, core::f64::consts::PI, 0, 1),
        generator(core::f64::consts::PI, -1.0, 1.0, 1, 2),
        cone_rim(1.0, core::f64::consts::PI, 0.0, 2, 3),
        generator(0.0, 1.0, -1.0, 3, 0),
    ];
    assert_eq!(
        require_iso_rectangle(&cone(), &bow, band()),
        Ok(()),
        "the shape door admits it: certified generators, rims at the extremes"
    );
    assert!(
        matches!(
            require_one_chart_branch(&cone(), &bow, band()),
            Err(PropsError::NotOneChartBranch { edge: 1, .. })
        ),
        "the first generator's span runs through the apex"
    );
}

/// **The cone's quiet side.** A single-nappe band — the same rims and
/// generators with both slants positive — is admitted by both doors,
/// and so is a generator that ENDS at the apex (the cone cap every
/// `revolve` mints).
#[test]
fn a_single_nappe_cone_band_and_an_apex_endpoint_are_admitted() {
    let band_face = vec![
        cone_rim(1.0, 0.0, core::f64::consts::PI, 0, 1),
        generator(core::f64::consts::PI, 1.0, 2.0, 1, 2),
        cone_rim(2.0, core::f64::consts::PI, 0.0, 2, 3),
        generator(0.0, 2.0, 1.0, 3, 0),
    ];
    assert_eq!(require_one_chart_branch(&cone(), &band_face, band()), Ok(()));
    let cap = vec![
        cone_rim(1.0, 0.0, core::f64::consts::PI, 0, 1),
        generator(core::f64::consts::PI, 0.0, 1.0, 1, 2),
        generator(0.0, 1.0, 0.0, 2, 0),
    ];
    assert_eq!(
        require_one_chart_branch(&cone(), &cap, band()),
        Ok(()),
        "a generator that ENDS at the apex never leaves its branch"
    );
}

// ---------------------------------------------------------------------
// The immune kinds, executed rather than asserted in prose
// ---------------------------------------------------------------------

/// **The cylinder has no branch to leave.** Its chart has no
/// singularity: `v` is the axial coordinate and a generator is a line
/// PARALLEL to the axis, so no span of it can reach an axis point;
/// rims are coaxial circles at constant `v`. The row runs the extreme
/// spans anyway — a generator ten radii long and a rim wrapping twice
/// round — and both are admitted.
#[test]
fn a_cylinder_has_no_chart_singularity_for_an_arc_to_cross() {
    let cyl = Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let rim_at = |z: f64, t0: f64, t1: f64, a, b| {
        edge(
            Curve3::Circle {
                center: p3(0.0, 0.0, z),
                axis: v3(0.0, 0.0, 1.0),
                radius: 1.0,
                u_ref: v3(1.0, 0.0, 0.0),
            },
            t0,
            t1,
            a,
            b,
        )
    };
    let face = vec![
        rim_at(-5.0, 0.0, 2.0 * core::f64::consts::TAU, 0, 0),
        edge(
            Curve3::Line {
                origin: p3(1.0, 0.0, 0.0),
                dir: v3(0.0, 0.0, 1.0),
            },
            -5.0,
            5.0,
            0,
            1,
        ),
        rim_at(5.0, 0.0, core::f64::consts::TAU, 1, 1),
    ];
    assert_eq!(require_one_chart_branch(&cyl, &face, band()), Ok(()));
}

/// **A torus minor circle never meets the axis on a ring torus**, so
/// its `u` is constant over the whole circle and no span leaves the
/// branch — including a span that wraps the minor angle right past
/// its period. (The extent that wrap implies is a different question,
/// and MESH-10's fold owns it.)
#[test]
fn a_torus_minor_circle_arc_stays_on_one_meridian_at_any_span() {
    let tor = Surface::Torus {
        center: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        major_radius: 1.0,
        minor_radius: 0.25,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let minor = |u: f64, t0: f64, t1: f64, a, b| {
        edge(
            Curve3::Circle {
                center: p3(u.cos(), u.sin(), 0.0),
                axis: v3(-u.sin(), u.cos(), 0.0),
                radius: 0.25,
                u_ref: v3(u.cos(), u.sin(), 0.0),
            },
            t0,
            t1,
            a,
            b,
        )
    };
    let face = vec![
        minor(0.0, 0.0, 1.5 * core::f64::consts::TAU, 0, 1),
        minor(1.0, 1.5 * core::f64::consts::TAU, 0.0, 1, 0),
    ];
    assert_eq!(require_one_chart_branch(&tor, &face, band()), Ok(()));
}

/// **A plane is not this predicate's question**, and it says so typed
/// rather than answering "yes" for a chart it has no singularity in.
#[test]
fn a_plane_is_refused_typed_rather_than_answered() {
    let plane = Surface::Plane {
        origin: p3(0.0, 0.0, 0.0),
        normal: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    assert!(matches!(
        require_one_chart_branch(&plane, &[], band()),
        Err(PropsError::NotIsoRectangle { .. })
    ));
}
