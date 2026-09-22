---
id: four-pick-state-vocabularies-in-one-create-module
kind: issue
title: create.rs answers 'what is this button waiting for' in four unrelated vocabularies
status: open
opened: 2026-09-21
priority: P1
cost: D
---


## Finding

Found by AUTH-1's style reviewer (AUTHOR, PR 2955, 2026-09-21) under
Q8 — the once-per-review end-to-end read of the largest file touched —
confidence `likely`. `crates/viewer/src/pane/create.rs` is ~1175 lines
and answers *"what is this button waiting for, and why is it
disabled"* in four unrelated vocabularies:

1. `seat_line`, for the seated tools;
2. `BlendTarget`'s `Display`, for the blend tool;
3. a `blocked: Option<&'static str>` rendered once and gating one
   `add_enabled`, for `add_profile_ui`;
4. and, as of AUTH-1, `DatumKindChoice::unmet_seat` plus
   `FaceFrameFault` plus a bare `format!`, for `add_datum_ui`.

**Nothing about any single diff is unreasonable, which is the shape
the question is about** (`docs/prompts/reviewer-style-lane.md` Q8:
accumulation is invisible by construction because nothing in this
project's process ever reads a whole file). The reviewer's sharper
version: `add_profile_ui`'s single `blocked` and `add_datum_ui`'s
three-value hand-written de-duplication are two forms in ONE file
answering one question two ways, and the de-duplication rule — render
the fault unless it is `NoFace`, because the unmet-seat sentence is
expected to have said the same thing — is an invariant held by
convention where a type would do.

## Also noted in the same read, and not this row's

The file spells one concept `"feature {}"` in seven places and
`"node {}"` in two. Pre-existing, unrelated to the pick-state
question, and recorded here only so the next reader of this file does
not report it as new. `pane/properties.rs` has two more `"feature {}"`
sites.

## Why P1

`work/README.md`'s Priority: *"a special case that should be handled
uniformly, and the rest of that class, which mostly falls under no
tidier heading than itself."* This is that heading.
