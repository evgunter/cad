---
id: shared-vertex-crossings-that-tie-or-interleave-are-unprobed
kind: issue
title: "SharedVertexCrossings: a pair paired along its Out arcs, with other pieces on both ways round, has no run; which pairing a shared vertex takes is open"
status: closed
opened: 2026-10-03
closed: 2026-11-11
priority: P1
cost: M
pr: 6832
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

## Fork dispatched (2026-11-11)

GAUGE orchestrator. Three designers, A, B and C, weigh the design question under `docs/DESIGN-FORK-PROTOCOL.md` (protocol `2126b860b63`). The arm byte drew three; the draw and labels are on `analysis/design-fork/shared-vertex-pairing`. All three have the same problem statement, and this section's options are given to them as hypotheses, not candidates. The outcome goes to Ev as an `[ev]` PR if it changes ratified text or the designers split. Otherwise the orchestrator decides it and records why.

## Decided (fork `shared-vertex-pairing`, 2026-11-11)

Three designers (fork-log row 261) were unanimous on their first reports. No ratified text governs the pairing: the insertion contract is agent-written in `insert.rs`'s module docs. So the GAUGE orchestrator decides it, with no `[ev]` PR.

- **The fault is the op-chosen start.** `pairing_start_turns` pairs a crossing pair from the germ whose run lies on A's kept side, "the one the finish needs". For `∪` and `∖` that lays a pair with four or more crossings along A's Out arcs, where the other operand's pieces sit. The rule was written on 2026-10-04 (`c50f2ff22e6`), two days before `zip::split_cones` (one vertex per cone) removed the finish's need for it.
- **The pairing is read from the pose.** Each crossing pair pairs consecutively along the In arcs of its **lead** vertex's link: the arcs inside the partner piece. The lead is the vertex other crossing pairs share, or A's where neither is shared. The other operand's pieces at the point are disjoint, so no other pair's cut lies on those arcs, and every run at a shared vertex is clear by construction, for both the laminar and the crossing class. The other solid's runs nest as B's do today (`b_runs`), read per solid. The op leaves the insertion.
- **The finish takes the correspondence as given.** `setopfinish`'s vertex-set shape check ("conflicting seam vertex correspondence", with its `shared_cut` and `one_ring` exemptions) retires: `split_cones` reads the cones. Designer C's probe measured it: with the start flipped where a shared vertex's runs were not clear and the check off, `four_pairs_battery` builds 5,190 of 5,190 lines sound, and the cleave witness builds all six ops at the oracle's volumes.
- **The reconcile** keeps the tie reading and strut-in-strut (`holds_whole`). The run-turning loop and `hang_at_shared`'s re-reading of turned runs should go dead, and are removed or become invariants once measured. `SharedVertexCrossings` shrinks to the one-arc case (its own closed row).
- **The typed refusal is not an acceptable final state** for either class: both have a defined body, and the kernel builds it.
- **Not done:** nesting across pairs (the row's option 3); with this pairing nothing needs it. The one shape that might, two crossing pairs sharing both vertices, is filed as `two-crossing-pairs-sharing-both-vertices-refuse-non-manifold-result`.

## Dispatch (the build, 2026-11-11)

GAUGE orchestrator. One implementer in lane `gauge-svc`, branch `gauge/in-arc-pairing`. Review tier: a single full review. The change deletes mechanism more than it adds, and it is measured over the whole battery, the slow sets and the cleave witness. But it moves the pairing of every `∪` and `∖` vertex pair with four or more crossings, so the review carries correctness claims.

## Reopened (the build stopped, 2026-11-11)

The build of the decision above, PR 6832 at `0fd699740c`, stopped at a design question. The battery built 5,190 of 5,190 sound, and the cleave witness built all six ops. But two families that build on main failed:

- **Nested cones at a shared vertex.** The four editor-core `names::emit_topo::touch_reread_rows` tests, `x − y` of `over_a_void`. `y` is a pyramid less a nested pyramid at the same apex, so its two vertices there bound nested cones. The lead's In arc against the inner vertex holds the outer pair's cuts, and the build's invariant fired. The premise "one operand's pieces at a point are disjoint" is false: its curves at a point never cross, but the regions they bound can overlap.
- **A tangent ellipse rim in `∪`.** Four `a_closed_conic_in_the_partners_face` tests refused `UnpairedLooseEnds { count: 4 }`. The vertex is not shared. Its In arcs are slivers of the coincident crescent, coded In for `∪` by `recl` (Eq. 15.3). Pairing along them joins germs of two result cones, which the join's one-site rim lane cannot close.

The orchestrator ran two more reconciliation rounds with the same three designers. They were shown the build's evidence (round 2), then each other's reports (round 3). Rounds and token counts are on `analysis/design-fork/shared-vertex-pairing`.

## Decided, revised (fork `shared-vertex-pairing`, round 3, 2026-11-11)

All three converged in round 3, each having measured its own prototype. No ratified text governs the pairing, so the orchestrator decides it again, with no `[ev]` PR. This replaces the pairing and reconcile bullets of `## Decided` above. The finish bullet stands.

- **The default start is the kept side (main's rule).** A vertex pair pairs consecutively from the start whose runs lie on the side the op keeps. The op contributes only that fact, which is the one the result is selected by. Two reasons:
  - a kept arc is a stretch of one cone of the result, so a null edge along it joins germs of one cone, which the join's one-site rim lane needs;
  - the codes the pairing reads already carry the op at coincident faces, so an op-free rule over them is not free of it.
  - The Out default was measured wrong. On a pin hanging into a pocket with the rims tangent (the mirror of family 2), Out refuses `∩` with `UnpairedLooseEnds { count: 4 }`, where main and the kept start build.
- **At a shared vertex, a blocked pair takes the other start.** A pair whose kept-side runs would hold another pair's crossings takes its other start, where that is clear. The pairs at one vertex choose together, by the fewest departures from the kept side. Designer B measured that flipping one blamed pair at a time dead-ends on the cleave witness's `y ∪ cube`.
  - A and C argued that a clearance test reading only germs makes each pair's choice independent. If the built reading is that, the joint choice reduces to a per-pair one. The build measures which holds.
- **Turning retires**, and `hang_at_shared` with it: taking the other start covers every turn that clears a run. The tie reading (`tied_held`), strut-in-strut (`holds_whole`) and forced struts stay.
- **`SharedVertexCrossings` stays typed where no choice is clear.** That happens only where a pair's curve has other curves on both sides: three nested cones of one operand at an apex, or a shell beside a separate piece. No witness of either exists. The known way to build them is B's round-2 construction: pair along the partner's kept arcs, and place a vertex's chords as one non-crossing family. It is filed as a limit row.
- **Withdrawn:** pairing along the lead's In arcs; "the op leaves the insertion"; and the premise that one operand's pieces at a point are disjoint. The insertion contract states instead that partner curves are disjoint curves, not disjoint disks.
- **Measured by the designers** (debug, prototypes on `0fd699740c`):
  - the battery at 5,190 sound, line for line equal across two runs;
  - the cleave-witness file at 27 of 27;
  - family 1 at 4 of 4 and family 2 at 22 of 22;
  - topo, sweep and editor-core `ci` green under A's variant.
  - Not run: the slow set beyond the family-2 file, and the gates.

## Dispatch (the rebuild, 2026-11-11)

GAUGE orchestrator. The same implementer, in lane `gauge-svc`, rebuilds PR 6832 to the revised decision. Review tier: unchanged, a single full review with correctness claims.

## Closed (GAUGE, lane `gauge-svc`, PR 6832, 2026-11-11)

Built as re-decided (`## Decided, revised`).

**The rule as built.**
- A vertex pair of more than two crossings pairs in its lead's walk from the kept side (`insert::start_pairs`, `insert::pairing_leads`). Two survivors keep main's record-order run.
- At a shared vertex the pairs choose their starts together, the fewest off the kept side, ties by partner key (`insert::choose_starts`, `insert::blocked`).
- No run turns, and `hang_at_shared` stays retired.
- `SharedVertexCrossings` is typed where no choice is clear.
- The finish's vertex-set check stays retired.

**Evidence.**
- **Battery:** `four_pairs_battery`, release, gives 5,190 `OK SOUND` and 0 `ERR` (main `da005e4b27`: 4,208 and 982 `SharedVertexCrossings`). The 4,208 lines that built on main are identical in face, edge and vertex counts and in their lines.
- **Independence:** the joint choice equalled the per-pair one at all 5,353 shared-vertex choices measured (battery and `ci`).
- **Cleave witness:** `three_corners_alternating_round_the_cube_build_every_op`, all six ops, also under reversed vertex pairs.
- **Family 1:** the four `touch_reread_rows` pass. Re-pinned at eight corners held as several vertices; which operand names a pinch vertex follows the pairing, and the PR body leaves that weighing to review.
- **Family 2:** all 22 tests of its file pass.
- **The dual pose:** `a_peg_in_a_pocket_tangent_to_its_rim_builds_its_unions_and_intersections`.
- **Mutants**, all on this head: an Out default (the dual `∩`), an In default (family 2), no other start (the witness, `SharedVertexCrossings`) and the restored check (the witness, `JoinDesync`) are each red on a named test. Greedy one-pair flipping survives, consistent with the measured independence.
- **Slow sets:** topo and sweep with `--ignore-default-filter` at default ε ran 7,803 tests with 8 red, all 8 red on main too.

**Filed:**
- `a-pair-curve-with-partner-curves-on-both-sides-refuses-shared-vertex-crossings` (GAUGE, P3), the typed limit;
- HONEJOIN's `a-one-vertex-rim-in-its-partners-coplanar-face-closes-only-when-its-germs-share-a-null-edge` (P2), the dual pose's one-segment `∖`.

**Evidence added:**
- RESTREAD's `census-cross-solid-curved-pairs-undecidable-on-shell-results`: the dual `∩`'s tier 3′ is `CensusUndecidable` only, not a decided failure;
- ONECOPY's `t4-ports-unpinned-for-a-kept-reading-ahead-of-an-answer`.
