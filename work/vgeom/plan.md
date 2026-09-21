# VGEOM — the plan

the viewer's geometry, camera and numeric renders

Re-scoped 2026-09-20 by VGEOM's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**22 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `a-count-slot-launders-a-typed-nan-into-zero` | E | A typed NaN in a Count slot commits 0, past the finiteness refusal props.rs promises names it |
| P0 | `a-fields-text-commits-within-the-renders-own-tolerance` | D | a field's text is accepted within the render's relative tolerance and committing it moves the value by that much |
| P0 | `a-nan-edge-distance-wins-its-boundary-rather-than-losing` | E | A NaN edge-pick distance is installed as best and never displaced, beating every legitimate candidate |
| P0 | `the-fields-door-has-no-width-bound-at-all` | E | number_text has no width bound at all: a large field value is spelled in hundreds of characters |
| P1 | `corner-count-substitutes-u32-max-for-a-length-it-could-not-cast` | E | A corner count that does not fit u32 is drawn as u32::MAX rather than refused |
| P1 | `cursor-projection-is-f32-in-a-module-whose-matrices-are-f64` | D | cursor_projection's home argument names f64 doors for an f32 function, and the f64-to-f32 matrix cast it needs has four spellings and no home |
| P1 | `finite-bounds-yield-an-infinite-scene-radius` | D | Bounds of a few hundred orders of magnitude pass both guards and return radius = inf as a scene radius |
| P1 | `flatten-emits-every-vertex-before-it-judges-any-of-them` | E | A non-finite vertex position is emitted unconditionally, above the arc guards that would refuse it |
| P1 | `pickindex-tie-break-rests-on-a-comment` | E | The pick tie-break's NaN disposition is a comment, not a guard |
| P1 | `renders-that-multiply-a-finite-guarded-length-spell-the-product-inf` | E | three chrome renders multiply a finite-guarded length and spell the product inf |
| P1 | `sketch-headings-guard-zero-length-but-not-an-infinite-one` | E | sketch.rs's two 2-D direction sites guard a zero length and not a non-finite one |
| P1 | `the-point3-to-gpu-corner-cast-is-at-three-sites` | E | three sites cast a Point3 to a GPU corner and the prose reconciling them names two |
| P1 | `the-shader-encodes-a-mark-strength-nothing-bounds` | D | The paint path a NaN actually reaches is the shader, and the Rust/WGSL parity row compares constants only |
| P1 | `world-per-px-answers-a-scale-for-a-viewport-it-could-not-measure` | E | world_per_px's height guard does not bound a NaN, so it answers Some(NaN) where None is its refusal |
| P3 | `viewer-array-lowered-vector-ops-escaped-the-hand-rolled-sweep` | E | the hand-rolled vector-op sweep missed its own dot half, and one datum row asserts something no value can break |
| P4 | `the-one-free-transform-is-the-only-total-door-in-camera` | E | camera's header promises typed refusal and the transform it just adopted is total, and the placement prose is the same length it replaced |
| None | `committed-profiles-are-not-drawn-in-the-viewport` | None | A profile vanishes from the viewport once it is committed (Ev-requested, high priority) |
| None | `datum-grid-lines-are-too-prominent-and-cover-profile-lines` | None | Datum grid lines are far too prominent and draw over profile lines (Ev-requested, high priority) |

## Order

By class, not by file: **every row here is a non-finite value reaching
a place that assumed it could not.** A typed NaN in a `Count` slot
commits 0; a NaN edge-pick distance is installed as best and never
displaced, so it beats every legitimate candidate; `world_per_px`
answers `Some(NaN)` where `None` is its refusal; bounds of a few
hundred orders of magnitude return `radius = inf`.

Take the four cheap P0 rows first — they are one unit and they are the
ones with user-visible consequences — then the guard rows beneath
them. `the-shader-encodes-a-mark-strength-nothing-bounds` is last and
is a design question: the paint path a NaN actually reaches is the
shader, and the Rust/WGSL parity row compares constants only.

## Review posture

OPEN, for this program's first dispatch. VGEOM inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
