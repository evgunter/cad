---
id: band-linear-spelling-not-swept
kind: issue
title: Band::linear is the canonical spelling of the linear band and ~20 sites still open-code Band::new(eps, k·eps) — sweep or record why not
status: review
opened: 2026-08-31
github: 1408
refs: [1399]
branch: fix/band-linear-sweep
pr: 1732
---

## From GitHub issue 1408

Opened 2026-08-31; 0 comments.

(SEAT orchestrator) Class finding from SEAT-1's dual review (PR #1399), filed per the findings-need-a-durable-home rule. Both reviewers converged on it independently.

SEAT-1 made `Band::linear(tol)` the one spelling of the linear decision band at every kernel verb door (the doors now derive it themselves). But the unit swept the *parameter*, not the *spelling*: after it, ~20–26 sites tree-wide still open-code the identical derivation as `Band::new(x.eps, x.k * x.eps)` — including production code, not just test helpers:

- `crates/profile/src/path.rs:1470`
- `crates/profile/src/sugar.rs:481`
- `crates/profile/src/validate.rs:1012`
- `crates/profile/src/path/arc_fillet.rs:480`
- `crates/sweep/src/test_support.rs:91`
- plus the surviving `fn band()` test helpers feeding the six sub-door functions that legitimately still take a `Band` (the offset machinery beneath `shell`, `point_in_solid`, `mate::solve::fold_pair`).

Two spellings of one rule is the Q1 drift shape: a change to the canonical derivation (or to `Tolerance`'s fields) would have to find every inlined twin by grep. A follow-up unit should either rewrite the inlined sites to `Band::linear` (pure spelling, no numeric change — same argument as SEAT-1's) or record per-site why the inline form is load-bearing.

Line numbers are as of `0b291b2` and will drift; the greps `Band::new\(.*eps` / `\.k \* .*\.eps` re-find the population.

## Home

`work/seat/` — SEAT's charter §1 is band derivation at operation entry, and this is SEAT-1's own residue: the canonical `Band::linear` spelling it established, unswept at the remaining sites.

## Closed

`Band::linear(tol)` is now the one spelling of the linear decision band
wherever a band is built from the run's tolerance. The derivation
identity was read, not assumed: `Band::linear(tol)` is
`from_zero_threshold(tol, tol.eps())` -> `from_thresholds(tol.eps(),
tol.k())` -> `Band::new(tol.eps(), tol.k() * tol.eps())`
(`crates/geom-core/src/predicate.rs`), the same two operands in the same
order. The twin property is structural rather than per-site: `Tol` is
`pub struct Tol(())` and its `eps()`/`k()` read the process global, so a
`Tol` cannot carry a non-global tolerance.

### The census, and how to re-derive it

At merge base `7514cc6`, `Band::new(` occurs **149** times in `*.rs`; 6
of those are inside comments; **143 are call sites**. All 143 were
classified by their argument expressions — that is what makes the
"left inline" list below a census rather than a sample. 26 were
rewritten, and HEAD carries **117** (123 occurrences - 6 prose);
143 - 26 = 117 exactly.

```
git grep -o 'Band::new(' <ref> -- '*.rs' | wc -l          # occurrences
git grep -n 'Band::new(' <ref> -- '*.rs' \
  | grep -E ':\s*(//|///|//!)' | wc -l                    # prose
```

An earlier revision of this item said "130", which was neither figure:
it counted occurrences outside `crates/geom-core/src/predicate.rs` (19
there — the constructor's own definition, its doc examples and its
validation rows) and still included the 6 prose mentions. The corrected
number is above.

### Rewritten (26 sites)

Production doors, `Tol` witness in hand:

- `crates/profile/src/path.rs` `linear_band` — rewritten.
- `crates/profile/src/sugar.rs` `arc_fillet_trims` — rewritten.
- `crates/profile/src/validate.rs` `validate_with` — rewritten.
- `crates/profile/src/path/arc_fillet.rs` `resolve` — now calls the
  crate's existing `linear_band` door rather than re-spelling
  `Band::linear(tol).map_err(PathError::Band)`, which that door is.
- `crates/sweep/src/test_support.rs` `all_links` — rewritten; the
  `let tol = tol.get();` shadow existed only to reach the two fields.

`fn band()` helpers feeding the sub-doors that still take a `Band` — no
signature changed:

- `crates/topo/tests/`: `census_g2_carrier.rs`, `r1_mate5_probe.rs`,
  `r2_probes.rs`, `mate5_cyl_eps_rung.rs`.
- `crates/sweep/tests/` via a `tol() -> Tol` helper (9):
  `review_arms2_r1_probes.rs`, `review_pr12_probes.rs`,
  `verbs_arms1_annulus.rs`, `m5_pr12_refusals.rs`,
  `verbs_rim_r1_probes.rs`, `verbs_arms3.rs`,
  `review_verbs_rim_lever_probes.rs`, `verbs_arms2_bud.rs`,
  `verbs_rim_closed_lever.rs`.
- `crates/sweep/tests/` via `Tol::witness().get()` (7): `m5_pr12_die.rs`,
  `m6_5_fillet_naming.rs`, `review_d2_adv_probes.rs`, `m6_surgery.rs`,
  `review_m6_surgery_probes.rs`, `m5_pr12_battery.rs`, `bitdump.rs`.
- `crates/profile/tests/rejections.rs` — the site binding e to a local
  first (`Band::new(eps, tol().k() * eps)`), the shape a pattern keyed
  on `.k * .eps` cannot see.

All 16 sweep suites above then lost the wrapper entirely: they are
modules of one aggregate binary whose `common/approx.rs` already
exports `pub fn band()`, so they now import it. Six of them defined
`band()` and never called it (`bitdump.rs`, `m5_pr12_die.rs`,
`m6_5_fillet_naming.rs`, `review_arms2_r1_probes.rs`,
`review_d2_adv_probes.rs`, `review_m6_surgery_probes.rs`) — dead code
the aggregate's `dead_code` allowance hid. The remaining copies are
filed as `band-helper-duplicated-across-suites`.

### Repaired, not merely classified

`crates/geom-core/tests/band_tolerance.rs` is the row that pins
`Band::linear` to (e, K*e), and this unit named it as the one
load-bearing inline form. It was also **broken**, in the same `#[test]`:

1. It asserted `20*eps` definite and `3*eps` in-band — true only for
   3 < K < 20, though `CAD_AMBIGUITY_K` accepts any K > 1. Every
   multiplier is now derived from `tolerance.k`.
2. The overflow-residue row built its arm from `f64::MAX / 2.0`, so
   `escalate = K*(MAX/2)` only overflows for K > 2. The arm is now
   derived from `MAX*(1+K)/(2K)` — a fraction in (1/2, 1] that is under
   MAX and over MAX/K for every K > 1.
3. The module header claimed every assertion was written relative to the
   run's e "not to a fixed value" — true of e, false of K, which is the
   sentence that made the literal multipliers invisible.

Verified over the CI e matrix x K in {1.5, 2, 3.5, 10, 30, 100}: 18/18
pass, where K = 1.5 and K = 2 failed before. The residue is stated at
the site: the arm must be subnormal to land near MAX, so at the tightest
e the construction runs out of significand for K within ~1e-4 of 1.

### Left inline, with the reason

- `crates/geom-core/tests/band_tolerance.rs` (three assertions) — the
  expected values that pin the derivation. Routing them through
  `Band::linear` would make the row assert `x == x`.
- Bands that are deliberately not the run's, and are therefore different
  bands rather than twins: `crates/sweep/tests/common/approx.rs`;
  `crates/geom-brep/tests/tcost_k1_budget_exit.rs`;
  `crates/geom-brep/tests/pcurve_p1a_meter.rs`;
  `crates/geom-brep/tests/pcurve_p1b_r2_probes.rs`;
  `crates/geom-brep/tests/r2_probes.rs` and
  `crates/geom-brep/src/certify.rs`; `crates/geom-brep/src/ssi/certify.rs`,
  `crates/geom-brep/src/ssi/march.rs`,
  `crates/sweep/tests/sf2b_r1_probes.rs`;
  `crates/geom-core/src/interval.rs` and `src/predicate.rs`; and ~100
  fixed-literal bands.

  `tcost_k1_budget_exit.rs` is **structurally forced**, not merely
  chosen: `Band::linear` takes only a `Tol` and derives e from the run,
  and `from_zero_threshold` is private, so no door states a band at an
  explicit e. Its doc previously read "a band shaped like `Band::linear`
  at an explicit e", which is false on the K edge; it now says so.

  Two of these sites carry **no sentence at all** saying why they are
  not the run's band (`sweep/tests/common/approx.rs:428`, and
  `tcost_k1_budget_exit.rs` before this change) — so no blanket "each
  says so at its own site" claim is made here.

- The scalar twin and the literal-K spelling are their own items:
  `band-derivation-has-a-scalar-twin`,
  `literal-k-where-the-runs-k-belongs`.

### Blind spot

The item's two patterns are keyed on the identifier `eps`, so neither
sees thresholds bound to differently named locals, and `\.k \* .*\.eps`
cannot see the method spelling `.k() * eps` at all. The sweep therefore
relied on the full enumeration of all 143 call sites instead. A third
pattern (`\.k() \* .*eps`) is what surfaced the scalar-twin class.

What the enumeration still cannot see: a band built anywhere other than
a literal `Band::new(` call — through a helper taking (zero, escalate),
a `Band` cloned or deserialized from another, or a derivation assembled
across statements far enough apart that reading the enclosing function
was the only way to catch it. The scalar-twin family is the part of that
residue found by hand; a band constructed via indirection would not
have been.
