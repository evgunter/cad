---
id: an-iso-arc-certificate-never-checks-the-edge-span-lies-in-its-arc
kind: issue
title: The IsoArc certificate never checks that the edge's span lies in the image's arc
status: open
priority: P3
opened: 2026-10-15
---


`PcurveCache::certify` on a `Pcurve::IsoArc` (`run_iso_arc_checks`,
`crates/geom-brep/src/pcurve_cache.rs:6337`) bounds
`sup_{g∈[0,1]} |B(g) − Ĉ(g)|`: the wall's boundary row `B` against the
circle's rational-quadratic form `Ĉ` over the image's own arc
`[at0, at0 + angle]`. For `t` on that arc, `S(P(t)) = B(g(t))` and
`Ĉ(g(t)) = C(t)` hold exactly, so the envelope bounds the edge's miss
over the image's arc — and over any sub-span of it (sound, loose).

Nothing checks that the edge's `[t0, t1]` lies inside `[at0, at0 + angle]`.
Off that arc, `iso_arc_g` (`pcurve_cache.rs:632`) clamps the
sub-arc locate, so the envelope covers nothing there. A stated image
SHORTER than its edge is then caught only by check 3's samples (the
miss grows toward the sampled endpoint: a probe in the #5198 review of a
π/2 edge with a π/4 stated image refused `MapResidual`), not by a
certified bound.

Older than #5198, which opened a public door to it:
`EdgeCurve::certify_via_stated` (`crates/geom-brep/src/certify.rs:1580`)
and `recertify_via_stated` certify any stated `IsoArc` as it stands, and
`plane_nurbs_incidence`'s doc cites this item.

**Fix:** a decided `[t0, t1] ⊆ [at0, at0 + angle]` at the head of the
`IsoArc` checks (with ε headroom, metered in metres along the arc), its
definite failure a typed refusal; a row stating a shorter image.
