---
id: a-plane-doors-uncertified-root-is-split-at-with-no-re-read-of-its-height
kind: issue
title: A conic's plane-door root is split at with no re-read of its height off the plane; only the edge certificate's size limit stands behind it
status: open
opened: 2026-11-23
priority: P3
cost: M
parent: the-certified-lanes-harmonics-of-a-conic-are-formed-about-its-centre
refs: [7510]
---


Found 2026-11-23 by the conic-harmonic measurement lane's class sweep (`analysis/oneround/conic-harmonic/sweep.md` on `oneround/conic-harmonic-probe`, `78dbbbcf1`). Latent: no body was built wrongly through any door probed.

`curve_face::pose::plane_conic` (the plane door, `plane_crossing_lane`) answers a conic's crossing of a plane as an uncertified closed form, and its doc says the root "is not a point certified on the plane". Near the anchor of an ellipse held on its arc the root's height off the plane, judged against the stored curve at 60 digits, is 3.5e-10 m at a semi-axis of 1e6 m, 2.7e-9 m at 1e7, 3.1e-8 m at 1e8 and 4.6e-7 m at 1e9: inside ε at 1e-9 up to 1e6, in the band's gap at 1e7, past K·ε from 1e8. Posed about the stored point the same root lands at the parameter's last bit (the lane's M1).

Its callers act on the root without re-reading it:
- `topo::boolean::reduce` (the conic-against-plane arm): `p = carrier.eval(t)`, `contfp_decided` places `p` in the face (it takes a point "already on the plane"), and on In, OnEdge or OnVertex the edge is SPLIT at `t`. The vertex is on the edge exactly and off the plane by the root's height.
- `topo::boolean::factored_crossing`: roots against two planes, a node where two are within the band, then `landing(y, face, carrier.eval(t))` and a `CurvedEvent::Pierce` at `t`; no height re-read found.
- `topo::splitting`, through `curve_face::cross()`: the same pose; not read further.

**What keeps it right today is elsewhere.** End to end through `split` twice, at the largest sections that build (semi-axes 6.1e3 and 6.1e4 m), every built vertex stands within 0.06 ε of the true crossing. Larger sections are refused at their mint by the edge certificate's `KernelLimit`, at about the size where the root's height would reach ε. A door that mints such an edge without that certificate has nothing between the root and a built vertex.

`ring_path::plane_roots` (uncertified roots up to 2.6e-7 m off at 1e6) has one caller, `path_parity`: each root is read in or out of the arc's span and flips a parity; no point is built. Whether a parity can flip on a root that far off was not probed (the operand is the same far-section body, buildable only to about 6e5).

Closes with the certified-lane row's conversion of these readers to the stored point, or sooner with a re-read of the root's height where the edge is split.

## Moved to CONIC (2026-11-23)

Moved from ONEROUND by `git mv` when that track was split on Ev's answer to PR 7523 (`work/conic/log.md`); the id and the body above are unchanged.
