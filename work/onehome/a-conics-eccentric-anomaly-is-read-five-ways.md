---
id: a-conics-eccentric-anomaly-is-read-five-ways
kind: issue
title: A point's eccentric anomaly on a conic is read five ways, with two spellings of which branch
status: open
opened: 2026-11-02
priority: P1
cost: M
---

Found by the conic-frame fork's designer weighing (PR 6325, 2026-11-02).

The angle θ of a point on a circle or ellipse, `atan2(w·v_ref/b, w·u_ref/a)`, is read in five places: `geom::Curve3::param_near` (the branch nearest a `near` parameter), `topo::ring_path::Conic::param` with `branch(.., span)`, `ruled_region::Conic::angle` with `near_span`, `splitting::containment::ConicArc::unit`, and `chord_join::SectionConic::param`. Two rules pick the branch against a span (`ring_path::branch` and `ruled_region::near_span`), which are two spellings of one rule.

It rides the conic-frame decision (PR 6325): once the frame is one `geom` type, this is one method on it (`param_near`) plus one span-branch helper beside it, and the topo copies repoint. Dispatch it with or after that unit.
