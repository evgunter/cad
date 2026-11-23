---
id: the-tours-k-probe-reads-lily-wall-8-at-the-probe-scalar-where-it-refuses-nomarch
kind: issue
title: The tour's k-probe reads the lily's wall 8 at the Probe scalar, where it refuses NoMarch and not the cell budget the wall pins, so two k_probe_brackets rows are red on main
status: closed
opened: 2026-11-15
priority: P1
cost: E
parent: nightly-1115-residue-the-reds-no-lane-holds
closed: 2026-11-15
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

## Closed (GREEN, `green/nightly-1115`, PR 6985, 2026-11-15): pinned per scalar

Wall 8's probe now states what the wall refuses with at each scalar it
is read at: `Unaccounted { cause ∋ "cell budget" }` where the scalar is
`f64`, `NoMarch` naming the recording scalar where it is
`geom_core::Probe`, and nothing else at either
(`demos/tour/src/lily.rs`, wall 8's closure, keyed on
`<S as Real>::NAME`). With it `demo-tour k-probe` writes its dump at
ε 1e-6, 1e-9 and 1e-12 (run locally, dev profile, `--features probe`:
2621210, 2604832 and 2094233 lines; at 1e-12 the lily stops at its own
pinned frontier before the walls, as before).

The Probe arm is a pin, not the wanted reading. What is still open, and
the unbisected candidate merge (PR 6862), moved to the owner:
`work/sectarms/the-torus-spline-germ-section-has-no-march-lane-at-the-recording-scalar.md`.

## After the close (2026-11-17)

The wall-8 pin held on the 11-16 and 11-17 nightlies, and the two rows then stopped one assertion later: `bracketed bodies at eps 1e-9` / `1e-6`, measured `{}` against `{"lily_leaf_a"}`. That expectation had been stale since PR 6092 (QUAD, 10-30), hidden behind the wall-8 stop. Re-blessed in GREEN's 11-17 PR; QUAD's log has the bisect.
