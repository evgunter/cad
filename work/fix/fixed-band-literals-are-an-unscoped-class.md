---
id: fixed-band-literals-are-an-unscoped-class
kind: issue
title: 55 sites decide against a hard-coded Band::new(1e-9, 1e-8) instead of the run's band, and nobody has established which are deliberate
status: open
opened: 2026-09-11
refs: [band-derivation-has-a-scalar-twin, band-helper-duplicated-across-suites]
---



(FIX orchestrator) Disclosed by the `band-helper-duplicated-across-suites`
sweep half (PR 2377) and filed here at the moment of disclosure, per
`work/README.md` — the lane reported it and did not file, which is
what the implementer discipline asks of a lane outside its fence.

## The class

`Band::new(1e-9, 1e-8)` — a band with the run's ε and K written out as
digits — appears at **55 sites** across `crates/` (measured on
`4db3b3f`, a literal-string grep; a site spelling the same band with
different digits or through a const is invisible to it).

A band is the thing a decision is taken against. A fixed one asks a
different question from `Band::linear(Tol::witness())` at every ε row
but the default: the eps = 1e-6 and eps = 1e-12 lanes move the run's
band and leave a hard-coded one where it was. That is sometimes
exactly right — `crates/sweep/tests/m9_2_chart_region_loft.rs` keeps
one deliberately, because its digits are the chart-region extension's
own acceptance figures rather than the run's tolerance, and PR 2377
made it say so at the site under `common/mod.rs`'s marker rule.

**What is not established is how many of the other 54 are that, and
how many are inheritance.** No site but that one carries a reason.

## One framing to NOT carry forward — it was checked and it is false

The disclosing lane suggested these collide with the suites whose
header declares *"ε posture: no ε literal"*, a hard-coded band being
an ε literal. **Measured: the two populations are disjoint.** No file
carrying that posture header also carries a fixed `Band::new(1e-…)`.
The finding survives without it, but weaker than first stated: it is
an unexamined class, not a live contradiction. Recorded because the
stronger version reads as the sharper finding and would have sent a
taker looking for a conflict that is not there.

## What a taker owes

Per site, the same question and one of two outcomes: the digits are
this row's own acceptance figures, in which case say so AT the site
(the `m9_2_chart_region_loft.rs` precedent, and `common/mod.rs`'s
marker rule where a `common` tree holds the alternative); or they are
the run's band written out, in which case they are the
`band-linear-spelling-not-swept` class and collapse onto
`Band::linear`.

**This is a reading pass with a long tail, not a mechanical sweep** —
the decision is per site and a wrong collapse changes what a row
decides against, which `docs/prompts/implementer-discipline.md` §3 is
explicit is never free.

## Fence

Every site is under `*/tests/*` or a `#[cfg(test)] mod tests` block in
`crates/*/src`, so the `*/tests/*` family (S-TCOST's and S-TINT's by
design) and each crate's owning program both. Cut by fence.

## Refs

Sibling of `band-derivation-has-a-scalar-twin` and
`band-helper-duplicated-across-suites`; both are about the run's band
being restated, this one is about it being replaced.
