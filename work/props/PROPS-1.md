---
id: PROPS-1
kind: unit
title: The lost-correlation members of the linalg audit — mirror_across_plane and reject_from respelled, one re-baseline pass
status: dispatched
parent: certified-lane-non-real-contract-audit
branch: props/1-linalg-lost-correlation
opened: 2026-09-05
refs: [certified-lane-non-real-contract-audit, 1277, 1143]
---

Spec: `docs/PROPS-1-SPEC.md` (binding; deleted at landing per the spec
lifecycle). The first unit of the linalg interval-honesty lane, cut
ahead of S-CERT's exit because no live PR touches
`geom-core/src/linalg/{frame,vec,point}.rs` (measured against #1877,
#1879, #1828 and #1617 at dispatch) — `work/props/log.md`, the early
opening entry.

What: `frame::mirror_across_plane`'s translation respelled through the
Householder identity so the anchor is mentioned once;
`Vec3::reject_from` respelled through the triple cross so `self` is
mentioned once; `Point{2,3}::lerp` decided and left with its `Interval`
cost stated at the site. Both respells move `f64` bits, so the unit
takes the golden / k-lint / render accounting both members owed, once.
Measure first (an evidence instrument in `cert3_evidence.rs`'s shape),
soundness pins in both lanes, drift accounted never absorbed.

Not here: the `rotation_about` diagonal floor
(`rotation-about-diagonal-width-floor`), the `orthonormal_basis` sign
hull (`interval-orthonormal-basis-sign-hull`, the next unit on the
file), `normalize`'s overflow (S-CERT's until the inheritance).
