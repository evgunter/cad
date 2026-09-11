---
id: band-helper-duplicated-across-suites
kind: issue
title: 36 test suites carry a byte-identical fn band() wrapper; the free half is collapsed, the rest needs a shared home
status: open
opened: 2026-09-04
refs: [band-derivation-has-a-scalar-twin]
---


## The finding

`fn band() -> Band { Band::linear(Tol::witness()).unwrap() }` — a
three-line wrapper with no parameter to vary — is written out once per
test suite. By the same argument `band-linear-spelling-not-swept`
makes about the derivation, this is that defect one level up: a change
to how a suite obtains the run's band has to find every copy by grep.

Counts, `grep -A2 '^fn band() -> Band {'` filtered to bodies calling
`Band::linear`:

- merge base `7514cc6`: **32**
- after that unit's spelling sweep: **52** — the sweep did not create
  the duplication, but it converted 20 inline `Band::new(ε, K·ε)`
  wrappers into textually identical ones, which is what made the class
  visible
- after collapsing the free half (this branch): **36**

## What was collapsed, and why only that

All 16 `crates/sweep/tests/` suites the unit rewrote are modules of
`crates/sweep/tests/all.rs` (`autotests = false`, one aggregate binary,
`mod common;` declared once), and `crates/sweep/tests/common/approx.rs:61`
**already is** `pub fn band() -> Band { Band::linear(Tol::witness()).unwrap() }`.
Those 16 now say `use crate::common::approx::band;` — no new module, no
new sharing decision, just pointing at the home that existed.

Six of the 16 turned out to define `band()` and **never call it**
(`bitdump.rs`, `m5_pr12_die.rs`, `m6_5_fillet_naming.rs`,
`review_arms2_r1_probes.rs`, `review_d2_adv_probes.rs`,
`review_m6_surgery_probes.rs`). The aggregate allows `dead_code`, so
nothing warned. Those lost the wrapper with no import added.

## What is left, and what it needs

36 copies remain: **24 in `crates/sweep`** (suites the unit did not
touch — same binary as `common::approx::band`, so equally free, and
mechanical), **10 in `crates/topo`**, **1 each in `crates/step-import`
and `crates/geom-core`**.

The topo ten are the interesting case: `crates/topo/tests/all.rs`
declares `mod common;` but `crates/topo/tests/common/` has **no** band
helper, so collapsing them means *creating* a shared home — a sharing
decision, not a pointer change, which is why it is filed rather than
done. `crates/geom-brep/tests/shared/tol.rs` is the worked precedent
for exactly that move, and its own header says it exists to kill "the
three-line wrapper, once per suite, forty-eight times".

## Not this class

A `band()` whose body is not the run's band — a fixed `1e-9 .. 1e-8`,
`ROW_EPS`, `4·DRIFT` — is a different band per suite and must not be
pointed at any shared home. `geom-brep/tests/shared/tol.rs`'s header
carries that census for its own crate.

## The free half is dispatchable NOW; the rest shares one decision with its sibling (FIX orchestrator, 2026-09-11)

These two band rows —  this one and
`band-derivation-has-a-scalar-twin` — are one subject seen from two
levels, and they overlap in population (`crates/profile/tests/bool*_probes.rs`
carries both a `fn band()` wrapper and the bare-`f64` derivation).
They are now cross-referenced. But they do **not** all wait on the
same thing, and cutting them by "band row" rather than by *what each
site needs decided* would hold a mechanical change behind a design
question for no reason. The cut that matters:

**Free — no decision, dispatch on its own.** The **24 remaining
`crates/sweep` copies**. They are modules of the same aggregate binary
as `crates/sweep/tests/common/approx.rs:61`, which already *is*
`pub fn band() -> Band { Band::linear(Tol::witness()).unwrap() }`.
The change is `use crate::common::approx::band;` and deleting the
wrapper — the identical move this row already made for 16 sibling
suites, with the same `mod common;` already declared. No new module,
no sharing decision, no door question. Also re-run the dead-wrapper
check that found six unused copies last time: `autotests = false` with
`dead_code` allowed means nothing warns, so a copy that is never
called just disappears with no import added.

**Waits on one decision, shared with the sibling row.** The
**10 `crates/topo`** copies plus the singletons in `step-import` and
`geom-core` need a shared home *created* (`crates/topo/tests/common/`
has no band helper; `crates/geom-brep/tests/shared/tol.rs` is the
worked precedent, and its own header says it exists to kill exactly
this). That is the same question the sibling row asks one level down:
**does the `(zero, escalate)` pair get a named door, and on what — a
`Tolerance` method, or a per-crate test helper?** Answer it once, for
both rows, and both mechanical sweeps follow.

**Fence, and it is the documented awkward one.** Every site in both
rows is under `*/tests/*`, which is S-TCOST's and S-TINT's territory
by design — the `*/tests/*` family `work/README.md` names as the bulk
of the unrecorded double-claim pairs and the reason that lint stays a
warning. Name it in the PR body. TCOST's open PR #2351 touches only
`scripts/` (checked 2026-09-11) — re-check before pushing.
