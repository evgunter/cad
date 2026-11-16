---
id: two-spellings-of-a-wrap-edges-tip
kind: issue
title: Two spellings of a wrap edge's tip: insert::wrap_tip_at reads one corner, validate::wrap_tips counts valence, reconciled only in prose
status: closed
opened: 2026-11-16
priority: P2
cost: E
closed: 2026-11-16
branch: onehome/wrap-tips-door
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

## Closed

The wrap flag has one home, `validate::wraps`, which both readers take:
`validate::wrap_tips` and `validate::wrap_tip_at` (moved out of the boolean engine, `[topo.homes]`). The door's doc says why the
two vertex readings stay two: mid-run, valence counts the struts already
minted at the tip, and the corner reading does not. The sweep row
`the_two_tip_readings_agree_on_every_wrap_bearing_body`
(`crates/sweep/tests/a_wrap_tip_sits_on_its_faces_chart_singularity.rs`)
reads both through `topo::test_support::wrap_tip_readings` and pins that
they agree at rest. It covers the one-face ball (two tips), the hemisphere
and the cone (one each), and the extruded circle, the revolved annulus and
the ring torus (wrap edges, no tips). It goes red if either reading is
mutated.
