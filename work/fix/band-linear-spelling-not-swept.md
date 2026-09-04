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
`from_zero_threshold(tol, tol.eps())` → `from_thresholds(tol.eps(),
tol.k())` → `Band::new(tol.eps(), tol.k() * tol.eps())`
(`crates/geom-core/src/predicate.rs`), the same two operands in the same
order, so every rewrite below is a spelling and not a threshold.

### Rewritten (26 sites)

Production doors, `Tol` witness in hand:

- `crates/profile/src/path.rs` `linear_band` — rewritten.
- `crates/profile/src/sugar.rs` `arc_fillet_trims` — rewritten.
- `crates/profile/src/validate.rs` `validate_with` — rewritten.
- `crates/profile/src/path/arc_fillet.rs` `resolve` — rewritten.
- `crates/sweep/src/test_support.rs` `all_links` — rewritten; the
  `let tol = tol.get();` shadow existed only to reach the two fields and
  is gone with them.

`fn band()` helpers feeding the sub-door functions that still take a
`Band` — spelling only, no signature touched:

- `crates/topo/tests/`: `census_g2_carrier.rs`, `r1_mate5_probe.rs`,
  `r2_probes.rs`, `mate5_cyl_eps_rung.rs` — rewritten.
- `crates/sweep/tests/` via a `tol() -> Tol` helper:
  `review_arms2_r1_probes.rs`, `review_pr12_probes.rs`,
  `verbs_arms1_annulus.rs`, `m5_pr12_refusals.rs`,
  `verbs_rim_r1_probes.rs`, `verbs_arms3.rs`,
  `review_verbs_rim_lever_probes.rs`, `verbs_arms2_bud.rs`,
  `verbs_rim_closed_lever.rs` — rewritten.
- `crates/sweep/tests/` via `Tol::witness().get()`: `m5_pr12_die.rs`,
  `m6_5_fillet_naming.rs`, `review_d2_adv_probes.rs`, `m6_surgery.rs`,
  `review_m6_surgery_probes.rs`, `m5_pr12_battery.rs`, `bitdump.rs` —
  rewritten.
- `crates/profile/tests/rejections.rs` — rewritten. This is the site
  that binds ε to a local first (`Band::new(eps, tol().k() * eps)` with
  `eps = tol().eps()`), the shape the item predicted would evade a
  pattern keyed on `.k * .eps`.

### Left inline, with the reason

- `crates/geom-core/tests/band_tolerance.rs` (three assertions) — **load
  bearing.** This is the row that pins `Band::linear` to (ε, K·ε); the
  inline `tolerance.k * tolerance.eps` is the independently spelled
  expected value. Routing it through `Band::linear` would make it assert
  `x == x` and delete the only check that the derivation is what this
  item calls canonical.
- Bands that are deliberately **not** the run's — each says so at its own
  site, and none is the canonical derivation, so rewriting any of them
  would be a numeric change, not a spelling one:
  `crates/sweep/tests/common/approx.rs` (`Band::new(eps, eps * 10.0)`, a
  literal 10 over a parameter ε);
  `crates/geom-brep/tests/tcost_k1_budget_exit.rs` (`DEFAULT_K * eps` —
  the compiled default K, not the run's, at an explicit ε);
  `crates/geom-brep/tests/pcurve_p1a_meter.rs` (`ROW_EPS`);
  `crates/geom-brep/tests/pcurve_p1b_r2_probes.rs` (a fixed 1e-6 zero
  with the run's K); `crates/geom-brep/tests/r2_probes.rs` and
  `crates/geom-brep/src/certify.rs` (4·DRIFT .. 40·DRIFT);
  `crates/geom-brep/src/ssi/certify.rs`, `crates/geom-brep/src/ssi/march.rs`
  and `crates/sweep/tests/sf2b_r1_probes.rs` (ladders over explicit
  zeros); `crates/geom-core/src/interval.rs` and
  `crates/geom-core/src/predicate.rs` (property rows over the scaling
  policy itself); and ~100 fixed-literal bands (`1e-9, 1e-8` and kin),
  which are pinned test scales, not derivations.
- The **scalar twin** — a class, not an instance, and left whole: about
  fifteen sites derive the same two thresholds as bare `f64`s and never
  build a `Band` (`crates/profile/tests/bool11_probes.rs`,
  `bool12_probes.rs`, `bool12r2_probes.rs`, `bool12_r1_probes.rs`,
  `r1_bool11_review_probes.rs`, `r2_bool11_review_probes.rs`;
  `crates/topo/tests/m5_pr8_bvh_diff.rs`, `seat3_flush_detector.rs`;
  `crates/editor-core/tests/lib_sel2_flush.rs`;
  `crates/step-import/tests/review_r1_tier_gate_probes.rs`;
  `crates/geom-brep/tests/offa_r1_probes.rs`). They carry the same drift
  risk this item names, but `Band::linear` returns a `Band`, so the fix
  for them is a different door, not this spelling — filed as its own
  finding rather than absorbed here.

### Blind spot

The item's two patterns (`Band::new\(.*eps`, `\.k \* .*\.eps`) are keyed
on the *identifier* `eps`, so neither can see a derivation whose
thresholds are bound to differently named locals — and the second cannot
see the method spelling `.k() * eps` at all, which is why
`crates/profile/tests/rejections.rs` shows up under only one of them. The
sweep therefore did not rely on either: every one of the 130 `Band::new`
call sites in the tree was enumerated and classified by its argument
expressions, which is what makes the "left inline" list above a census
rather than a sample. What that enumeration still cannot see: a band
built somewhere other than a literal `Band::new(` call — through a
helper taking (zero, escalate), a `Band` deserialized or cloned from
another, or a derivation assembled across statements far enough apart
that reading the enclosing function was the only way to catch it. The
scalar-twin family above is the part of that residue found by hand; a
band constructed via indirection would not have been.
