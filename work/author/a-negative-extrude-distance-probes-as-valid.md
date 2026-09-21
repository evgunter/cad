---
id: a-negative-extrude-distance-probes-as-valid
kind: issue
title: The probe reports every negative extrude distance as valid, so a length field's only floor is the single point zero
status: open
opened: 2026-09-15
priority: P0
cost: E
---


Measured on `chrome/bounds-honesty` while checking what
`crates/viewer/tests/valid_range.rs`'s downward assertion actually
discriminates. The document is that row's own: one
millimetre-authored `thickness` parameter driving an `Extrude`
`distance`, origin 8 mm.

Evaluating the document at nine thicknesses, at each of the three ε
rows CI gates, failing nodes each time:

| thickness | default ε | `1e-6` | `1e-12` |
| --- | --- | --- | --- |
| -8 mm | `{}` | `{}` | `{}` |
| -1 mm | `{}` | `{}` | `{}` |
| 0 | `{RecipeNodeId(2)}` | `{RecipeNodeId(2)}` | `{RecipeNodeId(2)}` |
| 1e-6 m | `{}` | `{RecipeNodeId(2)}` | `{}` |
| 5e-6 m | `{}` | `{RecipeNodeId(2)}` | `{}` |
| 1e-5 m | `{}` | `{}` | `{}` |
| 2e-5 m, 1e-4 m, +8 mm | `{}` | `{}` | `{}` |

**Two facts, and only one of them moves with ε.**

1. **A negative distance builds, at every ε row.** Presumably it
   extrudes the other way; the probe, whose oracle is "no failure the
   baseline did not have", therefore reports it valid. Forcing the
   probe's seed to one metre gives `low: Open { probed: -2047.992 }`:
   the search walks two kilometres into negative thickness and reports
   the whole span as nothing-new-fails. **This is the finding**, and ε
   does not touch it.
2. **How wide the failing region above zero is, is ε's to set.** At
   the default ε and at `1e-12` the only failing thickness is exactly
   `0`; at `1e-6` everything below about `1e-5` fails too. One rule —
   an extrusion the tolerance cannot tell from zero — whose width
   follows ε, which is unremarkable on its own and is recorded here
   only because **an earlier version of this row asserted the
   single-point floor as a flat fact.** It was measured at one ε.

## Why it matters here rather than being a curiosity

1. **`crates/viewer/src/bounds.rs`'s own examples assume a floor.**
   The module talks about a bound as "where a failure appears that was
   not there before" and the panel renders `valid from …`; on a length
   field driving an extrude, that wording is carried by a failure at
   exactly one representable value.
2. **A bracket around it is an arithmetic accident.** A direction
   brackets a failing region this narrow only by landing a doubling
   inside it. Measured on this document at the default ε: a 1 mm seed
   brackets (the floor is 8 seeds out, and 8 is a power of two); an
   0.8 mm seed reports `low: Open { probed: -1.6304 }`; a 1 m seed
   reports `Open { probed: -2047.992 }`. Any row asserting a bracket
   on a length field rests on that coincidence, which is what
   `work/chrome/probe-rows-assert-in-one-direction-only.md`'s
   discharge note now says out loud. At `ε = 1e-6` the region is wide
   enough (~1e-5) that the same ladder lands in it for a different
   reason — the bracket is not evidence of a point.
3. **The reading a user gets is arguably wrong**, not merely coarse: a
   plate whose thickness is -5 mm is not a plate, and a panel saying
   "nothing new fails down to -2 km" is the kind of confident wrong
   answer this module's fail-loud posture exists to keep out.

## It may not be viewer's to fix

Plainly: **this may be a kernel question rather than a viewer one.**
If a negative extrude distance is meant to build (extruding along the
reversed normal is a reasonable reading) then nothing is wrong below,
and what is left is that a dimension a user thinks of as a THICKNESS
has no declared non-negative domain — a document/parameter question,
not a probe one. If it is not meant to build, the refusal belongs in
evaluation, where `work.py territory` puts
`crates/editor-core/src/eval/mod.rs` on WIRE. Either way the probe
reports faithfully what the kernel decides, so the fix is not in
`crates/viewer/src/bounds.rs`.

What is CHROME's either way: the panel's wording for a field whose
only failure is a point, and any row that assumes a floor.

## Home

CHROME, because the evidence and the affected rows are in
`crates/viewer`. Re-home it to whichever program owns the answer once
the question above is decided — the header move, per `work/README.md`.
