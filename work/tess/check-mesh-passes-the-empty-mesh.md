---
id: check-mesh-passes-the-empty-mesh
kind: issue
title: validate::check_mesh answers Ok on a mesh with zero triangles
status: open
opened: 2026-09-18
---


Measured by the rim-only-cap survey (`tess/rim-only-cap-diag` at
`83833e586`): a two-cap sphere whose both faces emit nothing tessellates
(debug assertions off) to `triangles = 0`, and
`mesh::validate::check_mesh` returns `Ok(())` — watertightness holds
vacuously over no edges, and `signed_volume` is 0. `tessellate_impl`'s
census comment already says so ("`check_mesh` passes the empty patch")
as a reason the census exists; nothing records it as a question about
`check_mesh`.

The call: whether the validator's contract is "closed 2-manifold" (the
empty mesh qualifies, and the doc should say the empty mesh passes and
why) or "the mesh of a solid" (then zero triangles is a refusal with
its own name). TESS-1 removes the one known producer; this row is about
the validator's claim, which outlives the producer.
