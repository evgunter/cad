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

## Question for Ev (2026-09-06, LIB orchestrator; `[ev]` PR)

Which of the four options in this file's body closes the hole. The
orchestrator's recommendation first:

- **(A) A shared scene crate.** Lift the tour's assembly-authoring
  functions (`demos/tour/src/assembly.rs`'s document builders, not the
  rendering) into a small crate both `demo-tour` and a kernel-workspace
  test depend on; a code-tier test then RE-AUTHORS the scene and pins
  the saved text against `crates/pncad-py/tests/corpus/bench/`, the
  `plate_param` pattern. Structure, derived constants and placement
  literals all go red together. Cost: the tour's structure gains a
  crate, and the demo-purpose rule (a demo is written the way a user
  would write it) has to survive the move — the builders stay
  user-shaped, only their home changes.
- **(D) Narrow the claim.** Rewrite the corpus test's header so the
  corpus claims only "the tour's scene as of its last regeneration,
  with these five constants and these four post boxes pinned", and
  delete the sentence that schedules a mechanism. Honest, cheap, and
  the hole stays.
- (B) building `demo-tour` in the code tier for one pin, and (C) a
  render-lane diff that does not run on every PR, are listed in the
  body and not recommended: (B) is a release build of the whole kernel
  in a detached workspace for one text pin, (C) lands the check where
  a PR cannot see it.

Recommendation: (A) if the builders separate from the render deps
cleanly (a read of `demos/tour/Cargo.toml` says they should: the scene
functions use `pncad` only); (D) otherwise, with the header rewritten
in the same PR that decides.

## Ruled (Ev, PR 2019, 2026-09-06): **(E) — delete the committed bytes**

Not one of the four options above: Ev's question ("why are these
committed?") exposed that the corpus's premise is stale. G18B closed
Python's assembly authoring the day LIB-G18a landed, and
`crates/pncad-py/tests/test_assembly_author.py` already authors the post,
the shelf, the flat-pack layout and the mated stand from nothing into a
temp `Workspace` (`BenchWorkspace`). So: `test_assembly_eval.py` builds
its store from the authored scene (the builders shared between the two
files), saving the documents to the temp store before evaluating so the
LOAD path stays exercised; `corpus/bench/` and its MANIFEST are deleted;
the header's false live claim ("Python cannot AUTHOR an instantiate
node") goes with them; the constant-reading guard stays, pointed at the
shared Python constants, so a tour drift still reds; `demo-tour
asm-corpus` is retired or kept as a demo-only door, the unit decides and
says which. Dispatchable as a LIB unit.
