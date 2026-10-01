---
id: a-negative-extrude-distance-probes-as-valid
kind: issue
title: The probe reports every negative extrude distance as valid, so a length field's only floor is the single point zero
status: closed
opened: 2026-09-15
priority: P0
cost: M
design: true
closed: 2026-10-01
pr: 3551
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
   `work/vacuity/probe-rows-assert-in-one-direction-only.md`'s
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

## Re-priced 2026-09-30: a design fork, not a drive-by

Priced `E` and parked as a drive-by "waiting for a lane in `bounds.rs`", but the row's own "It may not be viewer's to fix" section is the reason it is not dispatchable. Either a negative extrude distance is meant to build, and the gap is that a thickness has no declared non-negative domain (a document/parameter question), or it isn't, and the refusal belongs in evaluation (`eval/mod.rs`, WIRE's ground). The probe reports the kernel faithfully either way. So it is re-priced `M` with `design: true`. It goes through the designer lanes (`docs/prompts/designer.md`) and then an `[ev]` PR before any lane builds it.

## A depth and a side (2026-09-30)

Two designers weighed this fork independently and then read each other's reports. Both now recommend the same answer. The evidence and both rounds are in `work/author/log.md` (2026-09-30, "negative-extrude fork").

**What the evidence points at.** The failure is not the probe's. One slot, `Extrude.distance`, holds two intents: how thick, and which side of the sketch plane. As the value passes through zero the body shrinks to nothing and reappears on the other side. Anything built on its end cap follows it there silently. Both designers measured this: a boss on the block's `Cap(End)` rebuilds below the plane at −10 mm, and no node fails. The only trace is a zero whose failing band is the tolerance's width, which a sampling probe steps over unless a doubling happens to land in it.

**The answer both recommend.** An extrude's distance is a **depth**, definitely positive at the tolerance, refused in the kernel's extrude door through the sign decision it already makes (`extrusion_normal_component`). Its direction is a separate **structural** choice on the node, `side`: along or against the sketch normal. What that makes true:
- No expression can flip the side by changing sign.
- Typing −5 mm refuses, and the refusal points at `side`.
- Every failure below a thickness is a half-line, so the range probe finds the floor for every seed at every ε, with no probe change.

**What is given up.** An expression can no longer choose the side by its sign (`top_z − plane_z`). "Extrude up to a plane" is the feature that says that honestly.

**Rejected by both.**
- *Keep the signed distance and have the probe ask "is this the same build?"* (compare decision logs). Measured: ordinary edits flip algorithmic decisions without anything changing that an author would call structural (78 of 138 small edits on `plate_param`, and 4 of 29 corpus documents under their own recorded bump), so the range would collapse to a point on most documents with a boolean.
- *A declared non-negative domain on the document parameter.* The parameter does not know which slots consume it, a literal slot has no parameter to carry it, and it would be a second source of truth beside the door.

**The rule behind it, and its reach.** "A size an operation covers is positive; its direction has one home." A revolve's signed angle beside a directed axis, and a pattern's signed spacing or step beside its direction, have the same shape (likely for revolve, unmeasured for patterns). Its reach was the open question; Ev ruled it below.

**Not waiting on this ruling** (CHROME's, filed separately): the range panel's Open sentence claims more than the probe sampled, and a bracket's invalid end could name what refused there.

## Ruled 2026-10-01 (Ev, #3551): (a)

**An extrude's distance is a depth, definitely positive at the tolerance, and its direction is a structural `side`** (along or against the sketch normal). A non-positive depth refuses in the kernel's extrude door, through the sign decision it already makes.

**The rule is adopted:** "a size an operation covers is positive; its direction has one home." Revolve's signed angle and a pattern's signed spacing or step are its named follow-ons, each weighed on its own.

**Ev's follow-up, a requirement on the refusal.** A negative depth must refuse with a recourse that shows how to write the extrude the other way, i.e. set `side`, not merely that it refused. ("presumably one of the followups will be an error message that shows how to easily write the extrude going the other direction without using a negative distance").

**Where the work went.** The design is decided, so this row closes. The work is on its owners' slates:
- **`work/edit/extrude-distance-is-a-depth-and-a-side`** (EDIT; P0). This is the node shape, the edit vocabulary, persistence and the schema bump. Its kernel half is in `sweep::Extrusion`/`ExtrudeError` on CARVE/STRUT ground, and its eval wiring on WIRE's. It carries the refusal requirement above. Its knock-ons:
  - `docm9_range` A2's only `DecisionFlip` fixture needs a replacement;
  - the tests that author negative distances on purpose need re-spelling with `side`;
  - `pncad-py`'s `Node.extrude` changes.
- **`work/author/the-create-pane-has-no-extrude-side`** (AUTHOR; blocked on the EDIT row). The viewer half: a `side` control in the extrude form, and `SessionOp::AddExtrude` carrying it.
- **`work/carve/revolve-angle-is-a-signed-size-beside-a-directed-axis`** and **`work/edit/pattern-spacing-is-a-signed-size-beside-a-direction`**: the rule's named follow-ons, as design rows.
- CHROME's `the-range-panels-open-sentence-claims-values-it-never-sampled` (already filed) is independent of the ruling.
