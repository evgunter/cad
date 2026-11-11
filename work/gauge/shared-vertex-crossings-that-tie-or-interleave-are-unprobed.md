---
id: shared-vertex-crossings-that-tie-or-interleave-are-unprobed
kind: issue
title: "SharedVertexCrossings: a pair paired along its Out arcs, with other pieces on both ways round, has no run; which pairing a shared vertex takes is open"
status: open
opened: 2026-10-03
priority: P1
cost: M
design: true
pr: 6828
---


## What

`insert::reconcile_shared` (`crates/topo/src/boolean/insert.rs`)
refuses `BooleanError::SharedVertexCrossings` when a pair at a vertex
several crossing pairs cut has no run clear of the other pairs' cuts.

Built on `fuse/shared-vertex-tie`:

- **Cuts that tie in one corner** (a pinch line lying flat in a face of
  the shared corner): placed by the runs (`insert::tied_held`), the
  side each run takes being read off its piece's faces. Witnesses:
  `a_pinch_line_flat_in_the_shared_corners_face_builds_in_every_op`
  and `three_pieces_two_of_whose_cuts_tie_build_in_every_op`.
- **Two dangling null edges tied in one corner**: they splice by their
  lower germs, so `strut_anchor`'s tie is a `ClassificationInvariant`.
  Witness: `two_dangling_null_edges_meeting_on_the_pinch_line_build_in_every_op`.

Built on `fuse/strut-holding-a-cut`:

- **A dangling null edge whose segment holds another's whole**: the
  inner hangs at the outer's tip (`insert::holds_whole`), strictly
  inside at one end at least. Witnesses:
  `a_dangling_null_edge_holding_another_pairs_cut_builds_in_every_op`,
  `a_dangling_null_edge_holding_two_pairs_cuts_builds_in_every_op`,
  `a_dangling_null_edge_inside_another_along_one_end_builds_in_every_op`.

Still refusing:

- **Two pairs whose runs are one arc**: reachable as two dangling
  null edges with one segment, and refusing typed:
  `two-dangling-null-edges-with-one-segment-refuse-shared-vertex-crossings` (closed, PR 3953).
  For two fans, the other way round is the complement on the opposite
  side, so one turns: no refusal.
- **A fan both of whose ways round hold a cut (interleave)**: disjoint
  pieces do alternate round a convex corner, but a run that is its own
  piece's In-arc is clear, so this needs a pair paired across its
  Out-arcs (four crossings, the In-arc wrapping the A orbit's first
  sector). That shape refuses `PairingMismatch` first:
  `work/cleave/a-corner-crossing-another-four-times-refuses-pairing-mismatch.md`.
  Unreached.

## Owed

Re-probe the interleave arm once the `PairingMismatch` row is fixed.

## Landed (FUSE, PR 3943, 2026-10-03)

The tie arms are built, and the strut tie is an invariant. Review tier:
single FULL, with one fix pass that added three things:
- strut cuts take their side from angular ends;
- `reconcile_shared` runs to a bounded fixed point;
- a row that turns red under the old leaving-germ splice.

The row stays open at P0 for the reachable arm. Its pinch-end 3′
failure is now its own row,
`a-carried-row-whose-ends-split-into-null-edge-copies-is-dropped`.

## Landed (FUSE, PR 3950, 2026-10-03)

A dangling null edge holding another pair's cut whole now nests the
inner one at its tip. The pit-and-spike witness and two spikes build in
every op.
- **Review:** single FULL. One MAJOR: the one-arc tie built in one
  insertion order only. In the other order it hit `JoinDesync`, where
  main refused typed. The fix pass returns the one-arc tie to a typed
  refusal, in its own row
  (`two-dangling-null-edges-with-one-segment-refuse-shared-vertex-crossings`),
  and builds its witnesses both ways round.
- **Reprioritized:** what remains here is the interleave arm, which no
  witness reaches until cleave's `PairingMismatch` row lands. No
  reachable refusal is left on this row, so it drops from P0 to P3.


## Reached (JOIN, branch `join/reflex-corner-vertex-vertex`)

With cleave's `PairingMismatch` row fixed on that branch, the
interleave arm has a witness: cleave's
`three_corners_alternating_round_the_cube_refuse_three_ops`.
`y ∩ cube`, `cube ∪ y` and `cube ∖ y` refuse `SharedVertexCrossings` at
the cube's corner. The band's pair crosses that corner four times, and
the pairing those ops take has no run clear of the other two corners'
cuts either way round (`work/cleavetail/three-corners-alternating-round-a-corner-refuse-at-the-join.md`).

## Measured (FLUSH touch-key lane, 2026-10-12): the reconcile arm is reached

`join_pierce_runs_sweep::four_pairs_battery` (release, main `360cf55e`
and branch `flush/touch-key` alike, byte-identical) holds 982 lines
`ERR SharedVertexCrossings`. A site tag on each emitter (a lane probe,
not committed) put all 982 at `insert::reconcile_shared`'s pass, the
arm that finds no way round clear of another pair's cut
(`reconcile_pass`, the `chosen.ok_or_else` refusal). None reached the
cover arm in `hang_at_shared`, whose doc calls it "a backstop no known
pose reaches". So this row's "Unreached" no longer holds for the
reconcile arm: four cubes touching at `notch343`'s corner reach it.
Which of this row's two shapes (one arc, or interleave) each line is
was not read. Replay: `cargo test -p sweep --release --test all
four_pairs_battery -- --ignored --nocapture`.

## Dispatch

2026-11-11, GAUGE orchestrator. One implementer, lane `gauge-svc`, branch `gauge/shared-vertex-crossings`. The lane will:
- re-measure the `four_pairs_battery`'s `SharedVertexCrossings` lines on main;
- classify each by shape: one arc, interleave, or neither;
- read a representative of each class in the model;
- decide by meaning whether each refusal is honest (pin it typed) or a pose the reconcile should build (fix at the source);
- stop on a design question.

Review tier: a single full review if kernel code changes; the orchestrator's read if the outcome is pins only.

## Read (GAUGE, lane `gauge-svc`, 2026-11-11): a design question

Measured on main `0f42d98351` (with `gauge/orchestrator` merged, no
kernel change). The design question is in the last subsection.

**Count.** `four_pairs_battery` holds 982 lines
`ERR SharedVertexCrossings`, the same as on 2026-10-12. The other
5 189 − 982 lines are `OK SOUND`, and 143 poses skip. A local probe
tagged each emitter (not committed). All 982 come from
`insert::reconcile_pass`'s `chosen.ok_or_else` refusal, and none from
`hang_at_shared`'s cover arm. By op: `ab U` 246, `ab S` 246, `ba U`
236, `ba S` 236, `ba I` 18. The shared vertex is always the notch's
corner, in A for `ab` and in B for `ba`. Its orbit has four entries
over three physical sectors.

**Classes.** The probe printed every crossing pair's cuts round the
shared vertex in walk order, with each run's kind (fan or strut) and
code.

| class | lines | pairs at the vertex | example |
|---|---|---|---|
| interleave, laminar | 966 | 3 (251) or 4 (715); the refusing pair crosses 4 times (926) or 6 (40) | `four i=0 j=2 t=0 k=0 ab U` |
| crossing | 16 | 3 (8) or 4 (8); the refusing pair crosses 4 times | `four i=2 j=1 t=1 k=2 ab U` |
| one arc | 0 | | |

- **Interleave, laminar.** The refusing pair crosses the vertex four
  or six times. In 964 lines it is paired along its Out arcs, the
  stretches of the shared solid's boundary outside its partner piece
  (codes O). In the other 2 it is a B-nested In run whose own inner
  Out run holds the piece. Each other pair's two cuts lie together on
  one side of the refusing fan's chord, and both sides hold one. The
  runs form a laminar family across pairs, and no chord crosses
  another.
- **Crossing.** An Out strut of a four-crossing pair holds one end of
  each of two runs of another Out-paired pair (four or six
  crossings), and the other way round is not a run. The two pairs'
  Out runs interlock, so they form no laminar family.
- **One arc** (two struts with one segment) is not reached here. It
  stays typed under its own closed row.

Cleave's `three_corners_alternating_round_the_cube_refuse_three_ops`
is the laminar class. Three pairs meet at the cube's corner. The
band's pair crosses it four times, is paired along both Out arcs
(`FO, FO`), and holds the second corner's In fan on one side and the
third's In strut on the other.

**Reading.** On the sphere round the point, the other operand's
pieces at the shared vertex are disjoint. So a run that is its
pair's In arc never holds another pair's cut, and a pair crossing
twice always turns clear. A refusal therefore needs a pair with four
or more crossings that the pairing joins along its Out arcs, which
`insert::pairing_start_turns` picks from A's kept side for the
finish. Other pieces can sit on those Out arcs.

- **Laminar:** for the pairing taken, there is truly no way round. A
  result exists, though: the oracle's volume is defined, and so is a
  one-vertex-per-cone body. The topology allows two ways to build it:
  - the other pairing, where the refusing pair's runs are its In arcs
    and always clear;
  - keeping this pairing and nesting each other pair's run inside the
    Out run that holds it. The held run would mint at that run's
    copy, as `hang_at_shared` already does within one plan. The held
    runs are In runs inside an Out run, so the copy is the Out end of
    both, and `mint_directed`'s rule (no vertex is the In end of one
    null edge and the Out end of another) holds.

  The reconcile does neither. Its contract is that other pairs' runs
  are disjoint, apart from strut-in-strut (`holds_whole`).
- **Crossing:** no laminar family exists for the pairings taken. Only
  a re-pairing could build these.

The refusal is typed and true of the plan. It is not a statement that
the pose has no body.

**Measured with the other pairing** (a local experiment, not
committed). `reduce` re-planned the refusing pair with
`pairing_start_turns` inverted and retried, once per pair:

| class / op | builds `OK SOUND` | refuses `JoinDesync` |
|---|---|---|
| laminar, `U` | 452 | 22 |
| laminar, `S` | 6 | 468 |
| laminar, `I` | 0 | 18 |
| crossing, `U` and `S` | 0 | 16 |

No line went wrong. Every build passed the battery's volume, tier 2,
3′ and certificate checks. Every refusal was
`JoinDesync { what: "conflicting seam vertex correspondence" }`
(`finish::setopfinish`), a fault-class error, where the start rule
refused typed. In the cleave witness the other pairing builds `cube ∖ y`
(volume 0.97142669 = 1 − `cube ∩ y`, 3′ and tier 2 pass), and
`y ∩ cube` and `cube ∪ y` refuse `JoinDesync`. So whether the other
pairing builds depends on the pose, not on the op. JOIN's 2026-10
measurement, that the other start fails at the finish, no longer
holds there for `cube ∖ y`.

**The design question: which pairing does a pair take at a vertex
other crossing pairs share, when the start rule's runs have no clear
way round?** Options, with evidence:

1. **Keep refusing typed** (today). All 982 lines and the three cleave
   ops stay limits, and nothing builds wrong.
2. **Choose the start at a shared vertex by the shared solid's
   clearance** (its runs are its In arcs). This builds 458 of the 982
   lines and `cube ∖ y` today. The other 524 lines, `y ∩ cube` and
   `cube ∪ y` move from a typed limit to a `JoinDesync` fault, unless
   the finish's seam correspondence learns to take a kept vertex
   whose pair's runs are not copies. The start rule's doc says only
   A's side can be chosen. This option says the shared vertex
   chooses.
3. **Keep the start rule, and nest across pairs at the shared
   vertex.** The reconcile and `hang_at_shared` would read every
   crossing pair's runs at the vertex as one laminar family. A run
   another pair's Out run holds would mint at that run's copy. This
   covers the 966 laminar lines and the cleave witness by
   construction, and leaves the 16 crossing lines typed. It is
   unbuilt and unmeasured, and it changes what `reconcile_shared`
   promises (other pairs' runs disjoint).
4. **Options 2 and 3 together**, or 3 with 2's re-pairing only for the
   crossing class.

Both 2 and 3 change the insertion contract that the module docs of
`crates/topo/src/boolean/insert.rs` state, so neither is built here.
