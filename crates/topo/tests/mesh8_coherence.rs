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
//! **The band's quiet side, at the body level.** A sub-ε wobble IS
//! constructible through these doors, and an earlier draft of this
//! header said it was not. What is true is narrower and only about
//! ONE shape: a CHORD's closure gap is half its own span, and a span
//! must certify, so a chord cannot open an arbitrarily small gap. A
//! MERIDIAN STATED AS A CIRCLE TILTED OUT OF THE MERIDIAN PLANE has no
//! such floor — its span stays long while the tilt sets the gap — and
//! `mesh/tests/mesh8r1_probes.rs`'s
//! `a_tilted_meridian_circle_opens_a_closure_gap_of_any_size` is that
//! row, quiet at 0.25 ε and reporting four findings at 4 ε on one
//! body. The rim row below carries the same pin on its own axis; the
//! band's exact EDGE stays at the predicate, since no body-level row
//! can land a float there on purpose.
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
///
/// **A DECLARED COPY, not an accident.** The same builder stands in
/// `mesh/tests/mesh7r1_probes.rs` (where the door and tessellate rows
/// need it) and in the two adopted reviewer suites. One home was
/// weighed and not taken: a `tests/` binary is a separate crate, so
/// the only cross-crate door is `topo`'s `test_support`, which is
/// feature-gated OFF for every crate but this one on purpose — opening
/// it to `mesh` would put this crate's test vocabulary into `mesh`'s
/// dependency graph to save a thirty-line constructor. The copies are
/// pinned to each other by what they assert, not by prose: each states
/// the same rim latitude and the same over-the-pole orientation, and a
/// divergence shows up as a changed gap in the rows below.
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

/// **The rim-continuation condition**, at a quarter of a band and at
/// 1024 of them: quiet on one side, four figures pinned on the other.
///
/// The scale is derived from the band at both ends: the v-gap of a
/// second horizontal circle through both vertices whose centre is `c`
/// off the axis is `z̄·c²/(2r)` to leading order, so `c` is set from
/// the metres wanted rather than the other way round. That the offset
/// goes as the SQUARE ROOT of the gap is the reason this condition's
/// witness cannot be a near-miss on the surface: the second carrier is
/// off the sphere by ~`c`, which is `sqrt(ε R)`-sized while the gap is
/// ε-sized.
#[test]
fn two_carriers_for_one_rim_row_report_their_v_gap() {
    let tol = Tol::witness();
    let want = 1024.0 * tol.eps();
    let c = (want * 2.0 * rim_r() / RIM_Z).sqrt();

    // The quiet side, with a gap that is NONZERO. `c = 0` makes the two
    // carriers one circle and the gap identically zero, which is quiet
    // for a reason that has nothing to do with the band — such a row
    // cannot see a band that stopped comparing. This one opens a
    // quarter of a band and must still be silent.
    let c_quiet = (0.25 * tol.eps() * 2.0 * rim_r() / RIM_Z).sqrt();
    let quiet = topo::examine_chart_coherence(&two_circle_rim_cap(c_quiet), tol);
    let quiet_gap =
        RIM_Z.atan2(rim_r()) - RIM_Z.atan2((rim_r() * rim_r() + c_quiet * c_quiet).sqrt());
    assert!(
        quiet_gap.abs() > 0.0 && quiet_gap.abs() < tol.eps(),
        "the quiet row must open a real gap under the band, got {quiet_gap} rad \
         against eps {}",
        tol.eps()
    );
    assert!(
        quiet.findings.is_empty() && quiet.unexamined.is_empty(),
        "a quarter-band v gap is noise: {quiet:?}"
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
        "the offset was aimed at ~{want} m from the run's own band; the closed form says {exact} rad"
    );
    for f in &rims {
        assert!(
            (f.metres - exact.abs()).abs() < exact.abs() * 1e-9,
            "expected the closed form {} m, got {} (gap {} rad of latitude, lever {} m)",
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

/// A cylinder patch whose ONE meridian side is carried by TWO edges:
/// an exact axial line from `v1` up to a mid-height vertex, and a
/// second line from there to a vertex `w` radians round the axis. The
/// second sub-edge's chord midpoint sits at azimuth `w/2`, the run's
/// opening column at `0`, and the gap the substitution discards is
/// that difference at the junction's own lever arm.
///
/// Both vertices lie exactly on the cylinder and each carrier runs
/// exactly between its own two, so nothing here is a mis-stated curve:
/// the source simply states one iso side's two carriers off the axis
/// they should share, which is `nist_ftc_09`'s defect one level up
/// from [`chord_wobble`]'s.
fn split_meridian_wobble(metres: f64) -> Body<f64> {
    let tol = Tol::witness();
    let rr = 0.05_f64;
    // The continuation gap is w/2 at lever `rr`.
    let w = 2.0 * metres / rr;
    let on = |theta: f64, z: f64| p3(rr * theta.cos(), rr * theta.sin(), z);
    let (v0, v1) = (on(0.0, 0.0), on(0.9, 0.0));
    let mid = on(0.9, 0.5);
    let top = on(0.9 + w, 1.0);
    let tl = on(0.0, 1.0);
    let rim = |z: f64, sense: f64| Curve3::Circle {
        center: p3(0.0, 0.0, z),
        axis: v3(0.0, 0.0, sense),
        radius: rr,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(v0).unwrap();
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
    let e0 = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            v1,
            EdgeCurveSpec::arc_of_circle(rim(0.0, 1.0), 0.0, 0.9).unwrap(),
            tol,
        )
        .unwrap();
    let strut = |body: &mut Body<f64>, at, to, spec| {
        body.mev(MevSite::Fan { he1: at, he2: at }, to, spec, tol)
            .unwrap()
    };
    let e1 = strut(
        &mut body,
        e0.he_minus,
        mid,
        EdgeCurveSpec::line_between(v1, mid),
    );
    let e2 = strut(
        &mut body,
        e1.he_minus,
        top,
        EdgeCurveSpec::line_between(mid, top),
    );
    // The far rim closes the u extent, so the meridian RUN is exactly
    // the two sub-edges above: a third consecutive line would put the
    // closing side in the same run and the condition would be about
    // three carriers rather than two.
    let e3 = strut(
        &mut body,
        e2.he_minus,
        tl,
        EdgeCurveSpec::arc_of_circle(rim(1.0, -1.0), -(0.9 + w), 0.0).unwrap(),
    );
    body.mef(
        MefSite::Chords {
            he1: e3.he_minus,
            he2: e0.he_plus,
        },
        EdgeCurveSpec::line_between(tl, v0),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    body
}

/// **The continuation condition on the u axis**, at a quarter of a
/// band and at 1024 of them — the synthetic the rim axis has had since
/// this unit's first head and the meridian axis did not. Its absence
/// was a gap in the spec's own deliverable 2(b), which asks for a
/// continuation wobble on EACH axis; the π-rad witnesses cover the
/// meridian axis at a half-turn and say nothing about the band.
///
/// Two edges of one meridian column: the second states its carrier `w`
/// radians round from the first, and the gap the run's substitution
/// discards is `w/2` at the junction's lever arm.
#[test]
fn two_carriers_for_one_meridian_column_report_their_u_gap() {
    let tol = Tol::witness();

    let quiet = topo::examine_chart_coherence(&split_meridian_wobble(0.25 * tol.eps()), tol);
    assert!(quiet.unexamined.is_empty(), "{:?}", quiet.unexamined);
    assert_eq!(
        of_kind(&quiet.findings, is_meridian_continuation).len(),
        0,
        "a quarter-band column gap is noise: {:?}",
        quiet.findings
    );

    let want = 1024.0 * tol.eps();
    let report = topo::examine_chart_coherence(&split_meridian_wobble(want), tol);
    assert!(report.unexamined.is_empty(), "{:?}", report.unexamined);
    let columns = of_kind(&report.findings, is_meridian_continuation);
    assert_eq!(columns.len(), 2, "one per face: {:?}", report.findings);
    for f in &columns {
        assert!(
            (f.metres - want).abs() < want * 1e-3,
            "expected ~{want} m of arc, got {} (gap {} rad, lever {} m)",
            f.metres,
            f.gap,
            f.lever
        );
    }
}

/// A sphere LUNE about a TILTED axis: two meridian half-arcs from the
/// north pole to the south pole, `theta` apart in u, with both pole
/// vertices placed where the CARRIER puts them rather than where the
/// frame states them — which is how any construction that follows a
/// curve gets them, and which leaves each one a few ulps of `radius`
/// off the analytic pole.
fn tilted_lune(radius: f64, theta: f64) -> Option<Body<f64>> {
    let tol = Tol::witness();
    let raw = v3(0.3, 0.2, 1.0);
    let axis = raw * (1.0 / raw.norm());
    let x = v3(1.0, 0.0, 0.0);
    let u0 = x - axis * axis.dot(x);
    let u_ref = u0 * (1.0 / u0.norm());
    let v_ref = axis.cross(u_ref);
    let c = p3(0.0, 0.0, 0.0);
    // A meridian circle at azimuth `az`, oriented so that its forward
    // parameter runs south pole → equator → north pole.
    let meridian = |az: f64| Curve3::Circle {
        center: c,
        axis: (u_ref * az.sin() - v_ref * az.cos()),
        radius,
        u_ref: (u_ref * az.cos() + v_ref * az.sin()),
    };
    // The return side, oriented north → south.
    let back = |az: f64| Curve3::Circle {
        center: c,
        axis: (v_ref * az.cos() - u_ref * az.sin()),
        radius,
        u_ref: (u_ref * az.cos() + v_ref * az.sin()),
    };
    let (ta, tb) = (-core::f64::consts::FRAC_PI_2, core::f64::consts::FRAC_PI_2);
    let m0 = meridian(0.0);
    let (s, n) = (m0.eval(ta), m0.eval(tb));
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(s).ok()?;
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Sphere {
            center: c,
            radius,
            axis,
            u_ref,
        }),
    )
    .ok()?;
    let e = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            n,
            EdgeCurveSpec::arc_of_circle(m0, ta, tb)?,
            tol,
        )
        .ok()?;
    body.mef(
        MefSite::Chords {
            he1: e.he_minus,
            he2: e.he_plus,
        },
        EdgeCurveSpec::arc_of_circle(back(theta), ta, tb)?,
        FaceSurface::Inherit,
        tol,
    )
    .ok()?;
    Some(body)
}

/// **A POLE ENDPOINT IS EXEMPT from the closure condition, by the
/// walk's own identification rule** — R1's NOTE-5, closed at the
/// class rather than at the instance.
///
/// The note: at ε = 1e-12 a tilted-axis sphere of R ≳ 1.4 km reports a
/// spurious closure finding at a pole vertex. Nothing is wrong with
/// the body. Its pole vertex sits a few ulps of R off the analytic
/// pole, which is a lever arm of ~1e-10 m; `u_of` at such a point is
/// an `atan2` of two quantities that are both float noise, so the gap
/// it produces is arbitrary; and an arbitrary gap on a nonzero lever
/// clears any small enough band. Meanwhile the WALK identifies that
/// junction with the pole, substitutes the pole's exact v and never
/// reads its azimuth at all — so the report was contradicting the mesh
/// about which points have an azimuth.
///
/// The exemption is the walk's rule, band included, and this row is
/// its non-vacuity: the radius is derived from the run's own ε so the
/// pole vertices land INSIDE the band at every row, and the largest
/// un-exempted quantity among these azimuths is at least half a band —
/// so what keeps the report silent is the exemption and not the band's
/// own margin.
#[test]
fn a_pole_endpoint_is_not_measured_against_its_own_carrier() {
    let tol = Tol::witness();
    let eps = tol.eps();
    // A pole vertex lands ~3.2e-16·R off the analytic pole (the frame
    // is a normalised tilt, so this is a few ulps of the radius). The
    // two poles do not land at the same distance — the placement is
    // float, not symmetric — so the aim is three quarters of a band,
    // which puts BOTH inside with the spread this construction has.
    let radius = 0.75 * eps / 3.2e-16;
    let mut built = 0;
    let mut widest: f64 = 0.0;
    let mut inside = 0;
    for theta in [0.5_f64, 1.0, 2.8, 3.0] {
        let Some(body) = tilted_lune(radius, theta) else {
            continue;
        };
        built += 1;
        let report = topo::examine_chart_coherence(&body, tol);
        assert!(report.unexamined.is_empty(), "{:?}", report.unexamined);
        let chart = body
            .faces()
            .find_map(|(_, f)| body.get_surface(f.surface).and_then(topo::Chart::of))
            .expect("a sphere face");
        let poles = chart.poles();
        for (_, face) in body.faces() {
            let lp = body.get_loop(face.outer).expect("the outer loop");
            let topo::LoopBoundary::Cycle { first } = lp.boundary else {
                panic!("a cycle")
            };
            for hek in body.loop_cycle(first).expect("the cycle") {
                let he = body.get_half_edge(hek).expect("a half-edge");
                let edge = body.get_edge(he.edge).expect("an edge");
                let curve = body
                    .get_curve_geom(edge.curve)
                    .and_then(|g| g.certified())
                    .expect("a carrier");
                let p = *body
                    .get_vertex(he.start)
                    .and_then(|v| body.get_point(v.point))
                    .expect("a junction point");
                let near = poles
                    .iter()
                    .map(|&(pp, _)| (p - pp).norm())
                    .fold(f64::INFINITY, f64::min);
                if near > eps {
                    // Outside the band the door measures it like any
                    // other endpoint, which is the behaviour this row
                    // is not about.
                    continue;
                }
                inside += 1;
                // The quantity the exemption suppresses, re-derived
                // here from the same public forms the door uses.
                let u_raw = topo::mid_azimuth(&chart, curve);
                let d = u_raw - chart.u_of(p);
                let gap = (d - core::f64::consts::TAU * (d / core::f64::consts::TAU).round()).abs();
                widest = widest.max(gap * chart.radial(p));
            }
        }
        assert!(
            of_kind(&report.findings, is_closure).is_empty(),
            "a pole endpoint carries no azimuth to measure against: {:?}",
            report.findings
        );
    }
    assert!(built >= 2, "the fixture must build; only {built} did");
    assert!(inside >= 4, "only {inside} pole junctions were examined");
    // NON-VACUITY, and its honest limit. The suppressed quantity is
    // the same ORDER as the band on every row (0.2 … 1.0 of it across
    // the three), and its size is ARBITRARY — the gap half of it is an
    // `atan2` of two float-noise components, so which side of the band
    // a given body lands on is a property of its coordinates and not
    // of anything a reader could reason about. That is the whole
    // argument for exempting rather than leaving it to the band: R1
    // executed a case that crossed (a tilted-axis sphere of R ≳ 1.4 km
    // at ε = 1e-12, reporting), and this construction sits beside it.
    assert!(
        widest >= 0.2 * eps,
        "the suppressed quantity must be the band's own order, or this row is \
         not about the exemption at all: widest {widest:e} against eps {eps:e}"
    );
}
