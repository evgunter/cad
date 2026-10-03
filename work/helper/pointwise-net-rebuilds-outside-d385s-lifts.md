---
id: pointwise-net-rebuilds-outside-d385s-lifts
kind: issue
title: 44 test sites rebuild a validated NURBS net through new(..).unwrap() where map_points is the door
status: open
opened: 2026-09-15
refs: [1782]
priority: P3
cost: E
---

## Finding

`NurbsSurface::map_points` and `NurbsCurve2/3::map_points` already exist
— "the same net with every control point carried through `f`, the knot
vectors and the weights verbatim", built through
`from_validated_parts`, no `Result`. Forty-four sites in `crates/*/tests`
and `crates/*/examples` spell that map by hand and push the result back
through `NurbsSurface::new` / `NurbsCurve2/3::new` followed by
`.unwrap()` or `.expect(..)`. The knots are `X.knots*().clone()` and the
weights are `X.weights().to_vec()` at every one of them, so **no count
and no weight value changes and the `Result` is unreachable** — the same
shape `D320` named inside `crates/sweep/` and the same shape
`sweep-test-rebuilds-validated-net-for-v-reversal` (SCALAR) fixed for
the one v-reversal instance.

This is not `D385`. That row is the SCALAR-LIFT subcase —
`from_f64` coordinate by coordinate, whose door is `map_scalar` — and
names six NURBS sites. What is filed here is the rest of the pointwise
class: perturbations, translations, slides, mirrors and poison
injections, whose door is `map_points`. `D385`'s six sites are a subset
of the list below and the two rows should be decided together; its
retain/retire criterion (a reviewer-authored battery whose independence
from the door IS its regression value is exempt, decide per file) is
the right criterion here too and is not restated.

### The 44 sites

`crates/geom/tests/curves/`: `compose.rs:254`, `n1r2_dump.rs:37`,
`n1r2_lift_probes_interval.rs:209`, `nurbs_differential.rs:396`,
`nurbs_differential.rs:437`, `nurbs_interval.rs:126`,
`review_m5_pr3_attack.rs:88`, `review_m5_pr3_attack.rs:308`,
`review_m5_pr3_attack_interval.rs:40`,
`review_m5_pr3_attack_interval.rs:162`,
`review_m5_pr3_attack_interval.rs:210`, `span_window_pairing.rs:35`.

`crates/geom/tests/`: `span_bit_identity_ext.rs:74`,
`span_bit_identity_ext.rs:85`, `surfaces/nurbs_surface.rs:253`,
`surfaces/nurbs_surface_interval.rs:102`,
`surfaces/review_m5_pr3_attack.rs:216`,
`surfaces/review_m5_pr3_attack_interval.rs:52`,
`surfaces/span_window_pairing.rs:55`.

`crates/geom-brep/tests/`: `approx_surface.rs:290`,
`interior_iso_column.rs:99`, `interior_iso_column.rs:115`,
`interior_iso_review.rs:91`, `interior_iso_review.rs:106`,
`m5_pr7_ssi.rs:972`, `m5_pr7_ssi.rs:1222`, `offb_r1_probes.rs:164`,
`offset_fit.rs:737`, `pcurve_mirror_v.rs:156`,
`review_m5_pr3_e2e.rs:105`, `review_probes_m8_4.rs:108`.

`crates/step-import/tests/cert_n2r2_consumer_probes.rs:55`.

`crates/sweep/tests/`: `common/approx.rs:203`, `offc_r1_probes.rs:346`,
`r1_p2_probes.rs:337`, `verbs_offc_consumer.rs:134`,
`verbs_offc_consumer.rs:535`, `verbs_offc_consumer.rs:796`.

`crates/topo/tests/`: `fixture/mod.rs:258`, `fixture/mod.rs:267`,
`m6_3_chart_completion.rs:130`, `review_m6_2_probes.rs:123`,
`review_ssiflat_r1_probes.rs:119`, `review_ssiflat_r2_probes.rs:109`.

**Six of them are in `crates/sweep/`, which PR 1782's style review
already swept for this shape** (`D320`, `D321`). That sweep found the
hand-rebuilt v-reversal and missed these, so the pattern it used did
not match a net rebuilt with a descriptive `.expect("a translated net
…")` rather than a bare `.unwrap()`.

### What is NOT in the list

Eighteen further sites read structure off an existing curve or surface
but change a WEIGHT or a KNOT VECTOR — `w * k`, `vec![1.0; n]`, a
rescaled knot vector, a row slice of a surface net. Weight positivity
and finiteness and the count agreement are exactly what `new` checks, so
those `Result`s are reachable and the `unwrap` announces something.
They are listed in the SCALAR PR that filed this row and are not this
row's business: `crates/geom/tests/curves/compose.rs:244` and
`review_m5_pr3_attack.rs:283`; `crates/geom-brep/tests/`
`cert10_r1_probes.rs:326`, `cert7_r1_probes.rs:158`,
`cert7_r1_probes.rs:226`, `cert7_r2_probes.rs:114`,
`cert7_r2_probes.rs:147`, `interior_iso_column.rs:190`,
`interior_iso_review.rs:123`, `:361`, `:373`, `:407`, `:412`, `:432`,
`offb_r1_probes.rs:69`, `offb_r2_probes.rs:402`;
`crates/mesh/tests/patch_memo.rs:552`;
`crates/sweep/tests/m8_4_intersection_iso.rs:756`.

`demos/`, `tools/` and `benches/` name none of these constructors at
all, so the class does not reach them.

### The sweep, re-run as a DISJUNCTION

The sweep that filed this row kept a call only when its argument text
read **both** a knot vector and a control/weight vector off an existing
object. That conjunction silently excluded a whole shape — a call with
FRESH knots and a verbatim net and weights — and said nothing about it,
which is a blind spot a hit list is supposed to disclose. Re-run as a
disjunction (either read qualifies) over the same roots plus `src/`:
**405 constructor calls, 104 hits**, of which 78 are the conjunctive
hits already listed above and **26 are new**. Every new hit's
disposition:

**In the class — a rescaled knot vector over a verbatim net and
weights.** The knots are fresh (an affine rescale of the source's) and
the net and weights are carried whole, so no count and no weight value
changes and the `Result` is unreachable, exactly as in the pointwise
bucket. `map_points` is NOT the door here — it carries knots verbatim —
so these have **no existing door** and want one (a knot-rescale door on
the curve) or a retain per the criterion above:
`crates/geom-brep/tests/pcurve_general.rs:208`,
`review_m5_pr7b_ssi.rs:287`;
`crates/topo/tests/m6_3_chart_completion.rs:121`,
`review_ssiflat_r1_probes.rs:110`, `review_ssiflat_r2_probes.rs:100`;
`crates/geom-brep/tests/review_probes_m8_4.rs:134` (a deliberately
FOREIGN knot vector — `clamped([0,0,2,2], 1)` — over
`b.control()`/`b.weights()` verbatim; the row's subject is that the
certify refuses, and the `unwrap` announces nothing).

**In the class — one control point moved by INDEX.** See the
re-bucketing below: `crates/geom/tests/curves/fit_certify.rs:149`,
`review_m5_pr2_e2e.rs:251`, `review_m5_pr4_adversarial.rs:936`.

**Not the class — `new`'s check is live.** The control vector or the
weights are freshly built, so a count or a weight value really can
disagree: `crates/geom-brep/tests/interior_iso_column.rs:239`, `:276`
(a two-point carrier off `chart.knots_v()`);
`crates/sweep/tests/r1_p2_probes.rs:224`, `:715`,
`m8_4_intersection_iso.rs:386`, `crates/sweep/examples/r2_p2_consumer.rs:76`
(a widened chart, more columns than the source).

**Not the class — the row's subject IS the constructor's check.**
`crates/geom/tests/surfaces/span_window_pairing.rs:306` (comment: *"the
count relation is all `new` checks"*) and `:320` (`assert!(… .is_err())`).

### `#[cfg(test)]` modules under `src/` were outside the stated roots

The roots were `crates/*/tests`, `crates/*/examples`, `demos/`,
`tools/` and `benches/`, so a test module compiled from inside `src/`
was invisible to it. Including `crates/*/src` finds 25 further hits;
those inside a `#[cfg(test)]` module are in the class's scope and are
**EXCH's ground** (`crates/step-import/*`), filed here with their
dispositions because this row already carries the class:

- `crates/step-import/src/cr_r1_probes.rs:102`, `:140`, `:155` and
  `crates/step-import/src/recognize_curve.rs:659`, `:834`, `:1063` —
  knots and weights verbatim, ONE control point (or a parity-indexed
  subset) moved. **No existing door** — the same re-bucketing as below.
- `crates/step-import/src/cr_r1_probes.rs:121` and
  `recognize_curve.rs:889` — a genuine pointwise map (`p.x * (1 + e)`,
  `p.y *= …`) with knots and weights verbatim. **`map_points` is the
  door**; same bucket as the 44.
- `crates/topo/src/census.rs:4178` — control mapped pointwise to
  poison, knots and weights verbatim. **`map_points` is the door.**
- `crates/mesh/src/chords.rs:803`, `:958`, `:968` (a fresh weight
  vector), `:851` (`assert!(… .is_err())`),
  `crates/step-import/src/adopt.rs:1217` (a weight changes), `:1228`
  (a fresh net), `crates/step-export/src/writer.rs:1097` (fresh knots
  AND fresh weights) — **not the class**: `new`'s weight and count
  checks are live.
- `crates/geom/src/surfaces/nurbs.rs`'s `reversal_tests` — the
  hand-built direct permutations the reversal door is compared
  against. **Retain by construction**: the comparison IS the row.

Production (non-`cfg(test)`) `src/` sites are outside the class, but
two are the same shape and worth naming: `crates/topo/src/replace_face.rs:941`
and `:969` translate and map a `Curve3::Nurbs` payload by hand through
`NurbsCurve3::new(…)?` where `map_points` is the door. They propagate
with `?` rather than claiming, so no unreachable claim is being made —
but the rebuild is still hand-written.

### Re-bucketing: `map_points` carries no index

Five of the 44 sites above, and six of the `src/` sites, perturb ONE
control point — `ctrl[mid] += …` — and carry everything else verbatim.
`map_points` takes `impl Fn(Point3<T>) -> Point3<T>` and **carries no
index**, so it cannot express "move slot `mid`": a caller would have to
compare coordinates to recognize the point, which is worse than the
rebuild. These sites therefore have **no existing door**, and what this
row asks for them is an indexed door (`map_points_indexed`, or a
`with_control_point(i, p)`) or a retain — not a conversion:
`crates/geom/tests/curves/review_m5_pr3_attack.rs`,
`curves/nurbs_differential.rs`, `surfaces/nurbs_surface.rs`,
`crates/geom-brep/tests/m5_pr7_ssi.rs` (two sites), and the
`fit_certify.rs` / `review_m5_pr2_e2e.rs` /
`review_m5_pr4_adversarial.rs` sites the disjunction added.

`crates/geom-brep/tests/pcurve_mirror_v.rs:156` is a **retain by
construction**: the row is
`the_nurbs_arms_are_map_points_and_agree_with_a_re_validated_rebuild`,
whose whole content is the differential between the hand rebuild and
`map_points`. Converting it would delete the test.

### What the pattern still could not match

A balanced-paren scan of the argument text of every
`NurbsSurface::new(` / `NurbsCurve{,2,3}::new(` call. Even as a
disjunction it cannot see: a rebuild whose parts are ALL bound to
locals first with no `.knots()`/`.control()`/`.weights()` inside the
call itself; a rebuild reached through a helper function or a macro
body; the same shape on a validated type that is not one of these four;
and any constructor other than `new` (a net assembled elsewhere and
wrapped in `Surface::Nurbs(Arc::new(..))`). The counts are as of
`origin/main` at the SCALAR VREV fix pass's merge and a lane that lands
this owes a re-sweep.

## Was

Filed by SCALAR's `sweep-test-rebuilds-validated-net-for-v-reversal`
(the VREV door), whose spec §3 required this sweep.
