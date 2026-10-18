---
id: a-placement-pins-by-written-values-without-a-mate
kind: issue
title: "A placement pins by written values without a mate: stage 3 D′, the placement pivot"
status: open
opened: 2026-10-18
priority: P0
cost: H
refs: [transform-retires-into-a-placement]
---

Stage 3 D′, which follows D (`transform-retires-into-a-placement`); opened by the item `a-placement-relates-two-frames` (PR 5312). D10 `[placement]` was restated by PR 5312 (fork-log row 162; Ev approved). The clauses this builds:
- `[placement.spaces]`: "Only a placement relates two spaces". A space is a construction's coordinates or the world, and spaces form a DAG.
- `[placement.placement]`: "a bundle that leaves one unset is refused at the door, so every placement is pinned and an unwritten freedom cannot be represented". Also: "what reads it downstream reads the space's bodies, never a placement".
- `[placement.mates]`: "a placement whose source touches nothing has none, and its values state the whole motion".
- `[placement.values]`: where no direction is fixed, "the rotation is a written chain of at most three spins, each about the target space's x, y or z … no two in a row about one axis; turns do not commute, so the author writes the order". The slides run along the target's (swung) axes. "a placement whose values are all zero and which has no mate is the identity".
- `[placement.world]`: "The product is the world's carried bodies". An instance places another document's world or any space of it.
- DI5: free-move repositions one shown space's picture and may write display metadata.

D lands with `mate::stand` as the interim: a Plane mate from a face no variable reaches (an Axis mate for a plane-less root) plus written zeros. That is a legal bundle under #5312. D′ is the pivot:

- **The unmated chart.** `Value` without a mate (`Value.mate` optional), with `Coord` gaining slides along the target space's axes and the author-ordered spin chain. Where a mate fixes a direction, C-1's swung chart stands. This runs through `mate/value.rs`, `mate/solve.rs`, the edit doors, persist, the wire bytes and the content key. About 300 explicit `Value`/`AddValue` sites and 303 `Coord::Slide/Spin` sites at D's head.
- **F counts spins by axis**: a repeat about one axis in a row, or a fourth spin, is overconstrained.
- **Every freedom written.** An under-pinned bundle refuses at the door, so `Space::Loose`, `Evaluation::loose_copies`/`loose_roots`, the façade's loose-copy export refusal and the viewer's free-move over loose spaces retire. Free-move goes over shown spaces (DI5).
- **`stand` becomes an unmated placement into the world.** Its values are the datum chain's slides and spins, verbatim where a step is one slide or spin about a target axis and expressions otherwise, or six zeros. Retired with it:
  - `mate::anchor::Anchors` and the variable-free-face search;
  - the Axis-mate fallback for revolves and rings (row 161 Q1 is superseded; Q2, the loft stations, stands);
  - the in-plane/out-of-plane split;
  - the instance-anchor heuristic.
  `place_at`, `pin_at` and `frame_at` are re-spelled to match.
- **Inline at a moved landing** writes the landing's motion into the values, so `InlineError::RepinRefused` and `ValueOnAMovedWorld` retire.
- **Spec** `docs/INTENT-STAGE3-SPEC.md` §1, §4 and §5 restated to match. §1 settles how a space names its bodies to its readers when one source is placed into it twice; that fork is the orchestrator's designer pair, and its ruling comes before D′ starts.
- **Re-bless**: every placement's bits and bytes move (the corpus digests, goldens, the `.pncad`/`.cad` snapshots and the tour stops).

Supersedes `a-stand-on-an-instance-reads-its-part-by-the-names-shape`, `a-value-charted-on-the-target-reads-frame-carries-the-datum-chain` and `an-inline-at-a-moved-landing-reading-a-variable-refuses`. Not started until D has merged; the orchestrator dispatches it.
