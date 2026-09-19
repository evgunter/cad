# VGEOM — the viewer's geometry, camera and numeric renders (plan)

**STATUS: OPEN (2026-09-17).** Opened in VIEW's re-scope, on twenty-one
rows that arrived by `git mv` with their bodies unchanged. Live state
is `work/vgeom/log.md`'s tail and the item files beside this plan,
never this file.

Branch prefix (the #396 convention): **`vgeom/`** — unit branches
`vgeom/<unit>-<slug>`, orchestrator branch `vgeom/orchestrator`.
Away-channel tag `(VGEOM orchestrator)`. A/B ordinal band
**VGEOM = 5300–5399**.

## Charter

**Every row here sits on the path from a document's geometry to the
picture and to the figures printed beside it, and its defect is a
VALUE.** Two clauses, both drawn from the rows:

**A number reaches a person, or the picture, as something it is not.**
A NaN or an infinity or a magnitude too large for its slot crosses a
door whose own prose says it refuses such a thing, and is floored,
capped or cast into a plausible figure instead:
`a-count-slot-launders-a-typed-nan-into-zero` (a typed NaN commits 0
past the finiteness refusal `props.rs` names),
`corner-count-substitutes-u32-max-for-a-length-it-could-not-cast`,
`finite-bounds-yield-an-infinite-scene-radius`,
`world-per-px-answers-a-scale-for-a-viewport-it-could-not-measure`
(`Some(NaN)` where `None` is the stated refusal),
`a-nan-edge-distance-wins-its-boundary-rather-than-losing`,
`pickindex-tie-break-rests-on-a-comment`,
`flatten-emits-every-vertex-before-it-judges-any-of-them`,
`sketch-headings-guard-zero-length-but-not-an-infinite-one`,
`the-shader-encodes-a-mark-strength-nothing-bounds`,
`renders-that-multiply-a-finite-guarded-length-spell-the-product-inf`,
`id-readback-failure-reads-as-nothing-under-the-cursor` (a failed
readback answered as a legitimate "nothing there"). Or a finite number
is rendered, fitted or cast at a precision that makes it a different
value: `a-fields-text-commits-within-the-renders-own-tolerance`,
`the-fields-door-has-no-width-bound-at-all`,
`cursor-projection-is-f32-in-a-module-whose-matrices-are-f64`,
`the-point3-to-gpu-corner-cast-is-at-three-sites`,
`pickindex-merges-parts-on-a-rounded-t-it-never-converts`,
`a-flat-rung-pair-is-read-as-flat-below-it`,
`the-budgets-predicted-count-is-not-always-an-over-count`.

**Or a control never reaches the transform it names.**
`ctrl-wheel-reaches-no-zoom` (the toolkit routes the gesture into
`zoom_factor_delta` and the viewport reads only `smooth_scroll_delta`),
`the-one-free-transform-is-the-only-total-door-in-camera` (the camera
header promises typed refusal and the transform it adopted is total),
`viewer-array-lowered-vector-ops-escaped-the-hand-rolled-sweep` (a
hand-rolled `dot` beside the shared one, in the class its own sweep was
sweeping for).

**The test that separates this program from its three siblings.** A
VGEOM fix lands at the door that should have refused or converted, and
what it changes is what the viewer SHOWS. That is false of VNEWS, whose
rows are about the word a fact is spelled in and never about the value;
false of VSEAM, whose rows are about state that outlives the frame that
made it rather than a value wrong at one call; and false of VDOC, whose
fixes change no viewer behaviour at all.

Applying it the other way: **a row belongs here only if a wrong number,
or no number, reaches the screen.** A number that is merely
inconsistently spelled in prose is VDOC's.

## Ev's requests — high priority

Filed 2026-09-17 from Ev's own list of UI nits, and **ahead of the
order below**: Ev asked for these directly, so they are taken before
anything else on this slate. Each row carries Ev's note verbatim.

None open: the two rows filed here landed in PR 2859 on 2026-09-19.

## Order

E-first, and the ordering is by the fail-loud class rather than by
file, because the class is the unit of work: eleven of the
twenty-one rows are one defect — a value the code cannot honour, floored
into one it can.

1. **The refusal-floor sweep, in one class** —
   `a-count-slot-launders-a-typed-nan-into-zero`,
   `world-per-px-answers-a-scale-for-a-viewport-it-could-not-measure`,
   `finite-bounds-yield-an-infinite-scene-radius`,
   `corner-count-substitutes-u32-max-for-a-length-it-could-not-cast`.
   Four sites, one argument, three files (`props.rs`/`pane/properties.rs`,
   `input.rs`/`camera.rs`, `camera.rs`, `gpu.rs`). CHROME's
   `chrome/datums-substitution-sweep` is the SAME class in `datums.rs`
   and is dispatched on CHROME's side: **announce before taking any of
   these, and read that unit's shape rather than inventing a second
   one.** The sweep rule that produces the population goes beside the
   claim, per the VIEW register.
2. **The two sketch guards** —
   `flatten-emits-every-vertex-before-it-judges-any-of-them` (a
   non-finite vertex emitted above the arc guards that would refuse it)
   and `sketch-headings-guard-zero-length-but-not-an-infinite-one`.
   One file, same shape, take together.
3. **The pick index's three numeric doors** —
   `a-nan-edge-distance-wins-its-boundary-rather-than-losing`,
   `pickindex-tie-break-rests-on-a-comment`,
   `pickindex-merges-parts-on-a-rounded-t-it-never-converts`. All three
   are `pickindex.rs`, which is VSEAM's too: the seam the index lives
   behind is theirs and these three doors are ours. Announce, and do
   not take these while `ui-thread-work-after-the-index-seam` is
   dispatched on VSEAM's side.
4. **The field renders** —
   `the-fields-door-has-no-width-bound-at-all` and
   `a-fields-text-commits-within-the-renders-own-tolerance`. The second
   is the residue of a closed row and is the narrow band; the first is
   the unbounded one. The VIEW register's `desired_width` rule applies
   to both: a width is not a character budget, and a fix that leaves
   the width alone is delivered clipped.
5. **The budget fit** — `a-flat-rung-pair-is-read-as-flat-below-it` and
   `the-budgets-predicted-count-is-not-always-an-over-count`. Both are
   about `scene::fit_delta` and both carry measurements; the register's
   rule that a ratio between two measurements taken at different
   settings is not a ratio was earned on this exact ground, and both
   rows' numbers are re-derived before either is dispatched.
6. **The casts and the homes** —
   `cursor-projection-is-f32-in-a-module-whose-matrices-are-f64`,
   `the-point3-to-gpu-corner-cast-is-at-three-sites`,
   `viewer-array-lowered-vector-ops-escaped-the-hand-rolled-sweep`.
   Three rows, one question: where the conversion lives.
7. **Held, and why.** `the-shader-encodes-a-mark-strength-nothing-bounds`
   is the first row in this crate whose defect is inside WGSL, where the
   existing Rust/WGSL parity row compares constants only — it sets a
   convention for how a shader claim is held and is not a lane unit
   until that shape is decided. `ctrl-wheel-reaches-no-zoom` and
   `the-one-free-transform-is-the-only-total-door-in-camera` each carry
   an undecided fork about the camera's door set.
   `id-readback-failure-reads-as-nothing-under-the-cursor` needs the
   chrome side of the answer and crosses into VNEWS's ground.

## Inbound

No row in `review` is this program's; `a-pick-over-a-stale-picture-…`
and `id-query-is-keyed-on-the-generation-…` are VSEAM's and
`joined-notices-…` is VNEWS's.

## The register

**`work/view/plan.md`'s rule register binds every lane dispatched from
this program, inherited BY REFERENCE and not copied.** Read it in full
before writing a dispatch. Three of its rules were earned on this
program's own ground and a lane here will meet them: the δ round-trip
rule, the `desired_width` rule, and the fixed-precision-length census
that the orchestrator got wrong by quoting rather than re-deriving.

The reason it is not copied is the register's own: a claim fixed in one
place and stale in another contradicts itself, and four copies of a
register that is re-derived every wave guarantee four divergent copies
within a week. The register is also evidence — every rule in it is a
named failure at a named PR — and a copy detached from the program that
paid for it reads as a rule without its receipt.

**What that costs, said plainly:** `work/view/plan.md` goes when VIEW's
directory goes at its exit walk, and this reference dangles that day.
The register's permanent home is
`work/view/the-lane-register-has-no-home-after-views-directory-goes`,
open on VIEW's slate, and it is a precondition of VIEW's exit walk
rather than a follow-up to it. This section re-points when it lands.

## Review posture

**Inherited from VIEW unchanged (Ev, in-chat, 2026-09-04, reaffirmed
2026-09-04 evening; `docs/MODEL-AB-LOG.md`'s roster line).** No A/B
duals, no row in `docs/MODEL-AB-LOG.md`; the band stays claimed and
empty. The default is a style review against
`docs/prompts/reviewer-style-lane.md`, with a correctness arm added
only where a unit's failure mode is a confident wrong answer rather
than a refusal — which on this program's ground is most of the slate,
because a floored NaN is a confident wrong answer by construction.

## Exit shape

Every row above landed or ruled out, and the viewer's conversion and
refusal doors named once each with the sweep rule that produces their
population beside them. The walk convention applies; residue re-homes
per `work/README.md`.
