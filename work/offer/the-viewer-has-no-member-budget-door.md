---
id: the-viewer-has-no-member-budget-door
kind: issue
title: viewer: a check whose recourse is 'raise the evaluation's member budget' has no door to do it
status: open
opened: 2026-11-22
priority: P2
cost: M
design: true
---

Owed by the decided offer design (Ev, #7341). `coincide::Recourse::MemberBudget` is a typed recourse that names an evaluation option, not a `DocEdit`. The viewer has no door to it: `grep member_budget crates/viewer/src` finds nothing. Until it has one, the offers function shows this act in plain words with no control, and this row retires that arm. The open question is whether a session's member budget is a viewer setting, a per-run option, or document data. (OFFER orchestrator)
