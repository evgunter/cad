---
id: declared-opposite-orientation-refusal-is-unreached-by-any-row
kind: issue
title: three planar-door consumers are reached by no row with a reversed planar face — merge_faces's declared-pair rung, join's ring_run_ccw, rest's face_carrier — so dropping the sense at the door survives the suites there
status: open
opened: 2026-09-15
priority: P3
cost: E
---


## Finding

`crates/topo/src/merge_faces.rs`, `planes_declared_equal`'s declared-pair
rung: the two `PlaneDesc` normals are the faces' OUTWARD normals (the
sense folded in through `face_normal::plane_outward_normal`), and the
`oriented_plane_eq` verdict `SameOpposite` is what fires
`MergeCoplanarError::DeclaredOppositeOrientation` — "an opposite-sense
pair on one plane lands there by construction" (the comment at the
rung). No row in the tree names that variant
(`grep -rn DeclaredOppositeOrientation crates` finds only the
definition, the rung and its docs), and a mutant that makes the
planar door ignore the bit (`plane_outward_normal` folding `true` for
every face) survives `cargo nextest run -p topo -p sweep -p mesh -p
editor-core` at this rung: the six rows it does red are check 6's
(`tier_three_refuses_a_hand_flipped_face_sense` ×2,
`tier_three_refusal_is_surgical`), check 9's
(`check_9_refuses_a_ring_that_lies_outside_its_outer_loop`) and
`merged_outline_ring`'s (`verbs_1031b_arcwind` ×2). So a declared
coplanar pair whose two faces carry opposite senses is not exercised
by any row: the refusal is documented, typed and unreached.

What is owed is one row that declares such a pair (two coplanar faces
of one body, one of them reversed through `Body::set_face_sense`, the
pair declared through the merge door's `DeclaredCtx`) and asserts the
variant — the same shape as PR 2649's
`the_rim_routing_reads_the_second_faces_sense_and_not_the_firsts`. It
would also be the row that reds when the rung's normals lose the
sense.

Found by SENSE-FOLD (SCALAR, 2026-09-15), from the anti-vacuity mutant
over the folded planar door; `merge_faces.rs` is TOPO's by
`work/topo/program.md`.

## The class: three consumers, not one (SENSE-FOLD fix pass)

The same mutant — `face_normal::plane_outward_normal` folding `true`
for every face — survives at two more consumers of the planar door for
the same reason: every fixture that reaches them carries `sense: true`
on its planar faces.

1. `crates/topo/src/merge_faces.rs` `planes_declared_equal`'s
   declared-pair rung (above).
2. `crates/topo/src/boolean/join.rs` `ring_run_ccw`: decides an
   island's outer boundary as a `bool` — the mef run must wind CCW
   around the face's OUTWARD normal — so a dropped bit is a silently
   wrong role assignment on a reversed face, not a typed refusal. The
   one producer of a reversed PLANAR face through the public doors is
   a revolve's under-side annulus (`sense: false`, chart normal into
   the material), and a pocket subtracted into it would ring exactly
   that face. Tried in the fix pass and declined as not cheap: a FULL
   revolve's wall is one face a period wide and the containment lane
   escalates `bool_wall_trim_period` on it before the join; a QUARTER
   revolve (`Revolution::Partial(π/2)`) with a brick pocket at the
   sector's mid-azimuth refuses `Join(UnpairedLooseEnds { count: 2 })`
   — and so does the same pocket into the honest TOP annulus, so the
   refusal is the boolean's scope on a revolved wedge, not the bit.
   The row that closes this instance needs a boolean that admits a
   pocket into a revolved body, or a second producer of a reversed
   planar face; either is a fixture on BOOL's or TOPO's ground.
3. `crates/topo/src/boolean/rest.rs` `face_carrier`'s plane arm: the
   `CarrierDesc::Plane` normal handed to the REST ladder's carrier
   comparison. Reached only through a declared coincident pair, so
   the row is a declared planar pair with one face reversed — the
   same fixture shape as instance 1 with the declaration routed to
   the REST lane.

A row on any instance is a row the others can share the fixture of;
what the class needs is one reversed planar face on a body the
boolean's front door admits.

## Re-homed to ZIP, 2026-09-24 (ATREST orchestrator)

Moved from `work/atrest/` by `git mv`, id and body unchanged. All three
consumers this row wants guarded are on ZIP's ground —
`merge_faces.rs` (TOPO and ZIP), `boolean/join.rs` (ZIP),
`boolean/rest.rs` (TANG and ZIP) — and the owed rows are tests of
those consumers, not of the at-rest validator. It sat on ATREST's slate
from TOPO's cut, where the planar door it names
(`face_normal::plane_outward_normal`) was the anchor; the door is fine,
its downstream readers are what go unexercised.
