---
id: the-cone-torus-pose-spells-its-own-refusal-and-question-types
kind: issue
title: ConeTorusRefusal and KtQuestion restate TorusPoseRefusal and TorusQuestion beside it
status: open
opened: 2026-11-23
priority: P3
cost: E
---


Found in PR 7395's dual review (item 12, optional, not taken there).

`pair_section/cone_torus.rs` declares `ConeTorusRefusal`
(`Torus`, `Aperture`, `ApexRuling`, `Undecided`) and `KtQuestion`
(`Aperture`, `Axis`, `Apex`, `Gap`). They sit beside
`pair_section/torus.rs`'s `TorusPoseRefusal` and `TorusQuestion`, which
the other three torus poses share. So each needs its own mapping into
`SectionError` and its own `SsiError` reader: `From<ConeTorusRefusal>`
beside `From<TorusPoseRefusal>`, and `SsiError::of_cone_torus_pose`
beside `of_torus_pose`.

PR 7395's fix pass already folded the stance-to-refusal mapping it had
written twice into one place, `ConeTorusPose::unmarched`.

## What the taker owes

One torus-pose refusal type that carries the cone's questions, with
the two `From`s and the two `SsiError` readers merged. `TorusPoseRefusal`
is SECTTORUS ground, so this is a seam with that program.
