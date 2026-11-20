---
id: certify-rebased-run-certifies-an-edge-twice-on-identical-input
kind: issue
title: certify_rebased_run certifies four edges of a ball ∪ rod twice each on bit-identical whole input within one operation
status: open
opened: 2026-11-20
priority: P3
cost: E
---


Found by a SECT designer's probe (2026-11-20, ball ∪ rod, default ε, on
PR 7173's head): four edges each certified twice by `certify_rebased_run`
on bit-identical whole input, both calls succeeding. Its "ask again with
the current endpoints" branch runs only on `Err`, so a caller likely
invokes the run twice. Subsumed by `a-certificate-is-read-by-its-support`
if that lands; otherwise find the double caller.
