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

use crate::shared::point::{p3, v3};
use crate::shared::surf;
use crate::shared::tol::band;
use crate::shared::topo;
use crate::shared::topo::edge;
use geom::Curve3;
use geom::Surface;
use geom_brep::props::{
    LoopEdge, PropsError, curved_face, require_iso_rectangle, require_one_chart_branch,
};
use geom_core::Real;

// ---------------------------------------------------------------------
// Sphere — R = 10 mm about +Z at the origin (cert1_sphere_polar's)
// ---------------------------------------------------------------------

const RS: f64 = 0.010;

fn sphere<T: Real>() -> Surface<T> {
    surf::sphere(RS)
}

/// The rim at latitude `v`, `u0 → u1`.
fn rim<T: Real>(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<T> {
    topo::sphere_rim(RS, v, u0, u1, a, b)
}

/// The meridian great circle whose plane contains the axis at azimuth
/// `u`; its parameter IS the latitude on the `u` side, so `t = π/2` is
/// the north pole and `t ∈ (π/2, 3π/2)` descends the `u + π` side.
fn great<T: Real>(u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<T> {
    topo::sphere_great(RS, u, t0, t1, a, b)
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
    let face: Vec<LoopEdge<f64>> = vec![
        rim(b, 0.0, core::f64::consts::PI, 0, 1),
        great(0.0, core::f64::consts::PI - b, b, 1, 0),
    ];
    assert_eq!(
        require_iso_rectangle(&sphere::<f64>(), &face, band()),
        Ok(()),
        "the shape door certifies the carrier and the rim structure"
    );
    let exact = RS * RS * core::f64::consts::PI * (1.0 - b.sin());
    let fc = curved_face(&sphere::<f64>(), &face, 1.0, band()).expect("the flux lane measures it");
    assert!(
        (fc.area - exact).abs() / exact < 1e-12,
        "the closed form is exact on this face: {} vs {exact}",
        fc.area
    );
    let refusal = require_one_chart_branch(&sphere::<f64>(), &face, band());
    assert!(
        matches!(refusal, Err(PropsError::NotOneChartBranch { edge: 1, .. })),
        "the traversed arc runs over the pole, so it is not one chart branch: {refusal:?}"
    );
    // The payload's sentence is per-KIND (a sphere's azimuth jumps by
    // π at a pole; a cone's flips to the mirror nappe at the apex), so
    // it is pinned by `contains` on the fragments that carry the kind
    // and the mechanism, never as a whole string.
    let Err(e) = refusal else { unreachable!() };
    let text = e.to_string();
    for fragment in [
        "boundary edge 1",
        "sphere meridian arc",
        "contains a pole",
        "jumps by π",
    ] {
        assert!(
            text.contains(fragment),
            "{fragment:?} missing from {text:?}"
        );
    }
}

/// **The quiet side: an arc ENDING at a pole is admitted.** The
/// revolved cap — a full rim at latitude `b` and a meridian seam from
/// the rim to the north pole — is the shape every sphere cap in the
/// inventory has, and its span ends exactly at the pole. Both doors
/// admit it.
#[test]
fn an_arc_ending_exactly_at_a_pole_is_admitted() {
    let b = 0.5;
    let cap: Vec<LoopEdge<f64>> = vec![
        rim(b, 0.0, core::f64::consts::PI, 0, 1),
        great(core::f64::consts::PI, b, core::f64::consts::FRAC_PI_2, 1, 2),
        great(0.0, core::f64::consts::FRAC_PI_2, b, 2, 0),
    ];
    assert_eq!(
        require_iso_rectangle(&sphere::<f64>(), &cap, band()),
        Ok(())
    );
    assert_eq!(
        require_one_chart_branch(&sphere::<f64>(), &cap, band()),
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
        let face: Vec<LoopEdge<f64>> = vec![
            rim(b, 0.0, core::f64::consts::PI, 0, 1),
            great(0.0, core::f64::consts::PI - b, split, 1, 2),
            great(0.0, split, b, 2, 0),
        ];
        assert_eq!(
            require_one_chart_branch(&sphere::<f64>(), &face, band()),
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
    let face: Vec<LoopEdge<f64>> = vec![
        rim(b, 0.0, core::f64::consts::PI, 0, 1),
        great(0.0, core::f64::consts::PI - b, split, 1, 2),
        great(0.0, split, b, 2, 0),
    ];
    // Edge 1 spans `[split, π − b]`, which contains the pole at π/2.
    assert!(
        matches!(
            require_one_chart_branch(&sphere::<f64>(), &face, band()),
            Err(PropsError::NotOneChartBranch { edge: 1, .. })
        ),
        "a pole {} m inside the span is decisively inside it",
        d * RS
    );
    // The rim-bearing shape door and the closed form are unmoved.
    assert_eq!(
        require_iso_rectangle(&sphere::<f64>(), &face, band()),
        Ok(())
    );
}

/// **A rim is never measured against a pole.** An equatorial rim is a
/// great circle too — same centre, same radius — and the whole point
/// of splitting on `props_circle_axis_class` is that the pole-
/// membership arithmetic is a MERIDIAN's. A full equatorial rim
/// (span 2π, which contains every direction) must be admitted.
#[test]
fn an_equatorial_rim_is_not_a_meridian_and_is_admitted() {
    let band_face: Vec<LoopEdge<f64>> = vec![
        rim(0.0, 0.0, core::f64::consts::TAU, 0, 0),
        rim(1.0, core::f64::consts::TAU, 0.0, 1, 1),
        great(0.0, 0.0, 1.0, 0, 1),
        great(0.0, 1.0, 0.0, 1, 0),
    ];
    assert_eq!(
        require_one_chart_branch(&sphere::<f64>(), &band_face, band()),
        Ok(()),
        "a rim's v is constant over its whole circle, at any span"
    );
}

// ---------------------------------------------------------------------
// Cone — apex at the origin about +Z, half-angle π/4
// ---------------------------------------------------------------------

fn cone<T: Real>() -> Surface<T> {
    Surface::Cone {
        apex: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        half_angle: T::from_f64(core::f64::consts::FRAC_PI_4),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// The cone rim at signed slant length `v`, `u0 → u1`.
fn cone_rim<T: Real>(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<T> {
    let s = core::f64::consts::FRAC_1_SQRT_2;
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, v * s),
            axis: v3(0.0, 0.0, 1.0),
            radius: T::from_f64((v * s).abs()),
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
fn generator<T: Real>(u: f64, v0: f64, v1: f64, a: u32, b: u32) -> LoopEdge<T> {
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
    let bow: Vec<LoopEdge<f64>> = vec![
        cone_rim(-1.0, 0.0, core::f64::consts::PI, 0, 1),
        generator(core::f64::consts::PI, -1.0, 1.0, 1, 2),
        cone_rim(1.0, core::f64::consts::PI, 0.0, 2, 3),
        generator(0.0, 1.0, -1.0, 3, 0),
    ];
    assert_eq!(
        require_iso_rectangle(&cone::<f64>(), &bow, band()),
        Ok(()),
        "the shape door admits it: certified generators, rims at the extremes"
    );
    assert!(
        matches!(
            require_one_chart_branch(&cone::<f64>(), &bow, band()),
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
    let band_face: Vec<LoopEdge<f64>> = vec![
        cone_rim(1.0, 0.0, core::f64::consts::PI, 0, 1),
        generator(core::f64::consts::PI, 1.0, 2.0, 1, 2),
        cone_rim(2.0, core::f64::consts::PI, 0.0, 2, 3),
        generator(0.0, 2.0, 1.0, 3, 0),
    ];
    assert_eq!(
        require_one_chart_branch(&cone::<f64>(), &band_face, band()),
        Ok(())
    );
    let cap: Vec<LoopEdge<f64>> = vec![
        cone_rim(1.0, 0.0, core::f64::consts::PI, 0, 1),
        generator(core::f64::consts::PI, 0.0, 1.0, 1, 2),
        generator(0.0, 1.0, 0.0, 2, 0),
    ];
    assert_eq!(
        require_one_chart_branch(&cone::<f64>(), &cap, band()),
        Ok(()),
        "a generator that ENDS at the apex never leaves its branch"
    );
}

// ---------------------------------------------------------------------
// The immune kinds, executed rather than asserted in prose
// ---------------------------------------------------------------------

/// **The cylinder has no branch to leave** — and this row cannot go
/// red at the arm, which is worth saying rather than leaving a reader
/// to discover it. The cylinder arm is `Ok(())` unconditionally, so no
/// input reddens it; what this row DOES execute is the dispatch (the
/// door answers `Ok` rather than `Unimplemented` or a panic for a
/// cylinder, at spans no other row uses — a generator ten radii long,
/// a rim wrapping twice round). The REASON the arm is unconditional is
/// executed elsewhere and cannot be executed here: `Chart::poles()`
/// lists a cylinder's singularities as none, and `topo` is above this
/// crate, so the row that ties the arm set to that enumeration lives
/// at `mesh/tests/mesh11_arc_branch.rs::the_branch_doors_arms_mirror_
/// the_charts_own_singularities`. Asserted here, executed there.
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
    let face: Vec<LoopEdge<f64>> = vec![
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
/// branch — including a span that wraps the minor angle right past its
/// period. (The extent that wrap implies is a different question, and
/// MESH-10's fold owns it.)
///
/// Same caveat as the cylinder row above: the torus arm is `Ok(())`
/// unconditionally, so this row executes the dispatch and the wrap,
/// not the reason. The reason — `Chart::poles()` is empty for a torus
/// — is tied to the arm set by the `mesh`-side row that can see both
/// crates.
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
    let face: Vec<LoopEdge<f64>> = vec![
        minor(0.0, 0.0, 1.5 * core::f64::consts::TAU, 0, 1),
        minor(1.0, 1.5 * core::f64::consts::TAU, 0.0, 1, 0),
    ];
    assert_eq!(require_one_chart_branch(&tor, &face, band()), Ok(()));
}

/// **A plane is not this predicate's question**, and it says so typed
/// rather than answering "yes" for a chart it has no singularity in.
#[test]
fn a_plane_is_refused_typed_rather_than_answered() {
    let plane: Surface<f64> = Surface::Plane {
        origin: p3(0.0, 0.0, 0.0),
        normal: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    assert!(matches!(
        require_one_chart_branch(&plane, &[], band()),
        Err(PropsError::NotIsoRectangle { .. })
    ));
}

// ---------------------------------------------------------------------
// The floor, from both sides and along the whole band
// ---------------------------------------------------------------------

/// The half-cap split by a vertex `d` radians short of the north pole,
/// so the pole sits `d` INSIDE edge 1's span `[split, π − b]`. The
/// point deviation the door decides is the chord to the nearer span
/// end levered at `RS`, which for these offsets is `RS·d` to within a
/// relative `d²/24`.
fn split_short_of_the_pole(d: f64) -> Vec<LoopEdge<f64>> {
    let b = 0.5;
    let split = core::f64::consts::FRAC_PI_2 - d;
    vec![
        rim(b, 0.0, core::f64::consts::PI, 0, 1),
        great(0.0, core::f64::consts::PI - b, split, 1, 2),
        great(0.0, split, b, 2, 0),
    ]
}

/// **The floor is `Band::escalate`, not ε — the whole ladder.** The
/// unit's first rows exercised only the two ends (a `Zero` at half the
/// coincidence width, a decisive `Positive` at ten escalation widths)
/// and never the INDETERMINATE rung between them, which is the rung
/// that decides whether this door's threshold is `ε` or `K·ε`. It is
/// `K·ε`: a singularity five coincidence widths inside a span is
/// ADMITTED, because five widths is inside the ambiguity band and this
/// door admits everything that is not a definite `Positive`.
///
/// The rungs are derived from the run's own `Band`, so the row means
/// the same at every ε, and the indeterminate rung ASSERTS its band
/// placement (`zero < m < escalate`) from the same chord arithmetic
/// the door uses rather than trusting the name.
#[test]
fn the_branch_floor_is_the_escalation_threshold_and_not_epsilon() {
    let band = band();
    // Angular offsets whose point deviation at RS is the named
    // multiple of the band's own thresholds.
    let rung = |metres: f64| metres / RS;
    let indeterminate_mid = 0.5 * (band.zero() + band.escalate());
    for (name, d, refuses) in [
        ("0.25 x zero", rung(0.25 * band.zero()), false),
        ("1 x zero", rung(band.zero()), false),
        ("the indeterminate midpoint", rung(indeterminate_mid), false),
        ("0.99 x escalate", rung(0.99 * band.escalate()), false),
        ("1.01 x escalate", rung(1.01 * band.escalate()), true),
        ("4 x escalate", rung(4.0 * band.escalate()), true),
    ] {
        let face = split_short_of_the_pole(d);
        let got = require_one_chart_branch(&sphere::<f64>(), &face, band);
        assert_eq!(
            matches!(got, Err(PropsError::NotOneChartBranch { edge: 1, .. })),
            refuses,
            "rung {name}: offset {d} rad = {} m at the arc's lever; got {got:?}",
            d * RS
        );
        // The shape door and the closed form are unmoved at every rung.
        assert_eq!(require_iso_rectangle(&sphere::<f64>(), &face, band), Ok(()));
    }
    // The indeterminate rung really is indeterminate: its margin is
    // the chord to the nearer span end levered at RS, and it lands
    // strictly between the two thresholds.
    let d = rung(indeterminate_mid);
    let m = RS * 2.0 * (d * 0.5).sin();
    assert!(
        band.zero() < m && m < band.escalate(),
        "the midpoint rung's margin {m:e} must sit inside ({:e}, {:e})",
        band.zero(),
        band.escalate()
    );
}

/// **The same bracket on the SOUTH pole**, the second entry of
/// `sphere_meridian_pole_margins`' pair, which no other row in this
/// file crosses: every refusing sphere row above crosses the north
/// pole. Mirror side, same floor.
#[test]
fn the_branch_floor_brackets_escalate_on_the_south_pole_too() {
    let band = band();
    // The south pole sits at parameter 3π/2 on `great`'s circle.
    let south = 1.5 * core::f64::consts::PI;
    for (d, refuses) in [
        (0.99 * band.escalate() / RS, false),
        (1.01 * band.escalate() / RS, true),
    ] {
        let face: Vec<LoopEdge<f64>> = vec![
            rim(-0.5, core::f64::consts::PI, 0.0, 0, 1),
            great(0.0, core::f64::consts::PI + 0.5, south + d, 1, 2),
            great(0.0, south + d, core::f64::consts::TAU - 0.5, 2, 0),
        ];
        let got = require_one_chart_branch(&sphere::<f64>(), &face, band);
        assert_eq!(
            matches!(got, Err(PropsError::NotOneChartBranch { .. })),
            refuses,
            "south pole at {d} rad past the split; got {got:?}"
        );
    }
}

/// **The cone's floor, bracketed the same way, on both sides of the
/// apex.** The cone margin is a line parameter and IS metres, so the
/// bracket needs no lever: the apex `d` metres inside the span.
#[test]
fn the_cone_branch_floor_brackets_escalate_from_both_sides() {
    let band = band();
    for (d, refuses) in [
        (0.99 * band.escalate(), false),
        (1.01 * band.escalate(), true),
    ] {
        // Two mirror shapes: the apex `d` inside the span from the
        // negative end, and `d` inside it from the positive end.
        for (v0, v1) in [(-d, 1.0), (-1.0, d)] {
            let face: Vec<LoopEdge<f64>> = vec![
                cone_rim(1.0, 0.0, core::f64::consts::PI, 0, 1),
                generator(core::f64::consts::PI, v0, v1, 1, 2),
                generator(0.0, v1, v0, 2, 0),
            ];
            let got = require_one_chart_branch(&cone::<f64>(), &face, band);
            assert_eq!(
                matches!(got, Err(PropsError::NotOneChartBranch { .. })),
                refuses,
                "apex {d:e} m inside span ({v0}, {v1}); got {got:?}"
            );
        }
    }
}

/// **A `Line` that lies on no cone is not read as a generator** (the
/// arm's own rim/meridian guard). The sphere arm filters circles on
/// `props_circle_axis_class` before measuring a pole against them; the
/// cone arm now filters lines on `props_meridian_apex` the same way,
/// so a line that misses the apex is skipped rather than refused with
/// a sentence about a singularity it never approaches. Without the
/// guard this row answers `NotOneChartBranch` where the shape door
/// answers `props_meridian_generator`.
#[test]
fn a_line_that_misses_the_apex_is_not_read_as_a_generator() {
    let s = core::f64::consts::FRAC_1_SQRT_2;
    let offset: Vec<LoopEdge<f64>> = vec![edge(
        Curve3::Line {
            origin: p3(0.0, 1.0, 0.0),
            dir: v3(s, 0.0, s),
        },
        -1.0,
        1.0,
        0,
        1,
    )];
    assert_eq!(
        require_one_chart_branch(&cone::<f64>(), &offset, band()),
        Ok(()),
        "a line a metre off the apex has no apex in its span to cross"
    );
}

// ---------------------------------------------------------------------
// The asked-for lane: the predicate EXECUTED at the interval scalar
// ---------------------------------------------------------------------

/// **The branch door on `LoopEdge<Interval>`, refusing and admitting,
/// on both kinds.** The unit asked CI for the interval lane on the
/// argument that this predicate is `Decide`-generic and its
/// `copysign` / `min` / `decide` arithmetic is new code there; a lane
/// that only COMPILES the arithmetic does not test that argument.
/// These rows run it.
///
/// The interval scalar's `copysign` on a sign enclosure straddling
/// zero yields the two-sided hull `±chord`, which is indeterminate —
/// and indeterminate ADMITS here — so the refusing rows are placed
/// decisively past the floor (ten escalation widths), where the
/// enclosure is one-signed and the refusal must survive the widening.
#[cfg(feature = "interval")]
#[test]
fn the_branch_door_decides_at_the_interval_scalar() {
    use geom_core::Interval;
    let band = band();
    let b = 0.5;
    let far = 10.0 * band.escalate() / RS;

    // Sphere, refusing: the pole decisively inside edge 1's span.
    let split = core::f64::consts::FRAC_PI_2 - far;
    let crossing: Vec<LoopEdge<Interval>> = vec![
        rim(b, 0.0, core::f64::consts::PI, 0, 1),
        great(0.0, core::f64::consts::PI - b, split, 1, 2),
        great(0.0, split, b, 2, 0),
    ];
    assert!(
        matches!(
            require_one_chart_branch(&sphere::<Interval>(), &crossing, band),
            Err(PropsError::NotOneChartBranch { edge: 1, .. })
        ),
        "the interval lane must reach the same refusal, and name the same edge"
    );

    // Sphere, admitting: the cap whose meridians END at the pole.
    let cap: Vec<LoopEdge<Interval>> = vec![
        rim(b, 0.0, core::f64::consts::PI, 0, 1),
        great(core::f64::consts::PI, b, core::f64::consts::FRAC_PI_2, 1, 2),
        great(0.0, core::f64::consts::FRAC_PI_2, b, 2, 0),
    ];
    assert_eq!(
        require_one_chart_branch(&sphere::<Interval>(), &cap, band),
        Ok(()),
        "an arc ending at the pole is admitted at interval as it is at f64"
    );

    // Cone, refusing: the apex decisively inside the generator's span.
    let bow: Vec<LoopEdge<Interval>> = vec![
        cone_rim(-1.0, 0.0, core::f64::consts::PI, 0, 1),
        generator(core::f64::consts::PI, -1.0, 1.0, 1, 2),
        cone_rim(1.0, core::f64::consts::PI, 0.0, 2, 3),
        generator(0.0, 1.0, -1.0, 3, 0),
    ];
    assert!(
        matches!(
            require_one_chart_branch(&cone::<Interval>(), &bow, band),
            Err(PropsError::NotOneChartBranch { edge: 1, .. })
        ),
        "the cone arm decides at interval too"
    );

    // Cone, admitting: the single-nappe band.
    let single: Vec<LoopEdge<Interval>> = vec![
        cone_rim(1.0, 0.0, core::f64::consts::PI, 0, 1),
        generator(core::f64::consts::PI, 1.0, 2.0, 1, 2),
        cone_rim(2.0, core::f64::consts::PI, 0.0, 2, 3),
        generator(0.0, 2.0, 1.0, 3, 0),
    ];
    assert_eq!(
        require_one_chart_branch(&cone::<Interval>(), &single, band),
        Ok(()),
        "no apex in any span: admitted at interval"
    );
}
