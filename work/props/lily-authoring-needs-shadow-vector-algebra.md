---
id: lily-authoring-needs-shadow-vector-algebra
kind: unit
title: API friction — authoring the lily naturally meant building a shadow vector algebra beside Vec3
status: spec
branch: props/lily-vec3
opened: 2026-08-20
github: 796
refs: [757, 758, 759, 787, D79]
---

## From GitHub issue 796

Opened 2026-08-20; 0 comments.

A **library finding** from the demos, in the shape `memories/demo-purpose.md` requires: *"awkwardness met while writing a demo is a LIBRARY FINDING… never quietly work around it"*. Same class as #757/#758/#759 (S79). Surfaced by the style review of #787 (Track G lane G-b) during the first end-to-end read anyone has given `demos/tour/src/lily.rs`; the duplication half is recorded as smell-scan **S130(b)**, this half is the library question and belongs here.

## What the demo does

`demos/tour/src/lily.rs` — the fairy lantern, the tour's most geometrically involved scene and the one written most like a user modelling a real object — works almost entirely in bare `(f64, f64, f64)` tuples with hand-inlined vector algebra, and converts to the kernel's own types only at the API boundary:

| helper | site(s) | what it re-implements |
|---|---|---|
| `nrm` | `:225` | normalize |
| `rot` | `:171` | rotation about an axis |
| the radial-frame builders | `:611` (`bud`) and `:1184` (`sepals`) | an orthonormal frame from an axis |
| `v3` / `pt3` | `:209,213` | the conversion to `Vec3` / `Point3`, applied at the last moment |

**The duplication is gone; the finding is not.** `nrm` was two
byte-identical closures (then at `:395-397` and `:972-974`) until
SMELL Track X's `D79`(b) collapsed them to the single free fn above.
That was duplication removal only — nothing became `Vec3` — so every
row of this table is still a tuple helper standing in for the kernel's
own type, which is what this issue is about. The two radial-frame
builders are still spelled twice, and are not byte-identical: their
inputs differ in shape, so merging them is a change rather than a
collapse.

The same file uses `Vec3::norm` and `Vec3::cross` freely inside `mod review_probes` — i.e. when *checking* the geometry rather than *authoring* it. That asymmetry is the finding: the kernel's vector type was reached for when reading results back and avoided when composing them.

## Why this is evidence about the library, not about the file

The demo was not written to dodge anything; the author reached for tuples because, at authoring time, they were the shorter path. If composing a rotation, a normalize and a radial frame is more ergonomic in raw tuples than in `Vec3`, that is a fact about `Vec3`'s surface, and it is the fact a user modelling a plant would hit on day one. Candidate causes, none confirmed — this issue is the observation, not the diagnosis:

- no `Vec3::normalized()` / `Vec3::rotate_about(axis, angle)` on the type, so each has to be spelled out either way;
- no "orthonormal frame from an axis" door, which the file builds twice and `crates/geom`'s `azimuth::frame` builds a different version of (smell-scan S116(b) is about that one's transposed-destructure hazard);
- generic-scalar friction: `lily` is generic over `Scalar` and the authoring is in `f64`, so `Vec3<S>` would need `S::from_f64` at every literal — which may be the whole answer, and if so it is worth saying out loud, because it means *the kernel's vector type is awkward in exactly the generic code the kernel wants people to write*.

## What would close it

Either a door (or three) that makes the tuple detour unattractive, and `lily.rs` rewritten through it as the check; or a recorded finding that the detour is correct for demo-side narration code, stated at `lily.rs`'s header so the next reader does not re-derive it. Both are answers; silence is not.

**Not fixed in #787** — that lane's scope is `demos/`, and rewriting the lily's algebra is neither a style fix nor its call to make.

— Claude (smell-scan Track G, lane G-b)

## Decided (PROPS orchestrator, 2026-09-05 — a read-only census)

**No `Vec3` door is missing.** Every tuple helper in `lily.rs` has an
exact door today: `nrm` → `Vec3::normalize`; `v_len`/`v_dot`/`v_cross`/
`v_sub` → `norm`/`dot`/`cross`/`Sub`; `blade_frame`'s Gram–Schmidt →
`reject_from` then `normalize` then `cross`; `rot` (a rotation in the
xz-plane) → `Mat3::rotation_about(unit_y, a)` applied to the lifted
vector. The frame-from-an-axis builders pin `e1` to a scene convention
(the xz-plane) that no kernel door should choose for the caller, and
they are two lines of `cross`/`normalize` each.

**The friction is real and it is not a door**: `lily` is generic over
`S: Scalar` (transitively `Real`), the scene is described in `f64`
literals and `f64` trig, and `Real` has no mixed-scalar arithmetic, so
authoring straight into `Vec3<S>` puts `S::from_f64` on every literal —
which is what pushed the author to tuples. The right layer is the one
the author found: **compose the scene at `f64`, lift at the API
boundary** — but in `Vec3<f64>`/`Point3<f64>` with the kernel's own
doors, lifted through `Vec3::map(S::from_f64)`, not in bare tuples.
`skinned.rs`'s `normal_start_place` already does the same frame
construction in `Vec3<f64>`, non-generically, which is the shape the
whole tour should share.

**The unit** (`docs/PROPS-LILY-VEC3-SPEC.md`): rewrite `lily.rs`'s
authoring half through the doors above, delete the tuple algebra, and
state the layer rule at the file header in one paragraph. That is the
check the item asked for. At landing, `D79`'s member (b) is deleted
member by member (the row keeps (f), parked on `L2`); this item closes.
An E rider: single style review, outside the A/B experiment.

## Home

`work/issues/`: the question is about `geom_core::Vec3`'s surface — kernel ground LIB's `keep_out` explicitly cedes to VERBS and SEAT, neither of whose charters names it — while the demo-side half is already carried by the code-quality row `D79`, which cites this issue as its reason.
