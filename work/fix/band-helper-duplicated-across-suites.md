---
id: band-helper-duplicated-across-suites
kind: issue
title: test suites across six crates carry a byte-identical fn band() wrapper; sweep is collapsed, the rest needs a shared home
status: open
opened: 2026-09-04
refs: [band-derivation-has-a-scalar-twin]
branch: fix/sweep-band-helper
pr: 2377
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

**Counts here were stale and are re-derived at `8851abb` (2026-09-11).**
The figures this section used to carry (24 sweep / 10 topo / 1
step-import / 1 geom-core) were wrong in both directions: they missed
three crates entirely and undercounted a fourth sevenfold. Re-derive
before acting on any of them.

The remaining run's-band copies, filtered on the body actually
resolving the RUN's band (`Band::linear(...)`, or the `Band::new(tol.eps(),
tol.k() * tol.eps())` spelling that is exactly equal to it):

| crate | `tests/` | `src/` `#[cfg(test)]` |
|---|---|---|
| `topo` | 9 | 13 |
| `geom-core` | 7 | 2 |
| `geom-brep` | 3 | 5 |
| `editor-core` | 1 | — |
| `mesh` | 1 | — |
| `step-import` | 1 | — |
| `sweep` | 0 | — |

**Two populations, not one.** The `tests/` column (22 sites) is what a
shared per-crate test helper could absorb. The `src/` column (20 sites)
is in-crate `#[cfg(test)] mod tests` blocks, which cannot reach a
`tests/` helper tree at all — a different home question, and one this
row has never scoped.

The topo nine are still the interesting case: `crates/topo/tests/all.rs`
declares `mod common;` but `crates/topo/tests/common/` has **no** band
helper, so collapsing them means *creating* a shared home — a sharing
decision, not a pointer change, which is why it is filed rather than
done. `crates/geom-brep/tests/shared/tol.rs` is the worked precedent
for exactly that move, and its own header says it exists to kill "the
three-line wrapper, once per suite, forty-eight times". That crate
already HAS the home and still carries three copies beside it, which is
its own small finding.

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

## The sweep half is collapsed (2026-09-11, `fix/sweep-band-helper`)

Every `crates/sweep` site now reaches `crates/sweep/tests/common/approx.rs`'s
`band()`; **39 wrappers removed and 43 `use` sites now reach that one
home, across 47 files.** What it took, beyond
the pointer change the dispatch predicted:

- **The premise held.** `autotests = false`, one `[[test]] name = "all"`
  target, and `every_suite_file_is_aggregated` proves every `tests/*.rs`
  is a module of that binary. No sweep suite is its own test binary.
- **Deleting a wrapper orphans its imports.** 30 files lost a `Band`
  and/or `Tol` import that only the wrapper used. Clippy's
  `-D warnings` is what catches these, and the **default feature lane
  cannot see them all**: three of those 30 surface only under
  `--features interval` — one suite is `#![cfg(feature = "interval")]`
  at file level and two keep their rows inside a
  `#[cfg(feature = "interval")] mod certified`.
- **Three helper-module copies, not just per-suite ones.**
  `tests/common/cone_nappe.rs` held a second copy inside the shared tree
  itself; `tests/shell8_common.rs` held a third that six suites imported.
  Both are gone and their seven consumers point at `common::approx`.
- **The dead-wrapper re-check found one**, not six:
  `tests/shellfix1_bitdump.rs` defined `band()` and never called it. It
  lost the wrapper with no import added.
- **`common/mod.rs`'s own marker rule was being broken by every copy.**
  That file requires a suite keeping its own copy of something the
  `common` tree holds to say why AT the copy, carrying the literal
  ``NOT `common::``. No band copy carried one. The one deliberate
  survivor, `tests/m9_2_chart_region_loft.rs` (a FIXED `1e-9`/`1e-8`
  band, not the run's), now does.

**Not collapsed, deliberately:** `tests/m9_2_chart_region_loft.rs` (a
different band — see above) and `examples/p1b_r2_m2.rs` (an example
binary, which is not a module of `tests/all.rs` and cannot name
`crate::common` at all).

**What remains is the table above** — the shared-home decision this row
shares with `band-derivation-has-a-scalar-twin`, unchanged and still
undispatched. The row stays `open`.
