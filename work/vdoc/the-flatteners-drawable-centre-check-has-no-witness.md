---
id: the-flatteners-drawable-centre-check-has-no-witness
kind: issue
title: viewer: the flattener's drawable(centre) check has no row that can fail
status: open
opened: 2026-09-30
priority: P3
cost: M
---

Found by PATHS 5a (`retire-the-stored-bulge`, PR 3527). The preview's
arc flattener (`viewer::sketch::flatten`) asks that an arc's stored
centre is a drawable number before it samples the arc
(`.filter(|_| drawable([arc.centre.x, arc.centre.y]))`, and the
`PreviewError::Unflattenable` refusal behind it). Its one witness was
`path_authoring::a_far_millimetre_arc_draws_and_its_validation_refuses`,
whose far millimetre arc used to reach that refusal. Since 5a stores the
lowering's centre (the chord midpoint moved along its normal), that
centre is finite, the arc draws, and the row now pins the next
segment's validation refusal instead. No row reaches the centre check,
so deleting it would stay green.

**What would close it.** A row that hands the flattener an arc whose
stored centre is not drawable (overflowed or poisoned) with drawable
vertices — a table through the fixture door, since no construction now
mints one — and asserts the `Unflattenable` refusal names that vertex.
