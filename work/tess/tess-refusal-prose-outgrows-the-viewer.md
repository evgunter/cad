---
id: tess-refusal-prose-outgrows-the-viewer
kind: issue
title: mesh: the tessellation refusals are too long for the viewer (Ev's concision request)
status: open
opened: 2026-09-22
refs: [error-and-check-text-overflows-its-region]
---


## What

Refusal `Display` arms on this program's ground that the viewer shows
and that run past 50 words (literal words, before payload):

| words | site | arm |
|---|---|---|
| 95 | `mesh/src/types.rs` `TessellateError::UnsupportedCurvedDomain` | |
| 70 | `TessellateError::Plane` | |

`tess/2-refinement-in-the-ring` (#3080) was open when this was filed.

## The standard

The standard these arms are held to is stated once, in
`work/chrome/error-and-check-text-overflows-its-region.md` (section
"The standard a refusal is rewritten to"), with its word budget and the
test that enforces it.

## How the census was taken, and what it could not see

A static pass over every `impl Display for` block in `crates/*/src`,
split into match arms, counting the words of each arm's string
literals (a `{…}` placeholder counts as one word; a named recourse
constant such as `COINCIDENCE_RECOURSE` is NOT expanded, so a
constant-carrying arm is longer on screen than its count). A realistic
payload adds to every count: a nested refusal (`{e}`, `{source}`)
renders its own arm inside this one. Arms at or above 50 literal words
are listed. The pass does not prove each arm is reachable from the
viewer — most reach it through `NodeErrorKind`'s forwarding arms
(feature tree fault line, status line) or through the checks window —
and a `Display` written outside `impl Display` (a helper returning a
`String`) is not seen.

## Two more things the literal census could not see (ENCL, 2026-09-25)

Found by ENCL's `encl-refusal-prose-outgrows-the-viewer` unit, which
rendered every `geom_brep::patch_bound::PatchBoundError` note through
the feature tree (`editor-core/tests/refusal_concision_chains.rs`,
the `approx_recertify` rows under `Transform/ApproxRecertify/PatchBound/*`).

- **A forwarded note.** `TessellateError::UnsupportedNurbsFace`
  (`mesh/src/types.rs`) renders `PatchBoundError::note()` after its
  own 12-word opening (`mesh/src/nurbs_cert.rs` `face_err`). The longest
  note on main before that unit was 89 words
  (`RefinedWeightLostPositivity`); that unit shortened it to 41 and
  `NonPositiveWeight` from 60 to 34, so the arm is under budget on
  today's notes, but nothing renders it: a note that grows moves this
  arm with it.
- **A stage prefix on every arm.** 17 `TessellateError` arms open with
  `tessellate:`, the shape `test_utils::refusal::stage_prefixes`
  rejects on the feature-tree rows. The viewer draws them through
  `viewer::scene`'s `NotTessellated`. No row renders them against
  `test_utils::refusal::problems`.
