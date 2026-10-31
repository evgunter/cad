---
id: library-design-u6-promotes-a-flush-declaration-helper
kind: issue
title: LIBRARY-DESIGN U6 still plans a flush-declaration helper over CONTACT-DESIGN's declared pairs, which D10 retired
status: closed
opened: 2026-10-29
closed: 2026-10-30
priority: P4
cost: C
refs: [5997]
---


Found by the D10 exit walk (`work/intent/d10-exit-walk-report.md`, companion pages).

`docs/LIBRARY-DESIGN.md:203-210`, **U6 — declared-relation
ergonomics**, plans to promote "the flush-declaration helper … into the
library surface, per the coincidence ladder … and CONTACT-DESIGN's
vocabulary". That helper enumerated plane pairs to declare on a boolean.
Stage 4 F (#4641) deleted the declare surface at every layer, and stage
4 E (#4496) glues flush pairs on Zero with no declaration. So U6's first
half has nothing to promote. DESIGN's state-of-the-build list retires
the declared-contact seats.

U6's second half, the declared-offset relation, is a separate question.
SELECT-DESIGN §3 calls it "the declared-offset relation", and LB11
holds it.

Fix: strike U6's flush half, or re-word it as the D10 spelling: a flush
pair is one construction read twice (a mate), and an assertion quiets
the rest.

## Closed

2026-10-30 (`intent/stale-companion-pages`). U6's flush half now states
what the façade offers: a plane `Mate` with one sense (the first rung,
`coincide::stated_by_the_mates`), shared variables (the replay), or a
`Gap = 0` assertion (`checks::at_rest::quiet_row`). The flag mate DESIGN
`[coincidence]` names is not stated, because the `Flag` pose kind is
unbuilt (`msolve/flag-and-pivot-pose-kinds`). The declared-offset half
is unchanged.
