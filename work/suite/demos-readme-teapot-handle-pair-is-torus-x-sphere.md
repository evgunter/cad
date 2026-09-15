---
id: demos-readme-teapot-handle-pair-is-torus-x-sphere
kind: issue
title: demos/README.md says the teapot's handle union refuses on torus x cylinder; the pot's belly is a sphere
status: open
opened: 2026-09-15
---


## What

`demos/README.md`'s `teapot` row says the handle union refuses
*"`CurvedPairUnsupported` on a face-kind pair (torus × cylinder)"*. The
pot's belly is a SPHERE — the scene's own source says so at the pin
(*"The pot's belly is a SPHERE now, not the squared pot's cylinder"*)
and the probe prints the payload it pinned:

```
wall 2 — join the handle to the vessel ...: REFUSED TYPED,
CurvedPairUnsupported { op: None, operand: B, face: FaceKey(3v1),
kind: Torus, other_face: FaceKey(3v1), other_kind: Sphere }
```

(observed running `teapot::wall_probes` under D403's new in-bin test).
So the README names a pair the model does not have. The neighbouring
claim about the spout — past the pair rung, dying at
`CurvedEdgeUnsupported` on the canal's own seams — is correct and the
same run prints it.

## Fix

One word in the prose, but the row is long and its author may want to
re-read the whole sentence against the current scene rather than take a
substitution on trust. Found by D403's sweep; not fixed there because
the prose is not what that unit touched.

## Ground

`demos/` (Rust and Markdown) is in no open program's `paths` — Track X
left the tracker on 2026-09-11. Filed beside D403, which is SUITE's
other demos row.
