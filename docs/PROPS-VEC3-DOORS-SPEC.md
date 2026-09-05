# PROPS vec3-doors — `const fn new`, `Affine3::from_frame`, `SketchPlane::map`; no `Vec → Point` conversion

**Binding at dispatch** (PROPS program; the item is
`work/props/vec3-point3-const-and-conversion-doors.md` — read it in full;
an **E rider**: single style review, outside the A/B experiment, no row).
Read `docs/prompts/implementer-discipline.md` in full. Branch
`props/vec3-doors`, cut from `main`.

## The decisions (the item's two questions, answered)

1. **`Vec2/Vec3/Point2/Point3::new` become `const fn`, generic as they
   are.** A `const fn` may carry a `T: Real` bound as long as its body
   calls no trait method, and these bodies are struct literals. Verify
   at the pinned toolchain (1.97.0) that
   `const P: Point3<f64> = Point3::new(1.0, 2.0, 3.0);` compiles, and pin
   it with a doctest that USES the constant (a `const` that is never
   read is not a pin). If the bound refuses `const`, stop and report —
   the fallback (inherent `const fn` on the `f64` types only) is a
   different shape and the orchestrator picks it.
2. **No `From<Vec3<T>> for Point3<T>` (nor the 2-D twin).** A point is
   not a vector; the affine/linear distinction is a kernel decision
   (D2-shaped) and an implicit conversion would let a displacement be
   read as a position at every `.into()`. `Point3::origin() + v` stays
   the spelling, and the reason is written ONCE at `Point3`'s type doc
   (one sentence, present tense: what the spelling is and why there is
   no conversion door).

## The door that is actually missing

`Affine3::from_frame(origin: Point3<T>, u: Vec3<T>, v: Vec3<T>) -> Self`
in `crates/geom-core/src/linalg/affine.rs`: the placement whose linear
columns are `u`, `v`, `u × v` and whose translation is
`origin − Point3::origin()` — the body `SketchPlane::from_frame`
(`crates/profile/src/lib.rs:~462`) carries today, moved down to the type
that owns it. `SketchPlane::from_frame` becomes
`Self::new(Affine3::from_frame(origin, u, v))`: one home, the same
operations in the same order, so the stored frame is BIT-IDENTICAL —
pin it (a row comparing the twelve stored components by bits over a
corpus of frames incl. signed zeros, the `SketchPlane::origin()` doc's
concern). The doc at `Affine3::from_frame` states the caller's
obligation (`u ⊥ v`, unit — unchecked, as today) and that the third
column is computed, not stored from the caller.

`demos/tour/src/skinned.rs::normal_start_place` (`:246-258`) builds a
`SketchPlane` only to take `.placement`; it now returns
`Affine3::from_frame(path.eval(lo), u, n.cross(u))`. `f64` only;
bit-identical by construction (say so, measure once).

## The lift

`SketchPlane<T>::map<U: Real>(self, f: impl Fn(T) -> U) -> SketchPlane<U>`
through `Affine3::map` — the lift of the type that carries a frame, so
a frame built at one scalar lifts ONCE rather than component by
component at the door. Its doc says what the lift means: the stored
normal was computed at the SOURCE scalar and is lifted as a value,
which is not the same as constructing the frame at the target scalar
(at `Interval` the cross product of point intervals has rounding
width; the lifted one does not) — a caller chooses, and the doc names
both spellings. **Do not edit `demos/tour/src/lily.rs`**: its two
`from_frame` sites are the tour-wide layer-rule sweep's
(`work/issues/tour-scenes-lift-componentwise-not-through-map.md`) and
the choice above is that sweep's to make per site; name them in the
body as the consumers the door was minted for.

## Fence

`crates/geom-core/src/linalg/{vec,point,affine}.rs` (PROPS' linalg
lane), `crates/profile/src/lib.rs` (`SketchPlane` only — announced by
this spec; the orchestrator posts the seam), `demos/tour/src/skinned.rs`
(one function; the tour is Track X ground, the lily precedent).
`crates/pncad/src/authoring.rs` is READ (the `p2/p3/v2/v3` doors exist;
nothing is added there) and not edited.

## Posture

- Red-first: the `const` doctest (red on `main`: "calls in constants
  are limited to constant functions"); the bit-identity row is a
  measurement pin, not red-first — say so.
- ε posture: none. No `CI-Config:` trailer.
- D2-addendum: nothing retired; the refused door (`From`) is a
  decision recorded at the type, not a refusal path.
- Sweep obligation (discipline §5): every site in `crates/`, `demos/`,
  `tools/` that builds a `SketchPlane` only to read `.placement`
  (`grep -rn "from_frame(" | grep placement`, then read), and every
  struct-literal `Point3 { x:, y:, z: }` / `Vec3 { .. }` in a `const` or
  `static` position that `const fn new` now admits (`grep -rnE
  "(const|static) .*: *(Point|Vec)[23]"`); hit list with dispositions
  (convert only in the fence; name the rest); what the pattern cannot
  match.
- Review: single style review (`docs/prompts/reviewer-style-lane.md`),
  outside the experiment.
- Landing: the item `status: closed` with a `## Closed` section that
  records decision 2 as a ruling (so it is not re-asked); the spec is
  deleted at merge with its `## Per-merge deletion` line in
  `docs/DOC-LEDGER.md`; no `Co-Authored-By`; push early to
  `props/vec3-doors`.

## Acceptance

`const fn new` on the four types with a using doctest; one home for
the frame construction with the bit-identity row; `SketchPlane::map`
with the two-spellings doc; `skinned.rs` on the new door; the `From`
door refused in writing at `Point3`; hosted CI green on the full matrix.
