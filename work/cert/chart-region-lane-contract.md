---
id: chart-region-lane-contract
kind: ruling
title: ChartRegionLane's contract — is the census's minus-one-arm completeness sentence one Ev will have in the tree?
status: open
opened: 2026-09-04
needs_ev: true
refs: [CERT-M3, H5]
---


(S-CERT orchestrator) Filed at CERT-M3's PR (#1877), where the question is
argued in full under *A design question for Evan — `ChartRegionLane`'s
contract*. CERT-M2's census (PR #1559) found that `ChartRegionLane`, the
last of Track M's three lane traits, splits into a structural half and a
certified half ONLY with a new contract on the structural half, and that
the contract is a completeness claim about the coincidence census's own
coverage rather than a bound. CERT-M3 landed the row the census called
independently landable (the scalar-level absence split out as
`ValidationError::CensusLaneUnsupported`) and left the split itself here.

## The sentence

Splitting `ChartRegionLane` the way `EdgeNurbsLane` was split means
`census_and_certify` gains a structural half that runs every coincidence
arm except the conformal face-pair one, which needs the certified overlap
predicate. The sentence that half's doc would have to carry:

> *"The coincidence census, minus the conformal face-pair arm, is complete
> for the classes it does cover."*

That claims the census's remaining arms (the exact planar sweeps, the
vertex/edge/face coincidence arms, the declared-record confirm pass) are
individually complete over the classes they are written for, so dropping
the conformal arm subtracts exactly one class of contact and no part of
any other. The census's whole job is *no scan-to-bless in either
direction*, so a coverage statement about it is a claim about the kernel's
contact guarantee, not about a function.

A lane can verify the mechanical half (the conformal arm is the only arm
reaching `ChartRegionLane`; no other arm's result depends on it; the
structural half's error set is the composed one minus exactly the
conformal findings). A lane cannot verify the claim itself: whether the
covered classes are complete is a statement about the C9 exclusion ring's
missing first step (`CONTACT-DESIGN` C2 step 1) and about the same-solid
distinct-key and pure-tangency residues the census already names as open.

## The question

Is that sentence one you are willing to have in the tree as a public
door's contract?

- **Yes** → the split is mechanical and small; it becomes a unit on
  `H5` (Track M's remainder) with the sentence as its contract.
- **No** → `ChartRegionLane` stays until the exclusion ring lands, and
  `H5` says so.

## What this does not touch

`PcurveFittedLane`'s non-split (a representation question: what is a
fitted pcurve cache with no certificate) is `H5`'s other open question
and is not asked here.
