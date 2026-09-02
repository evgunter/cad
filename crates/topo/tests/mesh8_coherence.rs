//! **The three relocated coherence conditions, on their witnesses**
//! (issue 868). Each row builds a body whose data states one chart
//! coordinate twice, and pins what
//! [`topo::examine_chart_coherence`] measures between the two
//! statements — the gap, the lever arm, and the metres it opens.
//!
//! Two of the bodies are the recorded π-rad witnesses and are the
//! reason the relocation had to keep a detector rather than drop one:
//!
//! - **issue 1571** — a unit sphere with a rim at latitude `asin 0.5`
//!   and ONE great-circle arc from the rim's `u = π` end OVER the north
//!   pole to its `u = 0` end, assembled through the Euler doors. The
//!   traversed arc lies on TWO chart meridians, so the carrier's
//!   mid-parameter azimuth sits a half-turn from its own endpoint. The
//!   issue owns FIXING the arc premise; this row owns REPORTING it.
//! - **issue 723's shape** — the same arc split by one ordinary
//!   vertex, which is how the imported half-cap states it: two
//!   sub-edges of one meridian column, one on each side of the pole.
//!
//! The other two are synthetic and take their scale from the RUN'S OWN
//! ε, never from a literal: this file is on CI's
//! `eps ∈ {default, 1e-6, 1e-12}` matrix, and a literal offset states a
//! claim about one of the three.
//!
//! **What no row here can be.** A sub-ε wobble is not constructible
//! through these doors: an arc's span must certify, which puts a floor
//! of order ε on the arc length and therefore on the gap a chord can
//! open. The band's quiet side is pinned at the predicate instead, in
//! `topo::coherence`'s own unit rows.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::Tol;
use geom_core::{Point3, Vec3};
use topo::{Body, CoherenceCondition, CoherenceFinding, EdgeKey, FaceSurface, MefSite, MevSite};

fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}
fn unit_sphere() -> Surface<f64> {
    Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// The rim's latitude sine, and the rim radius that follows from it.
const RIM_Z: f64 = 0.5;
fn rim_r() -> f64 {
    (1.0 - RIM_Z * RIM_Z).sqrt()
}

/// **Issue 1571's body.** Returns it with the pole-crossing arc's key.
fn pole_crossing_half_cap() -> (Body<f64>, EdgeKey) {
    let tol = Tol::witness();
    let r = rim_r();
    let a = p3(r, 0.0, RIM_Z);
    let b = p3(-r, 0.0, RIM_Z);
    let rim = Curve3::Circle {
        center: p3(0.0, 0.0, RIM_Z),
        axis: v3(0.0, 0.0, 1.0),
        radius: r,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    // The great circle in the plane y = 0, oriented so the arc from `b`
    // to `a` passes over the north pole (increasing z first).
    let great = |axis: Vec3<f64>| Curve3::Circle {
        center: p3(0.0, 0.0, 0.0),
        axis,
        radius: 1.0,
        u_ref: v3(-r, 0.0, RIM_Z),
    };
    let mut g = great(v3(0.0, 1.0, 0.0));
    if g.eval(core::f64::consts::FRAC_PI_2).z < RIM_Z {
        g = great(v3(0.0, -1.0, 0.0));
    }
    let t_end = g.param_near(a, 0.0).unwrap();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a).unwrap();
    body.set_face_surface(seed.face, FaceSurface::New(unit_sphere()))
        .unwrap();
    let e_rim = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            EdgeCurveSpec::arc_of_circle(rim, 0.0, core::f64::consts::PI).unwrap(),
            tol,
        )
        .unwrap();
    let made = body
        .mef(
            MefSite::Chords {
                he1: e_rim.he_minus,
                he2: e_rim.he_plus,
            },
            EdgeCurveSpec::arc_of_circle(g, 0.0, t_end).unwrap(),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
    (body, made.edge)
}

/// A cylinder sliver face — a rim arc from `a` to `b`, and the straight
/// CHORD back. The chord is a line, so it reads as a meridian, and its
/// mid-parameter point sits at azimuth `θ/2` while its two endpoint
/// vertices sit at `0` and `θ`.
///
/// This is `nist_ftc_09_asme1_rd.stp`'s shape with the geometry made
/// exact: both vertices lie on the cylinder and the carrier runs
/// exactly between them, so nothing here is a mis-stated curve. The
/// source says "one line, endpoints off the axis it runs along", and
/// that is all it takes.
fn chord_wobble(metres: f64) -> Body<f64> {
    let tol = Tol::witness();
    let rr = 0.05_f64;
    // The closure gap is θ/2 at a lever arm of `rr`, so this opens
    // exactly `metres` of arc.
    let theta = 2.0 * metres / rr;
    let a = p3(rr, 0.0, 0.0);
    let b = p3(rr * theta.cos(), rr * theta.sin(), 0.0);
    let rim = Curve3::Circle {
        center: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius: rr,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Cylinder {
            origin: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, 0.0, 1.0),
            radius: rr,
            u_ref: v3(1.0, 0.0, 0.0),
        }),
    )
    .unwrap();
    let e = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            EdgeCurveSpec::arc_of_circle(rim, 0.0, theta).unwrap(),
            tol,
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: e.he_minus,
            he2: e.he_plus,
        },
        EdgeCurveSpec::line_between(b, a),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    body
}

/// A sphere cap whose ONE rim row is carried by TWO circles: the true
/// rim, and a second horizontal circle through the same two vertices
/// whose centre is off the axis by `c`.
///
/// Both carriers pass through both vertices EXACTLY, so no endpoint is
/// mis-stated — the two circles simply are not the same circle, which
/// is the whole of the condition. `c = 0` makes them one circle and
/// the row falls silent, which is the band's quiet side at the body
/// level.
///
/// The second circle cannot also lie on the sphere: a rim through two
/// given points is unique on every chart kind here, so a v-gap forces
/// the second carrier off the surface — by `~c`, which is of order
/// `sqrt(gap · R)` and therefore vastly larger than the gap itself.
fn two_circle_rim_cap(c: f64) -> Body<f64> {
    let tol = Tol::witness();
    let r = rim_r();
    let a = p3(r, 0.0, RIM_Z);
    let b = p3(-r, 0.0, RIM_Z);
    let true_rim = Curve3::Circle {
        center: p3(0.0, 0.0, RIM_Z),
        axis: v3(0.0, 0.0, 1.0),
        radius: r,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let other = Curve3::Circle {
        center: p3(0.0, c, RIM_Z),
        axis: v3(0.0, 0.0, 1.0),
        radius: (r * r + c * c).sqrt(),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let s = other.param_near(b, 0.0).unwrap();
    let e = other.param_near(a, 0.0).unwrap();
    let e = if e > s { e } else { e + core::f64::consts::TAU };
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a).unwrap();
    body.set_face_surface(seed.face, FaceSurface::New(unit_sphere()))
        .unwrap();
    let e1 = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            EdgeCurveSpec::arc_of_circle(true_rim, 0.0, core::f64::consts::PI).unwrap(),
            tol,
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: e1.he_minus,
            he2: e1.he_plus,
        },
        EdgeCurveSpec::arc_of_circle(other, s, e).unwrap(),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    body
}

/// Every finding of one condition kind, discriminated by variant.
fn of_kind(
    findings: &[CoherenceFinding],
    want: fn(&CoherenceCondition) -> bool,
) -> Vec<CoherenceFinding> {
    findings
        .iter()
        .copied()
        .filter(|f| want(&f.condition))
        .collect()
}

fn is_closure(c: &CoherenceCondition) -> bool {
    matches!(c, CoherenceCondition::MeridianClosure { .. })
}
fn is_meridian_continuation(c: &CoherenceCondition) -> bool {
    matches!(c, CoherenceCondition::MeridianContinuation { .. })
}
fn is_rim_continuation(c: &CoherenceCondition) -> bool {
    matches!(c, CoherenceCondition::RimContinuation { .. })
}

/// **Issue 1571's witness, reported.** The pole-crossing arc's
/// carrier-midpoint azimuth and its own `u = π` endpoint disagree by
/// exactly a half-turn, at the rim's radius — 2.72 m of arc on a unit
/// sphere, which no band of any size calls noise.
///
/// One finding per FACE: both faces of the body traverse that one arc,
/// and each states the condition about its own loop. The `u = 0`
/// endpoint is quiet, because the arc's mid-parameter point lies past
/// the pole on that side and its azimuth is that endpoint's exactly —
/// the gap is not a property of the edge, it is a property of the edge
/// AND the endpoint, which is why the vertex rides in the variant.
#[test]
fn the_pole_crossing_arc_reports_a_half_turn_closure_gap() {
    let (body, _) = pole_crossing_half_cap();
    let report = topo::examine_chart_coherence(&body, Tol::witness());
    assert!(report.unexamined.is_empty(), "{:?}", report.unexamined);
    let closures = of_kind(&report.findings, is_closure);
    assert_eq!(closures.len(), 2, "one per face: {:?}", report.findings);
    for f in &closures {
        assert!(
            (f.gap - core::f64::consts::PI).abs() < 1e-12,
            "a pole-crossing arc's carrier sits a half-turn from its own \
             endpoint, got {} rad",
            f.gap
        );
        assert!(
            (f.lever - rim_r()).abs() < 1e-12,
            "the lever arm is the endpoint's own distance from the axis, got {}",
            f.lever
        );
        assert!(
            (f.metres - core::f64::consts::PI * rim_r()).abs() < 1e-12,
            "got {} m",
            f.metres
        );
    }
    assert_eq!(
        of_kind(&report.findings, is_meridian_continuation).len(),
        0,
        "the arc is ONE edge here; the split form is the next row"
    );
}

/// **Issue 723's shape, reported.** Split that arc with one ordinary
/// vertex and the same half-turn shows up as a CONTINUATION: two
/// sub-edges of one meridian column, one on each side of the pole,
/// each stating its own column and disagreeing by π.
///
/// Both continuation findings appear — one per face — and so do the
/// closure findings, which is the difference a report makes: the walk's
/// assertion announced whichever condition it reached first and stopped
/// there, and on this body that was always the closure.
#[test]
fn a_split_pole_crossing_arc_reports_a_half_turn_column_gap() {
    let (mut body, arc) = pole_crossing_half_cap();
    let t = {
        let e = body.get_edge(arc).unwrap();
        let g = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
        let (t0, t1) = g.params();
        t0 + (t1 - t0) * 0.75
    };
    body.split_edge(arc, t, Tol::witness()).unwrap();
    let report = topo::examine_chart_coherence(&body, Tol::witness());
    assert!(report.unexamined.is_empty(), "{:?}", report.unexamined);
    let continuations = of_kind(&report.findings, is_meridian_continuation);
    assert_eq!(
        continuations.len(),
        2,
        "one per face: {:?}",
        report.findings
    );
    for f in &continuations {
        assert!(
            (f.gap - core::f64::consts::PI).abs() < 1e-12,
            "got {} rad",
            f.gap
        );
        assert!(
            f.metres > core::f64::consts::PI * 0.4,
            "a half-turn at the split vertex's own radius, got {} m",
            f.metres
        );
    }
    assert!(
        !of_kind(&report.findings, is_closure).is_empty(),
        "the closure condition is live on this body too, and a report says both"
    );
}

/// **The closure condition on a synthetic wobble**, at 1024 ε — the
/// `nist_ftc_09` shape, scaled to the run's own band. Both endpoints
/// are reported and both open the same arc, because the carrier's
/// mid-parameter azimuth sits exactly halfway between them.
#[test]
fn a_line_stated_off_axis_reports_its_closure_gap_at_both_ends() {
    let tol = Tol::witness();
    let want = 1024.0 * tol.eps();
    let report = topo::examine_chart_coherence(&chord_wobble(want), tol);
    assert!(report.unexamined.is_empty(), "{:?}", report.unexamined);
    let closures = of_kind(&report.findings, is_closure);
    assert_eq!(
        closures.len(),
        4,
        "two endpoints, two faces: {:?}",
        report.findings
    );
    for f in &closures {
        assert!(
            (f.metres - want).abs() < want * 1e-6,
            "expected {want} m of arc, got {} (gap {} rad, lever {} m)",
            f.metres,
            f.gap,
            f.lever
        );
        assert_eq!(f.eps, tol.eps(), "every finding carries its own band");
    }
}

/// **The rim-continuation condition**, on two carriers that are not
/// the same circle — and its quiet side, where they are.
///
/// The scale is derived from the band: the v-gap of a second
/// horizontal circle through both vertices whose centre is `c` off the
/// axis is `z̄·c²/(2r)` to leading order, so `c` is set from the metres
/// wanted rather than the other way round. That the offset goes as the
/// SQUARE ROOT of the gap is the reason this condition's witness cannot
/// be a near-miss: the second carrier is off the sphere by ~`c`, which
/// is `sqrt(ε R)`-sized while the gap is ε-sized.
#[test]
fn two_carriers_for_one_rim_row_report_their_v_gap() {
    let tol = Tol::witness();
    let want = 1024.0 * tol.eps();
    let c = (want * 2.0 * rim_r() / RIM_Z).sqrt();

    let quiet = topo::examine_chart_coherence(&two_circle_rim_cap(0.0), tol);
    assert!(
        quiet.findings.is_empty() && quiet.unexamined.is_empty(),
        "ONE circle carrying both sub-edges states one v: {quiet:?}"
    );

    let report = topo::examine_chart_coherence(&two_circle_rim_cap(c), tol);
    assert!(report.unexamined.is_empty(), "{:?}", report.unexamined);
    let rims = of_kind(&report.findings, is_rim_continuation);
    assert_eq!(rims.len(), 2, "one per face: {:?}", report.findings);
    // The gap in CLOSED FORM, not through the leading-order relation
    // that chose `c`: the two carriers state the same axial height and
    // different radii, and a sphere reads a rim's v as `atan2(h, ρ)`.
    // The inversion above is first-order and drifts a fraction of a
    // percent by 1e-6, where `c` is millimetres; the row asserts the
    // exact form and keeps the band only as the SCALE it was aimed at.
    let exact = RIM_Z.atan2(rim_r()) - RIM_Z.atan2((rim_r() * rim_r() + c * c).sqrt());
    assert!(
        (0.5 * want..2.0 * want).contains(&exact.abs()),
        "the offset was aimed at ~{want} m from the run's own band; the closed          form says {exact} rad"
    );
    for f in &rims {
        assert!(
            (f.metres - exact.abs()).abs() < exact.abs() * 1e-9,
            "expected the closed form {} m, got {} (gap {} rad of latitude,              lever {} m)",
            exact.abs(),
            f.metres,
            f.gap,
            f.lever
        );
        assert!(
            (f.lever - 1.0).abs() < 1e-12,
            "a sphere's v lever arm is its radius, got {}",
            f.lever
        );
    }
    assert_eq!(
        of_kind(&report.findings, is_closure).len(),
        0,
        "a rim row says nothing about any meridian: {:?}",
        report.findings
    );
}
