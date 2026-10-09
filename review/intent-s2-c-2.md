# INTENT stage 2 PR C (#4359): second review (sequential arm)

Head reviewed: **23bad579f**. Scope: C's change only, `git diff $(git merge-base 23bad579f
origin/intent/s2-b-reads)..23bad579f` (merge base b57e0376a). Protocol:
`docs/DUAL-REVIEW-PROTOCOL.md` rule 1, the second review after the fix pass. I did not read the
first review until this report was final. No glimpses of it.

## Verdict: NOT-MERGEABLE-AS-IS

The fix pass closes MAJOR-1 at the slot door and at load, and MAJOR-2 with a row that bites. But
**split** still re-points a remainder reader at the part's world, which is the exact thing the
orchestrator's ruling forbids ("split/inline never re-point a reader at a copy"), and
**inline** silently drops a copy. Both were shown by execution. Both fixes are local to
`refactor.rs`. Inline already refuses the matching cases, so split needs the same guard.

## Findings

**MAJOR-A. Split re-points a remainder reader at the instance's body, which is the part's whole
world, whatever the cut's placements hold.** `crates/editor-core/src/refactor.rs:2797-2830` collects
`cut_placed` from every cut `PlaceInWorld`, with no regard to its pose or to how many bodies the cut
places. `:3534-3545` then `SetParam`s each crossing read to `instance_body`, and that body is the
part's product (`eval/parts.rs`): every copy the part places, each at its pose.
- *Posed*: block `b` placed at +2z, with a kept `Transform` of `b` (+5x) that is also placed. Split
  `{frame, profile, b, p}`. The reader's own body digest moves from `4395003497507227030/28` to
  `12624048515438187782/28`. The reader now reads `b`'s world copy, the pose included: a
  construction reading the world, reached through split.
- *Two placed bodies at the identity*: the cut places `b` and `c`, and the reader reads `b`. The
  digest moves from `16463186990741482242/28` to `17848944405067511859/52`, so the reader now
  transforms both solids.
- Either way this breaks A4's acceptance ("split-then-evaluate equals unsplit evaluation") and
  contradicts the module doc at `refactor.rs:21-27`.
- Inline refuses the symmetric cases (`Uncarried::Posed`, `Uncarried::Bodies`, `:3905-3921`). Split
  has no counterpart, and test 12 (`intent_s2_c_world.rs:409`) covers only one identity placement.
- A kept placement of a cut body that the cut also places is a crossing read too, so it is
  re-pointed the same way. By reading, its copy then moves to the cut placement's pose. I did not
  execute that case.
- Probes: `review/intent-s2-c-2-probes.rs`, `probe_split_posed_placement_moves_a_repointed_reader`
  and `probe_split_two_placed_bodies_widen_a_repointed_reader`. Both are red at the head.
- Confidence: **sure**.

**MAJOR-B. Inline of an instance the host places twice silently drops a copy.** FORK-2b residue 2
allows a second identity placement. Take a host that places one instance twice at the identity, over
a part with one identity placement. Inline deletes both host placements (`refactor.rs:4296-4298`)
and carries the part's one placement once. The result has 1 placement (it had 2), and the product
digest goes from `10958814186987049123/52` to `16998746680022490150/28`. Nothing refuses and nothing
is reported. That breaks C1 ("the placed copies, once each") and A4's inline acceptance. The `posed`
arm refuses several host placements (`:3896`). The identity arm counts nothing.
- Probe: `probe_inline_of_an_instance_placed_twice_keeps_both_copies`, red.
- Confidence: **sure**.

**MINOR-C. A measure's site can name a placement, and its value moves with the pose.** `SitedRef.at`
is not an operand slot, so `check_read`/`first_operand_read_fault` never see it. A `Measure`
(distance) between the end caps of two copies, read at the placements with `StableName::in_copy`, is
accepted at the door. It evaluates to `0.0` with the copy at dz=0 and `2.0` at dz=2. No construction
can read a measure (VR4), so D10's construction rule holds. The ruling "the pose is read only by the
gather and export", and D10's "export reads its coordinates and nothing else does", do not hold.
Probe: `probe_a_measure_sited_at_a_placement`. Confidence: **sure** of the behaviour, **likely**
that it falls outside the ruling (the at-rest checks may be meant to cover it).

**MINOR-D. The viewer's Transform gesture now moves a placed body through a Transform.**
`crates/viewer/src/session.rs:2874-2897`: `add_transform` → `feature_over(self.placements_of(input),
Node::transform(..))` re-points the target's world placement at a `Transform` of it. The world pose
then has two homes: the placement's pose and a Transform under it. This is the "moving a body" shape
from Ev's 2026-10-03 message 2 ("you can't Transform an already placed part"). Before C, A10's tip
transfer had the same visible effect. Now an explicit gesture authors it, which is close to the "no
new Transform uses" ruling. Residue 1 names feature gestures in general and does not single out
Transform. Confidence:
**sure** of the behaviour, **unsure** whether it is in scope for C.

**NOTE-E. The base tree fails the same timing row.**
`name_words_rows::a_large_table_of_names_alike_at_no_citation_is_said_in_bounded_time` fails at the
head here (12.8 s against a 10 s budget, debug build, 4 vCPU) and fails identically at the merge
base b57e0376a (12.807 s). It is the machine, not C. Confidence: **sure**.

**NOTE-F. An instance still re-exports its part's world to host constructions.** A host boolean
reading an instance whose part places its body at a pose depends on that pose. The pose is read "by
the gather" only in the letter: `eval/parts.rs` takes the part's product. Inline's
`Uncarried::Posed` names exactly this. It is consistent with the ruling that the instance ships one
`body` port and with stage 3's item `an-instance-defines-one-body-per-part-placement`, but it is the
general form of MAJOR-A. Confidence: **likely**.

## First-review fixes, confirmed
- **MAJOR-1 (a construction could read a world copy).** Fixed at `edit.rs:1270-1278` (`check_read`,
  so it covers insert, `SetParam`, lists and `DocEdit::place`) and at `persist/check.rs:727-733`.
  - Executed: a union member list reading a copy refuses `ReadsWorldCopy`.
  - Mutant M1 (door check off) kills `no_slot_reads_a_world_copy_at_the_door_or_at_load`, and so
    does M4 (load check off).
  - Mate heads cannot name a placement: `names::verbatim_edge` lists `PlaceInWorld` as none, so the
    member walk stops (`mate/member.rs:250-276`).
  - Viewer picks on a copy seat the body (`viewer/src/world.rs:84-91`).
  - Remaining holes: MAJOR-A (split) and MINOR-C (measure sites).
- **MAJOR-2 (no posed placement anywhere).** Fixed. Mutant M2 (the pose evaluated, then ignored)
  kills `a_posed_placement_moves_its_copy_and_its_records_rigidly`. Mutant M8 (the copy drops its
  contacts) kills the same row and three `mate6_gather_mints` rows.
- **MINOR-1 (the unplaced-mate skip had no pin).** Fixed. Mutant M3 (the skip off) kills
  `mate6_gather_mints::the_gate_skips_a_mate_on_an_unplaced_member_and_mints_the_placed_ones`.
- **MINOR-5 ("world's coordinates").** Fixed in C's diff. C adds no `world coordinates` wording.
  ASSEMBLY A2 (`ASSEMBLY.md:49`, ratified, not in this diff) still says "one `Body` per world
  placement at its world coordinates", which is false of the code. It is filed as
  `an-instance-defines-one-body-per-part-placement`.

## Style
- `refactor.rs:2797-2830` vs `:3858-3921` (Q1). Split's and inline's "re-point the instance's
  readers" are the same rule in two directions. Inline grew `Uncarried::{Posed, Bodies}` and split
  grew nothing; MAJOR-A is that drift. **sure**
- `edit.rs:1237-1279` vs `persist/check.rs:690-745` (Q1, fix-mints-the-defect). The fix for MAJOR-1
  adds a third rule to two hand-kept spellings of one read check (door and load). Their order
  differs: the door checks `PartHalfPort` before `ReadsWorldCopy`, the load the reverse. **likely**
- `refactor.rs:3615`, `:3867`, `:3878` (Q1). "Identity placement" is spelled `pose.steps.is_empty()`
  three times, beside `Placement::is_identity_bits` (`placement.rs:605`) and `o.steps.is_empty()`
  for offsets (`:3073`, `:3753`, `:3796`), with no named predicate. **likely**
- `assembly.rs:1502-1508` (Q7, efficiency). Every mate reference walks `strict_ancestors` of every
  placement, which is O(mates × placements × N). Its "placed" also means "an ancestor of some
  placement", not "read by a placement" as the PR and the comment say: a member read only through a
  placed boolean counts as placed and goes on to the lift.
  **likely**
- `docs/INTENT-STAGE2-SPEC.md` §10 Risks ("An instance on the world gauge has its offset moved into
  its placement's pose") and §11 Q9 ("Recommendation: C's migration moves…") contradict §4's
  reworded migration note (Q4). **sure**
- `refactor.rs:3350-3386` (Q6). The split name re-anchor decides structure from a value
  (`eval_var_count` of the `Part` index). It is flagged interim against `[ev]` #4341, but no `work/`
  item schedules its retirement; only logs mention #4341. **likely**
- `viewer/src/world.rs:107-114`: `made` finds "the node a creation made" as the last minted id that
  is not a placement. That is a positional heuristic over an action's mint list. **unsure**
- Creation gestures place what they make (PR body). That is a GUI default rather than a kernel side
  effect, but it sits next to "nothing places as a side effect", and residue 1 names only feature
  gestures.
  **unsure**
- `product.rs:985-988`: a placement whose evaluation space is not the gathered one is skipped
  silently when other placements gather. This predates C (roots behaved the same), but the copy
  vanishes from the world product with no finding. **unsure**

## Claims exercised
- **C1.** Order and once-each were checked by mutant M5 (placements reversed), which kills test 6.
  `EmptyProduct` and `StrandedPlacement` are reachable (M9 kills test 10), and the recourse texts
  were read.
  **Falsified** by MAJOR-B (inline drops a copy).
- **C2.** Exercised: slot door, lists, load, mate walk, viewer seats, split, inline (posed and
  two-placement), measure sites. **Falsified** by MAJOR-A. Not exercised: Python payloads (they go
  through the same door, by reading), Promote/Fold, Rebind onto `Placed` names.
- **C3.**
  - All 29 corpus `PRE_C` rows were re-derived on the base tree b57e0376a by a probe, and match
    exactly at the default ε.
  - Test 6, the `intent_s2_c_world` rows, `mate6_gather_mints` and `asm4_split_inline` pass at 1e-6
    and 1e-12 (42/42 each).
  - The posed row moves the copy and its record (M2, M8).
  - The five committed-file rows were not re-derived on the base tree.
- **C4.** Three pins were run on both trees:
  - asm2a `row6` (reuse 1→2), docm4 `the_memo_still_serves…` (recompute 1→2) and m10_p_fence (all
    three rows) pass on the base with the old values and on the head with the new ones.
  - The reasons hold by reading: the placement node re-keys or is reused with its body.
  - The fence's "node count only" claim was not checked bit by bit.
  - p2_gauges 8→13 is 8 plus 5 placements.
- **C5.** Seven mutants, all killed: M1 door, M2 pose, M3 skip, M4 load, M5 order, M8 records, M9
  stranded. The two MAJOR mutants are M1 and M2.
- **Runs.**
  - editor-core with the slow set (`--profile default`, default ε, in 3 partitions): 2890/2891.
  - The one failure is NOTE-E, which fails identically on the base.
  - Not run: Python, viewer, pncad, tour.

## Appendix: what this review raised that the first did not
Read after the report above was final.
- **MAJOR-A is new.** The first review's MAJOR-1 named split's "copy arm" of `cut_placed`, and the
  fix dropped that arm. The first review did not see that the remaining arm re-points a reader at
  the instance's body. That body is the part's whole world: a posed copy, or several bodies.
- **New:** MAJOR-B (inline drops a copy), MINOR-C (measure sites read a placement) and MINOR-D (the
  Transform gesture moves a placed body).
- **New in the style lane:** the door and load spellings of the read check, the three spellings of
  identity placement, the spec §10/§11 Q9 contradiction, and the `made` heuristic.
- **Overlaps:** the `resolve_face` cost (first NOTE-7) and the silent own-space skip (first NOTE-1).
  My Q6 point on the `k` pick (no scheduled `work/` item) extends the first review's MINOR-2, which
  was about the missing interim flag.
