---
id: the-fresh-certificate-helper-recertifies-without-the-stored-image
kind: issue
title: assert_certificates_fresh re-certifies without the edge's stored image, so its fresh run meters the ends only and equals the mint's certificate just while the chart residual is exactly zero
status: open
opened: 2026-11-20
priority: P3
cost: E
---


Found by ONEROUND's `oneround/anchored-conic` lane (2026-11-20), tracing
why two rows moved under a variant that evaluates every conic from a
point on its arc.

`crates/sweep/tests/common/certificates.rs`, `assert_certificates_fresh`,
compares an edge's stored certificate with `EdgeCurve::recertify(start,
end, …)` run with no stored image. The mint's run meters the chart arm
of check 4 (`geom-brep/src/certify.rs`, `pcurve_map_residual`:
`p.distance(surface.eval(chart.pcurve.eval(t)))`, the carrier's point
against the surface's point of the edge's image); the helper's fresh
run printed no chart-residual line and meters the ends only.

The two agree today because that residual is exactly zero on these
fixtures: a circle's image on a plane (`Pcurve::Harmonic`, `Free { c, a,
b }`) and a meridian's on a sphere are the carrier's own centre-form sum
term for term. Where the residual is not zero the helper reports a
mismatch that no door caused. Measured under the variant, on the
fixture drum before the void door or `revert` touch it (edges 6v3 and
8v3): stored 8.04e-17 and 5.55e-17 at r = 0.5, fresh 6.12e-17
(`r·sin π`); the snowman's meridian stored 2.22e-16 at r = 1, fresh
1.22e-16. The rows that read it:
`revert_plane_charts::carried_cavity_certificates_equal_a_fresh_recertification`,
`snowman::the_fallback_assembly_carries_the_kept_operands_certificates`.

So the helper's claim, "the stored certificate is the fresh run's", is
checked against a run that meters less than the mint did. Fix: the
fresh run re-certifies with the edge's stored image, as the mint does
(or the helper says it compares the end checks only). Not traced: which
`recertify` door takes the image, and whether production re-certifies
without it anywhere.

P3 by `work/README.md` Priority (a guard that cannot see what it
guards); E.
