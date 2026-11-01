//! A carrier's held knots and weights as the knot algebra's plan
//! constructors read them (C6 S5b): the held constructors, through which
//! a carrier's knot operation decides its plans at every scalar.
//!
//! The constructors in [`super::algebra`] decide a plan's structure on an
//! `f64` vector. At point knots and weights that is their own vector, so
//! the plans are today's bit for bit. Where a run or a weight is wider
//! than a point, the plan is decided against a stand-in
//! (`Knots::plan_proxy`: each wide run at its bracket's midpoint, unit
//! weights), after every decision that reads a value has been certified
//! for every member against the held brackets: a parameter the operation
//! inserts or removes is a breakpoint, or lies in one span, for all of
//! them, or the operation refuses undecided
//! ([`KnotAlgebraError::ParameterUndecided`]). Such a plan carries the
//! right schedule and structure, its steps naming the input runs by
//! index, so the appliers at a scalar run it over the held values; its
//! `f64` reference values are the stand-in's, so it is marked
//! ([`CurvePlan::is_reference`]) and the `f64` appliers refuse it. A
//! weight it writes is certified by the carrier's door.

use std::borrow::Cow;

use super::algebra::{self, CurvePlan, KnotAlgebraError, RemovalStep};
use super::knot_values::Knots;
use super::knots::{KnotVector, SplineError};
use super::locate::SpanLocate;
use super::weights::Weights;

/// The held knots and weights as the plan constructors read them: the
/// vector the plans are decided against (`Knots::plan_proxy`, the
/// knots' own where every breakpoint is a point), the weights' own where
/// every knot and weight is a point and unit weights otherwise (module
/// docs), and whether they are the inputs' own. Borrowed where they are,
/// so the point path copies nothing.
#[allow(clippy::type_complexity)]
fn inputs<'a, T: SpanLocate>(
    knots: &'a Knots<T>,
    weights: &'a Weights<T>,
) -> Result<(Cow<'a, KnotVector>, Cow<'a, [f64]>, bool), KnotAlgebraError> {
    let kv = knots.plan_proxy().map_err(KnotAlgebraError::Structure)?;
    if knots.is_point() && weights.is_point() {
        return Ok((kv, Cow::Borrowed(weights.weights_f64_interim()), true));
    }
    Ok((kv, Cow::Owned(vec![1.0; weights.len()]), false))
}

/// That `u` is a breakpoint, or lies apart from every breakpoint, for
/// every member of the held knots: a point run equal to `u` is the one
/// breakpoint, and a wide run whose bracket holds `u`, its ends
/// included, leaves undecided whether `u` is in it, beside it, or in
/// which span. Where the domain's first or last breakpoint is wide, a
/// `u` at or past its bracket's outer end is outside every member's
/// domain, which `outside` names: no narrower knots would decide it.
///
/// # Errors
///
/// [`KnotAlgebraError::ParameterUndecided`] at the first such run, with
/// `u`'s distance to its bracket, rounded outward, as the margin;
/// `outside(u)` past a wide end.
fn certify_parameter<T: SpanLocate>(
    knots: &Knots<T>,
    u: f64,
    outside: impl Fn(f64) -> KnotAlgebraError,
) -> Result<(), KnotAlgebraError> {
    if knots.is_point() {
        return Ok(());
    }
    let last = knots.runs().count() - 1;
    for (run, (value, _)) in knots.runs().enumerate() {
        if value.point_value().is_some() {
            continue;
        }
        let Some((lo, hi)) = value.structure_bracket() else {
            return Err(KnotAlgebraError::Structure(SplineError::KnotIsPoison {
                index: run,
            }));
        };
        if (run == 0 && u <= lo) || (run == last && u >= hi) {
            return Err(outside(u));
        }
        if lo <= u && u <= hi {
            return Err(KnotAlgebraError::ParameterUndecided {
                u,
                run,
                margin: crate::MarginDiag::enclosure((lo - u).next_down(), (hi - u).next_up()),
            });
        }
    }
    Ok(())
}

fn outside_domain(u: f64) -> KnotAlgebraError {
    KnotAlgebraError::ParameterOutsideDomain { u }
}

fn marked(plans: Vec<CurvePlan>, reference: bool) -> Vec<CurvePlan> {
    if reference {
        plans
    } else {
        plans
            .into_iter()
            .map(CurvePlan::without_reference)
            .collect()
    }
}

/// [`algebra::insert_knot_plan`] over held knots and weights.
///
/// # Errors
///
/// [`KnotAlgebraError::ParameterUndecided`] where `u`'s place is not one
/// for every member; otherwise [`algebra::insert_knot_plan`]'s.
pub fn insert_knot_plan<T: SpanLocate>(
    knots: &Knots<T>,
    weights: &Weights<T>,
    u: f64,
    times: usize,
) -> Result<Vec<CurvePlan>, KnotAlgebraError> {
    certify_parameter(knots, u, outside_domain)?;
    let (kv, w, reference) = inputs(knots, weights)?;
    Ok(marked(
        algebra::insert_knot_plan(&kv, &w, u, times)?,
        reference,
    ))
}

/// [`algebra::refine_plan`] over held knots and weights.
///
/// # Errors
///
/// As [`insert_knot_plan`], for each value of `add`.
pub fn refine_plan<T: SpanLocate>(
    knots: &Knots<T>,
    weights: &Weights<T>,
    add: &[f64],
) -> Result<Vec<CurvePlan>, KnotAlgebraError> {
    for &u in add {
        certify_parameter(knots, u, outside_domain)?;
    }
    let (kv, w, reference) = inputs(knots, weights)?;
    Ok(marked(algebra::refine_plan(&kv, &w, add)?, reference))
}

/// [`algebra::remove_knot_plan`] over held knots and weights: `u` names
/// a breakpoint by its value, so it must be a point one.
///
/// # Errors
///
/// As [`insert_knot_plan`], [`KnotAlgebraError::KnotNotPresent`] past a
/// wide end; otherwise [`algebra::remove_knot_plan`]'s.
pub fn remove_knot_plan<T: SpanLocate>(
    knots: &Knots<T>,
    weights: &Weights<T>,
    u: f64,
    times: usize,
) -> Result<Vec<RemovalStep>, KnotAlgebraError> {
    certify_parameter(knots, u, |u| KnotAlgebraError::KnotNotPresent { u })?;
    let (kv, w, reference) = inputs(knots, weights)?;
    let steps = algebra::remove_knot_plan(&kv, &w, u, times)?;
    Ok(if reference {
        steps
    } else {
        steps
            .into_iter()
            .map(|step| RemovalStep {
                plan: step.plan.without_reference(),
                reinsert: step.reinsert.without_reference(),
            })
            .collect()
    })
}

/// [`algebra::elevate_plan`] over held knots and weights, decided on
/// the plan proxy ([`inputs`]). Its structural decisions read the
/// structure alone; its one value-reading decision, each recomposition
/// pass's forward/backward split, reads the proxy's knot values, and
/// any split is exact in ℝ for every member of the held knots, so it
/// chooses only rounding. No parameter is certified.
///
/// # Errors
///
/// [`algebra::elevate_plan`]'s.
pub fn elevate_plan<T: SpanLocate>(
    knots: &Knots<T>,
    weights: &Weights<T>,
) -> Result<Vec<CurvePlan>, KnotAlgebraError> {
    let (kv, w, reference) = inputs(knots, weights)?;
    Ok(marked(algebra::elevate_plan(&kv, &w)?, reference))
}

/// The held value of the point breakpoint `u` names, or `None` where
/// none does, decided for every member: a removal's reinsertion puts
/// back the knot it took out as the curve's own value.
///
/// # Errors
///
/// As [`remove_knot_plan`]'s certification of `u`.
pub fn breakpoint_at<T: SpanLocate>(
    knots: &Knots<T>,
    u: f64,
) -> Result<Option<T>, KnotAlgebraError> {
    certify_parameter(knots, u, |u| KnotAlgebraError::KnotNotPresent { u })?;
    // Certified apart from every wide bracket, so only a point run can
    // be `u`, for every member.
    Ok(knots
        .runs()
        .find(|(value, _)| value.point_value().map(f64::to_bits) == Some(u.to_bits()))
        .map(|(value, _)| value))
}

/// Where a split at `u` falls in the held knots: how many copies of `u`
/// they hold, decided for every member, `u` strictly inside the domain.
///
/// # Errors
///
/// [`KnotAlgebraError::ParameterOutsideDomain`] where `u` is not inside
/// every member's domain; as [`insert_knot_plan`] where its place is
/// not one for every member.
pub fn split_multiplicity<T: SpanLocate>(
    knots: &Knots<T>,
    u: f64,
) -> Result<usize, KnotAlgebraError> {
    certify_parameter(knots, u, outside_domain)?;
    // `u` is certified apart from every wide bracket, so the stand-in's
    // ends place it as every member's do.
    let proxy = knots.plan_proxy().map_err(KnotAlgebraError::Structure)?;
    let (d0, d1) = proxy.domain();
    if !u.is_finite() || !(u > d0 && u < d1) {
        return Err(outside_domain(u));
    }
    Ok(proxy.multiplicity_of(u).map_or(0, |(m, _)| m))
}

/// The two children's knots of a split at `u`, over knots saturated
/// there (`u` at full interior multiplicity, the degree): the first
/// holds every knot below `u` and `degree + 1` copies of `u`, the second
/// one copy of `u` and every knot from `u` on. With them, `f`, the first
/// index of `u`, at which the control points part: the first child takes
/// `0..f`, the second `f − 1..`. Each child's breakpoints are the
/// saturated knots' own, so each keeps its held value
/// ([`Knots::restructure`]); `fresh` is the carrier's entry of a value,
/// which no breakpoint here needs. The children's structure is decided
/// against the stand-in, which does not leave this module.
///
/// # Errors
///
/// As [`split_multiplicity`] where `u`'s place is not one for every
/// member; [`KnotAlgebraError::KnotNotPresent`] where `u` is not a
/// breakpoint of `saturated`; [`KnotAlgebraError::Structure`] where a
/// child is no clamped vector.
#[allow(clippy::type_complexity)]
pub fn split_knots<T: SpanLocate>(
    saturated: &Knots<T>,
    u: f64,
    fresh: impl Fn(f64) -> T,
) -> Result<(usize, Knots<T>, Knots<T>), KnotAlgebraError> {
    certify_parameter(saturated, u, outside_domain)?;
    let p = saturated.degree();
    let proxy = saturated
        .plan_proxy()
        .map_err(KnotAlgebraError::Structure)?;
    let knots = proxy.knots();
    let f = knots
        .iter()
        .position(|k| *k == u)
        .ok_or(KnotAlgebraError::KnotNotPresent { u })?;
    let mut k1: Vec<f64> = knots[..f + p].to_vec();
    k1.push(u);
    let mut k2: Vec<f64> = vec![u];
    k2.extend_from_slice(&knots[f..]);
    let k1 = KnotVector::clamped(k1, p).map_err(KnotAlgebraError::Structure)?;
    let k2 = KnotVector::clamped(k2, p).map_err(KnotAlgebraError::Structure)?;
    Ok((
        f,
        saturated.restructure(k1, &fresh),
        saturated.restructure(k2, &fresh),
    ))
}

/// A carrier's held knots as a reader of their `f64` vector takes them:
/// a knot-vector merge, which decides which breakpoints of two vectors
/// are one, and the surface's column builds. At a wide run that is a
/// decision across carriers this module does not make (C6 S5).
///
/// # Errors
///
/// [`KnotAlgebraError::Structure`] carrying
/// [`SplineError::CarrierKnotNotAPoint`] at the first run whose value is
/// wider than a point.
pub fn point_knots<T: SpanLocate>(knots: &Knots<T>) -> Result<&KnotVector, KnotAlgebraError> {
    if let Some(index) = knots.first_wide_run() {
        return Err(KnotAlgebraError::Structure(
            SplineError::CarrierKnotNotAPoint { index },
        ));
    }
    Ok(knots.structure_f64_interim())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use core::num::NonZeroUsize;

    use super::{
        breakpoint_at, elevate_plan, insert_knot_plan, refine_plan, remove_knot_plan, split_knots,
        split_multiplicity,
    };
    use crate::interval::Interval;
    use crate::real::{Bounds, Real};
    use crate::spline::algebra::{self, CurvePlan, KnotAlgebraError};
    use crate::spline::{KnotVector, Knots, SpanLocate, Weights};

    fn m(n: usize) -> NonZeroUsize {
        NonZeroUsize::new(n).unwrap()
    }

    /// A quadratic whose one interior breakpoint is `mid` (a point or an
    /// enclosure), at `Interval`, with weights `w`.
    fn held(mid: Interval, w: &[f64]) -> (Knots<Interval>, Weights<Interval>) {
        let i = Interval::from_f64;
        let knots = Knots::new(vec![(i(0.0), m(3)), (mid, m(1)), (i(1.0), m(3))], 2).unwrap();
        let weights = Weights::new(w.iter().map(|x| i(*x)).collect()).unwrap();
        (knots, weights)
    }

    /// The homogeneous chain over held knots (a carrier's enclosure arm),
    /// on one coordinate channel.
    fn run(
        plans: &[CurvePlan],
        knots: &Knots<Interval>,
        xw: &[(Interval, Interval)],
    ) -> Vec<(Interval, Interval)> {
        let mut held = knots.clone();
        let mut net = xw.to_vec();
        let poison = (Interval::from_f64(f64::NAN), Interval::from_f64(f64::NAN));
        for plan in plans {
            net = plan.apply_homogeneous(
                held.values(),
                &net,
                (poison, &Interval::from_f64),
                Interval::homogeneous_step,
            );
            held = held.restructure(plan.knots().clone(), Interval::from_f64);
        }
        net
    }

    /// **At point held inputs the held constructors are the `f64` ones,
    /// bit for bit (C6 S5b).** An insertion, a refinement, a removal and
    /// an elevation over held point knots and weights give the plans the
    /// `f64` constructors give over the same vectors: the same knot
    /// vector and the same point-applier answer, every bit, and each is a
    /// reference plan.
    #[test]
    fn at_point_inputs_the_held_constructors_are_the_f64_ones() {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.4, 1.0, 1.0, 1.0], 2).unwrap();
        let w = [1.0, 2.0, 0.5, 1.0];
        let x = [0.0, 1.0, 4.0, 2.0];
        let knots = Knots::from(kv.clone()).lift(Interval::from_f64);
        let weights = Weights::new(w.iter().map(|v| Interval::from_f64(*v)).collect()).unwrap();
        let same = |tag: &str, got: Vec<CurvePlan>, want: Vec<CurvePlan>| {
            assert_eq!(got.len(), want.len(), "{tag}: chain length");
            for (g, f) in got.iter().zip(&want) {
                assert!(g.is_reference(), "{tag}: a point plan is a reference plan");
                assert_eq!(g.knots().knots(), f.knots().knots(), "{tag}: knots");
                let bits = |p: &CurvePlan| -> Vec<u64> {
                    p.apply_points(
                        &x[..x.len().min(p.knots().control_count())],
                        f64::NAN,
                        |a, b, l| a + (b - a) * l,
                    )
                    .iter()
                    .map(|v| v.to_bits())
                    .collect()
                };
                assert_eq!(bits(g), bits(f), "{tag}: point applier");
            }
        };
        same(
            "insert",
            insert_knot_plan(&knots, &weights, 0.7, 1).unwrap(),
            algebra::insert_knot_plan(&kv, &w, 0.7, 1).unwrap(),
        );
        same(
            "refine",
            refine_plan(&knots, &weights, &[0.2, 0.7]).unwrap(),
            algebra::refine_plan(&kv, &w, &[0.2, 0.7]).unwrap(),
        );
        same(
            "elevate",
            elevate_plan(&knots, &weights).unwrap(),
            algebra::elevate_plan(&kv, &w).unwrap(),
        );
        let r = remove_knot_plan(&knots, &weights, 0.4, 1).unwrap();
        let f = algebra::remove_knot_plan(&kv, &w, 0.4, 1).unwrap();
        same(
            "remove",
            r.into_iter().map(|s| s.plan).collect(),
            f.into_iter().map(|s| s.plan).collect(),
        );
    }

    /// **Over a wide breakpoint, a parameter's place is decided for every
    /// member.** With the interior breakpoint `[0.4, 0.45]`: inserting at
    /// `0.42`, which some members' breakpoint equals and others' lies on
    /// either side of, refuses `ParameterUndecided` at run 1 with a margin
    /// that straddles zero; inserting at `0.3` or `0.5`, clear of the
    /// bracket, decides; a multiplicity read and a removal at `0.42`
    /// refuse the same way. Red where a parameter is placed against the
    /// stand-in alone.
    #[test]
    fn over_a_wide_breakpoint_a_parameters_place_is_decided_for_every_member() {
        let (knots, weights) = held(Interval::from_bounds(0.4, 0.45), &[1.0; 4]);
        for result in [
            insert_knot_plan(&knots, &weights, 0.42, 1).map(|_| ()),
            refine_plan(&knots, &weights, &[0.1, 0.42]).map(|_| ()),
            split_multiplicity(&knots, 0.42).map(|_| ()),
            remove_knot_plan(&knots, &weights, 0.42, 1).map(|_| ()),
        ] {
            match result {
                Err(KnotAlgebraError::ParameterUndecided { run: 1, margin, .. }) => {
                    let crate::ErrorTextReading::Enclosure { lo, hi } =
                        margin.diagnostic_f64_for_error_text()
                    else {
                        panic!("an enclosure margin");
                    };
                    assert!(lo <= 0.0 && 0.0 <= hi, "margin [{lo}, {hi}] straddles");
                    // Rounded outward past the nearest-rounded distances.
                    assert!(
                        lo < 0.4 - 0.42 && 0.45 - 0.42 < hi,
                        "margin [{lo}, {hi}] outward"
                    );
                }
                other => panic!("a parameter inside a wide breakpoint: {other:?}"),
            }
        }
        for u in [0.3, 0.5] {
            let plans = insert_knot_plan(&knots, &weights, u, 1).unwrap();
            assert!(
                plans.iter().all(|p| !p.is_reference()),
                "a plan over a wide breakpoint is no reference"
            );
        }
        assert_eq!(split_multiplicity(&knots, 0.5).unwrap(), 0);
    }

    /// **A plan over a wide breakpoint, applied over the held knots,
    /// encloses every member's.** Inserting at `0.3` and elevating, over
    /// the interior breakpoint `[0.4, 0.45]`: at the members `0.4`,
    /// `0.425` and `0.45`, each member's own `f64` result lies in the
    /// homogeneous result over the held knots. And the `f64` appliers
    /// refuse such a plan rather than read its stand-in values. Red where
    /// the plan's steps read the stand-in instead of the held values.
    #[test]
    fn a_plan_over_a_wide_breakpoint_encloses_every_members() {
        let mid = Interval::from_bounds(0.4, 0.45);
        let w = [1.0, 2.0, 0.5, 1.0];
        let x = [0.0, 1.0, 4.0, 2.0];
        let (knots, weights) = held(mid, &w);
        let i = Interval::from_f64;
        let xw: Vec<(Interval, Interval)> = x
            .iter()
            .zip(&w)
            .map(|(x, w)| (i(*x) * i(*w), i(*w)))
            .collect();
        for (tag, plans) in [
            (
                "insert 0.3",
                insert_knot_plan(&knots, &weights, 0.3, 1).unwrap(),
            ),
            ("elevate", elevate_plan(&knots, &weights).unwrap()),
        ] {
            let got = run(&plans, &knots, &xw);
            for member in [0.4, 0.425, 0.45] {
                // The member's own chain on its homogeneous channels, which
                // refine as polynomial ones: the schedule at unit weights.
                let kv =
                    KnotVector::clamped(vec![0.0, 0.0, 0.0, member, 1.0, 1.0, 1.0], 2).unwrap();
                let unit = vec![1.0; kv.control_count()];
                let poly = match tag {
                    "insert 0.3" => algebra::insert_knot_plan(&kv, &unit, 0.3, 1).unwrap(),
                    _ => algebra::elevate_plan(&kv, &unit).unwrap(),
                };
                let mut fx: Vec<f64> = x.iter().zip(&w).map(|(x, w)| x * w).collect();
                let mut fw: Vec<f64> = w.to_vec();
                for p in &poly {
                    fx = p.apply_points(&fx, f64::NAN, |a, b, l| a + (b - a) * l);
                    fw = p.apply_points(&fw, f64::NAN, |a, b, l| a + (b - a) * l);
                }
                assert_eq!(got.len(), fx.len(), "{tag} at {member}: extent");
                for (k, ((gx, gw), (wx, ww))) in got.iter().zip(fx.iter().zip(&fw)).enumerate() {
                    let near = |g: Interval, f: f64| g.lo() - 1e-12 <= f && f <= g.hi() + 1e-12;
                    assert!(
                        near(*gx, *wx) && near(*gw, *ww),
                        "{tag} at {member}: slot {k}"
                    );
                }
            }
            for plan in &plans {
                assert!(
                    plan.apply_points(
                        &[0.0; 8][..plan.knots().control_count().min(8)],
                        f64::NAN,
                        |a, b, l| a + (b - a) * l
                    )
                    .iter()
                    .all(|v| v.is_nan()),
                    "{tag}: the point applier refuses"
                );
                assert!(
                    plan.apply_certified(&[i(0.0); 8][..plan.knots().control_count().min(8)])
                        .iter()
                        .all(|v| !v.is_certified()),
                    "{tag}: the ring applier refuses"
                );
            }
        }
    }

    fn next_up(x: f64) -> f64 {
        f64::from_bits(x.to_bits() + 1)
    }

    fn next_down(x: f64) -> f64 {
        f64::from_bits(x.to_bits() - 1)
    }

    /// **A bracket's ends are inside it, and past a wide domain end every
    /// member is outside.** Over the interior breakpoint `[0.4, 0.45]`,
    /// `u` at either end refuses `ParameterUndecided` (the member at that
    /// end has `u` as its breakpoint, the others do not), and the next
    /// float outside either end decides. Over a last breakpoint
    /// `[0.99, 1.0]`, `u` inside it refuses undecided, but `u` at its
    /// outer end is past every member's domain: an insertion and a split
    /// refuse `ParameterOutsideDomain`, a removal `KnotNotPresent`, and no
    /// narrowing would change that; at a wide first breakpoint
    /// `[0.0, 0.01]` the same holds at its outer end `0.0`. A split's
    /// children refuse a `u` inside a bracket, the stand-in's own value
    /// included. Red where the bracket test is open, where an end
    /// bracket's outer side asks for narrower knots, or where a split
    /// cuts the stand-in.
    #[test]
    fn a_brackets_ends_are_in_it_and_past_a_wide_domain_end_is_outside() {
        let (knots, weights) = held(Interval::from_bounds(0.4, 0.45), &[1.0; 4]);
        for u in [0.4, 0.45] {
            assert!(
                matches!(
                    insert_knot_plan(&knots, &weights, u, 1),
                    Err(KnotAlgebraError::ParameterUndecided { run: 1, .. })
                ),
                "u = {u}, a bracket end"
            );
            assert!(matches!(
                split_multiplicity(&knots, u),
                Err(KnotAlgebraError::ParameterUndecided { run: 1, .. })
            ));
        }
        for u in [next_down(0.4), next_up(0.45)] {
            assert!(
                insert_knot_plan(&knots, &weights, u, 1).is_ok(),
                "u = {u}, the next float outside"
            );
        }
        let i = Interval::from_f64;
        let end = Knots::new(
            vec![
                (i(0.0), m(3)),
                (i(0.5), m(1)),
                (Interval::from_bounds(0.99, 1.0), m(3)),
            ],
            2,
        )
        .unwrap();
        let w = Weights::new(vec![i(1.0); 4]).unwrap();
        for u in [0.99, 0.995] {
            assert!(
                matches!(
                    insert_knot_plan(&end, &w, u, 1),
                    Err(KnotAlgebraError::ParameterUndecided { run: 2, .. })
                ),
                "u = {u}, inside the end bracket"
            );
        }
        for u in [1.0, 1.5] {
            assert_eq!(
                insert_knot_plan(&end, &w, u, 1).unwrap_err(),
                KnotAlgebraError::ParameterOutsideDomain { u }
            );
            assert_eq!(
                split_multiplicity(&end, u).unwrap_err(),
                KnotAlgebraError::ParameterOutsideDomain { u }
            );
            assert_eq!(
                remove_knot_plan(&end, &w, u, 1).unwrap_err(),
                KnotAlgebraError::KnotNotPresent { u }
            );
        }
        let start = Knots::new(
            vec![
                (Interval::from_bounds(0.0, 0.01), m(3)),
                (i(0.5), m(1)),
                (i(1.0), m(3)),
            ],
            2,
        )
        .unwrap();
        assert!(matches!(
            insert_knot_plan(&start, &w, 0.005, 1),
            Err(KnotAlgebraError::ParameterUndecided { run: 0, .. })
        ));
        assert_eq!(
            insert_knot_plan(&start, &w, 0.0, 1).unwrap_err(),
            KnotAlgebraError::ParameterOutsideDomain { u: 0.0 }
        );
        assert_eq!(
            split_multiplicity(&start, 0.0).unwrap_err(),
            KnotAlgebraError::ParameterOutsideDomain { u: 0.0 }
        );
        // A split's children are decided for every member too: at the
        // stand-in's own value inside the bracket it refuses, rather than
        // cut the knots at a value no member has.
        let proxy = f64::midpoint(0.4, 0.45);
        assert!(matches!(
            split_knots(&knots, proxy, Interval::from_f64),
            Err(KnotAlgebraError::ParameterUndecided { run: 1, .. })
        ));
    }

    /// **A restructured wide carrier keeps no `f64` vector.** Inserting at
    /// `0.7` over the interior breakpoint `[0.4, 0.45]`, the restructured
    /// knots hold the wide run's value still, so they are not points and
    /// keep no `f64` vector: the plan's vector there is the stand-in's
    /// midpoint, no value of the knots. Over point knots the restructured
    /// ones are points and their vector is the plan's; and with a weight
    /// wider than a point the plan is marked all the same. Red where
    /// `restructure` keeps the plan's vector as the knots' own, or where a
    /// plan over wide weights passes for a reference.
    #[test]
    fn a_restructured_wide_carrier_keeps_no_f64_vector() {
        let (knots, weights) = held(Interval::from_bounds(0.4, 0.45), &[1.0; 4]);
        let plans = insert_knot_plan(&knots, &weights, 0.7, 1).unwrap();
        let after = knots.restructure(plans[0].knots().clone(), Interval::from_f64);
        assert!(!after.is_point(), "a wide run keeps no f64 vector");
        assert_eq!(after.first_wide_run(), Some(1));
        let wide = after.values()[3];
        assert_eq!((wide.lo(), wide.hi()), (0.4, 0.45), "the run's held value");
        let (knots, weights) = held(Interval::from_f64(0.4), &[1.0; 4]);
        let plans = insert_knot_plan(&knots, &weights, 0.7, 1).unwrap();
        let after = knots.restructure(plans[0].knots().clone(), Interval::from_f64);
        assert!(after.is_point());
        assert_eq!(
            after.structure_f64_interim().knots(),
            plans[0].knots().knots()
        );
        // Over point knots with a wide weight, the plan is decided at the
        // stand-in's unit weights, so it is marked too.
        let (knots, _) = held(Interval::from_f64(0.4), &[1.0; 4]);
        let weights = Weights::new(vec![
            Interval::from_f64(1.0),
            Interval::from_bounds(1.0, 1.1),
            Interval::from_f64(1.0),
            Interval::from_f64(1.0),
        ])
        .unwrap();
        let plans = insert_knot_plan(&knots, &weights, 0.7, 1).unwrap();
        assert!(
            plans.iter().all(|p| !p.is_reference()),
            "a wide weight marks the plan"
        );
    }

    /// **A removal's reinsertion reads the curve's own breakpoint.** Over
    /// a cubic with the wide run `[0.4, 0.41]` beside the point
    /// breakpoints `0.2` and `0.7`: `0.2` names the held `0.2`, `0.3`
    /// names none, and `0.405` is undecided. Red where the reinsertion
    /// still asks the knots for an `f64` vector they do not have.
    #[test]
    fn a_removals_reinsertion_reads_the_curves_own_breakpoint() {
        let (knots, _) = cubic_with_a_wide_double_run();
        let at = breakpoint_at(&knots, 0.2).unwrap().unwrap();
        assert_eq!((at.lo(), at.hi()), (0.2, 0.2));
        assert!(breakpoint_at(&knots, 0.3).unwrap().is_none());
        assert!(matches!(
            breakpoint_at(&knots, 0.405),
            Err(KnotAlgebraError::ParameterUndecided { run: 2, .. })
        ));
    }

    type Runs = Vec<(f64, usize)>;

    /// A cubic whose breakpoints are `0.2`, the wide `[0.4, 0.41]` twice,
    /// and `0.7`, at `Interval`, unit weights; and its members' runs.
    fn cubic_with_a_wide_double_run() -> (Knots<Interval>, Vec<Runs>) {
        let i = Interval::from_f64;
        let knots = Knots::new(
            vec![
                (i(0.0), m(4)),
                (i(0.2), m(1)),
                (Interval::from_bounds(0.4, 0.41), m(2)),
                (i(0.7), m(1)),
                (i(1.0), m(4)),
            ],
            3,
        )
        .unwrap();
        let members = [0.4, 0.405, 0.41]
            .iter()
            .map(|e| vec![(0.0, 4), (0.2, 1), (*e, 2), (0.7, 1), (1.0, 4)])
            .collect();
        (knots, members)
    }

    type Held<'a> = &'a dyn Fn(
        &Knots<Interval>,
        &Weights<Interval>,
    ) -> Result<Vec<CurvePlan>, KnotAlgebraError>;
    type Member<'a> = &'a dyn Fn(&KnotVector, &[f64]) -> Vec<CurvePlan>;

    /// That the held plans, applied over the held knots, enclose each
    /// member's own `f64` chain, step for step on the same structure.
    fn encloses(tag: &str, knots: &Knots<Interval>, members: &[Runs], held: Held, member: Member) {
        let p = knots.degree();
        let n = knots.structure().control_count();
        let w: Vec<f64> = (0..n).map(|k| 1.0 + 0.3 * k as f64).collect();
        let x: Vec<f64> = (0..n).map(|k| ((k * 7) % 5) as f64).collect();
        let i = Interval::from_f64;
        let weights = Weights::new(w.iter().map(|v| i(*v)).collect()).unwrap();
        let plans = held(knots, &weights).unwrap();
        assert!(plans.iter().all(|q| !q.is_reference()), "{tag}: marked");
        let xw: Vec<(Interval, Interval)> = x
            .iter()
            .zip(&w)
            .map(|(x, w)| (i(*x) * i(*w), i(*w)))
            .collect();
        let got = run(&plans, knots, &xw);
        for runs in members {
            let kv = KnotVector::clamped(
                runs.iter()
                    .flat_map(|(v, k)| core::iter::repeat_n(*v, *k))
                    .collect(),
                p,
            )
            .unwrap();
            let poly = member(&kv, &vec![1.0; kv.control_count()]);
            assert_eq!(poly.len(), plans.len(), "{tag} at {runs:?}: chain length");
            for (a, b) in poly.iter().zip(&plans) {
                assert_eq!(
                    a.knots().structure(),
                    b.knots().structure(),
                    "{tag} at {runs:?}: structure"
                );
            }
            let mut fx: Vec<f64> = x.iter().zip(&w).map(|(x, w)| x * w).collect();
            let mut fw = w.clone();
            for q in &poly {
                fx = q.apply_points(&fx, f64::NAN, |a, b, l| a + (b - a) * l);
                fw = q.apply_points(&fw, f64::NAN, |a, b, l| a + (b - a) * l);
            }
            assert_eq!(got.len(), fx.len(), "{tag} at {runs:?}: extent");
            for (k, ((gx, gw), (wx, ww))) in got.iter().zip(fx.iter().zip(&fw)).enumerate() {
                let near = |g: Interval, f: f64| g.lo() - 1e-12 <= f && f <= g.hi() + 1e-12;
                assert!(
                    near(*gx, *wx) && near(*gw, *ww),
                    "{tag} at {runs:?}: slot {k}"
                );
            }
        }
    }

    fn removal_plans(steps: Vec<algebra::RemovalStep>) -> Vec<CurvePlan> {
        steps.into_iter().map(|s| s.plan).collect()
    }

    /// **The enclosure holds at a wide domain end, at a wide run of
    /// multiplicity two, and through removal.** Over a quadratic whose
    /// last breakpoint is `[0.99, 1.0]`: an insertion, an elevation and a
    /// removal of the point breakpoint beside it. Over a cubic with the
    /// wide run `[0.4, 0.41]` twice: an elevation, a double insertion, a
    /// refinement with values beside the bracket and a tie, and removals
    /// on either side of it. In each, at members across the bracket, the
    /// held plans over the held knots enclose the member's own chain, on
    /// its structure step for step. Red where a step reads the stand-in
    /// rather than the held values, or a decision differs by member.
    #[test]
    fn the_enclosure_holds_at_a_wide_end_at_multiplicity_and_through_removal() {
        let i = Interval::from_f64;
        let end = Knots::new(
            vec![
                (i(0.0), m(3)),
                (i(0.5), m(1)),
                (Interval::from_bounds(0.99, 1.0), m(3)),
            ],
            2,
        )
        .unwrap();
        let members: Vec<Runs> = [0.99, 0.995, 1.0]
            .iter()
            .map(|e| vec![(0.0, 3), (0.5, 1), (*e, 3)])
            .collect();
        encloses(
            "end: insert 0.7 twice",
            &end,
            &members,
            &|k, w| insert_knot_plan(k, w, 0.7, 2),
            &|kv, w| algebra::insert_knot_plan(kv, w, 0.7, 2).unwrap(),
        );
        encloses(
            "end: elevate",
            &end,
            &members,
            &|k, w| elevate_plan(k, w),
            &|kv, w| algebra::elevate_plan(kv, w).unwrap(),
        );
        encloses(
            "end: remove 0.5",
            &end,
            &members,
            &|k, w| remove_knot_plan(k, w, 0.5, 1).map(removal_plans),
            &|kv, w| removal_plans(algebra::remove_knot_plan(kv, w, 0.5, 1).unwrap()),
        );
        let (cubic, members) = cubic_with_a_wide_double_run();
        let add = [0.3, 0.39, 0.42, 0.42, 0.9];
        encloses(
            "double: elevate",
            &cubic,
            &members,
            &|k, w| elevate_plan(k, w),
            &|kv, w| algebra::elevate_plan(kv, w).unwrap(),
        );
        encloses(
            "double: insert 0.5 twice",
            &cubic,
            &members,
            &|k, w| insert_knot_plan(k, w, 0.5, 2),
            &|kv, w| algebra::insert_knot_plan(kv, w, 0.5, 2).unwrap(),
        );
        encloses(
            "double: refine",
            &cubic,
            &members,
            &|k, w| refine_plan(k, w, &add),
            &|kv, w| algebra::refine_plan(kv, w, &add).unwrap(),
        );
        for u in [0.2, 0.7] {
            encloses(
                &format!("double: remove {u}"),
                &cubic,
                &members,
                &|k, w| remove_knot_plan(k, w, u, 1).map(removal_plans),
                &|kv, w| removal_plans(algebra::remove_knot_plan(kv, w, u, 1).unwrap()),
            );
        }
    }
}
