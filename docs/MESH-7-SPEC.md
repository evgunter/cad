# MESH-7 — issues 727 then 726: explicit iso-rectangle doors, and the SHAPE question folded onto the named predicate

**Binding at dispatch** (S-MESH program, `docs/S-MESH-PLAN.md`;
difficulty logged pre-draw: **M**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issues 727 and 726 are the primary specification, bounded by the
S-MESH Q3 ruling (`docs/S-MESH-PLAN.md` §Rulings — Ev, in-chat,
2026-09-01: **explicit doors** — no consumer keeps a transitive
floor; each door that needs the iso-rectangle premise cites
`props_rim_level` itself; `mass_properties` stays a mass-properties
door, not a de-facto gate). SMELL §D row **C11**
(`docs/SMELL-SCAN-2026-08.md`) is this unit's row; the orchestrator
updates it at merge. Issue 723 (the sphere extent premise the fold
would have imported) is CLOSED by CERT-1 — read
`crates/geom-brep/tests/cert1_sphere_polar.rs`'s header before
relying on that.

## Situation

`mesh::curved`'s `require_swept_rectangle` / `entries_off_bbox`
answer TWO questions with one banded spatial check on the walked
UV polygon:

1. **SHAPE** — *is this face's domain an iso-parameter rectangle?*
   S58 gave that property one home,
   `geom_brep::props::curved::require_rims_at_extremes`
   (`props_rim_level`, refusing `PropsError::NotIsoRectangle`).
   `mesh` re-derives it from the polygon and its bbox — a third
   derivation of a named property, the fragmentation S58 closed
   everywhere but here.
2. **WALK CONSISTENCY** — *did the walk produce a consistent
   polygon?* (#653's ulp wobble; the reason the bar is spatial.)
   This is `mesh`'s own question and stays.

And the doors are wrong-shaped (issue 727): `mesh` is protected
from a notched iso domain only TRANSITIVELY — the boolean refuses
`CurvedPierceUnsupported`, `import_step`'s tier-3 check 7 refuses
`VolumeUncomputable { NotIsoRectangle }` by calling
`mass_properties`. A lane whose floor is another lane's inability
to answer, with no line of its own to change when that inability
goes away — the pre-#648 arrangement. The ruling ends it: **each
door cites the predicate itself.** This unit implements the `mesh`
side.

## FIRST, before the build — the door census, reported

Write the table and report it to the orchestrator before building:
every consumer that assumes the iso-rectangle premise (start from
`mesh::tessellate_curved`, `mesh::trimmed`, the walk, the boolean's
curved-pierce door, `topo::validate`'s tier 3, `import_step`,
`props` itself) — for each: what it assumes, which door protects it
TODAY, whether that protection is its own or transitive, and whose
ground the door is (this unit's = `crates/mesh` + the predicate's
public face in `props/curved.rs`; the boolean's door is S-BOOL's;
tier 3 and `import_step` are `topo`'s / step-import's — recorded,
not edited). Include the predicted disposition of the
`walk::iso_side_starts` qualification (deliverable 4). If the census
finds a consumer whose floor this unit cannot make explicit without
leaving its fence, say so — the orchestrator files the residue; do
not widen.

## Deliverables

1. **The predicate gets a public FACE-LEVEL door in `props`** (name
   yours; e.g. a `pub fn` in `geom_brep::props` that takes a body and
   a face and answers `Result<(), PropsError>` — `NotIsoRectangle {
   what }` with the same `what` names the flux lane uses), built from
   the existing per-kind boundary classification and
   `require_rims_at_extremes`, computing NO mass properties. One home,
   one band (the `RimArms` levers — props' metering, not mesh's).
   State at the door that it is the S58 single-home predicate and
   that a plane face is not its question (`curved_face` already
   refuses `Plane` typed — keep that behaviour consistent). If the
   classification today is entangled with the flux derivation such
   that a shape-only door needs more than extraction, STOP and report
   with the measurement.
2. **`mesh` cites it — an explicit door (issue 727, the ruling).**
   `tessellate_curved` calls the predicate on the face BEFORE the
   walk and refuses TYPED on `NotIsoRectangle` — a mesh-side refusal
   that wraps the props one and names the predicate (variant naming
   yours; D2 addendum row 2, valid input / lane not built; the
   recourse is the certified-quadrature lane, as props says). The
   existing prose at `require_swept_rectangle` that says "both of
   those upstream limits can still move without a line changing in
   `mesh`" becomes false and is rewritten to say which line now
   changes.
3. **The fold (issue 726).** `require_swept_rectangle` /
   `entries_off_bbox` are reduced to the WALK-CONSISTENCY question
   only: the doc rewritten to say so; the payload kept (the distance
   is what separates "re-author" from "kernel bug"); and the
   statement issue 726 asks for — **which of the two questions each
   existing `UnsupportedCurvedDomain` row/refusal was answering** —
   written in the PR with **a row per direction**: a notched iso
   domain (keyway on a cylinder) now refuses at the SHAPE door with
   the props name; a synthetic walk wobble still refuses at the
   spatial check; a face that passes both meshes bitwise as before.
   Red-first: with the shape door removed, the keyway row goes red
   at the spatial check instead (or passes — measure and record
   which; issue 726's claim is that the two derivations are not
   coincidentally similar).
4. **The `walk::iso_side_starts` qualification — survives or is
   CLOSED, never lost** (the issue's own bold). The recorded defeat
   is an obliquely cut SPHERE whose every plane section is a `Circle`
   (so it is not diverted to the trimmed lane) and whose collapsed
   walk can present a bounding-rectangle polygon to a guard that
   should refuse it. With the shape door running BEFORE the walk on
   rim structure, props' sphere classification (`sphere_boundary`
   admits a circle only as a coaxial rim or a meridian) is expected
   to refuse that face as `NotIsoRectangle` — which would CLOSE the
   qualification. Demonstrate with a witness (an obliquely cut sphere
   through the door the qualification names — a STEP fixture, or a
   constructed body if the topo doors allow it) both ways: refused
   at the new door; and with the door removed, the walk's collapse
   behaviour recorded as the qualification describes. If props does
   NOT refuse it, the qualification SURVIVES: say why at
   `walk::iso_side_starts` and at the door, and report — do not
   quietly keep either text.
5. **`mass_properties` stays a mass-properties door.** No edit to
   `topo::validate` tier 3 or `import_step` here — but the census
   (above) records, per door, that tier-3 check 7's protection is
   now REDUNDANT for `mesh` (the mesh cites the predicate itself) and
   still LOAD-BEARING for whoever else leans on it. That table rides
   the PR and lands in issue 727 at close.
6. **D9 / behaviour**: the mesh bytes of every body that meshes today
   are unchanged (the two-build digest MESH-4 established, at the
   three ε rows, over the tour corpus and the suites' bodies). The
   shape door may only ADD refusals, and only on faces the spatial
   check would otherwise have refused OR on faces props already
   refuses through tier 3 — **a body that meshes on main and refuses
   at the new door is a FINDING** (props stricter than the polygon
   test on a real face): report it with the face, do not soften the
   door and do not accept it silently.
7. **ε posture** (issue 1356): the shape door's band is props' own
   (no new comparand, no new margin — say so); the spatial check
   keeps MESH-4's `Eps` operations (the inventory pin reds on a bare
   comparison). Three-ε battery; the trailer decision argued (the
   predicate's decisions can differ on the interval lane — ask for it
   or say why not).
8. **Class sweep** (discipline §5): every other site in `crates/mesh`
   that RE-DERIVES a property `props` names (the walk's sphere
   rim/meridian admission at `walk.rs` ~886 is a documented analogue
   of `sphere_boundary`; `lib.rs` ~179 cites the flux-sign analogue)
   — enumerate and disposition (fold / keep-with-reason / not this
   issue's). Do not act on the walk's classification beyond the
   qualification's disposition in deliverable 4.
9. **Issues 727 and 726 CLOSE at this merge** — say so in the PR
   (keyword hygiene: the orchestrator closes). If the census leaves
   a consumer whose door is not this unit's to make explicit, the
   PR names it and the orchestrator files the residue on 727 before
   closing.

## Acceptance

- The face-level predicate door in `props`; `mesh` refusing typed
  through it before the walk; the spatial check reduced to the walk
  question with the per-direction rows; the qualification
  demonstrably closed or explicitly surviving; D9 digest identical
  on every body that meshes today; the door census in the PR; hosted
  CI green; gate record per head.

## Hard rules

- NO `Co-Authored-By`, no model names. "issue 727" / "issue 726"
  spelled out, no closing keywords.
- Scope fence: `crates/mesh` (curved.rs, walk.rs ONLY for the
  qualification's text and its witness row, types.rs for the
  refusal, suites); `crates/geom-brep/src/props/curved.rs` +
  `props/mod.rs` + `lib.rs` re-exports for the public door ONLY (no
  change to any flux or closed-form derivation — `props/curved.rs` is
  Track R ground on this program's leave; `props/quad.rs`,
  `patch_bound.rs` and the area lanes are S-CERT's — do not touch).
  NOT: `topo::validate`, `import_step`, the boolean (S-BOOL's door —
  record the seam), `walk.rs` classification decisions,
  `docs/MODEL-AB-LOG.md` / `docs/S-MESH-*.md` / SMELL edits (C11 is
  the orchestrator's to update at merge).
- Re-merge main before opening the PR.
