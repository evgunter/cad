---
id: main-red-the-lattice-discard-pin-moved-where-pred-5724-met-flush-5706
kind: issue
title: Main is red: the lattice corpus's bordered-discard pin moved where PRED's PR 5724 merged onto FLUSH's PR 5706; each passes alone
status: closed
opened: 2026-10-26
closed: 2026-11-10
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

Moved from `work/issues/` to `work/flush/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

## Closed

**Right: the count is the kernel's, and PR 5724 has no part in it.**
The pin keeps `(26_292, 2004)`; its comment now says what the stretches
are.

**Measured** (`flush/lattice-discard-pin`, a dump of every stretch per
body by its ends' output-stage points and settled cells, run with the
row's command):

- main `e653c96a`: `(26292, 2004)` at ε 1e-9, 1e-6 and 1e-12, and the
  three dumps are line-identical.
- `a8bb3a27` (before 5706) and `82f9f9f7` (5724's head): 25 512
  stretches, line-identical to each other.
- `4536e387` (5706's own head), `d2764de3` (5706 merged, without 5724)
  and `3adb0c3e` (5724 merged): 26 292, line-identical to each other
  and to main.

So 5706 alone moves the count. The bisect above read `d2764de3` as
passing. Rebuilt here (a `Compiling topo` line on each run), it fails
as `3adb0c3e` does. 5724's crate diff is all curved-carrier turn readers,
which no brick reaches.

**What moved.** No stretch went away; 780 came, all of them
`bordered_through` rows of operand B, in 279 bodies. Every one of those
bodies is an `OperandA` containment fallback: `a` whole, `b` absent.

| class | bodies | stretches | ends |
|---|---|---|---|
| `a ∪ b`, `b` inside `a` and flush with it | 33 | 96 | 96 EE |
| `a ∩ b`, `b` holds `a` and is flush on two caps | 6 | 48 | 48 VV |
| `a ∖ b`, `b` outside `a` resting on a face | 240 | 636 | 120 VV, 360 EV/VE, 156 EE |

**The reading**, by class:

- **∪**: `b`'s faces inside `a` are discards. Each borders the kept face
  of `a` that holds `b`'s coincident face, along its own edge, across
  that substituted fragment. Before 5706 the fallback recorded no
  discards here; now it agrees with `b ∪ a`, which takes the section
  path.
- **∩**: `b`'s caps are wider than `a`'s. Each is cut along `a`'s
  outline. The rest of the cap is a discard bordering `a`'s cap across
  the one-sided chords on `a`'s edges. This is `a ∩ big` in
  `boolean_covered::a_containment_fallback_cuts_a_dropped_face_held_in_part`.
- **∖**: `b`'s face on `a` is substituted by `a`'s face, whole or cut
  along `a`'s edges. The overhang borders `a`'s face across those
  chords. `b`'s side faces border it along their own bottom edges. This
  is the same test's `a ∖ lid`.

**Not a wrong discard:**

- Every face counted belongs to the absent operand.
- No moved body holds one stretch twice (the corpus's 10 476 duplicate
  pairs, a seam bordered from either side, are the same before and
  after).
- Each stretch with an end inside a kept face is left out, as the
  section path leaves it out (`ChordsAcross::substituted`).
- Each end inside an edge is a vertex the output stage's join absorbed
  into `a`'s whole edge; the naming reads it as bordering no edge.

On all 1000 poses of ∪ and of ∩, `op_fates` of `a ∘ b` equals that of
`b ∘ a` row for row (volume, substituted pairs, discards with their
borders), across 2000 pairs and 0 differences. That includes the 39 moved ∪/∩
bodies, whose other order takes the section path (`Seamed`).

**Why main went red.** 5706's gate did not select the slow-set row:
its diff names neither the test nor a path the filter maps to it.
Evidence added to
`work/ciw/a-diff-that-moves-corpus-ids-runs-none-of-the-slow-set-id-pins.md`.
