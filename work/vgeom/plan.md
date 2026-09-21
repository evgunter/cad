# VGEOM — the plan

the viewer's geometry, camera and numeric renders

Re-scoped 2026-09-20 by VGEOM's priority-seam cut
(`work/README.md`, Track size). **Live state is
`python3 scripts/work.py status --program vgeom` and the item files,
never this file.** The wave below says what is in flight and why each
grouping is one unit; it does not say what is OPEN, and a list here
that tried to would be stale by the next merge. It was, twice: the cut
wrote *"Nothing dispatched"* over two lanes in flight, and the slate
table that stood here until 2026-09-21 listed ten closed rows as live
and none of the six filed after it was written.

## The slate

Sized against a ceiling of 30 points (`work/README.md`, Track size).
The weight is the `load` column of `work.py status`, re-derived on
every run and stored nowhere — which is the repair for the table that
used to stand here.

## The 2026-09-21 wave — four lanes, eleven rows

Dispatched together because the four groupings are four different
questions and their files do not overlap; each lane's brief names the
other three's files as out of fence.

| unit | rows | the one question |
|---|---|---|
| `vgeom/pick-distance` | `a-nan-edge-distance-wins-its-boundary-rather-than-losing` (P0), `pickindex-tie-break-rests-on-a-comment` (P1) | `pickindex.rs`: a numeric comparison used as a domain test on a value with no total order |
| `vgeom/render-spelling` | `the-fields-door-has-no-width-bound-at-all` (P0), `renders-that-multiply-a-finite-guarded-length-spell-the-product-inf` (P1) | `widgets.rs`, `readout.rs`, `pane/view.rs`, `bounds.rs`, `props.rs`: what a render SPELLS at the top of its type |
| `vgeom/f32-seam` | `cursor-projection-is-f32-in-a-module-whose-matrices-are-f64` (P1), `the-point3-to-gpu-corner-cast-is-at-three-sites` (P1), `the-viewport-and-position-lanes-narrow-to-f32-with-no-door`, `the-one-free-transform-is-the-only-total-door-in-camera` (P4) | `camera.rs`, `marks.rs`, `scene.rs`, `pane/viewport.rs`, `gpu.rs`, `input.rs`: where the `f64` → `f32` conversion lives, and whether it refuses |
| `vgeom/deletions` | `viewer-array-lowered-vector-ops-escaped-the-hand-rolled-sweep` (P3), `mixfraction-has-two-constructors-where-one-would-do`, `headings-unit-vector-is-not-unit-at-the-bottom-of-the-range` (P4) | `display.rs`, `sketch.rs`, `theme.rs`, `tests/datum_draw.rs`: three repairs whose deliverable is a deletion or a corrected receipt |

**Why the four f32 rows are one lane and not two.** The cast rows and
the narrowing row both ask for a home for one conversion. Split across
two lanes they mint two near-parallel doors — the Q1 defect the unit
exists to close, re-minted by the fix that closes it, which
`docs/prompts/reviewer-style-lane.md` records as having held on every
unit of two whole tracks and as never prevented by naming it in a PR
body.

**Held out of the wave, and why.**

- `a-fields-text-commits-within-the-renders-own-tolerance` (P0) and
  `a-typed-field-hands-its-text-over-on-two-frames` are the **COMMIT**
  question — what a field's text does to the document — where
  `vgeom/render-spelling` has the **RENDER**. The first row draws that
  distinction itself, and AUTH-2 already answered its commit half for
  the two panel fields through `props::echoed`. They take the next
  wave, on files `render-spelling` is holding this one.
- `a-count-slots-cast-still-saturates-for-a-finite-value-too-large`
  (P2) needs a refusal vocabulary that does not exist anywhere: either
  a `DimensionError` arm in `crates/editor-core` (EDIT's and MSOLVE's)
  or a first numeric `session::Refusal` of the viewer's own (VNEWS's).
  A decision before a diff, and not this program's alone.

## Order

By class, not by file. The class the 2026-09-17 slate arrived as — **a
non-finite value reaching a place that assumed it could not** — is
mostly discharged: #2967 took the two sketch guards and #3000 the four
refusal-floor doors. `vgeom/refusal-floor` narrowed the statement on
the way past (`log.md`, 2026-09-21): three of its four rows were not a
non-finite value getting in at all, but **a guard sited one arithmetic
upstream of the overflow it is for** — correct about what it looks at,
which is why each survived a sweep. The test that survives is *does the
door's own answer get asked the question the door's prose asks of its
inputs.*

After this wave the slate is two questions rather than one class:
**what a field's text does to the document**, and **what a count that
does not fit is called**.

**The shape precedent, restored.** CHROME's
`chrome/datums-substitution-sweep` (#2644) is the same substitution
class in `datums.rs`, and this program's own instance of it is
`vgeom/refusal-floor` (#3000): the door answers an `Option` or a
`Result`, the callers take the refusal through the nothing-to-do they
already have, and the doc comment says why a floor or a substitute
would be the wrong repair. **Read that shape rather than inventing a
second one.** The 2026-09-20 cut deleted the sentence that said so, and
a dispatch written the next day quoted this file as saying it
*"plainly"* when it did not say it at all
(`program-md-cites-a-plan-section-the-re-scope-deleted`). `datums.rs`
and `bounds.rs` stay CHROME's under its 2026-09-15 carve-out, so a row
on either is announced to CHROME before it is taken.

`the-shader-encodes-a-mark-strength-nothing-bounds` is **closed**:
#2808 put the bound in the TYPE (`MixFraction`) rather than in a second
WGSL spelling, and filed the lanes it could not reach as
`the-viewport-and-position-lanes-narrow-to-f32-with-no-door`, which
this wave takes.

## Charter

**Every row here sits on the path from a document's geometry to the
picture and to the figures printed beside it, and its defect is a
VALUE.** A number reaches a person, or the picture, as something it is
not — a NaN, an infinity, or a magnitude too large for its slot,
crossing a door whose own prose says it refuses such a thing and being
floored, capped or cast into a plausible figure instead; or a finite
number is rendered, fitted or cast at a precision that makes it a
different value; or a control never reaches the transform it names.

**The test that separates this program from its siblings.** A VGEOM fix
lands at the door that should have refused or converted, and what it
changes is what the viewer SHOWS. That is false of VNEWS, whose rows
are about the word a fact is spelled in and never about the value;
false of VSEAM, whose rows are about state that outlives the frame that
made it rather than a value wrong at one call; and false of VDOC, whose
fixes change no viewer behaviour at all. Applying it the other way: a
row belongs here only if a wrong number, or no number, reaches the
screen.

**Restored 2026-09-21.** This section was written at the 2026-09-17
re-scope and deleted by the 2026-09-20 priority-seam cut, which applied
the newly-opened-program template to a program that was not new. VGEOM
was the only one of VIEW's four successors left with no charter test,
which is what `work/vgeom/vgeom-plan-has-no-register-section-and-no-charter`
found. The text is this program's own statement of its subject and
asserts no external authority.

## The register

**`work/view/plan.md`'s rule register binds every lane dispatched from
this program, inherited BY REFERENCE and not copied.** Read it in full
before writing a dispatch. Three of its rules were earned on this
program's ground and a lane here will meet them: the δ round-trip rule,
the `desired_width` rule, and the fixed-precision-length census the
orchestrator got wrong by quoting rather than re-deriving.

It is not copied because a claim fixed in one place and stale in
another contradicts itself, and four copies of a register re-derived
every wave give four divergent copies inside a week. The register is
also evidence — every rule is a named failure at a named PR — and a
copy detached from the program that paid for it reads as a rule without
its receipt.

**What that costs, said plainly:** `work/view/plan.md` goes when VIEW's
directory goes at its exit walk, and this reference dangles that day.
The register's permanent home is
`work/view/the-lane-register-has-no-home-after-views-directory-goes`,
open on VIEW's slate and a precondition of VIEW's exit walk rather than
a follow-up to it. This section re-points when it lands.

**Restored 2026-09-21**, with §Charter above and for the same reason.
`work/vgeom/program.md` had gone on pointing every lane at this section
for a day after the cut deleted it.

## Review posture

**No A/B duals and no row in `docs/MODEL-AB-LOG.md`**, inherited from
VIEW unchanged: style reviews, with a second correctness reviewer where
a unit's failure mode is a confident wrong answer rather than a
refusal. The band 5300-5399 stays claimed for bookkeeping and is
expected to stay empty.

**This is a recorded answer, not an open question**, and the roster
says so in as many words. `docs/MODEL-AB-LOG.md`'s 2026-09-17 clause,
on the four programs of VIEW's re-scope: *"All four inherit VIEW's
posture verbatim (Ev, in-chat, 2026-09-04, reaffirmed that evening; the
VIEW parenthesis above is the roster line they inherit): no duals and
no row recorded … Each program's `plan.md` §Review posture states it."*
Ev reaffirmed it to this orchestrator in chat at the hand-over that
opened this session (*"still no AB protocol"*).

**The 2026-09-20 priority-seam cut overwrote this section with the
template for a NEWLY OPENED program** — *"OPEN, for this program's
first dispatch … the first orchestrator answers it here rather than
inheriting an answer"* — and VGEOM is not one. The same roster entry
that records that cut is explicit about which programs the v7 triage
question is open for: EMIT, GATHER and **FIT**, the three it opened,
while *"WIRE and VGEOM are NOT closed and keep their bands 3700-3799
and 5300-5399"*. So the template reached the parent as well as the
children. Corrected here on the roster's own words; no new decision is
being taken and none is owed.
