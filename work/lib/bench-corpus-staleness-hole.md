---
id: bench-corpus-staleness-hole
kind: issue
title: Close the bench-corpus staleness hole - pin the tour's assembly structure, not just its constants
status: open
opened: 2026-08-29
github: 1186
refs: [1176]
---

## From GitHub issue 1186

Opened 2026-08-29; 0 comments.

Scheduling item for the disclosed gap in LIB-G18a (#1176). Filed because a banked "someone should do this properly" line is not a schedule, and the v5 A1 rule wants a number to point at.

## The hole

`crates/pncad-py/tests/corpus/bench/` is four committed `.pncad` documents, generated from the tour's own authoring functions (`demos/tour/src/assembly.rs`) via `demo-tour asm-corpus`. Committed bytes rot. The guard that exists — `test_the_corpus_still_matches_the_scene_it_came_from` — reads the five base dimension constants out of `assembly.rs` and fails if any moved.

It does not cover three things:

1. **Structure.** A fifth patterned post, a third mate, a different node order — constants untouched, corpus silently no longer the tour's.
2. **Derived constants.** `SEAT_A`, `SEAT_B` and `POST_SEAT` are computed from the five (`[POST_SECTION / 2.0, SHELF_DEPTH / 2.0, 0.0]` and friends). Change a *formula* and the guard reads the five unchanged bases and passes.
3. **Placement literals.** The layout's pattern spacing (`200 mm`), its rotation (`-PI/2` about +y) and the shelf's offset are literals in `layout_doc`, read by nothing.

LIB-G18a's fix pass closed (2) and (3) *as oracles* — `test_the_patterned_posts_sit_where_the_scene_places_them` now pins all four post boxes, so rotation, spacing and origin drift go red. That is a corpus row, not a tour guard: it pins what the committed bytes SAY, so it catches a bad regeneration but not a tour that moved without one. **(1) remains open**, and so does the general shape of the problem.

## Why it was not closed in the unit

The proper fix is the `plate_param.v15.pncad` pattern: a test that RE-AUTHORS the scene and pins the saved text. That needs the tour's authoring functions callable from a test in the code tier — and `demos/tour` is a **detached workspace** (its own `[workspace]` table; the root manifest excludes `demos`), built only by the render lane. So the guard cannot reach them today.

## Options

- **Lift the assembly documents into a small shared crate** both `demo-tour` and a kernel-workspace test depend on. Cleanest; touches the tour's structure.
- **Make the code tier build `demo-tour`** for one bless-style test. Expensive — a release build of the whole kernel in a detached workspace, for one text pin.
- **Have the render lane emit the corpus and diff it** against the committed copy. Cheap, but the check lands in a lane that does not run on every PR.
- **Accept it** and narrow the corpus's claim in its own header, which is what LIB-G18a's header does today.

## Acceptance

Either a mechanism that reds when `assembly.rs`'s bench scene changes without regenerating `tests/corpus/bench/`, or a ratified decision that the corpus's claim is narrower than "this is the tour's scene" — written into the test header, replacing the disclosure that is there now.

Cited from the test header in `crates/pncad-py/tests/test_assembly_eval.py`.

## Home

The corpus and its guard live in `crates/pncad-py/*`, LIB's `paths:` territory, and the census/audit gates on the bindings are its charter.
