# CONTACT-12: the edge-on-face overlap lane cuts at boundary crossings

**Binds one implementer lane.** Deleted at merge;
`work/contact/CONTACT-12.md` survives. Read
`docs/prompts/implementer-discipline.md` in full first.

Branch `contact/12-ef-crossing-cuts`, from `main`. Read the row in full:
`work/contact/overlap-lane-boundary-crossing-cuts`. Then read:
- `crates/topo/README.md`, the Grandfathered rungs paragraph. This is
  Ev's ratified unified strength and its grandfather roster (2026-09-01):
  the unconfined backing rungs migrate one at a time, each measured.
- `census.rs`'s module docs on the rungs and on the reach gap.
- `ef_bound_backed`, `ef_overlap_lane` and `ef_overlap_cells`.
- The anomaly pin `review_mate4a_r2_probes::r2_an_unrelated_declared_pair_backs_the_ef_bound`,
  and `mate4a_ef_bound_rung.rs`.
- PR #1496 (MATE-9), which measured the migration and kept the
  grandfather.

## The gap

`ef_overlap_cells` cuts an edge lying in a face's plane only at the
face's boundary VERTICES on the edge's line. Where the face's boundary
crosses the edge away from any vertex, no cut is made. The cell is then
bounded at the edge's own endpoints, which can lie far outside the face.
MATE-9's measured witness is the declared straddle seat: dive cell
midpoint (0.45, 0.3, 0.5), cell bounded at 0 and 0.9.

Two things stand on the cells:
- **The region-confined `ef_bound_backed`.** MATE-9 built it and
  measured it red on exactly the straddle seat.
- **The touch analysis.** It reads an edge-on-face overlap at its first
  cell's point (`census.rs`, ~:3396).

## The work, in order

1. **Cut at boundary crossings.** Cut the edge wherever it crosses one of
   the face's boundary edges transversally in the face's plane, as well
   as at coincident vertices.
   - Decide the crossing metrically: a point's signed distance, in
     metres, as CONTACT-7's `metric` does. A levered direction is not
     enough. This track's recurring defect is a levered reading whose
     Zero is a verdict, so for each decide say which verdict it feeds
     and why its bound is sound for that verdict.
   - A crossing too close to call escalates, typed.
   - Curved boundary edges of a planar face: cut them too if a
     certified intersection exists; otherwise refuse, typed. Never skip
     silently.
   - A cell's midpoint is then inside or outside the face as a whole
     cell, never straddling.
2. **Measure the rung's migration.** Re-attempt MATE-9's region-confined
   `ef_bound_backed` under the ratified measured-migration protocol:
   - Run the full census-backed suites before and after.
   - Name every row that moves, and why.
   - The anomaly pin must go red under a correct re-attempt, exactly as
     it did under MATE-9. Re-baseline it to the confined behaviour (the
     unrelated pair backs nothing) and say so.
   - If the straddle seat and every certifying seat hold (MATE-9's
     roster: #969/#1063/MATE-4a/MATE-5/MATE-8), migrate. Drop
     `ef_bound_backed`'s face-pair arms from the grandfather roster in
     `crates/topo/README.md` and in `census.rs`'s docs; the note shrinks
     by one name.
   - If any seat breaks, stop and report the measurement. Do not
     grandfather anything new.
3. **The touch analysis's cell.** Check that the touch analysis reads
   the right cell now that cells are finer. Every cell of the overlap
   is a meeting, not only the first. If only the first is read today,
   that is a gap: read every cell, or say why the first suffices.

## Rows

- The straddle seat certifies under the confined rung.
- A face whose boundary crosses an edge away from any vertex. Show the
  cells, and that the one outside the face is not reported.
- A crossing within the band escalates.
- The anomaly pin, re-baselined.
- Each cut decide, pinned at the verdict it feeds.

## Interaction

The declared-only repair (`declared-only-meetings-clear-at-the-census-gate-unread`,
its design on `[ev]` PR 3422) will edit arm 2 of the same file later.
Keep this unit to the overlap lane, the rung and the touch analysis's
cell read. Do not restructure arm 2.

## Discipline

- Work in your own clone, with `CARGO_TARGET_DIR=/home/user/contact-12-target`,
  `CARGO_INCREMENTAL=0` and `CARGO_PROFILE_DEV_DEBUG=line-tables-only`.
- Wrap every cargo command in `with-build-slot.sh`, and run `df` first.
- No process listings, and kill only PIDs you recorded.
- Never run git in `/home/user/cad`.
- Commit and push often; the container can restart.
- Push the branch only; no PR.

Before hand-back, run all of the following:
- `topo` + `sweep` at three eps;
- ALL of `editor-core`;
- `test-utils`;
- the Python suite (maturin wheel);
- clippy, `cargo fmt --all --check`, gates, lint.

Write `docs/CONTACT-12-PR.md`, the PR body, with the measurement table.
