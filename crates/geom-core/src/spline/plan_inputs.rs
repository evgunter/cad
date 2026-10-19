//! A carrier's held knots and weights as the knot algebra's plan
//! constructors read them (C6 S5b): the held constructors, through which
//! a carrier's knot operation decides its plans at every scalar.
//!
//! The constructors in [`super::algebra`] decide a plan's structure on an
//! `f64` vector. At point knots and weights that is their own vector, so
//! the plans are today's bit for bit. Where a run or a weight is wider
//! than a point, the plan is decided against a stand-in
//! ([`Knots::plan_proxy`]: each wide run at its bracket's midpoint, unit
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

use super::algebra::{self, CurvePlan, KnotAlgebraError, RemovalStep};
use super::knot_values::Knots;
use super::knots::{KnotVector, SplineError};
use super::locate::SpanLocate;
use super::weights::Weights;

/// The held knots and weights as the plan constructors read them: their
/// own `f64` vectors where every one is a point, else the stand-in
/// (module docs), and whether they are the inputs' own. Returned, not
/// held: no field keeps the copy past the call.
fn inputs<T: SpanLocate>(
    knots: &Knots<T>,
    weights: &Weights<T>,
) -> Result<(KnotVector, Vec<f64>, bool), KnotAlgebraError> {
    if knots.is_point() && weights.is_point() {
        return Ok((
            knots.structure_f64_interim().clone(),
            weights.weights_f64_interim().to_vec(),
            true,
        ));
    }
    let index = knots.first_wide_run().unwrap_or(0);
    let proxy =
        knots
            .plan_proxy()
            .ok_or(KnotAlgebraError::Structure(SplineError::KnotIsPoison {
                index,
            }))?;
    Ok((proxy, vec![1.0; weights.len()], false))
}

/// That `u` is a breakpoint, or lies apart from every breakpoint, for
/// every member of the held knots: a point run equal to `u` is the one
/// breakpoint, and a wide run whose bracket holds `u` leaves undecided
/// whether `u` is in it, beside it, or in which span.
///
/// # Errors
///
/// [`KnotAlgebraError::ParameterUndecided`] at the first such run, with
/// `u`'s distance to its bracket as the margin.
fn certify_parameter<T: SpanLocate>(knots: &Knots<T>, u: f64) -> Result<(), KnotAlgebraError> {
    for (run, (value, _)) in knots.runs().enumerate() {
        if value.point_value().is_some() {
            continue;
        }
        let Some((lo, hi)) = value.structure_bracket() else {
            return Err(KnotAlgebraError::Structure(SplineError::KnotIsPoison {
                index: run,
            }));
        };
        if lo <= u && u <= hi {
            return Err(KnotAlgebraError::ParameterUndecided {
                u,
                run,
                margin: crate::MarginDiag::enclosure(lo - u, hi - u),
            });
        }
    }
    Ok(())
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
    certify_parameter(knots, u)?;
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
        certify_parameter(knots, u)?;
    }
    let (kv, w, reference) = inputs(knots, weights)?;
    Ok(marked(algebra::refine_plan(&kv, &w, add)?, reference))
}

/// [`algebra::remove_knot_plan`] over held knots and weights: `u` names
/// a breakpoint by its value, so it must be a point one.
///
/// # Errors
///
/// As [`insert_knot_plan`]; otherwise [`algebra::remove_knot_plan`]'s.
pub fn remove_knot_plan<T: SpanLocate>(
    knots: &Knots<T>,
    weights: &Weights<T>,
    u: f64,
    times: usize,
) -> Result<Vec<RemovalStep>, KnotAlgebraError> {
    certify_parameter(knots, u)?;
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

/// [`algebra::elevate_plan`] over held knots and weights. Its decisions
/// read the structure alone, so no parameter is certified.
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

/// How many copies of `u` the held knots hold, decided for every member.
///
/// # Errors
///
/// As [`insert_knot_plan`].
pub fn multiplicity<T: SpanLocate>(knots: &Knots<T>, u: f64) -> Result<usize, KnotAlgebraError> {
    certify_parameter(knots, u)?;
    let index = knots.first_wide_run().unwrap_or(0);
    let proxy =
        knots
            .plan_proxy()
            .ok_or(KnotAlgebraError::Structure(SplineError::KnotIsPoison {
                index,
            }))?;
    Ok(proxy.multiplicity_of(u).map_or(0, |(m, _)| m))
}

/// A carrier's held knots as a knot-vector merge reads them: their own
/// `f64` vector. Merging decides which breakpoints of two vectors are
/// one, which at a wide run is a decision across carriers this module
/// does not make (C6 S5).
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

    use super::{elevate_plan, insert_knot_plan, multiplicity, refine_plan, remove_knot_plan};
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
            multiplicity(&knots, 0.42).map(|_| ()),
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
        assert_eq!(multiplicity(&knots, 0.5).unwrap(), 0);
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
}
