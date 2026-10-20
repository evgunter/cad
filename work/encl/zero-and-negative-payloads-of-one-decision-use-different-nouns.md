---
id: zero-and-negative-payloads-of-one-decision-use-different-nouns
kind: issue
title: geom-brep: IntervalNotForward's Zero and Negative payloads use different nouns and signs; the torus ring/spindle and TubeNotSeparated 'along the whole edge' payloads are worded apart from their siblings
status: closed
pr: 5437
opened: 2026-10-19
priority: P4
cost: E
branch: encl/one-noun-per-decision
closed: 2026-10-20
---


(Filed by the ENCL orchestrator from the PR 5157 delta review.)

## What

These payloads in `crates/geom-brep/src/certify.rs` (`impl Display for CertifyError`) word one decision's arms apart:
- **`IntervalNotForward`.**
  - The Zero arm reads "the stored parameter interval is not shown to run forward (margin … m)": a signed margin, noun "parameter interval".
  - The Negative arm reads "the stored interval runs backwards by … m": a positive amount (`readable_below_zero`), noun "interval".
  - So one decision prints its margin with two signs and two nouns. The step-import `tier_gate` pins print "(length … m)" for the told form, a third noun.
- **The torus payloads** (search "ring" and "spindle" in the same Display). They name the torus's parts in words their siblings don't use.
- **`TubeNotSeparated`** says "along the whole edge" where its siblings name a sample.

## Repair shape

One noun, and one sign convention for a decision's margin, across its arms. That means either the signed margin everywhere, or the amount everywhere with the noun saying which way.

The class: for every `CertifyError` variant with more than one refused arm, read its arms' Displays side by side. Put the hit list in the PR.

## Evidence, 2026-10-19 (PR 5365)

`crates/geom-brep/src/certify.rs` now spells the forward-span sentence twice:
- `CertifyError::IntervalNotForward`'s Display (~717): "the stored parameter interval is not shown to run forward (length {} m)";
- `StatedSpan`'s Display (~1113): "{given} is not shown to run forward (length {} m)".

The two templates differ only in their subject. Give the sentence one home that takes the subject. (ENCL orchestrator)
