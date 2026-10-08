---
id: tilted-read-accepts-a-zero-touch-on-any-vertex-sharing-pair
kind: issue
title: moved_walls_cross accepts a Zero overlap on any pair that shares a vertex, wherever along their common line the touch is
status: open
opened: 2026-10-08
priority: P3
cost: E
refs: [shell-clearance-gate-skips-planar-pairs-tilted-off-antiparallel]
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
