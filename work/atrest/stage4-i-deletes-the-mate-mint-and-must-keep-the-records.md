---
id: stage4-i-deletes-the-mate-mint-and-must-keep-the-records
kind: issue
title: Stage 4 I deletes the mate mint and puts back rows only; nested mated assemblies would refuse unless the census door records them
status: open
opened: 2026-10-11
priority: P1
---


Found by the census-contact designers (fork-log row 113, PR 4666).

`docs/INTENT-STAGE4-SPEC.md` §9 says I removes the mate-minted declarations and that "The census runs over the product body. Every coincidence between copies it finds is a … row". With rows only, the mate-minted contact records go away too. `PartValue::contacts` is what carries a part's contacts across the seam, so from I on a nested mated assembly would refuse in its parent: the parent's census finds the child's contact inside one copy with no record (D1 (iii)).

The fix is ordering: I deletes only the mint, and mate contacts take the census door's path. That door is `the-census-records-what-it-decides`, which returns a row and a record citing it. Amend spec §9's I bullet, and park I on that door's unit when it is filed.
