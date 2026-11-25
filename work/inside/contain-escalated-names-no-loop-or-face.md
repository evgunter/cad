---
id: contain-escalated-names-no-loop-or-face
kind: issue
title: ContainError::Escalated drops the loop or face both its sources carried, so an in-band refusal of the face door names neither
status: open
opened: 2026-11-25
priority: P3
cost: E
---


Found in the style review of PR 7748 (`contain-error-drops-the-loop-its-carrier-named`,
lane `inside-contain-small`). That PR closed the shape for
`RayExhausted`, `Uncrossable` and `ArcEndClearOfCorners`; this arm is
the one left.

## The finding

`ContainError::Escalated { decision, escalation, diag }` carries no
loop or face. Both of its sources had one:

- `crates/topo/src/contain_loop.rs`, `From<PointInLoopError>`:
  `PointInLoopError::Escalated { r#loop, .. }` drops `r#loop`.
- `crates/topo/src/contain.rs`, `solid_err`:
  `PointInSolidError::Escalated { face, .. }` drops `face`.

The same holds for `PointInLoopError::write`'s `Escalated` arm
(`splitting/containment.rs`), which renders `decision.subject()` and
not the loop it carries.

## The two readings

- **The lane's argument (why it was left):** an escalation's subject is
  its decision. `placement_subject(decision)` names the question
  ("whether the point lies in the face"), the margin's payload names the
  predicate, and the lever is the decision's. The census routes it to
  `ValidationError::CensusEscalated { cause }`, which carries only the
  `Indeterminate`, and the boolean has its own escalation route. So
  neither consumer has anywhere to put a loop, and carrying one would
  change the census's escalation shape, not this conversion.
- **The reviewer's objection:** the enum's own invariant is that each arm
  names what stopped, for a consumer that renders it verbatim. Both
  sources had the key in hand and the conversion throws it away, which
  is the shape the PR closed for its siblings. "A consumer has nowhere
  to put it" is a reason to give the consumer somewhere, not to drop it.

## The repair

Carry the subject (loop or face) on `Escalated` and render it in
`ContainError::write`. Then decide whether `CensusEscalated` gets a
subject too, or whether the census names the face it is reading here
(`census.rs`, `read_containment`) the way its `CensusUnsupported` arm
does.
