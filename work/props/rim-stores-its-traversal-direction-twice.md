---
id: rim-stores-its-traversal-direction-twice
kind: issue
title: Rim stores the traversal direction twice (d_u: T and d_u_sign: Sign) and du_of_rims compares the exact one through the tolerance funnel
status: open
opened: 2026-09-11
refs: [877, S40]
---


## Carved out of `S40` (2026-09-11, by the WIRE orchestrator)

`S40` ("Residue and editing artifacts", accepted by Ev 2026-08-18)
was one file carrying four unrelated residues on three programs'
territory. WIRE inherited it whole in the cut of 2026-09-11 and owns
only one of the four; the rest are filed where the code lives, per
`work/README.md` ("when the owning program is clear, file the item
straight onto that program's slate"). `S40` keeps its id and its
`WitnessSlot` row; this is the third of its four bullets, verbatim
below. **Nothing about it was re-judged in the move** — the citations
are `S40`'s, written against a tree `#877` has since moved, and the row
was last read on 2026-08-18.

## Finding (`S40` bullet 3, verbatim)

- **Confidence**: sure

`Rim` stores the same traversal direction twice (`d_u: T` and
`d_u_sign: Sign`), and the exact one is compared through the tolerance
funnel — subtracting two exactly-±1 values and banding a result that is
always 0 or ±2 (`props/curved.rs`, `Rim`'s two fields and `du_of_rims`'
`props_rim_dir_group` decide — cited by target name per **S176(a)**; the
line numbers were written against a tree #877 moved). **STILL OPEN** —
which of the two representations is authoritative is a design call. #877
did not touch it: the margin is now levered at `RimArms::azimuth` rather
than a bare `arm`, which is the same comparison at a named lever.

## Why it is PROPS's

`crates/geom-brep/src/props/*` is PROPS's glob and no other open program
claims it, and "which representation is authoritative" is a call about
this program's own data, not a ruling to escalate. Worth reading beside
its sibling `rim-level-rule-manufactures-its-error-by-feeding-nan-into-classify`
— both are `props/curved.rs` and both came off `S40`.
