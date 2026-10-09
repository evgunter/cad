---
id: a-curved-merge-group-with-a-dangling-seam-refuses-as-period-closure
kind: issue
title: A curved merge group whose seam dangles without closing the period still refuses PeriodClosure, which names the wrong cause
status: open
opened: 2026-09-28
priority: P3
cost: E
---


Found by CONTACT-8's reviewer (2026-09-28); the behaviour is not new.
CONTACT-8 prunes dangling seam edges only in PLANAR merge groups. In
the curved branch of `merge_faces.rs`'s `merge_group`, any same-face
doubled edge still refuses `PeriodClosure`, even where the seam simply
dangles and no period closes. Either name the cause truly, or decide
whether curved groups prune too; that second question belongs to the
curved-properties side.
