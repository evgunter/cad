//! **The rim-free spherical wedge measures** (issue 542): a sphere face
//! bounded by two pole-to-pole meridian arcs on DIFFERENT great circles
//! — the partial revolve of an all-on-axis meridian, the natural ball's
//! wedge — is a lune `[u_A, u_B] × [−π/2, π/2]`, and the flux arm reads
//! it by Gauss–Bonnet (`curved::sphere_face`): great circles turn by
//! nothing, so the area is the turning at the two poles.
//!
//! Every row asserts the closed form, never a capture: the lune of
//! azimuth `Δu` on a ball of radius `R` has area `2R²·Δu` and its
//! band face contributes flux `s_f·R·area`, so a wedge of the ball
//! (caps through the centre, flux 0) has volume `(2/3)·R³·Δu`.
//!
//! **Which arc the face covers is the bit's.** Both poles lie on a
//! lune's loop, so the loop encodes no side (`MaterialSign::Unencoded`)
//! and reversing the loop, or flipping the sense bit, hands the arm the
//! complementary lune `2π − θ`. The four angles include `π` (the
//! two-band geometry) and `3π/2` (a wedge whose SHORT azimuthal arc is
//! on the wrong side).
//!
//! Offsets are none: the shapes are exact at every ε row, so this file
//! rides CI's `eps ∈ {default, 1e-6, 1e-12}` matrix without a literal
//! that claims one of them.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_6, PI, TAU};

use crate::shared::surf;
use crate::shared::tol::band;
use crate::shared::topo;
use geom::Surface;
use geom_brep::props::{
    LoopEdge, MaterialSign, PropsError, boundary_material_sign, curved_face, require_iso_rectangle,
    require_one_chart_branch,
};
use geom_core::{Real, Vec3};

/// The sphere under every row: R = 10 mm about +Z at the origin.
const RS: f64 = 0.010;

/// The four angles the unit pins: two short wedges, the hemisphere,
/// and a wedge past π.
const ANGLES: [f64; 4] = [FRAC_PI_6, FRAC_PI_2, PI, 1.5 * PI];

fn sphere<T: Real>() -> Surface<T> {
    surf::sphere(RS)
}

/// The meridian great circle at azimuth `u`, traversed from latitude
/// `t0` to `t1` (`±π/2` are the poles on the `u` side).
fn great<T: Real>(u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<T> {
    topo::sphere_great(RS, u, t0, t1, a, b)
}

/// The wedge between the meridian half-planes at azimuth `0` and
/// `theta`, as the loop that — with `sense = true` — covers the
/// azimuths `[0, theta]`: down the `u = 0` meridian, up the `u = theta`
/// one (interior-left about the outward normal puts the interior on
/// the `+u` side of a southward meridian). `reversed` is the same two
/// arcs traversed the other way round, which is the loop of the
/// COMPLEMENTARY lune `[theta, 2π]`.
fn wedge<T: Real>(theta: f64, reversed: bool) -> Vec<LoopEdge<T>> {
    if reversed {
        vec![
            great(theta, FRAC_PI_2, -FRAC_PI_2, 0, 1),
            great(0.0, -FRAC_PI_2, FRAC_PI_2, 1, 0),
        ]
    } else {
        vec![
            great(0.0, FRAC_PI_2, -FRAC_PI_2, 0, 1),
            great(theta, -FRAC_PI_2, FRAC_PI_2, 1, 0),
        ]
    }
}

/// The azimuthal width the face covers, derived from the construction
/// and not from the arm: the forward loop with `sense = true` covers
/// `theta`; reversing the loop OR flipping the sense hands the face
/// the complement, and doing both hands it back.
fn covered(theta: f64, reversed: bool, sense: bool) -> f64 {
    if reversed == sense {
        TAU - theta
    } else {
        theta
    }
}

/// The azimuth interval `[u0, u1]` the face covers, from the
/// construction: the interior leaves a southward-traversed meridian on
/// its `+u` side when the sense is `true` and on its `−u` side when it
/// is `false`; the forward loop's southward meridian is at `0`, the
/// reversed loop's at `theta`.
fn covered_interval(theta: f64, reversed: bool, sense: bool) -> (f64, f64) {
    let start = if reversed { theta } else { 0.0 };
    let du = covered(theta, reversed, sense);
    if sense {
        (start, start + du)
    } else {
        (start - du, start)
    }
}

/// **The natural wedge measures at all four angles, both senses, both
/// traversal directions, to the closed form** — `area = 2R²·Δu` and
/// `flux = s_f·R·area`, i.e. a ball-wedge volume `(2/3)·R³·Δu` per
/// `flux / 3`, with `Δu` the arc the loop actually encloses. The `π`
/// row is the two-band face, coplanar meridians, and measures by the
/// arm that always measured it.
#[test]
fn the_natural_wedge_measures_at_four_angles_both_senses_both_directions() {
    let band = band();
    for theta in ANGLES {
        for reversed in [false, true] {
            for sense in [true, false] {
                let du = covered(theta, reversed, sense);
                let kind = format!("theta = {theta:.6}, reversed = {reversed}, sense = {sense}");
                let fc = curved_face(&sphere::<f64>(), &wedge(theta, reversed), sense, band)
                    .unwrap_or_else(|e| panic!("{kind}: refused: {e:?}"));
                let area = 2.0 * RS * RS * du;
                let rel = (fc.area - area).abs() / area;
                assert!(
                    rel < 1e-12,
                    "{kind}: area {:.15e} != {area:.15e} (rel {rel:.3e})",
                    fc.area
                );
                let s_f = if sense { 1.0 } else { -1.0 };
                let volume = s_f * (2.0 / 3.0) * RS.powi(3) * du;
                let rel = (fc.flux / 3.0 - volume).abs() / volume.abs();
                assert!(
                    rel < 1e-12,
                    "{kind}: flux/3 {:.15e} != ball-wedge volume {volume:.15e} (rel {rel:.3e})",
                    fc.flux / 3.0
                );
            }
        }
    }
}

/// **The arc the arm integrated is the arc the loop encloses, checked
/// against a derivation that never reads the arm's rule.** The
/// boundary loop's vector area `½∮ (P − c) × dP` equals `∫ N dA` over
/// the face, which for the lune `[u0, u1]` about the outward normal
/// `s_f·(P − c)/R` is `s_f·(π/2)·R²·(sin u1 − sin u0, cos u0 − cos u1, 0)`
/// — the SAME magnitude for a lune and its complement and the OPPOSITE
/// direction, so a wrong-arc answer cannot pass this row while a
/// right-magnitude one can. The loop's vector area is taken from a
/// dense polygonal sample of the stored carriers in traversal order;
/// the tolerance is the sample's, four orders above rounding and four
/// below the sign it decides.
#[test]
fn the_covered_arc_is_the_one_the_loops_vector_area_points_into() {
    let band = band();
    let samples = 400;
    for theta in ANGLES {
        for reversed in [false, true] {
            for sense in [true, false] {
                let kind = format!("theta = {theta:.6}, reversed = {reversed}, sense = {sense}");
                let edges = wedge::<f64>(theta, reversed);
                // The arm's Δu, read back from its area.
                let fc = curved_face(&sphere::<f64>(), &edges, sense, band).unwrap();
                let du = fc.area / (2.0 * RS * RS);
                let (u0, u1) = covered_interval(theta, reversed, sense);
                assert!(
                    (du - (u1 - u0)).abs() < 1e-12,
                    "{kind}: the arm's Δu {du} is not the constructed interval's {}",
                    u1 - u0
                );
                let s_f = if sense { 1.0 } else { -1.0 };
                let closed = Vec3::new(u1.sin() - u0.sin(), u0.cos() - u1.cos(), 0.0)
                    * (s_f * 0.5 * PI * RS * RS);
                let mut pts: Vec<Vec3<f64>> = Vec::new();
                for e in &edges {
                    for k in 0..samples {
                        let s = k as f64 / samples as f64;
                        let t = if e.forward {
                            e.t0 + s * (e.t1 - e.t0)
                        } else {
                            e.t1 - s * (e.t1 - e.t0)
                        };
                        pts.push(e.carrier.eval(t) - geom_core::Point3::origin());
                    }
                }
                let mut va = Vec3::new(0.0, 0.0, 0.0);
                for i in 0..pts.len() {
                    va = va + pts[i].cross(pts[(i + 1) % pts.len()]) * 0.5;
                }
                let miss = (va - closed).norm() / closed.norm();
                assert!(
                    miss < 1e-3,
                    "{kind}: loop vector area {va:?} is not the covered lune's {closed:?} \
                     (rel {miss:.3e}); the complement's would be its negation"
                );
            }
        }
    }
}

/// **A split meridian on a wedge measures the wedge** — the `u = 0`
/// meridian in two pieces abutting at the equator beside the `u = π/2`
/// one is the quarter lune, and a great circle turns by zero at the
/// joint, so it measures `2R²·π/2` with no fold.
#[test]
fn a_split_meridian_wedge_measures_the_quarter_lune() {
    let band = band();
    let three = vec![
        great(0.0, FRAC_PI_2, 0.0, 0, 1),
        great(0.0, 0.0, -FRAC_PI_2, 1, 2),
        great(FRAC_PI_2, -FRAC_PI_2, FRAC_PI_2, 2, 0),
    ];
    let fc = curved_face(&sphere::<f64>(), &three, true, band).expect("the quarter lune");
    let exact = 2.0 * RS * RS * FRAC_PI_2;
    assert!(
        (fc.area - exact).abs() / exact < 1e-12,
        "area {:.15e} != {exact:.15e}",
        fc.area
    );
    assert!(
        (fc.flux - RS * exact).abs() / (RS * exact) < 1e-12,
        "flux {:.15e} != R·area",
        fc.flux
    );
}

/// **The same split on ONE great circle is the two-band face**: a
/// hemisphere stated as three coplanar arcs — the loop continuing
/// through both junctions — measures `2πR²`.
#[test]
fn three_coplanar_meridians_still_measure_the_hemisphere() {
    let band = band();
    let three = vec![
        great(0.0, FRAC_PI_2, 0.0, 0, 1),
        great(0.0, 0.0, -FRAC_PI_2, 1, 2),
        great(PI, -FRAC_PI_2, FRAC_PI_2, 2, 0),
    ];
    let fc = curved_face(&sphere::<f64>(), &three, true, band).expect("the two-band face");
    let exact = 2.0 * PI * RS * RS;
    assert!(
        (fc.area - exact).abs() / exact < 1e-12,
        "area {:.15e} != {exact:.15e}",
        fc.area
    );
}

/// **The doors answer for the wedge as they answer for any rimless
/// lune**: the wedge's meridians END at the poles, so MESH-11's branch
/// door admits it and the shape door admits every rimless lune; the
/// boundary encodes no material side for a rimless face. Neither
/// door's answer depends on which lunes the flux lane measures.
#[test]
fn the_doors_answers_for_the_wedge_are_those_of_any_rimless_lune() {
    let band = band();
    for theta in ANGLES {
        for reversed in [false, true] {
            let edges = wedge::<f64>(theta, reversed);
            let kind = format!("theta = {theta:.6}, reversed = {reversed}");
            assert_eq!(
                require_iso_rectangle(&sphere::<f64>(), &edges, band),
                Ok(()),
                "{kind}"
            );
            assert_eq!(
                require_one_chart_branch(&sphere::<f64>(), &edges, band),
                Ok(()),
                "{kind}"
            );
            assert_eq!(
                boundary_material_sign(&sphere::<f64>(), &edges, band),
                Ok(MaterialSign::Unencoded),
                "{kind}"
            );
        }
    }
}

/// **A slit refuses typed, in both of its readings.** Two pole-to-pole
/// arcs on ONE half-plane, traversed there and back, turn back on
/// themselves at each pole: a cusp, which has no turning angle
/// (`props_sphere_loop_cusp`). The face they claim is a slit of no
/// width or the ball less a slit — neither is measured, under either
/// sense.
#[test]
fn a_coplanar_slit_refuses_typed_in_both_senses() {
    let band = band();
    let slit = vec![
        great::<f64>(0.0, FRAC_PI_2, -FRAC_PI_2, 0, 1),
        great(0.0, -FRAC_PI_2, FRAC_PI_2, 1, 0),
    ];
    for sense in [true, false] {
        assert_eq!(
            curved_face(&sphere::<f64>(), &slit, sense, band).map(|_| ()),
            Err(PropsError::SphereLoop {
                what: "props_sphere_loop_cusp"
            }),
            "sense = {sense}"
        );
    }
}

/// **The hemisphere, bitwise.** At θ = π the two meridians lie on one
/// great circle and the loop continues through both poles, so
/// Gauss–Bonnet turns by nothing and the area is `2π·R²`:
/// `area = 0x3f4496b7c53c5b02` (6.283185307179586e-4 m²) and
/// `flux = ±0x3eda5a84d380747e` (the radial term `R·area`, signed by
/// the sense). A change to the arm's arithmetic moves a bit here.
#[test]
fn the_hemisphere_at_pi_measures_bitwise() {
    let band = band();
    for (sense, flux_bits) in [
        (true, 0x3eda5a84d380747e_u64),
        (false, 0xbeda5a84d380747e_u64),
    ] {
        for reversed in [false, true] {
            let fc = curved_face(&sphere::<f64>(), &wedge(PI, reversed), sense, band).unwrap();
            assert_eq!(
                fc.area.to_bits(),
                0x3f4496b7c53c5b02,
                "sense = {sense}, reversed = {reversed}: area {:e}",
                fc.area
            );
            assert_eq!(
                fc.flux.to_bits(),
                flux_bits,
                "sense = {sense}, reversed = {reversed}: flux {:e}",
                fc.flux
            );
        }
    }
}

/// The `u = theta` meridian carrier LEANED out of its meridian plane
/// by `lean` radians about its own equatorial direction: the axis
/// gains the component `sin lean` along the sphere axis, the arc still
/// departs from a point `lean·R` off the south pole toward the pole's
/// side of the plane, and its half-plane direction at the equator is
/// unchanged.
fn leaned_meridian(theta: f64, lean: f64, a: u32, b: u32) -> LoopEdge<f64> {
    let n = Vec3::new(theta.sin(), -theta.cos(), 0.0);
    let axis = Vec3::new(0.0, 0.0, 1.0);
    let (s, c) = lean.sin_cos();
    topo::edge(
        geom::Curve3::Circle {
            center: geom_core::Point3::origin(),
            axis: n * c + axis * s,
            radius: RS,
            u_ref: axis * -c + n * s,
        },
        0.0,
        PI,
        a,
        b,
    )
}

/// **A leaned meridian's slack is a measured, bounded slack.** A
/// carrier leaned by `0.5·zero/R` departs from a point `lean·R` off the
/// south pole and arrives `lean·R` off the north one, so the loop
/// closes within the coincidence band (`props_loop_closed`) and
/// the face measures: the lean rotates the arc about its equatorial
/// point, ANTISYMMETRICALLY — the sliver it adds north of the equator
/// it removes south of it — so the true area differs from the lune's
/// closed form by `O(lean²·R²)`; the row pins the answer against the
/// closed form to `lean² + 1e-12`, the bound that is load-bearing at
/// the coarse ε rows. A carrier leaned by `2·zero/R` opens the loop by
/// twice the band at each pole and escalates at the closure decide.
#[test]
fn a_meridian_carrier_within_the_closure_slack_measures_and_past_it_escalates() {
    let band = band();
    let theta = FRAC_PI_2;
    let within = 0.5 * band.zero() / RS;
    let edges = vec![
        great::<f64>(0.0, FRAC_PI_2, -FRAC_PI_2, 0, 1),
        leaned_meridian(theta, within, 1, 0),
    ];
    let fc = curved_face(&sphere::<f64>(), &edges, true, band)
        .expect("a lean inside the coincidence band is a meridian");
    let exact = 2.0 * RS * RS * theta;
    let rel = (fc.area - exact).abs() / exact;
    assert!(
        rel < within * within + 1e-12,
        "lean {within:e}: area {:.15e} vs {exact:.15e} (rel {rel:.3e})",
        fc.area
    );
    let past = 2.0 * band.zero() / RS;
    let edges = vec![
        great::<f64>(0.0, FRAC_PI_2, -FRAC_PI_2, 0, 1),
        leaned_meridian(theta, past, 1, 0),
    ];
    match curved_face(&sphere::<f64>(), &edges, true, band) {
        Err(PropsError::Escalated { cause, .. }) => assert_eq!(
            cause.predicate,
            Some("props_loop_closed"),
            "the lean escalates at the loop's closure"
        ),
        other => panic!("a lean of 2·zero/R must escalate at the closure, got {other:?}"),
    }
}

/// The same twelve lunes through the interval scalar: the turning at
/// each pole is an `atan2` of two departure directions a definite
/// angle apart, so no enclosure straddles its branch cut, and the area
/// enclosure must contain the closed form and stay tight.
#[test]
fn the_wedge_arm_encloses_the_closed_form_at_interval() {
    use geom_core::{Bounds, Interval};
    let band = band();
    for theta in ANGLES {
        for reversed in [false, true] {
            for sense in [true, false] {
                let du = covered(theta, reversed, sense);
                let kind = format!("theta = {theta:.6}, reversed = {reversed}, sense = {sense}");
                let edges: Vec<LoopEdge<Interval>> = wedge(theta, reversed);
                let fc = curved_face(&sphere::<Interval>(), &edges, sense, band)
                    .unwrap_or_else(|e| panic!("{kind}: refused at interval: {e:?}"));
                let exact = 2.0 * RS * RS * du;
                let (lo, hi) = (fc.area.lo(), fc.area.hi());
                assert!(
                    lo <= exact && exact <= hi && (hi - lo) < 1e-9,
                    "{kind}: interval area [{lo:e}, {hi:e}] must tightly enclose {exact:e}"
                );
                let s_f = if sense { 1.0 } else { -1.0 };
                let volume = s_f * (2.0 / 3.0) * RS.powi(3) * du;
                let (lo, hi) = (fc.flux.lo() / 3.0, fc.flux.hi() / 3.0);
                assert!(
                    lo <= volume && volume <= hi && (hi - lo) < 1e-9,
                    "{kind}: interval flux/3 [{lo:e}, {hi:e}] must tightly enclose {volume:e}"
                );
            }
        }
    }
}
