//! R1 adversarial probes for the M7-8 plane × NURBS edge lane.
//!
//! Attack surface: the between-samples envelope (a wiggle vanishing at
//! the whole sample schedule), the refusal boundary (smallest catching
//! displacement, off-plane and drift directions), the envelope's
//! soundness AND its tightness (certified sup against dense-sampled
//! true sup, bounded in both directions — domination alone is monotone
//! in the safe direction and an envelope that had degenerated would
//! satisfy it forever), and the transversality boundary (shallower
//! than the planted tangential).
//!
//! **ADOPTED INTO THE SHIPPED SUITE** (R1 MAJOR-1). These began as a
//! reviewer's private probes and are kept as permanent rows because
//! they are the ONLY tests that observe the unit's headline
//! obligation: with the chart-sup decision mutated away (zero the sup
//! before its `decide`), every other M7-8 row stays green at default
//! AND at 1e-12, while
//! [`a_wiggle_vanishing_at_the_whole_schedule_must_refuse`] and
//! [`the_certified_sup_bounds_the_dense_sampled_true_sup`] go red —
//! measured, both directions. The between-samples envelope is what
//! makes a declared carrier certifiable at all, so a refactor that
//! hollows it out must not be able to leave the suite green.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsCurve3;
use geom::{NurbsSurface, Surface};
use geom_brep::{EdgeNurbsLane, PlaneNurbsRefusal};
use geom_core::Tol;
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Vec3};
use test_utils::tightness::{Anchor, Sup};
use test_utils::vacuity::{self, Exposure};

fn quarter_cylinder_wall() -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = vec![
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 1.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 1.0, 1.0),
    ];
    let w = core::f64::consts::FRAC_1_SQRT_2;
    NurbsSurface::new(ku, kv, control, vec![1.0, 1.0, w, w, 1.0, 1.0]).unwrap()
}

fn transverse_plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn segment(a: Point3<f64>, b: Point3<f64>) -> NurbsCurve3<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    NurbsCurve3::new(k, vec![a, b], vec![1.0, 1.0]).unwrap()
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A spline carrier that interpolates `(1 + a·sin(32πt), 0, t)` at 257
/// chord points: exactly ON both surfaces at every one of the lane's
/// 33 schedule samples (t = k/32, where sin vanishes to ~1e-15), and
/// displaced by ~`a` radially (in-plane, off-wall) between them.
fn wiggle_carrier(a: f64) -> NurbsCurve3<f64> {
    let n = 257;
    let pts: Vec<Point3<f64>> = (0..n)
        .map(|i| {
            let t = f64::from(i) / f64::from(n - 1);
            Point3::new(
                (32.0 * core::f64::consts::PI * t).sin().mul_add(a, 1.0),
                0.0,
                t,
            )
        })
        .collect();
    let params: Vec<f64> = (0..n).map(|i| f64::from(i) / f64::from(n - 1)).collect();
    NurbsCurve3::interpolate_with_params(&pts, 3, &params).unwrap()
}

/// Dense-sampled true residual of `c` against the wall+plane pair, and
/// the number of samples whose foot-point projection did not converge.
///
/// That count matters to a caller comparing a certified sup against
/// this: a refused projection contributes `last_distance`, the final
/// iterate's distance, which is an UPPER estimate of the true distance
/// to the surface. While it is zero the returned value is a genuine
/// sampled sup and a bound may be required to dominate it exactly.
fn dense_true_sup(c: &NurbsCurve3<f64>, wall: &NurbsSurface<f64>) -> (f64, usize) {
    let mut sup: f64 = 0.0;
    let mut unconverged = 0usize;
    for i in 0..=4096 {
        let t = f64::from(i) / 4096.0;
        let p = c.eval(t);
        let plane_res = p.y.abs();
        let wall_res = match wall.project(p) {
            Ok(pr) => {
                let q = wall.eval(pr.u, pr.v);
                p.distance(q)
            }
            Err(e) => {
                unconverged += 1;
                e.last_distance
            }
        };
        sup = sup.max(plane_res.max(wall_res));
    }
    (sup, unconverged)
}

/// ATTACK 1: on-schedule, off-between-samples. If this certifies, the
/// envelope is grid-only and the lane is broken.
#[test]
fn a_wiggle_vanishing_at_the_whole_schedule_must_refuse() {
    let wall = quarter_cylinder_wall();
    let plane = transverse_plane();
    let a = 1e-3;
    let carrier = wiggle_carrier(a);
    let (truth, _) = dense_true_sup(&carrier, &wall);
    match f64::plane_nurbs_limbs(&carrier, &plane, &wall, 1.0, band()) {
        Ok(limbs) => panic!(
            "GRID-ONLY HOLE: a carrier displaced {truth:e} m between samples \
             certified with hull_sup {:e}",
            limbs.hull_sup
        ),
        Err(e) => {
            println!("R1 wiggle a={a:e}: true sup {truth:e} -> refused: {e:?}");
            if let PlaneNurbsRefusal::Limb { value, .. } = e {
                assert!(
                    value >= truth * 0.5,
                    "the certified bound must dominate the true displacement: \
                     {value:e} < {truth:e}"
                );
            }
        }
    }
}

/// ATTACK 1c: the PLANE-side envelope in isolation — a carrier
/// wiggling off-plane (+y) between samples with amplitude 1e-5, whose
/// wall-side residual is only ~A²/2 ≈ 5e-11 (inside ε). Only the
/// analytic operand's between-samples bound can catch it.
#[test]
fn an_off_plane_wiggle_must_refuse_via_the_plane_envelope() {
    let wall = quarter_cylinder_wall();
    let plane = transverse_plane();
    let a = 1e-5;
    let n = 257;
    let pts: Vec<Point3<f64>> = (0..n)
        .map(|i| {
            let t = f64::from(i) / f64::from(n - 1);
            Point3::new(1.0, a * (32.0 * core::f64::consts::PI * t).sin(), t)
        })
        .collect();
    let params: Vec<f64> = (0..n).map(|i| f64::from(i) / f64::from(n - 1)).collect();
    let carrier = NurbsCurve3::interpolate_with_params(&pts, 3, &params).unwrap();
    let (truth, _) = dense_true_sup(&carrier, &wall);
    match f64::plane_nurbs_limbs(&carrier, &plane, &wall, 1.0, band()) {
        Ok(limbs) => panic!(
            "PLANE-SIDE GRID-ONLY HOLE: off-plane-between-samples carrier \
             (true sup {truth:e}) certified with hull_sup {:e}",
            limbs.hull_sup
        ),
        Err(e) => println!("R1 off-plane wiggle a={a:e}: true sup {truth:e} -> {e:?}"),
    }
}

/// The lane's own enclosure floor on this fixture, in metres. The
/// `a = 0` carrier lies EXACTLY on both operands, so its true sup is
/// zero and whatever the lane certifies for it is the envelope's own
/// noise over the wall's rational control net — 1.099e-13 m. This is
/// one order above that.
///
/// It is a bound on an ENCLOSURE, not on a tolerance: `hull_sup` is
/// bit-identical at every battery ε (only the accept/refuse decision
/// moves), so the numerical coincidence with the finest battery ε and
/// with `Band::linear`'s zero width at that ε is a coincidence and
/// nothing keys on it.
///
/// **Its defence is that it is tight**, not that it is far from
/// disaster. Ceiling over certified sup — how much slack the ceiling
/// leaves — is 9.10× at a = 0, 2.40× at a = 1e-13 and 1.83× at
/// a = 1e-12. The quarter cylinder's own box (√3 m) is twelve orders
/// away and is no part of the argument, which is why the ceilings
/// below pass `Anchor::Unbounded`.
const ENVELOPE_FLOOR: f64 = 1e-12;

/// Above this sampled truth the additive form stops being the honest
/// one and a RATIO ceiling takes over. Ten times [`ENVELOPE_FLOOR`] —
/// an absolute constant, so which rungs make which claim never depends
/// on the enclosure being measured.
const RATIO_ARM_FROM: f64 = 10.0 * ENVELOPE_FLOOR;

/// ATTACK 1b: envelope soundness on the ACCEPT side — a certifying
/// carrier's certified sup must bound its dense-sampled true sup, and
/// must not stand far above it.
///
/// **The two ceilings, and why there are two.** The amplitude ladder
/// spans four orders, and the enclosure's own floor (`ENVELOPE_FLOOR`)
/// sits inside it. Below `RATIO_ARM_FROM` a ratio measures that floor
/// rather than the envelope — it runs inf, 4.579, 1.096 as the
/// amplitude rises — so those rungs take the additive form. At and
/// above it the ratio is tight in its own right: 1.000988 at
/// a = 1e-11 and 1.000012 at a = 1e-10. At the bottom of the ratio arm
/// the two forms are twenty-fold apart — 0.5% admitted against the
/// 10% an additive ceiling would allow there — and at the top they are
/// within a factor of two, where either would do.
///
/// **What makes both ceilings guards.** Coarsening the between-samples
/// schedule this row exists to attack — `PXN_FIT_SAMPLES` 33 → 25,
/// 17, 9, each still sound — moves the a = 1e-11 rung to 1.00660,
/// 1.00899 and 1.09195, and the a = 1e-13 rung to 11.54x, 13.47x and
/// 43.81x. Both ceilings go red at the FIRST of those, a 24%
/// coarsening: the ratio arm at a = 1e-11 and the additive arm at
/// a = 1e-13. The a = 1e-10 rung is admitted to be insensitive — it
/// reads 1.00096 even at nine samples — so the ratio arm's teeth are
/// entirely at the bottom of its range, which is where the envelope is
/// working for its living.
///
/// **ε moves which rungs certify, and nothing else.** `hull_sup` is
/// bit-identical at every battery ε; what varies is the lane's
/// accept/refuse decision, so the ladder is walked at every ε and each
/// rung claims what it can. At ε = 1e-12 every amplitude at or above
/// `RATIO_ARM_FROM` refuses, the ratio arm is unreachable, and the row
/// stands down by name rather than reporting green over an empty
/// claim.
#[test]
fn the_certified_sup_bounds_the_dense_sampled_true_sup() {
    let wall = quarter_cylinder_wall();
    let plane = transverse_plane();
    let eps = Tol::witness().get().eps;
    let mut seen = Exposure::new("pxn envelope accept side");
    for category in ["certified", "ratio arm", "additive arm"] {
        seen.add(category, 0);
    }
    for a in [0.0, 1e-13, 1e-12, 1e-11, 1e-10] {
        let carrier = if a == 0.0 {
            segment(Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 1.0))
        } else {
            wiggle_carrier(a)
        };
        let (truth, unconverged) = dense_true_sup(&carrier, &wall);
        // The domination below is exact, which it may be only while
        // every sample's foot point converged: a refused projection
        // contributes an OVER-estimate of the distance, and the
        // sampled sup would then no longer be a value the certified
        // sup has to dominate.
        assert!(
            unconverged == 0,
            "a={a:e}: {unconverged} of 4097 foot points did not converge, so the \
             sampled sup is an over-estimate and the exact domination below is \
             no longer the right claim"
        );
        if a == 0.0 {
            // The exact segment is not an interpolation: it lies ON the
            // plane and on the wall's u = 0 edge, so its true sup is
            // zero by construction and this is the one rung whose
            // fixture claim is structural rather than a magnitude.
            assert!(
                truth == 0.0,
                "the exact segment must lie ON both operands, but a dense scan \
                 finds {truth:e} — the fixture drifted and this rung no longer \
                 measures the enclosure's floor"
            );
        } else {
            // Anti-vacuity for the wiggles: the fit must carry the
            // amplitude it was given. One that flattened it leaves a
            // truth of ~0, and every comparison below holds for free.
            assert!(
                truth >= 0.5 * a,
                "the a={a:e} wiggle did not survive the fit: true sup {truth:e}"
            );
        }
        match f64::plane_nurbs_limbs(&carrier, &plane, &wall, 1.0, band()) {
            Ok(limbs) => {
                seen.note("certified");
                println!(
                    "R1 sup-bound a={a:e}: certified {:e} vs true {truth:e}",
                    limbs.hull_sup
                );
                let claim = Sup::new("plane x NURBS envelope", limbs.hull_sup, truth).dominates();
                // No box scale enters either ceiling: both are within
                // twelve orders of the enclosure's own floor and the
                // quarter cylinder's √3 m box bounds nothing here.
                let anchor = Anchor::Unbounded(
                    "the ceilings sit at the enclosure's own noise floor, twelve \
                     orders under the operands' box",
                );
                if truth >= RATIO_ARM_FROM {
                    seen.note("ratio arm");
                    claim.within(
                        1.005,
                        0.0,
                        anchor,
                        "the envelope is no longer residual-scaled — a 24% \
                         coarsening of PXN_FIT_SAMPLES reads 1.00660 at a = 1e-11, \
                         which this ceiling catches",
                    );
                } else {
                    seen.note("additive arm");
                    claim.within(
                        1.0,
                        ENVELOPE_FLOOR,
                        anchor,
                        "the envelope is no longer at its own floor — this ceiling \
                         leaves between 1.83x and 9.10x of slack across the rungs \
                         it covers, and a 24% coarsening of PXN_FIT_SAMPLES takes \
                         a = 1e-13 past it",
                    );
                }
            }
            Err(e) => println!("R1 sup-bound a={a:e}: refused {e:?} (true {truth:e})"),
        }
    }
    seen.report();
    seen.require(
        "certified",
        1,
        "the exact carrier lies on both operands and certifies at every battery ε, \
         so no amplitude certifying means the accept side went away and this row \
         compared nothing",
    );
    // The ratio arm needs an amplitude that both certifies and clears
    // `RATIO_ARM_FROM`; the lane certifies to about ε, so a fine enough
    // ε leaves no such rung. That is a stand-down, not a floor to meet.
    if eps <= RATIO_ARM_FROM {
        vacuity::stood_down(
            "pxn envelope ratio arm",
            "every amplitude at or above the ratio arm's threshold refuses at this ε, \
             so THIS RUN states no ratio ceiling on the envelope — only the additive \
             one at the enclosure's own floor",
        );
    } else {
        seen.require(
            "ratio arm",
            1,
            "at this ε the ladder reaches amplitudes three orders above the \
             enclosure floor, so the ratio ceiling is reachable and its absence \
             would mean the ladder stopped short",
        );
    }
}

/// ATTACK 2: the refusal boundary — uniform radial (in-plane) and
/// off-plane displacements, scanned down from 1e-3. Every outcome must
/// be typed; refusals must carry a bound of the displacement's order.
///
/// **Both assertions live inside match arms**, so a scan whose every
/// rung landed in the untyped `Err(_)` arm would state nothing and
/// report green — the vacuity
/// [`the_certified_sup_bounds_the_dense_sampled_true_sup`] closes above.
/// The row is about a BOUNDARY, so the floor is that both sides of it
/// were reached, per direction: a scan that only certified never found
/// the refusal, and one that only refused never found the accept side.
#[test]
fn displacement_scan_finds_the_refusal_boundary_typed() {
    let wall = quarter_cylinder_wall();
    let plane = transverse_plane();
    let eps = Tol::witness().get().eps;
    let mut seen = Exposure::new("pxn displacement scan");
    for dir in ["radial(+x, on-plane)", "off-plane(+y)"] {
        for outcome in ["certified", "limb refusal"] {
            seen.add(&format!("{dir} {outcome}"), 0);
        }
        let mut smallest_refused = f64::INFINITY;
        let mut largest_certified: f64 = 0.0;
        for k in 3..14 {
            let d = 10f64.powi(-k);
            let carrier = if dir.starts_with("radial") {
                segment(
                    Point3::new(1.0 + d, 0.0, 0.0),
                    Point3::new(1.0 + d, 0.0, 1.0),
                )
            } else {
                segment(Point3::new(1.0, d, 0.0), Point3::new(1.0, d, 1.0))
            };
            match f64::plane_nurbs_limbs(&carrier, &plane, &wall, 1.0, band()) {
                Ok(l) => {
                    largest_certified = largest_certified.max(d);
                    seen.note(&format!("{dir} certified"));
                    println!("R1 scan {dir} d={d:e}: CERTIFIES hull {:e}", l.hull_sup);
                    assert!(
                        d <= eps * 1.01,
                        "a displacement past ε certified: d={d:e} eps={eps:e}"
                    );
                }
                Err(PlaneNurbsRefusal::Limb { limb, value }) => {
                    smallest_refused = smallest_refused.min(d);
                    seen.note(&format!("{dir} limb refusal"));
                    println!("R1 scan {dir} d={d:e}: Limb {} = {value:e}", limb.name());
                    assert!(value > 0.1 * d, "bound must be of the displacement's order");
                }
                Err(e) => println!("R1 scan {dir} d={d:e}: {e:?}"),
            }
        }
        println!(
            "R1 scan {dir}: smallest refused {smallest_refused:e}, \
             largest certified {largest_certified:e}"
        );
    }
    seen.report();
    // The scan runs d from 1e-3 down to 1e-13 and the lane certifies at
    // about ε, so both sides are reached at every battery ε — the
    // coarse end refuses on a limb and the fine end certifies. A
    // direction missing either has stopped scanning across the
    // boundary, whatever the assertions inside the arms then say.
    seen.require_each(
        &[
            "radial(+x, on-plane) certified",
            "radial(+x, on-plane) limb refusal",
            "off-plane(+y) certified",
            "off-plane(+y) limb refusal",
        ],
        1,
        "this row pins a BOUNDARY, so a direction that reached only one side of it \
         has asserted nothing about where the boundary is",
    );
}

/// ATTACK 2b: tangential drift ALONG the intersection — a subsegment
/// of the true locus. The LANE certifies it (it is on both surfaces);
/// the segment-identity duty belongs to the door's endpoint checks.
/// Recorded so the division of labour is explicit.
#[test]
fn a_drifted_subsegment_of_the_true_locus_certifies_at_the_lane() {
    let wall = quarter_cylinder_wall();
    let plane = transverse_plane();
    let carrier = segment(Point3::new(1.0, 0.0, 0.2), Point3::new(1.0, 0.0, 0.9));
    let r = f64::plane_nurbs_limbs(&carrier, &plane, &wall, 0.7, band());
    println!("R1 drift subsegment: {r:?}");
    assert!(r.is_ok(), "a true-locus subsegment is on both surfaces");
}

/// ATTACK 4: the transversality boundary, shallower than the planted
/// 0° tangential — a plane through the ruling tilted α off the tangent
/// plane. Every outcome must be typed; no panic, no silent accept in
/// the sub-ε regime.
///
/// **The only assertion lives inside the `Ok` arm**, so a scan whose
/// every angle refused would state nothing and report green. Like the
/// displacement scan above, this row pins a BOUNDARY and its floor is
/// that both sides of it were reached: the shallow angles must refuse
/// and the steep ones must certify, or the sub-ε accept the assertion
/// guards was never approached from the accepting side.
#[test]
fn near_tangential_scan_is_typed_at_every_angle() {
    let wall = quarter_cylinder_wall();
    let carrier = segment(Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 1.0));
    let eps = Tol::witness().get().eps;
    let mut seen = Exposure::new("pxn near-tangential scan");
    for outcome in ["certified", "refused"] {
        seen.add(outcome, 0);
    }
    for k in [1, 2, 3, 6, 8, 9, 10, 12] {
        let alpha = 10f64.powi(-k);
        // Normal (cos α, sin α, 0), plane through (1,0,0) containing z:
        // sin θ against the wall's radial normal is sin α.
        let plane = Surface::Plane {
            origin: Point3::new(1.0, 0.0, 0.0),
            normal: Vec3::new(alpha.cos(), alpha.sin(), 0.0),
            u_ref: Vec3::new(0.0, 0.0, 1.0),
        };
        match f64::plane_nurbs_limbs(&carrier, &plane, &wall, 1.0, band()) {
            Ok(l) => {
                seen.note("certified");
                println!(
                    "R1 tangency a={alpha:e}: certifies, sin={:e}",
                    l.min_sin_theta
                );
                assert!(
                    alpha.sin() > eps,
                    "sub-ε transversality silently accepted at α={alpha:e}"
                );
            }
            Err(e) => {
                seen.note("refused");
                println!("R1 tangency a={alpha:e}: {e:?}");
            }
        }
    }
    seen.report();
    // The ladder spans α = 1e-1 down to 1e-12 against a lane whose
    // transversality floor is ε, so at every battery ε the steep end
    // certifies and the shallow end refuses.
    seen.require_each(
        &["certified", "refused"],
        1,
        "this row pins a BOUNDARY, and its only assertion is inside the accepting \
         arm — a scan that never certified guarded nothing, and one that never \
         refused never reached the tangential regime it is named for",
    );
}
