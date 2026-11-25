---
id: the-from-f64-split-leaves-five-questions-dl5-2-does-not-settle
kind: issue
title: The from_f64 split leaves five questions DL5.2 does not settle: count_ratio, a count a refinement fixed, a ratio's numerator, the fixture and concrete classes, a run option's frame
status: open
opened: 2026-11-24
priority: P1
design: true
parent: from-f64-is-split-by-where-the-number-comes-from
---


Found by the two reviews of PR 7721 (`from-f64-is-split-by-where-the-number-comes-from`), which built the doors DL5.2's last row names (`crates/geom-core/README.md`). Each question below is one the rule does not answer; the PR chose, said so, and nothing was moved in value by any choice (both reviews: base and head agree bit for bit on `geom-core` at `f64`, `Interval`, `Sym<f64>` and `Dual64`).

1. **`Real::count_ratio` is an addition to "a count as an integer".** It enters the one `f64` quotient `i as f64 / n as f64` as a point and a `Sym` literal. Thirteen call sites. The reviews' reading of each:
   - nine are schedule stations or cell corners, places by DL5.2's second row entered as constants: `geom-brep` `certify.rs` (`schedule_param`, and the mapped source's parameter twice), `pcurve_cache.rs` `sample_schedule`, `pcurve_cache/spline_incidence.rs`, `pcurve_cache/projected.rs` (a cone line's cuts), `curve_face/pose.rs`, `geom` `carrier_speed.rs`, `curves/nurbs.rs` (`length_upper_bound`). All but possibly the last are dyadic, so the rounded point and the rational agree;
   - two are exact values: `geom` `azimuth.rs` (`m·π/2`), `topo` `pcurves/decision.rs` (`n` is 1 or 2);
   - `sweep/src/skin.rs` has two with an inexact `k/m`: a point evaluated at, and a stored knot. Only the knot needs the rounded quotient: it must be the `f64` the mesher's breaks and knot merging hold, so the rational's enclosure would be wrong there.

   Either keep `count_ratio` for the knot and send the stations to the place row (`a-place-entered-through-from-f64-reads-as-a-constant-at-sym`), or keep it as the schedule's spelling everywhere. The second is what the tree holds.
2. **A count fixed by a numeric refinement or by banded decisions.** `geom-brep/src/chart_singularity.rs` `walk_loop` (`total / T::count(2 * samples.len())`, the samples an adaptive bisection left, whose quotient the next round is picked against) and `topo/src/boolean/fate.rs` `tip_wraps` (`T::count(halves)`, the corners a banded decision admitted as half turns). The count door's doc says a count is a literal "while the structure it counts is the one the valuation was taken on"; whether a count the values chose is structure is the question.
3. **`Ratio::eval` enters a reduced numerator through `Real::count`** (`editor-core/src/ratio.rs`). The value is right (exact, the exact rational at `Sym`); a numerator counts nothing. It also keeps `count(num) / count(den)`, the rational the rule names, where `count_ratio` elsewhere avoids it.
4. **The `fixture` and `concrete` classes have no rule.** `scripts/gates/from-f64-sites.tsv` lists 29 `fixture` sites: production code behind `cfg(test)` or the `test-support` feature, each lifting `f64`-built data into `T` (DL5.2's third row for any `Sym` evidence built on them), each a bare door for whoever enables the feature. It lists 12 `concrete` sites: a point of the concrete `Interval`, classed by the receiver's spelling, asking nothing about the number (`topo::clearance::support_separation` enters bracket ends that way; both reviews read that one as a sound outward-rounded bound).
5. **What `ComputingTwist`'s frame is.** `editor-core/src/eval/mod.rs` `ComputingTwist::at` enters an axis, an angle and a translation that `for_node` computes at run time from a node's ordinal and mint digest (`sqrt`, `sin`, `rem_euclid`). No document unknown moves them, so a literal at `Sym` is true of them; nobody authored them. They are back on the bare door, listed `owed` to `from-f64-sites-the-split-left-on-the-bare-door`: a fixture, an owed site, or a fifth source ("a constant the run chose").
