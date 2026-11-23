The from-f64 red is fixed and pushed: 672e300e3a, still a work-in-progress head. The nine samples now step in `T` (`step = (t1 − t0) / T::from_f64(8.0)`, a literal, then `t = t + step`), and `from-f64-allowlist` is green locally.

On NaN, your premise doesn't hold here. The fold calls `Real::max`, which propagates NaN (real.rs:623, "either input NaN ⇒ NaN"), unlike `f64::max`. So a poisoned sample poisons the margin, and the decision's poison arm answers `Escalated` with the poisoned margin as its diagnostic; the fold does not drop it.

I kept escalation rather than a hand-made defect, because `Real::is_poison`'s doc says predicates go through `Decide`'s poison arm and the method never replaces one. The kind table's doc now says so. New row: `incidence_at_the_shape_door::a_carrier_that_evaluates_to_no_finite_point_escalates`, a line with a NaN direction on the unit sphere, which answers `Escalated` under `props_carrier_on_surface`.

The pre-push chain has restarted on 672e300e3a. The report follows when the runs finish.
