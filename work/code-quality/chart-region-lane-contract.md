---
id: chart-region-lane-contract
kind: ruling
title: ChartRegionLane's contract — is the census's minus-one-arm completeness sentence one Ev will have in the tree?
status: open
opened: 2026-09-04
needs_ev: true
refs: [H5, 1877, 1878]
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


## Corrected at CERT-M3's dual (2026-09-05)

The census's premise — *"the conformal face-pair arm is the only arm
reaching `ChartRegionLane`"* — was FALSE, found by the unit's first
blinded reviewer and confirmed at adjudication. The trait has two methods
(`chart_overlap`, `declared_overlap`; `crates/topo/src/chart_region.rs`)
and three consumers in `crates/topo/src/census.rs`:

| arm | method | what a structural half would do |
|---|---|---|
| conformal face-pair sweep (`census.rs:1627`) | `chart_overlap` | go silent — no conformal pair is examined |
| declared-record confirm pass (`:2777`) | `declared_overlap` | go silent — no `PatchContact` record is confirmed, so a stale one is not reported either |
| edge-edge crossing backing (`:1366` via `pair_region_verified`, `:511`) | `declared_overlap` | fold to `false` — every consulted pair reads as unverified, so crossings a declaration DOES back are refused as if it did not |

The third arm is what makes this a contract question rather than a
refactor: a structural half's error set is not "the composed one minus
the conformal findings" — it also GAINS findings, because unbacked
crossings stay loud by design. The mechanical half (which arms reach the
trait, that no other arm depends on them, the error-set difference) is
now verified and is the table above; the completeness claim itself is
not a lane's to verify.

## The sentence

The one a structural half's doc would have to carry is closer to:

> *"The coincidence census, minus the conformal face-pair arm and the
> declared-record patch confirm, is complete for the classes it does
> cover; and a crossing whose backing pair could not be verified is
> refused rather than passed."*

It claims that the census's remaining arms — the exact planar sweeps,
the vertex/edge/face coincidence arms, the curve-record confirm — are
individually complete over the classes they are written for, so dropping
two arms subtracts exactly two classes of contact and no part of any
other; and that the third arm's conservative direction is the sound one
to be left in. The census's whole job is *no scan-to-bless in either
direction*, so this is a claim about the kernel's contact guarantee, not
about a function — a statement about the C9 exclusion ring's missing
first step (`CONTACT-DESIGN` C2 step 1) and the same-solid distinct-key
and pure-tangency residues the census already names as open.

## The question

Is that sentence one you are willing to have in the tree as a public
door's contract?

- **Yes** → the split is mechanical (three arms, two methods); it becomes
  a unit on `H5` (Track M's remainder) with the sentence as its contract.
- **No** → `ChartRegionLane` stays until the exclusion ring lands, and
  `H5` says so.

## What this does not touch

`PcurveFittedLane`'s non-split (a representation question: what is a
fitted pcurve cache with no certificate) is `H5`'s other open question
and is not asked here. A third question from the same unit — whether the
certified form of the at-rest doors should become their DEFAULT name,
evicting `Body<Dual64>` from `validate_pseudomanifold` — lives on
`work/cert/lane-keeping-at-rest-doors-skip-the-m7-8-class.md` (PR #1877)
and is not asked here either.
