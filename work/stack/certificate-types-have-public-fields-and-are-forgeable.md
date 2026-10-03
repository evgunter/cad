---
id: certificate-types-have-public-fields-and-are-forgeable
kind: issue
title: MassProperties and the certificate family (PcurveCertificate, certify::Certificate, SsiCertificate, OffsetCertificate, CertifiedLeaf) have public fields — a downstream crate can forge a certified claim nothing computed
status: open
opened: 2026-09-08
refs: [2134]
priority: P3
cost: D
---

Found by BOOL-9's second review (PR 2134, R2 MINOR-3) and filed by the
S-BOOL orchestrator on PROPS's slate because `topo::props::MassProperties`
is the headline case: four `pub` fields, so a crate with no features
compiles `MassProperties { volume: 1.0, surface_area: 1.0, volume_pad:
0.0, area_pad: 0.0 }` — a zero-width certified enclosure around a
volume nothing computed. The same shape recurs across the certificate
family: `geom_brep::PcurveCertificate` (5 pub fields),
`geom_brep::certify::Certificate`, `SsiCertificate`,
`geom::OffsetCertificate`, `editor_core::CertifiedLeaf` (owners vary;
PROPS holds the props one and is asked to route the rest or split this
item). BOOL-9 shut the analogous door on the profile vertex table (a
forgeable CACHE is a wrong shape); a forgeable CERTIFICATE is a wrong
claim, which is worse. What a unit decides: private fields with a
crate-internal mint per certificate (the `ValidatedProfile` precedent),
a dev-only door for the fixture writers if any exist (survey first), and
the consumers kept bit-identical. Difficulty M (survey-first).

## A sibling door (ENCL, 2026-09-25)

`geom::ApproxSurface::certify(spec, certifier)` is public and takes any
closure returning an `OffsetCertificate`, so the type's own doc sentence,
"an uncertified approximating surface is unrepresentable", holds only as
"one no certifier has seen": a caller can hand in a closure that returns
whatever certificate it likes. Tier 3's re-derivation is what keeps a
forged one from passing validation. Found by an ENCL designer lane
reading `crates/geom/src/surfaces/approx.rs`; no probe was run. The same
shape as this row, one door over.

Signed: (ENCL orchestrator)
