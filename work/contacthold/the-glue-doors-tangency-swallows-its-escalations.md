---
id: the-glue-doors-tangency-swallows-its-escalations
kind: issue
title: The glue door's tangency read swallows every refusal, escalations included, so an in-band tangency surfaces later as CurvedBooleanUnsupported
status: open
opened: 2026-10-12
priority: P3
cost: E
---


## What

glue::tangency tests the witness lane with .is_ok(), discarding an escalation; the stage that meets the pair then refuses CurvedBooleanUnsupported, naming a declaration recourse D10 retires. An in-band tangency should escalate typed at the door.

Found by a designer of the overlapping-tangency fork (fork log row 118), read from the code at 63288134b8, not measured.
