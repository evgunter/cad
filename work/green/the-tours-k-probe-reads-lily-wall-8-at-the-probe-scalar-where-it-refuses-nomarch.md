---
id: the-tours-k-probe-reads-lily-wall-8-at-the-probe-scalar-where-it-refuses-nomarch
kind: issue
title: The tour's k-probe reads the lily's wall 8 at the Probe scalar, where it refuses NoMarch and not the cell budget the wall pins, so two k_probe_brackets rows are red on main
status: open
opened: 2026-11-15
priority: P1
cost: E
---


Filed by the BOXFINISH orchestrator, from a measurement its chart-ends
lane made while attributing the reds of nightly run 43680833019. Not
BOXFINISH's: the rows are red before the first of the merges that
nightly suspected.

**What is red.** `demos/tour` `tests/k_probe_brackets.rs` (behind
`--features probe`):
`k_probe_brackets::k_probe_brackets_the_lofted_leaf_at_eps_1e_9` and
`..._at_eps_1e_6`. `..._and_the_sepals_at_eps_1e_12` is green. Both
run the `demo-tour k-probe` binary at the telemetry Probe scalar; the
child exits 101 from `demos/tour/src/walls.rs:33`.

**Measured** (the same on both trees, ε 1e-9):
- on main `dbb21b4a` and on `bdb498df`, the first parent of PR 6959's
  merge, `lily::wall_probes`' wall 8 (`try_union(stem, leaf_a)`)
  refuses `GermSectionUnbuilt { a_kind: Torus, b_kind: Nurbs, why:
  Refused(NoMarch { scalar: "telemetry probe" }) }`;
- the wall's pin (`demos/tour/src/lily.rs`, wall 8's closure) wants
  `why: Refused(SectionError::Unaccounted { cause ∋ "cell budget" })`.

**Inferred, not traced.** CLEAVETAIL re-pinned wall 8 on 2026-11-12
(`work/cleavetail/two-tour-wall-probes-moved-on-main.md`, after PR
6862) to the refusal the `f64` tour reads at 1e-9 and 1e-6. At the
Probe scalar the torus × spline germ section refuses for want of a
march lane before it reaches the accounting that refusal comes from,
so one pin cannot hold both scalars. Which merge first made the rows
red is not bisected; PR 6862 is the candidate.

**Why it matters beyond the two rows.** The nightly's `k-lint
(dev-probe)` job panics at the same wall in its ε 1e-6 demo sweep, so
the large-K lint and the driver K-telemetry lint after it do not run
("the probe sweep did not finish its dumps").

**Closed when** wall 8's probe states what the wall refuses with at
each scalar it is read at, and the two rows and the dev-probe sweep
run through.
