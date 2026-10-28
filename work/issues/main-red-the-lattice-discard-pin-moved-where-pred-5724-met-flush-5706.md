---
id: main-red-the-lattice-discard-pin-moved-where-pred-5724-met-flush-5706
kind: issue
title: Main is red: the lattice corpus's bordered-discard pin moved where PRED's PR 5724 merged onto FLUSH's PR 5706; each passes alone
status: open
opened: 2026-10-26
priority: P0
cost: M
refs: [5724, 5706]
---


Filed by the SPLICE orchestrator (2026-10-26). The SPLICE touch-doors
lane found it red running the topo slow set; the orchestrator bisected
it.

## What

`topo` `fused_into_live_cells::every_fused_vertex_of_the_lattice_corpus_settles_on_a_live_cell`
(slow set) is red on main from the merge of PR 5724 (`3adb0c3e`):

    bordered stretches, and those with an end inside an edge or a face
      left: (26292, 2004)
     right: (25512, 1392)

Its other two pins (2721 bodies; (12 732, 336, 432) settled fusions)
hold.

## Bisect (each run: `cargo nextest run -p topo --test all --run-ignored all -E 'test(every_fused_vertex_of_the_lattice_corpus_settles_on_a_live_cell)'`)

- `a8bb3a27` (main after PR 5719 and 5729): PASS.
- `d2764de3` (main after FLUSH PR 5706, the containment fallback's
  discards): PASS.
- `82f9f9f7` (PRED PR 5724's own head, which carries 5719 but **not**
  5706): PASS.
- `3adb0c3e` (5724 merged onto `d2764de3`): **FAIL**, as above.

So neither PR alone moves it. The two meet at the merge: 5706 rewrote
how the boolean records discards (`boolean/fate.rs`, `discard.rs`,
`ops.rs`), and 5724 changed the turn rule's readers
(`splitting/containment.rs`, `spiric_arc.rs`, `chord_join.rs`,
`chart_region.rs`, `offset_axial.rs`). Both PRs' gates were green
because the row is in the slow set, and each branch lacked the other's
change. That is the class `ciw/a-diff-that-moves-corpus-ids-runs-none-of-the-slow-set-id-pins`
names.

## Owed

Decide whether the new counts are the kernel being right (re-baseline,
saying what moved) or a wrong discard. The owner is split between
FLUSH (the discards the pin counts) and PRED (the change that moved
them); the first of the two to pick this up moves the file onto its
slate. Brick × brick only, so no spline or spiric edge is involved.

## 2026-10-28 — pinned on main (GREEN, `green/full-suite-pins`)

Nightly run 40637814390 (main `a1cfd1cfdb`), `full suite`; re-run on main `91c83c38`. Still `(26292, 2004)` at every ε. The pin now reads `(26_292, 2004)`
with a comment naming this row; closing it decides whether that count
is right and rewrites the comment either way.
