---
id: a-split-touching-at-a-notch-apex-mints-a-vertex-twice
kind: issue
title: pncad-py panics on a node refusal whose message carries a Debug role path, where it should raise
status: open
opened: 2026-10-10
priority: P3
cost: E
---

## What

A unit-height extrude of the notched pentagon `(0,0) (2,0) (2,2) (1,1)
(0,2)` split by the plane `y = 1` — which touches the notch's apex
`(1,1)`, leaving two runs above it — refuses at name emission with a
kernel-bug invariant: "the copy above the split of the start cap vertex
over the start of loop 0 step 4 … (role path `[OnToolVertex { side:
Above, of: CapVertex(Start, …) }]`) was minted twice". Measured from
Python on `intent/s3-a-poses` with the tool spelled both as a
`Node.datum_plane` and as a `PoseDef.in_frame` plane; the datum spelling
is unchanged by that branch.

Python then panics rather than raising: `pncad-py`'s prose gate
(`crates/pncad-py/src/py/mod.rs:667`, reached from `py/value.rs:139`)
finds the refusal rendered with `Debug` where its message belongs.

## Fix shape

Two defects: the split's emission names the apex's above copy once per
run, where two runs above one ON vertex need two names (the coincidence
row `crates/topo/src/splitting/mod.rs:717` records exactly this touch);
and the node-error message for an emission invariant carries a `Debug`
rendering of the role path. Reproduce:

```python
from pncad import Doc, Node, Formula, evaluate, m
doc = Doc()
frame = doc.sketch_frame()
pts = [(0, 0), (2, 0), (2, 2), (1, 1), (0, 2)]
sq = doc.insert(Node.polygon([(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in pts], plane=frame))
body = doc.insert(Node.extrude(sq, Formula.length_in(1, m)))
tool = doc.insert(Node.datum_plane(
    (Formula.length_in(0, m), Formula.length_in(1, m), Formula.length_in(0, m)),
    (Formula.literal(0.0), Formula.literal(1.0), Formula.literal(0.0)),
))
evaluate(doc).value(doc.insert(Node.split(body, tool)))
```

## Re-homed to BIND (2026-10-17, WIRE orchestrator)

The file held two findings. The naming half (the apex's copy minted twice)
is WIRE's `a-split-through-a-pinch-names-both-tip-copies-alike`, which now
cites this reproducer; the `[ev]` rule for it is PR 5258. What is left here
is pncad-py's: the prose gate (`crates/pncad-py/src/py/mod.rs`, reached from
`py/value.rs`) panics instead of raising when a refusal's message carries a
`Debug` rendering. Re-check it against the tree first — the role path's
spelling has changed since (`OnToolVertex` has no `side` any more) — and
whether the emitter's invariant text still renders through `Debug`. The id
is unchanged; retitled to the half that remains.

## The reproducer no longer reaches the gate (2026-10-18)

The naming half is built (WIRE, the Split vertex `Keeps` rule): the notched
pentagon split at `y = 1` now names with no refusal
(`crates/editor-core/tests/split_pinch_keeps.rs`, the same document built
in Rust; the Python snippet was not re-run), so the prose gate's panic
needs another refusal whose message carries a `Debug` rendering to
reproduce.

Signed (wire-keeps lane).
