---
id: two-rims-read-coplanar-refuse-unsupported-or-untyped
kind: issue
title: Two ellipse rims read coplanar refuse at frontiers — CurvedPierceUnsupported when definitely apart, FallbackExtentUnsupported when one curve within the band
status: open
opened: 2026-10-14
---

Found by the body pin added on `cleave/carrier-cross-one-curve`
(`crates/sweep/tests/a_ruling_lying_on_a_wall.rs`
`two_rims_the_band_reads_coplanar_are_one_curve_only_within_it`),
measured at ε = 1e-9.

Two radius-0.01 rods on one axis, cut below planes through the origin
of slopes 100 and 100(1 + k·ε). The rims cross at `(0, ±0.01, 0)`;
their far ends lie k·ε apart, and the planes' tilt carries 0.01·k·ε, so
the transverse row reads the planes parallel at every k below 100.

1. **k = 30 and 80: a definite reading refused as unsupported.**
   `carrier_cross.rs` `one_curve` reads Positive (the rims are two
   curves), `meetings` answers `None` (no coplanar conic pair is held),
   and all six ops refuse `CurvedPierceUnsupported { face: FaceKey(4v1),
   edge: EdgeKey(11v1) }`. The rims definitely cross, so under the
   governing rule this is a missing capability (the coplanar
   ellipse × ellipse meeting in `meetings`), not a band reading.
2. **k = 0.1, 0.5 and 0.9: one curve, then refused as unsupported.**
   `one_curve` reads Zero: within the band the rims are one curve,
   which is a definite reading, not an in-band one. All six ops then
   refuse `FallbackExtentUnsupported` ("tangent or near-tangent
   carriers and no crossing layer saw an event"). A refusal after a
   definite reading is the kernel's missing capability, a frontier
   (PR 4335's rule), never the user's lever: the fix is to build the
   coincident-cut case, not to escalate it.

The pin asserts both refusals exactly; the fix updates it.
