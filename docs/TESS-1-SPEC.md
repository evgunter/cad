# TESS-1 — a meridian-free curved face refuses typed (spec)

Binding on the implementer, alongside
`docs/prompts/implementer-discipline.md`. Item: `work/tess/TESS-1.md`.
Evidence: `work/tess/rim-only-sphere-cap-panics-at-census.md`
§Measured, and branch `tess/rim-only-cap-diag` (`83833e586`), whose
rows and fixtures you lift.

## The claim

The swept-rectangle lane (`curved::tessellate_curved` over
`walk::loop_polygon`) meshes a face whose boundary walk has BOTH kinds
of traversal: rims give it its u-extent, meridians its v-extent, and a
pole is a meridian's endpoint (the walk's module-doc premise). A loop
that is rims only has no v-extent. Today that loop walks to a
zero-height polygon, passes `require_swept_rectangle` (every entry is on
its degenerate box), triangulates to nothing, and `tessellate` returns
`Ok` with a hole where the face is — caught only by the debug-profile
census, as a panic whose message blames the neighbouring face.

After this unit such a face refuses, typed, before anything is emitted,
in every profile.

## What decides it

**The structure of the traversal list, and nothing read from a float.**
The refusal fires iff the face's single loop classifies with no
`TravKind::Meridian`. Not `area2 == 0`, not `v0 == v1`, not a triangle
count: those are consequences, and the never-infer doctrine wants the
rung that causes them. `walk_anchor`'s doc already names the shape ("a
single cyclic run with no opening … such a loop has no meridian") and
calls it unreachable; it is reached through STEP import and the Euler
door, and that sentence is corrected.

## Deliverables

1. **A typed refusal** on `TessellateError`, raised from the walk (or
   from `tessellate_curved` on the walk's classification — wherever the
   traversal list is first in hand; say which and why), carrying the
   face and the surface kind. It is D2 addendum row 2 — valid input,
   unbuilt lane — and its doc says so, names the structural fact, and
   says what a caller can do (the seamed statement of the same face
   meshes: two half-faces on meridians through the pole, which is what
   every native verb mints). A NEW variant, not `UnsupportedCurvedShape`
   (whose `source` is props' refusal, and props admits the sphere cap)
   and not `UnsupportedCurvedDomain` (the walk did not leave a box).
   Every exhaustive consumer of `TessellateError` takes the arm —
   the python tag table and whatever else the compiler names; list them
   in the PR.
2. **Rows**, lifted from `tess/rim-only-cap-diag` and made assertions:
   the sphere cap + disc at both poles and more than one latitude, the
   two-cap sphere, MESH-12's `two_level_rim_cap` at Δv = 0, the cone
   apex cap, the one-rim cylinder face, and the STEP fixtures
   (`rimonly1.step`, `rimonly2.step`) imported then tessellated. Each
   asserts the variant and its payload, not `is_err()`. **Q3 applies**:
   with the guard deleted, the debug profile must go red on these rows
   (the census panics, or the variant is absent) — run that mutant once
   and say in the PR what it printed.
3. **One positive control beside them**: the seamed twins (revolved
   dome, `ball ∩ slab`) still mesh watertight, so the refusal is shown
   to separate the two statements of one solid rather than to refuse
   caps.
4. **The torus.** A torus chart is periodic both ways; measure what a
   one-loop torus face whose traversals are all of one kind does today,
   and either bring it under the same refusal by the same structural
   fact or say in the PR why the fact does not transfer. Do not guess.
5. **The census message** in `tessellate_impl` states what it counted
   and stops asserting a cause it cannot know (an unpaired segment has
   two causes: faces that did not identify an edge, and a face that
   emitted nothing).
6. **Prose that this unit makes false or true**: `walk.rs`'s module-doc
   pole premise (now enforced — say by what), `walk_anchor`'s
   "unreachable", the census comment's "collapses onto one rim level"
   sentence, and the module header of
   `crates/topo/tests/mesh12_rim_row_reach.rs`. Comments state the
   invariant, not this history.

## Sweep (discipline §5 — assume it is a class)

The shape is **a tessellation lane that can return `Ok` with an empty
patch for a face with a non-empty loop**. Read `planar.rs`,
`trimmed.rs` and `curved.rs` for every path to `Ok(patch)` and say, per
path, what stands between a degenerate polygon and an empty emission.
Hit list with dispositions in the PR body, and what your reading could
not see. A hit outside this unit's structural fact is FILED, not fixed.

## Scope

In: `crates/mesh/*`, the named `topo` test header, consumers the
compiler forces. Out, and do not start: emitting the cap (the interior
pole) — that and import normalization are one open design question with
Ev; `check_mesh`'s answer on an empty mesh
(`work/tess/check-mesh-passes-the-empty-mesh.md`); the doors in
`geom_brep::props::curved` (PROPS'). `docs/guide/meshing.md` is LIB's:
if it enumerates refusals, file a row on LIB's slate rather than edit.
File `crates/step-import/tests/poleguard.rs`'s stale door enumeration
(the cap row's last paragraph) on its owner's slate.

## Acceptance

Hosted CI green at the full matrix (twelve test jobs, five k-lint jobs,
read at the STEP level). A mesh byte that moves on any tour/wild body is
a finding about this spec — none should, since no corpus body carries
the face — so say what moved and why if one does. The tess-budget
baseline and demo pins are other programs'; re-pin nothing silently.

## Review

Full v6 dual; the orchestrator dispatches it on your frozen head. NO
Co-Authored-By trailer in lane commits (blinding overrides the harness
convention); if one lands in a pushed commit, note it in the PR body and
carry on. Your report and PR body never name the model you are.

## Landing

Status `review` on `work/tess/TESS-1.md` when the PR opens (set `pr:`
and `branch:`). Do NOT merge and do NOT close the item: the
orchestrator merges after the dual and the union fix pass, closes the
unit and deletes this spec at merge.
