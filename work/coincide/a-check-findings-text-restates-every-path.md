---
id: a-check-findings-text-restates-every-path
kind: issue
title: a check finding's text spells every read path and both margins, and is unreadable
status: open
opened: 2026-10-31
priority: P1
---


## Finding

Ev (in chat, 2026-10-31), reading the Checks window in the viewer,
called this finding text "crazy verbose":

> check unproven-coincidence: copies Place 8e8a553cb250 and Place
> 599f7cca2f65: the side wall over loop 0 step 1 of Extrude
> a59299af4a92, through read e1a972ff4570 at Assemble 2fb2165d1430
> continues on one carrier with the side wall over loop 0 step 1 of
> Extrude "Extrude 2" (92f3ffaab5c3), through read a2f825fc834b at
> Assemble 2fb2165d1430, decided from values (two cells at rest the
> census read as one, margin 0e0), is not proven structural: the two
> cells are two constructions, placed or reflected apart; with every
> variable a symbol, neither the carriers' verdict nor the deciding
> margin is shown identically zero within the door's budget — Recourse:
> assert the coincidence at its site; what decided it, read as a
> function of the variables, outgrew the door's budget

It is one sentence of about 120 words, and most of it is provenance a
person does not act on: two `through read <tag> at Assemble <tag>`
paths, the census's internal reading ("two cells at rest the census
read as one, margin 0e0"), and the budget's mechanics said twice.
What the person needs is which two faces, that they look coincident
but could not be proven so, and what to do. The composing site is
`evidence_story` in `crates/editor-core/src/checks.rs` (the
`decided from values ({}, margin {:e}), is not proven structural`
format) with its reason from `crates/editor-core/src/coincide.rs`
(`outgrew the door's budget`).

The at-rest overlap finding has the same shape: reproduced with
`examples/drive.rs` (two rectangles extruded at the origin, the second
placement added to the product), its text names six bounding faces in
full and a three-way recourse, also as one block (`INTERFERENCE_RECOURSE`,
`checks.rs`).

## Direction

A short headline (the two faces, the verdict in plain words, the
recourse), with the provenance available on demand rather than inline
— the full text stays the `Display` a log or a test reads. The faces
should be links the person can click, which the GUI half of
`work/chrome/a-findings-face-reference-reads-as-stale.md` needs anyway.
