---
id: a-kill-bridging-joints-that-straddle-the-pole-lever-band-writes-a-reset-the-pass-decides-a-shift
kind: issue
title: A kill whose two bridged joints straddle the pole-lever band writes a reset where the pass decides a shift
status: open
opened: 2026-10-04
priority: P4
cost: E
refs: [a-kill-that-re-anchors-a-loops-first-leaves-its-rows-a-period-off-the-pass]
---

Found by the build of the re-anchor ruling (R, PR 4024), which made a
pcurve row an image plus a joint element (`crates/topo/src/joint.rs`).

A kill sums the elements of the two joints it bridges
(`JointElement::then`, called through `Body::bridged_joint` in
`crates/topo/src/euler.rs`). Where either summand is a reset (a joint whose
lever `pcurve_loop_pole_joint` did not decide definitely nonzero), the
sum is a reset. The pass decides the new joint's kind at the predecessor's
exit (`decide_joint` in `crates/topo/src/pcurves.rs`), which is where the
first summand was decided, not the second. So where the second summand's
lever (read at the killed half's exit, the same vertex within ε) decided
`Zero` or escalated and the first's decided `Positive`, the kill writes a
reset where the pass writes a shift, and tier 3 (check 4) reads the kind
mismatch as a `LoopDiscontinuity`.

It needs a lever inside the band at one reading of a vertex and definitely
outside it at the other, so on the sphere and the cone only, within ε of a
pole or an apex, where the arm varies with `v`; a cylinder's lever is the
same at every point. **Measured: 0 hits** (the build's probe over sweep's
`ci` profile and topo's suite, every complete periodic face after `kev`,
`kef`, `kef_minting` and `kemr`: all byte-equal to the pass).

Shape of a fix: the sum takes its kind from the first summand, which is
the pass's reading point, and where that is a shift and the second a
reset, the azimuth periods the reset dropped are unknown keys-only: the
kill would have to re-decide that one joint, which needs a band, or
refuse. Pin it with a cone joint at `v` inside the band before choosing.
