---
id: torus-windows-a-hair-apart-in-tube-angle-are-undecided-at-the-certified-scalar
kind: issue
title: Two torus windows a hair apart in tube angle are refused undecided at the certified scalar (1e-6, 1e-12)
status: open
opened: 2026-11-01
priority: P1
cost: M
refs: [validate-tier3-curved-boundary-containment]
---

## What

The region arm refuses a valid body at the certified (interval) scalar.
On the donut's outer torus face, plant two windows over one azimuth span,
mirrored about the face's middle in tube angle: `((-0.5, 0.5), (-1, -g))`
and `((-0.5, 0.5), (g, 1))`. For `g` ≤ 0.003 rad, tier 3 refuses the face
`CycleSideUndecided` at ε 1e-6 and 1e-12. The margin is
`bool_torus_region_span`, an enclosure over about `[-3.06, 4.22]`, at the
gap-side vertices of one window read against the other window's parallel
edge. At the default ε (1e-9) the same bodies validate, for every `g` in
`{0.001, 0.003, 0.01, 0.03, 0.05, 0.1}`. At f64 they validate at every ε,
except `g` = 0.003 at 1e-6, where the gap is within the band and the refusal
is a correct in-band escalation (`bool_torus_region_roots_extreme`, margins
about 4.5e-6).

Found by the re-check of PR 6163 at `8f9aea64`, probe
`recheck_e1_close_detail` (branch `restcurve/review-quill-6163-recheck` @
`01afd3c7`, `crates/sweep/tests/review_recheck_probe.rs`). The reviewer
reports the same enclosure on the base. The fix that settled the
re-check's other finding, the span-before-component fallback in
`crates/topo/src/boolean/torus_region.rs` (`landings`, the meridian-leg ×
parallel-edge arm), does not settle this one.

## Why it is not wrong, only unread

The answer is an escalation, never a wrong side: the arm refuses
`CycleSideUndecided` with its margin (D4 ¶1). What is wrong is that a body
with a gap of 1 to 3 mrad (far wider than K·ε at 1e-6 or 1e-12, which are
1e-5 and 1e-11) goes unread at the certified scalar, while 1e-9 reads it.

## Where to look

The span reading's enclosure is a whole turn plus, so the azimuth `θ` of
some root on the other window's parallel edge is not certified. Its point
lies close to that edge's plane, where `atan2` of the point's offset from
the parallel's centre is poorly conditioned. Likely the root's leg
parameter is a near-double root: the leg passes the gap's tube angle
almost tangentially to the plane. Then the fix is in how
`first_harmonic_leg_roots` encloses a near-tangent pair, or in aiming the
paths off the gap's tube angle. Why 1e-9 settles while 1e-6 and 1e-12 do
not has not been traced: the aimed targets are ε-dependent.

## Not pinned as a row

Today's refusal holds at 1e-6 and 1e-12 and not at the default ε, so a row
pinning it would assert a different outcome per ε for the same body. The
fix's row is the probe's case asserting `Ok(())` at every ε. Until then the
probe on the review branch reproduces it.
