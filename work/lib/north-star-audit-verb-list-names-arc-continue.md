---
id: north-star-audit-verb-list-names-arc-continue
kind: issue
title: docs/guide/north-star-audit.md's verb list still names arc_continue after BOOL-10 retires it
status: open
opened: 2026-09-08
refs: [BOOL-10, 2135]
---

Reported by BOOL-10's class sweep (PR 2135; the guide is LIB's, so the
unit reported rather than edited): `docs/guide/north-star-audit.md:157`
lists `arc_continue` among the path verbs. BOOL-10 removes the verb
(`arc_to(spec.split(n))` is the declared-tangent-joints arc form that
replaces its one authored use); `docs/GUIDE.md`'s pip-ball example was
re-authored in the unit because it is an executed block. The audit line
is prose and wants the same re-spelling once BOOL-10 merges. Difficulty
XS.
