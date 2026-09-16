//! **The rim-free spherical wedge measures** (issue 542): a sphere face
//! bounded by two pole-to-pole meridian arcs on DIFFERENT great circles
//! — the partial revolve of an all-on-axis meridian, the natural ball's
//! wedge — is a lune `[u_A, u_B] × [−π/2, π/2]` whose azimuthal width
//! the boundary states exactly, and the flux arm reads it.
//!
//! Every row asserts the closed form, never a capture: the lune of
//! azimuth `Δu` on a ball of radius `R` has area `2R²·Δu` and its
//! band face contributes flux `s_f·R·area`, so a wedge of the ball
//! (caps through the centre, flux 0) has volume `(2/3)·R³·Δu`.
//!
//! **Which arc the face covers is read structurally, and the rows
//! drive both readings.** The face's interior at a meridian arc is
//! `sense × traversal` — the outward normal crossed with the traversal
//! tangent — so reversing the loop, or flipping the sense bit, hands
//! the arm the complementary lune `2π − θ`. The four angles include
//! `π` (the two-band geometry, which the coplanar arm already measured
//! and must keep measuring by the same closed form) and `3π/2` (a
//! wedge whose SHORT azimuthal arc is on the wrong side).
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

/// **A rimless boundary of three meridians on two great circles is
/// neither the two-band face nor a wedge, and refuses typed** — the
/// `u = 0` meridian split at the equator beside the `u = π/2` one. The
/// closed form's premise is one pair of half-planes; three arcs on two
/// circles state no lune the arm may integrate, and the refusal names
/// that rather than answering for the pair it can see.
#[test]
fn three_meridians_on_two_great_circles_refuse_typed() {
    let band = band();
    let three = vec![
        great(0.0, FRAC_PI_2, 0.0, 0, 1),
        great(0.0, 0.0, -FRAC_PI_2, 1, 2),
        great(FRAC_PI_2, -FRAC_PI_2, FRAC_PI_2, 2, 0),
    ];
    assert_eq!(
        curved_face(&sphere::<f64>(), &three, true, band).map(|_| ()),
        Err(PropsError::NotIsoRectangle {
            what: "rimless sphere face whose non-coplanar meridians are not one pair (neither \
                   the two-band face nor a wedge)"
        })
    );
}

/// **The same split on ONE great circle is still the two-band face**:
/// the count refusal is gated on the pair being non-coplanar, so a
/// hemisphere stated as three coplanar arcs measures `2πR²` exactly as
/// it did before the wedge arm existed.
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

/// **The doors answer for the wedge exactly as they did before the arm**
/// (deliverable 4, as measured: the wedge's meridians END at the poles,
/// so MESH-11's branch door ADMITS it and the shape door always did).
/// The flux lane's premise moved; neither door's did, and the boundary
/// still encodes no material side for a rimless face.
#[test]
fn the_doors_answers_for_the_wedge_are_those_of_the_merge_base() {
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

/// The same twelve lunes through the interval scalar: the wedge arm's
/// `atan2` is asked only once `props_band_coplanar` is definitely
/// nonzero, so no enclosure straddles its branch cut, and the area
/// enclosure must contain the closed form and stay tight.
#[cfg(feature = "interval")]
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
