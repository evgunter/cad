---
id: pointwise-net-rebuilds-outside-d385s-lifts
kind: issue
title: 44 test sites rebuild a validated NURBS net through new(..).unwrap() where map_points is the door
status: open
opened: 2026-09-15
refs: [1782]
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

### The pattern, and what it could not match

A balanced-paren scan of every `NurbsSurface::new(` /
`NurbsCurve{,2,3}::new(` call under `crates/*/tests`,
`crates/*/examples`, `demos/`, `tools/` and `benches/` (282 calls),
keeping those whose ARGUMENT TEXT reads both a knot vector and a
control/weight vector off an existing object. It cannot see: a rebuild
whose parts are bound to locals first — **14 further calls have such a
read within the 25 lines above them**
(`crates/geom/tests/curves/fit_certify.rs:149`,
`nurbs_differential.rs:305`, `review_m5_pr2_e2e.rs:251`,
`review_m5_pr3_attack.rs:110`, `review_m5_pr4_adversarial.rs:936`;
`crates/geom-brep/tests/pcurve_general.rs:208`,
`review_m5_pr7b_ssi.rs:287`;
`crates/sweep/tests/m8_4_intersection_iso.rs:386`,
`r1_p2_probes.rs:224`, `r1_p2_probes.rs:715`;
`crates/sweep/examples/r2_p2_consumer.rs:76`;
`crates/topo/tests/m6_3_chart_completion.rs:121`,
`review_ssiflat_r1_probes.rs:110`, `review_ssiflat_r2_probes.rs:100`);
a rebuild reached through a helper or a macro body; the same shape on a
validated type that is not one of these four; and any constructor other
than `new` (`Surface::Nurbs(Arc::new(..))` over a net assembled
elsewhere). The counts are as of `origin/main` at the SCALAR VREV
merge base and a lane that lands this owes a re-sweep.

## Was

Filed by SCALAR's `sweep-test-rebuilds-validated-net-for-v-reversal`
(the VREV door), whose spec §3 required this sweep.
