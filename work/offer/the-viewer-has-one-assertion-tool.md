---
id: the-viewer-has-one-assertion-tool
kind: unit
title: viewer: one assertion tool authors a claim over a clearance, Gap or Distance measure between two picks
status: closed
opened: 2026-11-22
priority: P1
cost: M
pr: 7348
branch: offer/the-viewer-has-one-assertion-tool
closed: 2026-11-22
---

The first half of the decided offer design (`a-refusal-offers-no-action-in-the-viewer`, `## Design` and `## Decided`; Ev, #7341). Today the viewer's only assertion door is `SessionOp::AddClearance`. A `Claim::Clearance` cannot quiet any check, because [assertions.quiet] accepts only a `Claim::Value` over a `Gap` or `Distance` output `= 0`. So "add an assertion", the advice the checks give, has no door.

## The work

- **One session op and one tool** for authoring an assertion. It takes two picks, a measure (clearance, `Gap` or `Distance`), a relation and a bound. It writes the kernel's existing node vocabulary: the selections, the measure node, and `Node::Assertion` with the claim. Each is one action and one undo.
- The existing clearance tool becomes this tool with the clearance measure chosen. There must not be two assertion tools.
- The op takes its arguments as values, so the offers unit can fill them from a finding. The check's Assert button is this op, pre-filled.
- Rows:
  - an assertion authored over `Gap = 0` between two coincident faces quiets the at-rest coincidence finding at that site;
  - a clearance assertion does not;
  - every op, undone, leaves no trace.

Whether `Claim::Clearance` and `Gap = 0` should both exist is `work/clear/a-clearance-claim-and-a-zero-gap-value-say-one-thing-two-ways` (open). This unit builds the tool over the claims as they are.

## Closed (2026-11-22, PR #7348)

`SessionOp::AddAssertion { a, b, claim }` is the viewer's one assertion door. A claim is either a clearance with its bound shape, or a `Value` over a kernel `MeasureVerb` (Gap or Distance) with a relation and a bound. It writes what the kernel's own measure and assertion doors write, as one action. The clearance tool is that tool with clearance chosen.

A `Read` seat takes an operand as the measure reads it. A finding's raw cell names are ⊔-keyed and are NOT that. Fill the op only from the kernel's minted value (`Quieting`), as recorded on `a-refusal-offers-no-action-in-the-viewer`.
