---
id: the-cones-rung3-certificate-has-no-metres-enclosure
kind: issue
title: The cone has no implicit enclosure in metres, so every fitted or rational carrier on a cone pair refuses the rung-3 certificate's tube
status: open
opened: 2026-10-24
priority: P1
cost: M
refs: [torus-certificate-runs-root-free-through-its-quartic, offset-lane-has-no-conic-carrier]
---


Filed by OFFSET's conic designer pair (2026-10-24, `[ev]` PR 5652).

`crates/geom-brep/src/ssi/enclose.rs` `implicit_enclosure` (and
`has_implicit_enclosure`) serve the plane, the sphere and the cylinder
only; the cone and the torus refuse because "their metres forms carry a
`sqrt`". So `analytic_rung3`'s limbs (the sup-norm hull and the
uniqueness tube, `crates/geom-brep/src/certify.rs`, `EdgeCurve::certify`)
cannot certify any `Curve3::Nurbs` carrier on a cone pair. The only record
of the gap is `crates/geom-brep/src/intersect.rs`'s route notes, which name
cone×cylinder, cone×cone, cone×sphere and cone×torus as "blocked on the
cone's exact meters conversion"; no work item named it.

It may now be ordinary work: C9 admits `√` since Ev's ruling on #3517
(2026-10-01), and the torus's metres form turned out root-free
(`torus-certificate-runs-root-free-through-its-quartic`, on this slate).
Both designers judged the cone's the simpler of the two (likely, unread
in full). Check also whether the `intersect.rs` route notes are stale
after #3517.

Why P1: it is the shared blocker for every cone pair beyond plane×cone's
ellipse, and for draft's cylinder→cone arm. If Ev rules on PR 5652 that a
certificate's obligation follows the carrier's kind, the plane×cone
hyperbola/parabola arcs wait on it too. Filed by the OFFSET orchestrator.
