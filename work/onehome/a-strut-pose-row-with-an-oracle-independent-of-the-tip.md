---
id: a-strut-pose-row-with-an-oracle-independent-of-the-tip
kind: issue
title: No row reaches a strut at a closed edge's lone vertex with an oracle independent of the star's tip
status: open
opened: 2026-11-16
priority: P3
cost: M
---

Found by the review of PR 7008 (2026-11-16).

The pierce refuses a strut at a closed edge's lone vertex with a germ
along it, because no answer there is checked: no row reaches the pose (a
section along a closed piercing edge puts the whole edge on the pierced
face). The owed row lives today only in a comment
(`crates/topo/src/boolean/vtxfac.rs:1381-1388`) and in the closed row
`the-pierce-reads-a-struts-facing-the-stars-site-already-names`.

## Done when

A row reaches the pose, with an oracle independent of the star's tip
(`crate::star::Site::tip`), so an answer there can be checked and the
pierce's refusal lifted or kept on evidence.
