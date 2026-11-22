---
id: a-clearance-claim-and-a-zero-gap-value-say-one-thing-two-ways
kind: issue
title: a Clearance claim and a Value claim over a Gap measure both say 'the gap between these', and only the second quiets a check
status: open
opened: 2026-11-22
priority: P2
cost: M
design: true
---

Found by OFFER's designers (2026-11-22), who were weighing `work/offer/a-check-offers-no-way-to-add-the-assertion-it-asks-for`. Both checked it in the tree.

The document has two ways to assert something about the gap between two things:
- `Claim::Clearance`, which is what the viewer's only assertion door, `SessionOp::AddClearance`, writes;
- `Claim::Value` over the output of a `Gap` or `Distance` measure.

The at-rest checks accept only the second as quieting a coincidence: see [assertions.quiet], and `checks::at_rest::quiet_row`, which matches it. So a person who answers a check's "assert the coincidence" with the clearance tool writes an assertion that leaves the finding loud.

The question is whether both claims should exist, and if so, what each means that the other does not. It may be a `[one-way]` question for Ev. OFFER's offer unit will build a general add-assertion door over the measure claims. It needs this answered to know whether clearance is one of that door's measures or a separate claim. (OFFER orchestrator)
