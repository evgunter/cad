# BOOL-5 — issue 542: the rim-free spherical-wedge props arm

**Binding at dispatch** (S-BOOL program, `work/bool/plan.md`; difficulty
logged pre-draw: **M**). Read `docs/prompts/implementer-discipline.md`
in full before starting. The primary specification is the issue item
`work/bool/revolve-wedge-rim-free-band-volume.md` (issue 542) and the
slate entry in `work/bool/plan.md`; this document binds the unit.

## Situation

A revolve band face with NO latitude rims whose two meridians are NOT
coplanar — the partial revolve of an all-on-axis meridian, the natural
ball's wedge — makes tier-3 mass properties refuse: `sphere()`
(`crates/geom-brep/src/props/curved.rs`, the rimless arm) requires
`props_band_coplanar` (every meridian axis parallel to the first) and
sets `du = π`, the two-band face's closed form. A wedge whose meridians
are a band apart in azimuth is a legal face (tiers 1–2 pass, naming is
total) whose volume nothing computes: `VolumeUncomputable`, typed and
loud. Since M9-D1 (issue 530) the natural wedge is authorable, so the
scope limit is visible; `crates/sweep/tests/m9_d1_r2_probes.rs` pins
two shapes at tiers 1–2 only with a pointer to issue 542.

The flux lane's closed form is `R² · Δu · (hi − lo)` with the radial
term signed by the face side. On a rimmed face Δu comes from the rims
(`du_of_rims`); on the rimless two-band face Δu is π by construction.
The wedge's Δu is the azimuthal angle BETWEEN the two meridian planes —
a quantity the boundary states exactly (two meridian axes), which the
arm currently refuses to read.

## FIRST, before the build — two measurements, reported

1. **The wedge's admission set today.** Build the natural wedge through
   `revolve` at several partial angles (π/6, π/2, π, 3π/2 — the last
   two are the interesting ones: a wedge past π has the SHORT arc
   between its meridian planes on the wrong side) and through the Euler
   doors; record at which tier each refuses and under which name; run
   `examine_chart_coherence` and MESH-11's branch door on each (the
   wedge crosses both poles — is it one chart branch? the mesh door
   must keep refusing what it refuses today, bitwise). Report the table.
2. **What the meridian pair states.** For each shape, the two meridian
   axes (from `sphere_boundary`'s `meridian_axes`), the signed azimuth
   between them about the sphere's axis, and whether the face's
   `sense` bit plus the traversal direction determine WHICH of the two
   azimuthal arcs (θ or 2π − θ) the face covers — the flux lane must
   integrate the face's own arc, never the shorter one by assumption.
   State the rule you will decide by (a structural fact of the loop's
   traversal — never a value coincidence) and its named key.

STOP conditions: the side/arc rule cannot be read structurally from the
loop (a value-only rule would be a never-infer violation — report and
wait); a certified door lets a wedge past 2π reach the arm (that is
MESH-12's territory — cite `props_meridian_span_winding` and stop).

## Deliverables

1. **The wedge arm** in `sphere()`'s rimless branch: when the meridian
   axes are NOT coplanar, Δu is the azimuth between the two meridian
   planes on the side the face covers, decided under a NEW named key
   (`props_wedge_azimuth`, band and lever stated — the lever is the
   sphere radius; the coplanar case stays `props_band_coplanar` and the
   two-band closed form, bitwise unchanged); the side via the existing
   `SphereFluxSide::Sense` (the rimless face's sense bit IS the flux
   side — keep that contract). D2: a wedge whose meridians coincide
   (Δu → 0) refuses `props_face_extent`-style, not silently; a
   three-meridian boundary that is not two bands and not one wedge
   refuses typed under a named `what`. `docs/predicate-dimension-audit.md`
   gains the new key's row.
2. **The volume, checked against the closed form**: the wedge of a ball
   of radius R over azimuth θ has volume `(2/3)·R³·θ` and the band
   face's flux contribution follows; rows pin the natural wedge at the
   four angles against the closed form to the flux lane's pad, both
   signs of `sense`, both traversal directions (reversed loop ⇒ the
   complementary arc, `2π − θ`), on f64 and `--features interval`.
3. **The two probe sites promote to tier 3**
   (`m9_d1_r2_probes.rs`: `partial_wedge_pole_export_is_direction_safe_both_signs`,
   `partial_with_hole_exports_outer_poles_and_no_hole_poles`) and their
   pointer comments come out; red-first: run them at tier 3 against the
   unchanged arm first.
4. **Every consumer flips together**: `mass_properties`,
   `boundary_material_sign` (tier-3 check 6), the shape door
   `require_iso_rectangle` and MESH-11's `require_one_chart_branch` —
   the last two must NOT change their answers for the wedge (the mesh
   walk still cannot read a pole-crossing face; the flux lane can) —
   pin that the doors' answers are bitwise those of the merge base.
5. **D9**: MESH-4's two-build digest at three ε rows identical; the
   release tour byte-identical (`diff -rq`); no row count moves but by
   the rows this unit adds. CERT-1's rows untouched and green.
6. **ε posture** (issue 1356): one new key; state dimension, lever and
   band at the fn, the audit row and the PR.
7. **Class sweep** (discipline §5): every other rimless-arm consumer of
   `props_band_coplanar`; the torus rimless case (is there an analogous
   wedge on the torus that `fold_torus_meridians` refuses?) — measure,
   report, do not act.

## Acceptance

Both measurements reported before the build; the natural wedge reports
its volume at all four angles to the closed form, both senses, both
directions; the two probes at tier 3; the doors bitwise unchanged; D9
identical; hosted CI green; gate record per head.

## Hard rules

- NO `Co-Authored-By`, no model names; no closing keywords; "issue 542"
  spelled out (the orchestrator closes the item).
- Scope fence: `crates/geom-brep/src/props/curved.rs`'s `sphere()`
  rimless branch and its helpers (Track R fence ground, S-MESH's —
  taken by the recorded seam; disclose every Track R row's file
  reached), `docs/predicate-dimension-audit.md`, the geom-brep props
  suites, `crates/sweep/tests/m9_d1_r2_probes.rs` (the two sites and
  their comments only). NOT: `sphere_boundary`'s parse beyond reading
  `meridian_axes`, the pole helper, MESH-11's branch door, MESH-12's
  span decides, `certify.rs`, the mesh walk, the torus arm.
- Merges on green after the dual (no design surface).
- Re-merge main before opening the PR.
