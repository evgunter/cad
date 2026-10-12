---
id: tilted-read-accepts-a-zero-touch-on-any-vertex-sharing-pair
kind: issue
title: moved_walls_cross accepts a Zero overlap on any pair that shares a vertex, wherever along their common line the touch is
status: closed
opened: 2026-10-08
priority: P3
cost: E
refs: [shell-clearance-gate-skips-planar-pairs-tilted-off-antiparallel]
branch: shell/tilted-read-gaps
pr: 4467
closed: 2026-10-10
---


Disclosed by PR 4311, which added the tilted read
(`moved_walls_cross`, `crates/topo/src/shell.rs`).

Two moved planar faces that share a vertex hold that vertex's moved
copy both, so the line their planes share meets both faces there and
the overlap decide (`shell_moved_walls_overlap`) lands on Zero. The
read accepts a Zero overlap on such a pair. It does not check that
the touch is AT the shared vertex: a vertex-sharing pair whose moved
faces touch somewhere else along that line, at a single point or
within the band, passes. A positive overlap still refuses on every
pair, so what is open is a touch, never a crossing of measurable
length.

Same shape, one scale up, as CLEAR's
`self-intersection-drops-every-vertex-sharing-face-pair-globally`
(a global exclusion where the justification is local). The local
discharge here is cheap: compare the Zero interval's position along
the line with the shared vertex's moved point, and accept only the
touch there.

No fixture reaches it today; none was built.

## Closed

2026-10-10, PR 4467. `walls_cross` (split out of `moved_walls_cross`)
takes each shared vertex out of the overlap, widened to twice the
escalation width, and the moved common edges with it. What is left is
decided: Positive is a crossing and refuses `OffsetsCross`. So is Zero,
which is now a touch away from where the faces are joined. The
`shell_moved_walls_touch_vertex` read this needed first was folded into
that excision before merge. Two joined faces come arbitrarily close at
the vertex they share, so at a fine eps an in-band margin there
escalated the tour's sectioned vessel.

A touch is read whichever side of the line the face lies on: each face
is cut twice, a point on the line counted with one side and then the
other, and the two cuts united. The first version counted it with one
side only, so a vertex touch, or an edge within the band of the line,
showed on half the configurations and flipped with the pair's order
(second review of PR 4467).
`a_contact_on_the_line_is_read_from_either_side_in_either_order` pins
both faces' sides in both orders.

Pinned at unit level by `the_tilted_read_accepts_a_touch_only_at_a_shared_vertex`:
a touch at `0.5` refuses at overlap zero, and the same pair meeting only
at its shared vertex clears. No row through `shell` reaches a Zero
overlap away from a joint. On a square pyramid, whose opposite faces
share only the apex, one face of each pair cuts `L` empty, so no
overlap is decided at all.
