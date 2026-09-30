# LINALG — the plan

geom-core's vectors, frames and interval conventions: the answers that are wrong at Interval

Opened 2026-09-20 by PROPS's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**20.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `interval-orthonormal-basis-sign-hull` | H | Vec3::orthonormal_basis returns a sign-hulled frame at Interval when n.z encloses zero |
| P0 | `nan-sign-is-not-stable-under-code-motion-so-d9s-fixed-order-covers-non-nan-only` | D | A Mat3/Affine3 product's NaN sign and payload differ between debug and release because LLVM commutes fadd across inline sites, so D9's fixed-order determinism holds for non-NaN outputs only |
| P0 | `pole-branch-pick-two-integer-shift` | H | chord_join sphere-pole branch pick hands shift_branch a two-integer shift at Interval when an entry azimuth lands on the previous exit |
| P0 | `sector-shape-mints-indeterminates-through-an-invalid-helper` | D | sector_shape mints Indeterminates through a local invalid() helper after a definite sign |
| P0 | `torus-meridian-orient-builds-a-frame-on-an-undecided-normalize` | E | torus_meridian_orient builds a hand Gram-Schmidt frame on an undecided normalize |
| P1 | `geom-core-linalg-has-no-array-doors` | E | geom-core linalg has no [f64;3] or [[f64;3];3] conversions, so every stored-array frame in editor-core lowers to Vec3/Mat3 by hand |
| P1 | `interval-backend-signed-zero-conventions` | D | The interval backend's signed-zero conventions: a stale inari comment at interval.rs, abs(−0.0) = −0.0, and * and / dropping the bit |
| P1 | `point3-has-no-order-and-vec3-no-sup-norm-door` | E | Point3 has no order and Vec3 no sup-norm door — the two spellings the tour lift sweep could not route through a door |

## Order

`interval-orthonormal-basis-sign-hull` first, and read FRAME's
sign-hull unit before specifying it: Ev ruled (option 1, on #1944)
that the frame constructor crosses the normal with a decided world
axis and transfers no sign, and that ruling governs this row's answer
as well.

Then `pole-branch-pick-two-integer-shift` and
`sector-shape-mints-indeterminates-through-an-invalid-helper` — both
throw away a sign or a branch the code has already proved, which is
the same mistake twice.
`nan-sign-is-not-stable-under-code-motion-...` is a DESIGN.md question
(how wide D9's determinism claim actually is) and should be put to Ev
rather than answered here.

## Review posture

OPEN, for this program's first dispatch. PROPS inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
