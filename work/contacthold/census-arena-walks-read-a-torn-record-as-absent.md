---
id: census-arena-walks-read-a-torn-record-as-absent
kind: issue
title: The census's own arena walks read a torn record as absent
status: open
opened: 2026-10-06
priority: P3
cost: M
---


## What

(Found by the TOPO lane that converted
`torn-hops-read-as-absent-in-the-split-the-chord-join-and-the-reach-rules`,
whose remit in `census.rs` was the reach rules only: `face_reach_in`,
`boundary_axial`, `boundary_reach`, `edge_reach_in`. Those now panic
on a hop past the face or edge they resolve.)

The census runs on bodies tier 1 admits, so a record miss inside it is
a kernel bug, and D2 row 4 (Ev on PR 4006) says it panics naming its
premise. The census's own walks read one as absent instead. Candidates
from `get_*(` / `.<arena>.get(` followed by `?`, `continue`,
`filter_map`, `if let Some` or `is_none()`, in production code
(line numbers ride along and may be stale):

- `edge_is_line` (`census.rs:1058`): a torn curve reads as "not a line",
  so the edge leaves the exact sweeps.
- `face_cycles` (`:1075`): a lost loop or an unwalkable cycle answers
  `Err(loop)`, and each caller says what that means.
- `snapshot` (`:1094`, `:1107`, `:1135`, `:1154`): a vertex whose point
  is torn is dropped by `filter_map`, an edge whose half-edge, vertex
  or point is torn is dropped by `?`, a face whose surface is torn is
  read as not planar.
- `sweep_cross_solid_backstop` (`:4731`, `:4756`, `:4775`, `:4783`,
  `:4847`, `:4948`, `:4982`, `:5059`, `:5145`): faces, half-edges and
  surfaces read through `if let Some` / `let Some(..) else`, a miss
  skipped or read as no box.
- `confirm_curve_and_patch_records` (`:5634`): a contact's edge curve
  read through `.and_then(..)`.

**HOLD (D10, PR 3990).** `ee_cross_backed` (`:2057`),
`sweep_conformal_patches` (`:2369`) and the patch arm of
`confirm_curve_and_patch_records` (`:5690`) read declared face pairs
and contact records: their own logic reads a declaration, so they wait
for D10. Where a declared key is stale, `StaleContactDeclaration` is
the honest typed answer and stays.

## Direction

As the boolean's record-hop unit did it (PR 4066): per site, decide
whether the key is the caller's (typed) or a record hop past a
resolved record (`live::linked` / `proven` / `face_surface_linked` /
`edge_curve_linked` / `Body::face_loops_linked`, a panic naming the
premise), with a torn-body witness per converted read that goes red
under a mutation restoring the old reading.

Moved from SECTOR to CONTACTHOLD on 2026-11-20, by `git mv`, when SECTOR merged into CONTACTHOLD: SECTOR held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one CONTACTHOLD carries.

## Merged in: `census-face-walks-skip-torn-hops-silently` (2026-11-25)

Folded in at the CONTACTHOLD cleanup (`docs/doc-ledger/contacthold-leaves-the-tracker.md`): the same mechanism, fixed by the same change. Its title was: census's backstop and snapshot skip torn boundary hops silently, and a reviewer-probe block asserts nothing. Its body follows, headings demoted.

(TOPO fix lane on PR 4099, from the PR 4099 review's S3 and S4; both
pre-existing.)

### What

**Per-hop silent skips in census's face walks.** census's backstop
(`crates/topo/src/census.rs`, the cross-solid backstop's closures
`face_vertices`, `face_points` and `line_bounded`, ~`:4727-4795`) and
`snapshot` (~`:1147-1163`) walk a face's boundary through census's own
`face_cycles` and then step past each member hop that does not resolve:
`body.half_edges.get(he)`, `body.vertices.get(hd.start)` and
`body.points.get(v.point)` under `if let Some`. The `face_cycles` doc
says what an unwalkable *loop* means is the caller's to say, and the
callers do say it; nothing says what a torn *member* hop means.

One torn face meets two postures: `face_points` drops a torn hop and
refuses (`FaceUnboundable`, ~`:4857`) only when the whole result is
empty, while `reach_box` (`face_reach`, ~`:4886`) on the same face then
panics on the same link through `Body::face_boundary_linked`. A face
with one torn member among live ones gets a hull of the survivors from
`face_points` and a panic from `reach_box`.

**A reviewer-probe block in the test module.** `census.rs` keeps a
block headed `CERT-N2 R2 reviewer probes (not for merge)` (~`:6911`).
Its row `n2r2_class7_face_reach_partial_box_and_census_decision` only
`eprintln!`s its readings (`near == far ? …`) and asserts nothing past
a fixture count, so it is not a gate (implementer discipline §8: drop
it, or `#[ignore]` it with its run command).
*Discharged by PIPE's S350 (PR 4482):* the row is replaced by
`a_net_poisoned_in_one_channel_has_no_reach_and_clears_no_pair`, which
gates, and the block's banner now names what it holds.

### Fix shape

Decide per site whether a member hop is a link (then read it through
`face_boundary_linked` and let it panic like `reach_box`) or a typed
refusal (then refuse the face, as `FaceUnboundable` does for an empty
result); either way one torn face meets one posture. Drop or
`#[ignore]` the probe block.

### Re-homed at TOPO's cut (2026-10-11)

Moved from TOPO to PROBE, whose charter is the guards and generators that cannot go red. TOPO kept only the Euler surgery rows when it dropped to P2.

Moved from PROBE to TOPO on 2026-11-20, by `git mv`, when PROBE merged into TOPO: PROBE held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one TOPO carries.
