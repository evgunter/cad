---
id: the-product-list-is-kept-by-a-default-at-the-edit
kind: issue
title: "Stage 2 FORK-2: what the product is (re-run as FORK-2b: the world, every copy a world placement defines)"
status: closed
opened: 2026-10-07
priority: P0
cost: E
closed: 2026-10-07
refs: [d10-one-way-to-say-intent-is-unbuilt]
---

Stage 2 replaces A10's product rule (the root set is the sink set of the consuming graph) with D10's explicit list. Under D10 nothing consumes anything, so A10's maintenance clause ("a new sink appends; an insert consuming roots takes the first one's place; a delete restores the orphaned inputs") has no graph meaning. Raised as FORK-2 by the stage-2 spec (`docs/INTENT-STAGE2-SPEC.md` §11, PR 4216). It blocks unit C's (`the-product-is-an-explicit-list`) ratified text.

A designer pair converged after four rounds: one default applied at the edit, and `SetProduct` as the one override. The question and both reports are in the `[ev]` PR. Ev's answer closes this row and writes A10.

**Re-run as FORK-2b (2026-10-07).** Ev's comments on #4220 showed that the edit-time default carried A10's "the tip replaces its operands" over from the consuming model. A fresh designer pair took Ev's direction as stated (no node replaces another; operands do not appear because nothing places them in the world) and converged: the product is the world, the copies that world placements define. The `[ev]` PR now carries that answer; fork-log rows 80 (superseded) and 85.

**Ruled (Ev, 2026-10-07, PR 4220):** the product is the world, every copy a world placement (`PlaceInWorld`, an operation reading one `Body` and defining its copy) defines; nothing places as a side effect. Residues: (1) Python and the Rust façade place only when told, with one semantics between them; the viewer's feature gestures re-point the world placement ("if it is annoying we can add auto repointing later"); (2) a second identity placement of one body is allowed, the at-rest gate judging the coincident copies; (3) the stage-2 placement carries its pose (`PlaceInWorld { body, pose }`), transient until stage 3's mates. Unit C builds it; D-2's closure, `PlacedUnderTwoRoots`/N4 and A5's minting lift change with units B and F.
