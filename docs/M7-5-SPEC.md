# M7-5 — Band-seam re-mint (seamless periodic bands)

Orchestrator work order for the M7 band-seam unit, recorded in-repo
per working convention. Substrate evidence: the band-seam substrate
inventory (file:line for every claim below was verified against the
lane at implementation time).

## The unit

OCC-exported STEP never splits periodic faces: a lateral
cylinder/torus band arrives as ONE face with 2 full-period rim bounds
and NO seam generator. The import gate at
`crates/step-import/src/entities.rs` (the multi-bound curved-face
refusal) refuses it typed ("seamless periodic band"). Two wild-corpus
files are pinned as refusal fixtures for exactly this class:
`cq_red_cube_blue_cylinder.step` (1 cylinder band, face #54) and
`nist_ftc_11_asme1_rb.stp` (4 bands: cylinders #95/#135, tori
#175/#187, rim edges SHARED across bands — #128 joins #135+#175, #91
joins #95+#187). This unit mints the seam generator at normalize
level (the apex_cone/full_torus/edge_free_face sibling) and flips
both fixtures to first-class imports. The native target shape: ONE
face, one loop `[rim_hi…, seam⁻, rim_lo…, seam⁺]` — the seam edge
used twice, `EdgeGeometry::Seam` adopted by the existing rung
(adopt.rs), per-half-edge pcurves already designed for this.

## Binding design decisions (ruled by the orchestrator)

D1. **Seam azimuth = the surface's own u_ref azimuth, always.**
Never re-chart u_ref to a rim vertex — `Seam` is SPATIALLY the u_ref
half-plane (ratified convention; certification meters
SeamHalfplane/SeamSide against it), and rewriting the file's chart
placement to dodge a split would mutate imported geometry beyond
need. Where a rim has no vertex at the u_ref azimuth, SPLIT the rim
there (generalize `split_at_midpoint` to split-at-parameter;
`expand_split_uses` patches every sharing face — load-bearing for
ftc_11's shared rims). Consequences for free: seam endpoints lie in
the half-plane by construction; ftc_11's cylinders (vertices AT
u_ref) need zero splits; cq + ftc_11 tori take their unavoidable
splits.

D2. **Winding: mint-side.** Derive the band's winding via
`chart_direction`, cross-check against `same_sense`, and REFUSE an
inverted band pre-body with a typed error (the full_torus posture) —
never ship it to kernel check 6 (that stays the backstop). Applies
to both chart types (wall_inversion doesn't cover tori; this mint
does).

D3. **Scope: cylinder + torus bands** — the measured need; the
refusal survey confirms only these two fixtures are band-class.
Detection recognizes the band SHAPE chain-agnostically (each of the
2 bounds wraps the chart's full period, via `chart::uv_of`); minting
handles what the fixtures need; cone/sphere-zone bands and any
harder chain configuration keep a typed refusal whose message names
this unit's pattern as the recourse. The entities.rs carve-out
passes ONLY detected bands; the genuine ring/NURBS refusal stays
(dm1's 7 multi-bound NURBS faces must still refuse — reword the
message honestly since "is not done here" retires).

## Sub-units (commit + push after each)

1. **Spec-in-repo + detection + gate carve-out**: this document in
   the first commit; then band detection at `face()`, tagged through
   to shell level.
2. **The band seam mint** (new normalize.rs pass beside
   apex_cone/full_torus): split rims at u_ref azimuth where needed
   (split-at-parameter + expand_split_uses; endpoint params via
   `geometry::endpoint_params` within the ε_in budget), mint the
   generator EdgeSpec (cylinder → ruling Line segment; torus →
   minor-circle meridian arc at u_ref), rewrite to ONE single-loop
   FaceSpec honoring D2, record a new `StructureNormalization` kind
   (SeamlessPeriodicBand) with census mapping. Update normalize.rs
   module docs — the "sub-arc of a carrier the file states" wording
   must widen honestly to cover minted rulings (same license as the
   sphere's minted meridians; say so).
3. **Fixture flips + oracles**: derive BOTH `.expect` sidecars via
   FreeCAD (protocol per `nist_ftc_09_asme1_rd.stp.expect`). Include
   KERNEL_* override lines where the minted census diverges from
   OCC's counts (expected — shared-rim splits change counts; derive
   them, don't fudge). Move both files WILD_REFUSALS → WILD_IMPORTS;
   replace `the_periodic_band_refusal_is_a_named_kernel_gap` with a
   positive normalization-reporting pin; `no_wild_file_panics` stays
   at 13; update the dialect pins. S9 duty: the three sites that
   state the gap (entities.rs refusal text, normalize.rs docs, the
   named-gap test) each get their retiring update — no stale "not
   done here" text survives.

## Acceptance (all executed foreground, numbers in the report)

- wild.rs full suite green including: both flipped fixtures
  importing first-class (census + volume vs FreeCAD oracle + tiers
  1/2/3), the wild ε-window rows ([1e-10, 1e-8] must keep holding —
  ftc_11 is a NIST 12-digit-truncation file), no-panic at 13, the
  normalization-reporting pin.
- Seam certification passes on minted seams
  (SeamSurface/SeamHalfplane/SeamSide) — exercised through the
  import path at the standard 3 ε rows (the step-import suite's ε
  matrix: default 1e-9, 1e-6, 1e-12).
- probe_vol's imported⇒measurable obligation holds on both fixtures
  (RingOnCurvedFace unreachable for them).
- Existing suites unchanged-green: step-import (own-corpus
  round-trip, freecad, corpus_fold); topo/geom-brep untouched
  (EXPECTED — kernel-crate edits are a STOP-and-report event).
- Local battery scope (time-to-signal): the step-import suite + the
  wild 3-ε rows foreground; hosted CI proves the rest. No
  full-workspace battery.
