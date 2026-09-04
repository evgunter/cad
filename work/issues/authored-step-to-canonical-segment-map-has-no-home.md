---
id: authored-step-to-canonical-segment-map-has-no-home
kind: issue
title: "The authored-step to canonical-segment map has no home: its two halves are DOCM's and BOOL's, and neither owner can site it alone"
status: open
opened: 2026-09-04
refs: [focus-marking-is-per-node-not-per-segment]
---

The announce VIEW's plan item 4 has owed since 2026-09-03, written as
a file because `work/README.md` is explicit that disclosing a residue
in prose is not scheduling it. `work/view/plan.md` said "two
announces"; a sentence in a plan is not one.

Filed in `work/issues/` because **no single program can own it** —
that is the finding, not an accident of filing.

## What is wanted

`work/view/focus-marking-is-per-node-not-per-segment.md` needs to
light the walls a profile step swept: viewport focus marking is per
`RecipeNodeId`, so a profile step cannot mark the faces it produced.
The missing reading is a map from an **authored step** to the
**canonical segments** it became.

## Why it has no home

The two coordinates are in two programs' territory and the map is
neither:

- the authored `step` is `ProfileProgram::step_args`
  (`crates/editor-core/src/program.rs:653`, read at `:1264`) —
  **DOCM's**;
- the canonical `segment` is `crates/profile`'s canonicalization —
  **BOOL's** (`crates/profile/src/*`, with `path/arc_fillet.rs` shared
  with FILLET).

A door on either side alone does not produce the map: DOCM can say
what steps a program authored and BOOL can say what segments a path
canonicalized to, and the correspondence between them is the thing
that exists in neither crate. So this is not "VIEW needs a door from
DOCM" (which is the shape of
`work/view/next-id-has-no-layer3-door.md`, and is answerable by DOCM
alone). It is a question about **where a value that belongs to two
crates lives**, and VIEW has no standing to answer it — nor, on its
own, does either owner.

## What the answer has to settle

1. **Which side computes it.** Canonicalization is where the
   correspondence is actually known — it is the step that turns
   authored steps into segments — which argues for `crates/profile`
   emitting it and `editor-core` carrying it. The counter-argument is
   that `crates/profile` has no vocabulary for an authored step today
   and would grow one to serve a consumer two layers up.
2. **Whether it survives the lowering** at all, or is re-derived. If
   the map is recomputed by the viewer from both endpoints, no door is
   needed and no crate grows a field — at the cost of a derivation
   that can disagree with the one that produced the geometry, which is
   the shape this project treats as a defect elsewhere.
3. **Whether anything but the viewer wants it.** If a second consumer
   exists (naming, selectors, a check), that decides (1) on its own.

## Who is blocked

`work/view/focus-marking-is-per-node-not-per-segment.md` — VIEW's plan
item 4, which cannot start until this is sited. It is the only known
consumer today, which is why the question has gone unasked: it looks
like VIEW's problem and it is not VIEW's to answer.

Signed: (VIEW orchestrator)
