---
id: a-certifying-scalar-holds-an-option-of-a-lane-it-has-no-constructor-for
kind: issue
title: At Interval, Sym and Probe three lanes are an Option of a type with no constructor there, answering None at run time
status: open
opened: 2026-11-20
priority: P2
cost: M
parent: from-computed-claims-a-zero-tangent-at-a-dual
design: true
---


From the review of PR 7190 (both reviewers), 2026-11-20. A design question for the owner, not a defect in what the PR built.

## What

DL5.3 says a scalar's lanes are "an associated type with no values at a dual, never an `Option` a dual answers `None` to". PR 7190 built that for a dual. At the other scalars the same pattern is still there for the lanes whose derivation is written at one scalar only:

- `OffsetFitLane`, `SectionLane`, `SsiLane`: constructed at `f64` alone (`OffsetFitLane::fit`, `SectionLane::f64`, `SsiLane::marched`). `impl AtRestPolicy for Interval`, `Probe` and `Sym<T>` declare `type SsiLane = SsiLane<Self>` (and the other two) and answer `None`: an `Option` of a type that has no constructor at that scalar.
- `ClearanceLane`: `None` at `f64`, `Some` at `Interval` and at `Sym` over a base that has it.

The mechanism the PR built can say each absence by type (`type SsiLane = NoLane` at `Interval`). The rule does not say whether it should: `NoLane` is documented as what a scalar that does not certify holds, and `Interval` certifies. The policy trait's own doc calls these `None`s "a statement about the derivation", a different fact from certification rights.

## The question

Is "this lane's derivation is not written at this scalar" to be a type fact, as "this scalar does not certify" now is? If yes, by which type: `NoLane` (and its doc widens), or a second uninhabited type that says "not derived here".
