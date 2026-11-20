---
id: encl
kind: program
title: ENCL — certified enclosures: what a certificate claims, and what it is worth
status: ready
opened: 2026-09-20
group: kernel/geom/numeric
prefix: encl/
tag: (ENCL orchestrator)
ab_band: 5800-5899
paths: [crates/geom-brep/src/offset_fit.rs, crates/geom-brep/src/patch_bound.rs, crates/geom-brep/src/offset_meters.rs, crates/editor-core/src/stackup.rs]
keep_out: [CUT FROM PROPS 2026-09-20 with Ev's agreement in chat — S-CERT's ground and this program is its second successor (S-CERT exited into PROPS at #1924; PROPS cut this half out when its slate passed sixty rows); the certificate TYPES this program seals are spread over topo::props, pcurve, certify, ssi and offset_fit, so their owners' files are reached by announced seam and the forgeable-fields row names each one; crates/geom-core/src/* stays PROPS' including ring_interval.rs, which every bound here reads; SHARED GROUND IS EXPECTED and is not a conflict (Ev, in chat, 2026-09-20) — awareness when a lane is live on the same file is what the two programs owe each other, STACK merged in on 2026-11-20 (docs/doc-ledger/stack-merges-into-encl.md) - it held no P0 or P1 row, from STACK: opened by ENCL's 2026-09-20 priority-seam cut (Ev, from STACK: in chat) per work/README.md Track size - ENCL measured 40.5 budget points and was cut into tracks meant to run in PARALLEL (Ev, from STACK: in chat: for these high priority tracks it is ideal to have several components that can be worked on in parallel), from STACK: the siblings are ENCL STACK and they share their parent's territory by design - shared ground is legitimate by the README's 2026-09-20 rule and what is owed is awareness while a lane is LIVE, from STACK: so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled rather than drawing a fence, from STACK: PROPS owns the analysis lane and the flux arms, from STACK: QUAD owns props/quad/, from STACK: SSI and PCERT own the geom-brep certification lanes]
---

**The offset fit and the patch bound, after the 2026-09-20 cut**: the
arithmetic of a certified enclosure, as opposed to what the
certificate claims.

`TangentParallel` admits a 90-degree crossing described as a tangent
intersection — wrong in the accepting direction. At eps 1e-12 the
offset fit refuses EVERY curved NURBS chart. `patch_bound`'s rational
arm hulls the f64-refined net, so the described patch escapes its own
bound. And tightening a cell bound can RAISE the door's `hull_sup`,
which PROPS' mignitude-floor unit established and left here: the
door-level bound is not monotone in the cell bound, so "refine until it
fits" is not a strategy.

ENCL was cut on 2026-09-20 (Ev, in chat) from 40.5 budget points into
STACK (what the certificate claims) and this remainder. ENCL keeps its
band 5800-5899.

Charter and order: `work/encl/plan.md`; narrative in `work/encl/log.md`.

## From STACK (merged 2026-11-20)

STACK — what a certified enclosure CLAIMS: the stackup's hull, the non-real contract and the forgeable certificate family

The contract half of certified enclosure, split from the arithmetic
half ENCL keeps. P2 because these rows serve `docs/DESIGN.md`'s reach
goal — error propagation, distributions over model parameters — rather
than anything shipping today.

A stackup's certified worst-case hull pads by the LEAF width where the
lane's own width is what it owes; E5's contribution bounds from
`Dual<Interval>` derivative enclosures are unbuilt; and nothing states
what a certified lane owes when a value goes non-real, which is a
contract audit rather than a defect.

The fourth row is the latent one: `MassProperties` and the certificate
family — `PcurveCertificate` and four siblings — have **public fields**,
so a downstream crate can construct a certificate that was never
earned. P3 on its own terms (nothing forges one today) and here because
it is the same question: what a certificate's existence is evidence
of.

Charter and order: `work/encl/plan.md`; narrative in `docs/doc-ledger/stack-merges-into-encl.md`.
