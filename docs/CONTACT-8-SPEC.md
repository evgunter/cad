# CONTACT-8 — the merge prunes dangling seam edges; a boolean never ships an unglued planar group

**Binds one implementer lane.** Deleted at merge; `work/contact/CONTACT-8.md`
survives. Read `docs/prompts/implementer-discipline.md` in full first.

Branch `contact/8-dangling-seam`, from `main`. The row is
`work/contact/area-overlap-contact-admitted-but-unmerged-refuses-at-the-next-step`;
read it in full, including "The cause, corrected" and "Ruled". The
design is ratified. `docs/DESIGN.md`, "Maximal-faces precondition and
the merge stage", states it as landed by PR 3350. Read that clause, and
PR 3350's description (both designers' analyses).

## What is wrong

Two blocks' coplanar caps are declared in contact and overlap in AREA.
Their shared seam bends 90° at the corner. `merge_coplanar_faces`
licenses the group, glues it, and kills the first shared edge. The
second is left dangling, with its corner vertex free. The door kills a
dangling strut only when the seam is straight (a collinearity test in
`redundant_subdivision_vertex` / the `straight_seam` pre-decision in
`merge_group`), so it reports `GroupNotClosed{ScaffoldingEmptyLoop}`.
The declared regime records that as `merge_skipped` and ships two
coplanar caps. The next boolean's F7 gate refuses them as an
undeclared contact of a pair the user declared.
`crates/topo/tests/merge_skip.rs`'s L-corner row pins the skip.
Reproducer: `a = [0,1]³` and `f = [0.5,1.5]² × [0,1]` with
`flush_declarations`, then union with any third brick touching either
block.

## Settled design (ratified)

**S1. Prune by topology, not geometry.** After the group's faces are
joined, repeatedly delete every shared (doubled) edge that has a free
end, together with that end (`kev`), whatever the angle.
- Only a doubled edge with no free end separates a ring; that is the
  existing `kemr` step, which runs after the pruning.
- Drop the collinearity test and its numeric band and escalation arm,
  and drop the "exactly two shared edges" guard. A shared CHAIN of `k`
  edges kills its `k − 1` interior junctions.
- Every deletion is recorded in `MergedGroup::killed_vertices`.
- State the argument at the site: a dangling edge inside a face
  encloses no area, so deleting it and its free end leaves the merged
  face's region exactly as it was.

**S2. A boolean never ships a planar group it was licensed to merge
and could not glue.** The step refuses with the merge's own typed
reason: flip `GroupRegime` for declared planar groups to the refusing
regime. Only a curved group's skip stays recorded in `merge_skipped`,
because F7 accepts its cut form. Any refusal text uses the shared
endings (`geom_core::predicate`'s `KERNEL_DEFECT_ENDING` family, ENCL's
PR 3346) where they fit.

**S3. Records.** A contact record citing a deleted free end drops as
consumed, under the same strict rule as a zip-fused vertex (tier 3′).
Verify this with a row. The merge never fuses vertices and never
removes a vertex from a face's boundary, and the ratified clause says
so. If you find a case where pruning would do either, stop and report.

**S4. Check before deleting.** Check whether the collinearity licence
was ever load-bearing: read `redundant_subdivision_vertex`'s history
(`git log -S` on it, and on the F7 pole-half change of 2026-08-28).
If a reason other than a test pin shows up, stop and report. If the
function loses every production caller, delete it.

## Rows

- `merge_skip.rs`'s L-corner row flips: the caps merge, `merge_skipped`
  is empty, the volume is unchanged, and tier 2 and tier 3 are green.
  Re-sign it with the reason.
- The reproducer end to end:
  - `a ∪ f` publishes two `Merged` rows and one cap each, top and
    bottom;
  - `∪ c` with its declarations succeeds with no skips;
  - `∪ c` WITHOUT its declarations refuses a correct cross-operand
    undeclared contact.
- A four-way junction (`f7d_delta_probes` D3), a seam chain of three
  edges, a seam with a hole, and a straight seam
  (`f7_pole_split_cap_repairs_to_one_face`, which must pass unchanged).
- `f7d_delta_probes` D1, D2 and D4 and the `verbs_f7_*` probes: re-sign
  each one that moves, with the reason. D2's escalation arm goes. The
  prose about collinearity is corrected.
- S2: a planar declared group the merge still cannot glue refuses the
  step. If no reachable shape exists after S1, say so and pin the
  regime with a unit row on the regime itself.
- S3: a record citing a pruned vertex drops.
- A class sweep: every other door that skips or ships a group it could
  not glue.

## Discipline

- Your own clone and `CARGO_TARGET_DIR=/home/user/contact-8-target`,
  with `CARGO_INCREMENTAL=0`.
- `with-build-slot.sh` for every cargo command.
- `df` first.
- No doc-gate script.
- No pattern kills, no process listings; never list the shared
  scratchpad.
- Push the branch only; no PR.
- Run the `editor-core` concision rows, perf12 and docm6, and
  `-p test-utils`, under nextest.
- Write `docs/CONTACT-8-PR.md`.
