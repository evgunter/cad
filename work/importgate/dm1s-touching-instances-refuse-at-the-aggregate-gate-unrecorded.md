---
id: dm1s-touching-instances-refuse-at-the-aggregate-gate-unrecorded
kind: issue
title: dm1's bolts and nuts touch the l-bracket, and the assembled body's at-rest gate refuses their shared vertices as unrecorded contacts, naming no solid
status: open
opened: 2026-11-04
---


Filed by the PCERT main-reds lane (`pcert/main-reds-e2-e5`), whose fix
let dm1's arc rims mint (a row at end weights 1 and 2 holds its rim,
`geom_brep::Pcurve::IsoArc`'s `ends`). That moved
`tests/fixtures/wild/stepcode/dm1-id-214.stp` one door further.

## Measured

The l-bracket component alone now imports
(`r1_dm1_probe::the_l_bracket_alone_adopts_its_reversed_slit`, census
`(1, 1, 10, 21, 14)`). Its volume enclosure
`[3.43388e-6, 3.43846e-6]` m³ holds an independent mesh volume of
`3.43577e-6` m³, from a 1e-5 tessellation that admesh reads as
watertight. Full dm1 (one bracket, three bolts, three nuts, seven placed
instances) refuses at every ambient band (1e-9, 1e-6, 1e-12) in
`step_import::gate3_findings` (`crates/step-import/src/lib.rs`), the
aggregate gate over the assembled body:

    step import: the assembled body is a body the kernel's shared
    at-rest validation gate refuses (266 verdicts): UnrecordedContact {
    contact: VertexVertex { a: VertexKey(2v1), b: VertexKey(41v1) },
    witness: "(0.005055109324576802, 0.010000000000000005,
    0.06500000000000003)", margin: … 5.12e-17 … } — two vertices at one
    position at (…) is an unrecorded contact. Recourse: move the geometry
    so the faces clearly touch or clearly part; …

The 266 verdicts are `UnrecordedContact` (`VertexVertex`, then
`VertexOnEdge`, …, at margins of 1e-18 to 1e-16) and
`CensusUndecidable`. Their witnesses sit where the bolts and nuts seat
on the bracket.

## The questions

1. Instances that touch by design are the normal case for an assembly.
   `gate3`'s doc says the second pass "confirms the records the first
   one's touches minted" (`topo::record_at_rest`), yet these contacts
   reach the findings unrecorded. Either the record pass misses this
   touch class between instances, or the file's placements leave the
   touches inside the band without being Zero. Measure which before
   changing anything.
2. The refusal names no file entity. `TierInvalid { solid: None, … }`
   carries kernel keys and a witness point, and its recourse is to move
   the geometry, which an importer cannot do for a file's own
   placements. `r1_dm1_probe` used to assert that every typed refusal
   names a `MANIFOLD_SOLID_BREP`. It now pins `solid: None` as the
   aggregate gate's shape, citing this row.

Reproduce:

    cargo nextest run -p step-import -E 'test(r1_dm1_probe)'
