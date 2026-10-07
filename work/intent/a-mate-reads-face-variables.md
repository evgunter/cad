---
id: a-mate-reads-face-variables
kind: issue
title: D10 stage 2 PR F: a mate's sides read Face variables; A12's reading edges and the mates-are-not-edges carve-out retire
status: parked
opened: 2026-10-07
priority: P0
cost: H
blocked_on: [select-defines-face-and-edge-variables]
refs: [split-and-inline-over-a-mate-read-at-a-union-are-unmeasured]
---

INTENT stage 2, PR F. Spec: `docs/INTENT-STAGE2-SPEC.md` §7.

Mate sides become `Face` slots read through selects. `mate::reading_edges` (A12) is
deleted, and A9 runs over `Doc::upstream` while keeping the gauge edges (Q3). The
at-rest gate maps the select's resolution instead of re-resolving against the product
table (Q8). `split-and-inline-over-a-mate-read-at-a-union-are-unmeasured` gets its
rows here, because this PR rewrites the four refactor sites it names.
