---
id: a-curved-merge-group-with-a-dangling-seam-refuses-as-period-closure
kind: issue
title: A curved merge group whose seam dangles without closing the period still refuses PeriodClosure, which names the wrong cause
status: open
opened: 2026-09-28
priority: P4
cost: E
---


Found by CONTACT-8's reviewer (2026-09-28); the behaviour is not new.
CONTACT-8 prunes dangling seam edges only in PLANAR merge groups. In
the curved branch of `merge_faces.rs`'s `merge_group`, any same-face
doubled edge still refuses `PeriodClosure`, even where the seam simply
dangles and no period closes. Either name the cause truly, or decide
whether curved groups prune too; that second question belongs to the
curved-properties side.

Moved from SECTOR to CONTACTHOLD on 2026-11-20, by `git mv`, when SECTOR merged into CONTACTHOLD: SECTOR held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one CONTACTHOLD carries.

Moved from `work/contacthold/` to `work/topo/` on 2026-11-25, when CONTACTHOLD closed (`docs/doc-ledger/contacthold-leaves-the-tracker.md`); id unchanged. The cleanup re-read the row against main at `6003877ad` and found it live. Re-banded P3 → P4: a loud refusal that names the wrong cause.
