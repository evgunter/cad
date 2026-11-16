---
id: two-spellings-of-a-wrap-edges-tip
kind: issue
title: Two spellings of a wrap edge's tip: insert::wrap_tip_at reads one corner, validate::wrap_tips counts valence, reconciled only in prose
status: open
opened: 2026-11-16
priority: P2
cost: E
---

Found by the review of PR 7008 (2026-11-16).

A wrap edge's tip is spelled twice:

- `insert::wrap_tip_at` (`crates/topo/src/boolean/insert.rs:2439`) reads
  one corner: a wrap edge whose two ends differ, the vertex one of them.
- `validate::wrap_tips` (`crates/topo/src/validate.rs:5871`, read again at
  `:8077`) counts valence: a valence-1 end of a wrap edge.

Only prose reconciles the two (`wrap_tip_at`'s doc): at rest they are the
same set, and mid-run the corner reading stands, since valence would count
the struts already minted at the vertex. One fact decided at two sites is
a fact that can drift.

## Done when

The tip is decided at one home that both readers take, or a row pins that
the two agree on every at-rest body the suites build, with the mid-run
difference stated where the corner reading is kept.
