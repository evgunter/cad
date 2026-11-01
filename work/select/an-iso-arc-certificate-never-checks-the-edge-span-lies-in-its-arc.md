---
id: an-iso-arc-certificate-never-checks-the-edge-span-lies-in-its-arc
kind: issue
title: The IsoArc certificate never checks that the edge's span lies in the image's arc
status: closed
priority: P3
opened: 2026-10-15
closed: 2026-10-23
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

## Closed

`iso/arc-rim-sub-arc` (ISO, the IsoArc half of
`iso-derivation-arms-assume-an-edge-spans-the-charts-whole-domain`), as a
matter of course: that change mints images whose arc is wider than their
edge's span (a split rim's child takes the row over the row's own arc),
so the certificate has to decide the span lies in it. `run_iso_arc_checks`
now decides `at0 ≤ t0` and `t1 ≤ at0 + angle` after check 2's forward
interval, each metered by the radius (`pcurve_iso_arc_span`); a span in
the band of an arc end reads as on it, a definite miss refuses
`IsoUnsupported { "an arc rim whose edge runs past its image's arc …" }`,
and an undecided one escalates at `ParamSpan`. Pinned by `geom-brep`'s
`onpair_plane_nurbs::an_iso_arc_refuses_a_span_past_its_arc` (a span
inside the arc certifies; one reaching before its start or past its end
refuses typed, before its samples are read).
