---
id: circle-residual-harmonics-needs-torus-arm
kind: issue
title: circle_residual_harmonics needs a torus arm — the declared-Rest torus boolean lane's last blocker
status: open
opened: 2026-09-01
github: 1489
refs: [1477, 968]
---

## From GitHub issue 1489

Opened 2026-09-01; 0 comments.

Filed at MATE-7a's adjudication (PR #1477) as the scheduled home for its disclosed deviation 3, per the unit's own stop-report and both review arms' confirmation.

**The blocker.** `geom_brep::circle_residual_harmonics` (implicit.rs) carries Plane/Sphere/Cylinder arms only. The boolean reduction's circle rung takes the frontier door on `circle_residual_extremes = None` (`reduce.rs` ~1172–1176) BEFORE the C8 declared-cover rung is consulted — and every edge of a torus body is a circle, so no torus×anything operation can complete regardless of declarations. MATE-7a's gate admission, carrier rung and rim routing are all landed and pinned; two suite rows hold this exact boundary.

**Whose ground.** `geom-brep`'s certified enclosure family is VERBS' germ ground (MATE-7a was fenced out of it, correctly — the enclosure is real certified-numerics work: the residual of a circle against a torus implicit, harmonically bounded). `docs/VERBS-GERMARMS-SPEC.md` item 4 already maps the adjacent territory. Routed to VERBS by this filing; S-MATE does not block on it (issue 968 stays open on the kissing arm independently).

**What completing it buys**: with this arm plus the operand-box tightening (see the sibling issue filed alongside), the declared-Rest torus lane completes end-to-end — MATE-7a's report puts it one function away.

## Home

`work/verbs/` — the issue routes itself to VERBS explicitly; `geom-brep`'s certified enclosure family is VERBS' germ ground and `docs/VERBS-GERMARMS-SPEC.md` item 4 maps the adjacent territory.
