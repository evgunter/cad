---
id: a-split-touching-at-a-notch-apex-mints-a-vertex-twice
kind: issue
title: A split whose plane touches a notch's apex mints the apex's above copy twice, and Python panics on the refusal
status: open
opened: 2026-10-10
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
