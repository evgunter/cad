---
id: census-face-walks-skip-torn-hops-silently
kind: issue
title: census's backstop and snapshot skip torn boundary hops silently, and a reviewer-probe block asserts nothing
status: open
opened: 2026-10-06
priority: P4
cost: E
---


(TOPO fix lane on PR 4099, from the PR 4099 review's S3 and S4; both
pre-existing.)

## What

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

## Fix shape

Decide per site whether a member hop is a link (then read it through
`face_boundary_linked` and let it panic like `reach_box`) or a typed
refusal (then refuse the face, as `FaceUnboundable` does for an empty
result); either way one torn face meets one posture. Drop or
`#[ignore]` the probe block.
