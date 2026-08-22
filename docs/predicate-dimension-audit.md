# Predicate-comparand dimensional audit (M7, rim-dimensional unit)

Decision-boundary comparands in `geom-brep` and `topo` — every
`classify` / `require_zero` / `require_extent` / `decide` funnel call
and every raw `sign_within` use in shipped code — audited for the
ratified ε semantics (D4): **a margin classified against the linear
band must be a LENGTH in meters — the point deviation from specified
geometry**. Angles and dimensionless quantities meter through a named
lever arm (θ·r); squared quantities are rooted (or divided by a length,
the `/2r` linearization) before comparison; a product of two lengths is
an area-dimensioned defect (the class of the fixed `du_of_rims` bug and
the #89 in-band landing).

**Not *every* comparand, and not the workspace's.** Two bounds, both
deliberate, both measured rather than asserted — see *Coverage of the
two crates, measured* below the head matter. **(1) Two crates.** The
sweep reads `geom-brep` and `topo` and stops, because the dimensional
argument is made where the comparand is built and the out-of-ledger
crates make theirs at their own sites; the clause-(i) note below says
so of `profile`, `sweep`, `editor-core`'s eval/naming and
`geom-curves`, and the funnel-bypass paragraph after the tables calls
its scoping *"on purpose"*. The bound binds the **table**, not the
document: F12, F13, F14 and F15 are `editor-core` rows, carried here
because this is where the argument that named them lives. So **this is
not the workspace's predicate-name roster** — that is
`docs/K-REPORT.md` § *"The inventory method, restated"*, whose seven
orphans are all outside these two crates by construction (`profile`
×2, `sweep` ×2, `demos/tour` ×3), a fact *about* this bound and not a
hole in it. **(2) Not complete inside them.** Of **246** funnel-reaching
names in the two crates the tables and their prose reach **223**, carry
an individual `dim` verdict for **121**, and miss **23** entirely; the
23 are listed under *Uncovered names* below the tables and are §D's
**D46**. The word *every* was this document's for a year and it was
never true of the second bound; it is retired here rather than
footnoted.

**One qualifier the first paragraph's *"every raw `sign_within` use"*
leaves open**: the sweep covers **shipped** code — `topo/src/seqgen.rs`'s
candidate filter is a raw `sign_within` under `#[cfg(test)]`, never
instantiated at the recording scalar, and is outside it.

Trigger and method: the `props_rim_level_group` defect (fixed in this
unit — `crates/geom-brep/src/props/curved.rs`, the `RimLevel` enum)
metered a cone's already-length rim-level difference by `× arm`,
manufacturing an area. This document is the systematic sweep for the
rest of the family, and the input to the typed-margin (Margin-typed
classify seam) design conversation. Status column: OK (dimension
verified), FIXED (corrected — in the originating rim-dimensional unit
unless the row names the follow-up unit that did it), FLAG (defect or
concern found — disposition in the findings section).

**Living document.** A row and its disposition entry must never
disagree; retiring a finding updates both. Retired so far: F1 plus the
eight inline class-(c) sites (rim-dimensional unit, #197); F5 and most
of F6 (M6-3, #192); F3 and F4 (the F3+F4 dimensional unit — F3 was the
tree's last funnel bypass, so predicate-name attribution in the K
telemetry is now complete); F6's residue and F7 together (the typed-
margin fold-in — one quantity, the NURBS carrier's metric rate, that
three sites had handled three different ways).

**Known rot in the `file:line` column, recorded rather than swept
here.** Every pointer in the 143-row table below is hand-written, and
no test, lint or CI row checks any of them: an edit to a cited file
silently shifts every row below it. The two `splitting/rules.rs` rows
were re-resolved in #661 because that PR moved them (+2 lines) — and
re-resolving them showed the class in miniature: two of their four
pointers had been exactly right on main and were moved by this PR, and
the other two were **already stale by 3** before it. `DESIGN.md` cites
this audit as the migration ledger, so the pointers are load-bearing
for readers even though nothing enforces them. A sweep that re-resolves
every row — or that replaces line numbers with grep-able predicate
names, which do not move — is owed, and is not a side errand of
whichever PR happens to touch a cited file: **it needs its own unit.**

**Clause-(i) migration (the margin dimensional convention's typed
seam — executed by the margin-migrate unit).**
`geom_core::k_stats::decide` now takes a `Margin<T>` by signature;
every call site in the workspace constructs its margin through a
blessed door (`of` / `levered`+`sagitta`+`levered_inv` /
`norm3`+`norm2` / `metered` / `over_lever`; an unused `rooted` door
was dropped at the fix pass — dead surface, and `per_boundary` was
RENAMED and re-scoped to `over_lever` when Evan's layering ruling moved
the consistency backstops out of the seam, see below), and each row's
"comparand" column is the door's justification (out-of-ledger crates —
profile, sweep, editor-core's eval/naming, geom-curves — argue their
doors inline; their comparands are the same length shapes). Rows whose
comparand this ledger FLAGS as not-a-length are carried through the
seam by `geom_core::k_stats::decide_flagged(name, margin, band, row)`
— the finding lane: no `Margin` is constructed, the row id is a
compile-time argument at the site, and grepping `decide_flagged`
enumerates the clause-(i) debt exactly (F2 ×4,
F10 ×1 — one loop over seven rigidity residuals — F13 ×1, F14 ×1,
F15 ×1 —
8 shipped sites, tracked as issue #214 and pinned by
`geom-core/tests/flagged_census.rs`: no new site ships without a row
here, and the count only moves together with this section).

**The `- **FNN**` bullet headings in *Findings (dispositions)* below are
machine-read** (#801). `flagged_census.rs` extracts the fourth argument
of every shipped `decide_flagged` call and requires it to name one of
them, so a row renumbered, retitled out of that bullet form, or never
written fails the suite rather than sitting as an unresolvable citation
in the kernel. Reformat the headings and the census says so — it refuses
to pass on an empty row set rather than reporting a hole of zero. The
site COUNT above is still hand-synced with the constant in that test;
that half derives from nothing and is §S13's *magic count*.
Composition disclosure (review MIN-2): `Margin` deliberately has no
arithmetic, so a margin whose FINAL op is a plain length
sum/difference/min/max may carry a lever, root, or quotient INSIDE the
`of` argument — six shipped sites do (each dimensionally verified, all
named in the PR #213 census): `props/curved.rs` sphere `props_rim_fit`
(inline root), cone `props_rim_fit` (inline lever `|v|·sinα`), cylinder
`props_meridian_on_surface` (inline norm), `fillet/battery.rs`
`fillet3_radius_headroom` (inline quotient `r²/arm`) and
`fillet3_spine_regularity` (inline lever `r²·κ`), and
`pcurve_cache.rs`'s sphere `polar_rate` fallback (inline `·aa/radius`).
Structural debt to keep named, not a defect: full door-composition
would need Margin arithmetic, which the convention refuses. The
recorded margin stream is bit-identical by construction (each door
performs exactly the operation the bare site performed); the
probe-census diff row is the executed proof. F12 stays OUTSIDE the
seam by its unchanged disposition (below).

**The invariant lane (Evan's #213 layering ruling).** The consistency
backstops — `volume_backstop` / `volume_backstop_operand` /
`volume_backstop_violation`, inequalities between integral RESULTS
(wrong-component detectors, never accuracy gates) — are outside the
length seam by design: they decide on bare `T` through
`k_stats::decide_invariant` (no `Margin` minted — not a door, not
debt), keeping their predicate names and margin values byte-identical
in the K stream, and a certified violation surfaces as the
Corrupt-class `ResultVolumeImplausible` ("kernel invariant violated —
this is a bug", with a report affordance), separated in type and voice
from every validity refusal. The former `per_boundary` door is renamed
`over_lever` and re-scoped to the genuine geometric decisions
(mean-width 2A/P, mean-thickness V/A containment, chart-orientation
areas, the crossing advance).

Factor conventions used throughout (verified against definitions):
`Curve3::Line.dir` unit ⇒ line parameter is arc length (m);
`Circle`/`Ellipse` parameters are radians with radii/semi-axes in m;
all stored surface axes/normals/`u_ref` unit; `implicit_residual` is
`/2r`-normalized to meters; `implicit_gradient` unit on-locus;
`curvature_lever_arm` meters; `TangentJet.kappa_rel` 1/m;
`speed_lower_bound()` meters per parameter unit.

## Coverage of the two crates, measured

**Against main at `43e2998d`** — carried explicitly, because this is a
survey written into the tree it surveys (§D's D23). *(Taken three
times: `f87b203`, then `a0a6e1a5` after main moved `census.rs` and
`splitting/rules.rs` under it, then here after `topo` gained `live.rs`
and rewrote `split.rs`. Every figure reproduced at all three.)*

**What is in scope.** K-REPORT's restated rule: a name is in scope if
it reaches the `geom_core::k_stats` funnel *however it is spelled at
the call site*. In these two crates that is ten spellings — `decide`,
`decide_flagged`, `decide_invariant`, and the `check_residual` /
`classify` / `classify_len` / `require_zero` / `require_extent` /
`gap_is_zero` / `signed_is_zero` wrappers.

**Names — the deliverable.** **246** distinct predicate names: **210**
written as a literal at one of those spellings, and **36 carried** by a
module-private `const`, a struct field or a local table
(`sector_shape.rs`'s three consts, `ray_parity::ParityRows` twice over,
`transform.rs`'s two arrays, `carrier_eq.rs`'s margin tuples,
`contact_verify.rs`'s and `splitting/order.rs`'s loop tuples,
`pcurve_cache.rs`'s winding closure, `pcurves.rs`'s two closures).

| the document's relation to a name | count |
|---|---|
| carries an individual row with a `dim` verdict | **121** |
| named only in prose, no `dim` column | 3 |
| reached only through a family cell or slash-list | 99 |
| **reached, on the most generous reading** | **223** |
| **recorded nowhere, under any reading** | **23** |

**Reach is not verdict, and the asymmetry runs one way.** The 99 family
matches are a judgement: glosses like *"pm_census vv/ve/vf/ef gaps,
spans, residuals"* and *"sphere/torus meridian checks"* were read as
covering every name they plausibly reach, which is the reading most
favourable to this document. So **223 is a ceiling on REACH**, **23 a
floor on the hole** — a stricter reader moves names out of the 99 and
into the 23, and nothing can move one out of the 23, because those
names appear nowhere above this section in any form. And **reach is not
a dimensional verdict**: the number of names this document has actually
dimensioned, one row and one `dim` cell each, is **121**. The other 102
are covered by a family gloss or a sentence, which is a claim about a
family and not a check on a comparand.

**Sites are NOT the deliverable, and this is why.** Counting them is
still useful as a cross-check, so the ledger is below — but D19's
lesson (*"a count of SITES was never the right measure of a NAME
roster"*) has a second edge here: **about thirty helpers in these two
crates fix a name internally and have more than one caller** —
`require_extent`, `require_rim_incidence`, `du_of_rims`,
`classify_dihedral`, `enters_material`, `point_in_loop`,
`volume_backstop`, `tangent_locus_relation` and the rest — so *"where a
decision is posed"* and *"where the funnel is called"* differ by a wide,
convention-dependent margin. The roster is unharmed (every one of those
names is a literal at the wrapper's own funnel call, so all are in the
210), which is exactly the point: the name count is stable under the
convention and the site count is not.

| | |
|---|---|
| raw matches of the ten spellings | 322 |
| − prose inside doc comments (`boolean/ops.rs:3`, `chord_join.rs:188`, `sector_shape.rs:412`, `:456`) | 4 |
| − a `format!("decide({rung}"` string inside a test (`chord_join.rs:2481`) | 1 |
| − two calls to an unrelated local `classify` closure (`splitting/join.rs:278`) | 2 |
| **= funnel call sites** | **315** |
| − forwarding hops (first argument is a name *parameter* of the enclosing fn or closure, so the name is chosen by its caller) | 13 |
| **= sites at which a name is fixed** | **302** |

**Both halves of the measurement have a blind spot, and neither is a
roster alone** (K-REPORT's framing; it reproduces here). A code scan
misses names not written at a funnel site — the 36 carried ones, 15% of
the roster. A corpus column misses names the corpus never exercises —
**80** of the 246 do not appear in the committed M7 baseline at all,
and that baseline in turn still carries six spellings the tree has
retired (`bool_sector_*` / `split_sector_*`, unified to `sector_*` by
#652). Re-deriving:

```sh
# code half — every funnel site, all ten spellings, both crates.
# It also matches doc-comment prose and `fn` definitions; the ledger
# above says which, and they are subtractions, not sites.
grep -rnE '\b(decide|decide_flagged|decide_invariant|check_residual|classify|classify_len|require_zero|require_extent|gap_is_zero|signed_is_zero)\s*(::<[^()]*>)?\s*\(' \
  crates/geom-brep/src crates/topo/src
# behavioural half — what the committed baseline emitted
zcat docs/k-report-data/m7-eps-1e-9.csv.gz | tail -n +2 | cut -d, -f2 | sort -u
# coverage — compare against THIS FILE WITH ITS TWO SELF-DESCRIBING
# SECTIONS REMOVED. `Uncovered names` lists the 23 and this section
# names helpers that share spellings with predicates, so a re-derivation
# that reads the whole file finds a hole of zero and calls it closed.
sed -e '/^## Coverage of the two crates/,/^## geom-brep/d' \
    -e '/^### Uncovered names/,/^## Findings/d' \
    docs/predicate-dimension-audit.md
```

The first command is a **starting set, not an answer**: a site whose
first argument is not a literal has to be read, and a spelling not in
the alternation is invisible to it — `require_extent` was, until this
measurement, absent from the alternation while being named in this
file's own first paragraph. That residue is this table's standing cost,
disclosed rather than discovered.

**Nine names carry the K vocabulary and never reach the funnel**, so
they are correctly outside the 246 and a reader who greps for one
should know why. They live only in an `Indeterminate.predicate` —
seven through `predicate: Some("…")` (`carrier_kind`,
`contact_tangent_independent`, `contact_rest_senses_opposed`,
`contact_rest_ladder_invariant`, `transversality`,
`plane_nurbs_transversality_reported`, `validate_probe`) and two
through an `invalid(band, "…")` helper (`bool_contfp_boundary`,
`pm_census_containment`). None decides anything, none appears in the M7
baseline, and none has a comparand to dimension.

**Why no gate on 121 / 223 / 246 / 302 — the third answer to Q6.**
Not "it is guarded" and not "dating it is enough": a gate would have to
fix the family-matching convention in code, and that convention is the
judgement this section is careful to expose rather than freeze. A green
gate would assert a reading, not a fact. What *is* mechanical is
already elsewhere — `geom-core/tests/flagged_census.rs` pins the
`decide_flagged` count, and the K sweep recomputes the emitted-name set
on every merge. The 23 are scheduled as their own unit (§D's **D46**),
which is what actually moves the number.

## geom-brep

| site | predicate | comparand | dim | status |
|---|---|---|---|---|
| dihedral.rs:140 | dihedral_arm | min(curvature arms, extent) | m | OK |
| dihedral.rs:151 | dihedral_wedge | sinθ (unit-gradient cross) × arm | m | OK |
| enters.rs:84 | enters_material_arm | caller arm (contract: m) | m | OK |
| enters.rs:95 | enters_material | cos(unit,unit) × arm | m | OK |
| enters.rs:141 | tangent_sector_order2_arm | caller arm | m | OK |
| enters.rs:153 | tangent_sector_order2 | normal curvature (1/m) × arm²/2 | m | OK |
| newell.rs:165 | newell_plane_residual | (p−centroid)·n̂ | m | OK |
| certify.rs:849/858 | interval_span_forward/winding (Circle) | span·radius / (τ−span)·radius | m | OK |
| certify.rs:872/877 | interval_span_forward/winding (Ellipse) | span·minor (conservative) | m | OK |
| certify.rs:883 | interval_span_forward (Line) | span (t IS arc length) | m | OK |
| certify.rs:1164 | nurbs_span_meter | knot-domain length × speed_lower_bound() — the net's arc-length lower bound, reparametrization-invariant | m | OK (metered door; the collapsed-arm gate on the meter) |
| certify.rs:1175 | interval_span_forward (Nurbs) | span × (m/param) | m | OK |
| certify.rs:917/925 | carrier_endpoint_start/end | point distance | m | OK |
| certify.rs:950/958/992/1000 | carrier/tangent_on_surface_1/2 | implicit_residual (/2r-normalized) | m | OK |
| certify.rs:1015 | tangent_second_order | κ_rel·arm²/2 | m | OK |
| certify.rs:1035 | tangent_normal_parallel | sinθ / κ_rel (arm = 1/κ_rel, the D4 ¶1 tangency lever) | m | OK (note N4) |
| certify.rs:1047 | carrier_matches_mapped_source | point distance | m | OK |
| certify.rs:1057/1068/1076 | carrier_on_seam_* | residual / radial·unit | m | OK |
| certify.rs:1103/1112 | tangent_hull_sup / tube_margin | m residual sums; κ·arm² | m | OK |
| certify.rs:1143/1151/1162 | witness_* | residuals / point distance | m | OK |
| intersect.rs:554/560/591 | pc_axis_plane_parallel / parallel_gap / rim_alignment | sin×extent; r−gap; sin×r | m | OK |
| intersect.rs:694 | ps_frame_seam | (sin−0.5)·r — deterministic frame tie-break, not a coincidence question | m | OK (note N5) |
| intersect.rs:705 | ps_center_gap | r − center-plane distance | m | OK |
| intersect.rs:834–882 | cc_* (radius eq, axes parallel, coaxial, gap, coplanar) | lengths / sin×extent / common-perpendicular | m | OK |
| intersect.rs:1000/1006/1043 | pn_apex_*, pn_axis_normal | m·unit; trig diff×extent; sin×rim r | m | OK |
| pcurve_cache.rs:1225/1233 | pcurve_chart_azimuth_affine / winding | (rad coeff)×radius | m | OK |
| pcurve_cache.rs:1268 | pcurve_map_residual | mapped point distance | m | OK |
| pcurve_cache.rs:1964 | pcurve_interval_forward (harmonic) | span × param_rate | m | OK |
| pcurve_cache.rs:1988 | pcurve_azimuth_period (harmonic) | (τ−extent)·azimuth_lever | m | OK |
| pcurve_cache.rs:1894 | pcurve_interval_meter (fitted/iso gate) | carrier parameter extent × param_rate (a NURBS net's knot domain × its certified speed lower bound) | m | OK (metered door; the collapsed-arm gate) |
| pcurve_cache.rs:2310 | pcurve_trim_containment | chart-param overhang × `chart_arms_at` (the cone arm from the check's own boxes since M6-3) | m | OK (metered door) |
| pcurve_cache.rs:2382 / :2868 | pcurve_interval_forward (fitted / iso) | span × param_rate — a NURBS carrier's rate IS its certified speed lower bound | m | OK (metered door; the meter gated at :1894) |
| pcurve_cache.rs:2397 | pcurve_azimuth_period (fitted) | rad headroom × `chart_arms_at`'s azimuth lever (the cone's `v_sup·sin α`) | m | OK (levered door) |
| pcurve_cache.rs:1664 | pcurve_chart_radial_moving | Σ m-norms BARE (amplitude is metres) | m | FIXED (M6-3) |
| pcurve_cache.rs:1680/1772/1791 | pcurve_chart_orientation / sphere meridian | m² ÷ radius | m | OK |
| pcurve_cache.rs:1752 | pcurve_sphere_chart_frame | m at :1770, dimensionless at :1836 (tie-break) | mixed | FLAG (note N5) |
| pcurve_cache.rs:1759–1829 | pcurve_sphere_chart_* | m-scaled coefficients / rooted | m | OK |
| pcurve_cache.rs (iso lane, M7) | pcurve_iso_boundary / iso_axis_u/v / iso_domain | chart-param values/extents/overhangs × stretch bounds (m per chart unit) | m | OK (metered door; added by the clause-(i) migration) |
| pcurve_cache.rs (ARC-RIM iso class, M8-3) | pcurve_interval_forward / pcurve_iso_boundary | span × `param_rate` = arc LENGTH (the class's carrier is a `Curve3::Circle` by construction, so the rate is the radius); the sub-arc weight residual `w − cos(h/2)` metered at the radius | m | OK (metered door; its rate cannot be poison, which is why it carries no meter gate) |
| pcurve_cache.rs (iso/fitted lanes) | pcurve_envelope | certified sup bound (m) | m | OK (added by the clause-(i) migration) |
| pcurve_cache.rs (chart derivation, M6-3) | pcurve_cone/sphere/torus_chart_axial / _centered / chart_radial_moving | axial displacement sums; radial-offset norms; Σ m-norms | m | OK (added by the clause-(i) migration) |
| pcurve_cache.rs (chart derivation) | pcurve_chart_orientation / sphere/torus_chart_meridian | oriented area a×b·n̂ over its radius lever (m²/m) | m | OK (over_lever door; added by the clause-(i) migration) |
| pcurve_cache.rs (chart derivation) | pcurve_cone_chart_nappe (h0/h data) | axial heights (m) | m | OK; the hs COSINE fallback is FLAG F13 |
| pcurve_cache.rs (chart derivation) | pcurve_chart_azimuth_frame / sphere_chart_pole_frame / polar & meridional rates | metre projections/norms at six of the seven frame callers; on the CONE ruling lane's F13 fallback the frame input is a UNIT radial's projection — dimensionless. Tie-break-only either way (N5: the trilean picks between two formulas identical mod τ — verdict-neutral), and that lane is F13-flagged one decision earlier | m (mixed on the F13 lane) | OK as tie-break (N5; row corrected at the clause-(i) fix pass, review MIN-1) |
| props/curved.rs (`require_rim_incidence`) | props_rim_axis_parallel / props_rim_center_on_axis | sin×r_c; perpendicular offset | m | OK |
| props/curved.rs (`level_coincides`, `props_rim_level_group` call) | props_rim_level_group (Length) | level difference BARE (v is arc length) | m | FIXED (#89's unit) |
| props/curved.rs (`level_coincides`, `props_rim_level_group` call) | props_rim_level_group (Unit) | rooted (sin,cos) CHORD × `RimArms::level` (sphere ×R, torus ×minor) | m | **FIXED — N1 RETIRED** (S81: one rule, one arm. Was Δ(sin,cos) componentwise × `major` on the torus) |
| props/curved.rs (`du_of_rims`) | props_rim_dir_group | (±1 diff) × `RimArms::azimuth` ∈ {0, ±2·arm} | m | OK (note N2) |
| props/curved.rs (`du_of_rims`) | props_du_consistent | Δu (rad) × `RimArms::azimuth` | m | OK |
| props/curved.rs (`linear_rim_side`'s nested `side`) | props_rim_side | per-kind: bare (Length) / × `RimArms::level` (Unit) | m | FIXED (#89's unit) |
| props/curved.rs (`cylinder_boundary`'s line arm / `cone_boundary`'s line arm) | props_meridian_axial / props_meridian_generator | sin (or cos-diff) × parameter span (m for lines) | m | OK |
| props/curved.rs (the four `*_boundary` parses) | props_meridian_on_surface / props_rim_fit (all kinds) | residuals; sphere/torus fits ROOTED before compare | m | OK |
| props/curved.rs (the four `*_boundary` parses) | props_circle_axis_class | cos × r_c | m | OK (note N3) |
| props/curved.rs (`require_extent`, called from all four flux lanes) | props_face_extent | m levels; sin-levels ×R; dt×minor | m | OK |
| props/curved.rs (`cone_boundary`'s line arm) | props_meridian_apex | apex-line distance | m | OK |
| props/curved.rs (`cone`'s single-nappe check) | props_cone_nappe | slant levels (m) bare | m | OK |
| props/curved.rs (`sphere_boundary`'s meridian arm, `torus_boundary`, `torus_meridian_orient`) | props_meridian_great / props_band_coplanar / props_meridian_orient | lengths / sin×R / cos×minor | m | OK |
| props/curved.rs (`require_rims_at_extremes`, through `level_coincides`) | props_rim_level | per-kind: bare level difference (cylinder/cone `Length`) / rooted (sin,cos) chord × `RimArms::level` (sphere ×R, torus ×minor) | m | OK (note N7; N1 RETIRED. Generalised from the torus-only site to all four kinds by S58/#649, and unified with its sibling `props_rim_level_group` by S81 — ONE rule (`level_coincides`), one metric (the chord), one arm (`RimArms::level`), one fail direction; the two names are the funnel's recording channels, not two rules, and the metering is still carried by [`RimLevel`]. N7's near-polar sphere understatement applies here too, and here it is a REFUSAL that is affected. Pinned as scale twins by `geom-brep/tests/rim_dim_scale_twins.rs` and, in a suite CI runs, by `geom-brep/tests/s81_one_rim_level_rule.rs`.) |
| props/quad.rs:453 | props_quad_converged | ε·F − flux-width(m³)/(3·area(m²)) | m | OK |
| props/quad.rs:461 | props_quad_face_extent | area/perimeter (mean width) | m | OK |
| ssi.rs:645 | ssi_cs_tangency | radius/axis distance differences | m | OK |
| ssi/certify.rs:366–524 | ssi_on_locus / hull_sup / foot / chart | residuals, /2R linearizations, foot distances | m | OK |
| ssi/certify.rs:836 | ssi_tube_transversality | sin (unit triple product) × arm | m | OK |
| ssi/march.rs:295/310 | ssi_transversality_arm / ssi_transversality | arm (m); sin × arm | m | OK |
| ssi/march.rs:420/447/478 | ssi_step_progress / branch_open_end / closure_return | state × (m/state); scaled domain margins | m | OK |
| ssi/march.rs:484 | ssi_closure_tangent | cos(unit tangents) × whole-branch arc length | m | FLAG F9 |

## topo

| site | predicate | comparand | dim | status |
|---|---|---|---|---|
| boolean/contain.rs:83–115 | bool_contact_vertex/edge_span/edge | point/span/perpendicular distances | m | OK |
| boolean/insert.rs:197 | bool_strut_order | (unit germ dir diff)·(unit e_dir) × min sector arm | m | FIXED (was dimensionless); verified CODE-READ + suites-green only — the rare germ-fan lane fires in none of the unit's live twin/probe configs (review MINOR-2, stated) |
| boolean/insert.rs:262 | bool_germ_line | sin(n̂_a,n̂_b) × min sector arm | m | OK |
| boolean/join.rs:567/803 | bool_join_chord | germ-site chord LENGTH (the degeneracy gate: Zero ⇒ coincident sites, no polygon edge) | m | OK |
| boolean/join.rs:603/817 | bool_join_nearest | a DIFFERENCE of two chord lengths (nearest-candidate selection) | m | OK |
| boolean/join.rs:743/744 | bool_join_facing | unit germ dir · chord (cos × separation) | m | FIXED (was bare cosine, `/dist`) |
| boolean/join.rs:750/751 | bool_join_arc_facing | axis·((p−c)×dir) — radius-metered sine | m | OK |
| boolean/join.rs:1093 | bool_ring_run_winding | (n̂ · Newell sum) / run perimeter — 2A/P, the run's mean width | m | FIXED (F4; was a bare **m² AREA**) |
| boolean/ops.rs (`bounded`) | volume_backstop_operand | V/A — the operand's mean thickness | m | FIXED (F3); on the INVARIANT LANE since Evan's #213 layering ruling — bare `T`, outside the length seam by design |
| boolean/ops.rs (`check`, arm 2) | volume_backstop | ΔV/(A_got + A_bound) — mean boundary displacement | m | FIXED (F3); INVARIANT LANE (see above) |
| boolean/ops.rs (`check`, arm 1) | volume_backstop_violation | the same length, against the EXACT bit-hairline band — a sign question, not a magnitude one | m (band-free) | OK by design (note N6's category; #200 review MAJ-1); INVARIANT LANE (see above) |
| boolean/ops.rs:1194–1480 | bool_sphere_* | radius/gap differences; sin × radius | m | OK |
| boolean/plane_eq.rs:174/233 | bool_plane_parallel | sin(n̂1,n̂2) × arm | m | OK |
| boolean/plane_eq.rs:190/252 | bool_plane_orient | cos(n̂1,n̂2) × arm | m | FIXED (was bare cosine) |
| boolean/plane_eq.rs:203/265 | bool_plane_offset | signed-offset difference | m | OK |
| boolean/recl.rs:224–748 | side_code / bool_dir_same / bool_ee_collinear | cos/sin × sector arms | m | OK |
| boolean/reduce.rs:548–802 | bool_vertex_face_side / circle & line clearances | plane residuals, /2r residual extremes, sagitta dips | m | OK |
| boolean/rest.rs:401 | bool_join_chord | germ-site chord LENGTH | m | OK |
| boolean/rest.rs:411/413 | bool_join_facing | unit dir · chord | m | FIXED (was bare cosine) |
| boolean/rest.rs:421 | bool_join_nearest | a DIFFERENCE of two chord lengths | m | OK |
| boolean/sectors.rs:342–433 | bool_sector_within / bool_dir_* / bool_faces_parallel / side_code | sin/cos × sector arm (arm = shorter bounding chord, m; every caller passes unit dirs — verified) | m | OK |
| boolean/solid_contain.rs:438 | bool_wall_trim_period | (τ−width)·radius | m | OK |
| boolean/solid_contain.rs:462 | bool_wall_trim (cone term) | (cosΔ−cos h)·radius — effective arm sin(h)·r, collapses for narrow windows | m | FLAG F8 |
| boolean/solid_contain.rs:538/562/587 | bool_point_in_solid_plane | plane residual; /2r linearizations | m | OK |
| boolean/solid_contain.rs:645/655 | bool_point_in_solid_advance/order | ray parameters (m, unit dir) | m | OK |
| boolean/solid_contain.rs:691 | bool_point_in_solid_denom (plane) | cos(unit,unit), no arm | dimensionless | FLAG F2 |
| boolean/solid_contain.rs:743 | bool_point_in_solid_denom (cylinder) | sin²/2r | **1/m** | FLAG F2 |
| boolean/solid_contain.rs:763 | bool_ray_cylinder_disc | disc/(2r)² (self-documented, F3 of PR 9c) | dimensionless | FLAG F2 |
| boolean/solid_contain.rs:792 | bool_point_in_solid_denom (cylinder hit-outward — the pre-migration row mislabeled it "sphere"; the sphere lane reads outward structurally and its disc is over_lever at :850) | (unit·radial)/radius | dimensionless | FLAG F2 |
| boolean/solid_contain.rs:850/903 | bool_ray_sphere_disc / at_infinity | disc/2r (over_lever); volume/area (V/A mean thickness, over_lever — a genuine containment decision, not a backstop) | m | OK |
| boolean/vtxfac.rs:106/113/453 | side_code / bool_sector_coplanar / bool_germ_line | cos/sin × sector arm | m | OK |
| census.rs:313–599 | pm_census_vv/ve/vf/ef gaps, spans, residuals | point/line/plane distances and spans (unit dirs verified) | m | OK |
| census.rs:614–746 | pm_census_span_* / ee_gap / ee_span / ee_overlap | span arithmetic (m) | m | OK |
| census.rs:666 | pm_census_ee_parallel | sin(unit dirs) × min(edge lengths) | m | FIXED (was bare sine) |
| census.rs:812/831 | pm_census_confirm_* | distances / residuals | m | OK |
| merge_faces.rs:924 | bool_ring_run_winding | (n̂ · Newell sum) / loop perimeter | m | FIXED (F4) |
| pcurves.rs:631–641 / :1027 / :1273 | pcurve_loop_continuity / closure(_height) | Δu(rad)×`azimuth_arm`; Δv×`v_meter` | m on every ANALYTIC chart, and on a PLANE chart (its u/v ARE metres, so the `_ => 1` arm is exactly right); the `v_meter` fallbacks are 1 only where no polar arm exists | m | OK, except the NURBS chart: `azimuth_arm`'s `_ => 1` and the `v_meter` `unwrap_or_else(T::one)` fallbacks under-state a NURBS chart's stretch. FLAGGED as a cross-crate residue (an honest arm needs geom-brep's `nurbs_stretch_bounds`), tracked by issue #501 — not a `decide_flagged` site |
| pcurves.rs | pcurve_iso_side / pcurve_loop_pole_joint | chart-image point distance; local azimuth lever (m) | m | OK (added by the clause-(i) migration) |
| split.rs:197 | split_edge_param_interior | param spans × per-kind rate (1 / radius / minor / speed bound) | m | OK |
| transform.rs:139 | transform_rigid_* (7 residuals) | unit-column/orthogonality/det residuals, no arm | dimensionless | FLAG F10 |
| transform.rs:155 | transform_rigid_trans_finite_* | t·0 poison probe (0 or NaN by construction) | — | OK |
| validate.rs:1662/1847 | planar_face/boundary_residual | plane residuals | m | OK |
| validate.rs:1795 | tangent_second_order | κ_rel × arm²/2 | m | OK |
| validate.rs:2030 | bool_ring_run_winding | (outward · Newell sum) / loop perimeter | m | FIXED (F4) |
| validate.rs:2014 | positive_volume | volume/surface-area (the documented dimensional fix) | m | OK |
| sector_shape.rs (the three rungs) | sector_arm / sector_reflex / sector_straight | arm = shorter bounding chord (m); sin/cos × arm | m | OK — ONE implementation since the S5 sector-predicate unit, and since #652 ONE name set: the former `bool_sector_*` / `split_sector_*` pairs were the same computation on the same quantity, which is why this was already one row |
| splitting/classify.rs:81–286 | split_vertex_side / conic lane | plane residual; rooted amplitude; (rad)×minor semi-axis | m | OK |
| ray_parity.rs (via `containment.rs`'s `ROWS`) | point_in_loop_segment | a loop segment's own length — the degeneracy gate, through the `Margin::norm3` door | m | OK (split off `point_in_loop_boundary` by #712, which was deciding two questions under one name) |
| ray_parity.rs (via `containment.rs`'s `ROWS`) | point_in_loop boundary/side/advance | distances; m²/m advance | m | OK |
| splitting/containment.rs (the frame gate) | point_in_loop_arm | sin(member, plane normal) × loop extent (the member's in-plane fraction) | m | FIXED (was dimensionless schedule norm) |
| splitting/neighborhood.rs:228–309 | split_conic_departure / split_bisector_side | tangent×extent projections; bisector·n̂ × arm | m | OK |
| splitting/order.rs:73 | split_join_frame_arm | sin(member, plane normal) × points' spread (the member's in-plane fraction) | m | FIXED (was dimensionless schedule norm) |
| splitting/order.rs:111 | split_join_order_u/v | coordinate difference (m) vs the EXACT bit-level band (deliberate total-order device, documented) | m | OK (note N6) |
| splitting/rules.rs:132/151/202 | split_sector_extent / coplanar / enters arm | extent; sin×extent | m | OK |
| splitting/rules.rs:179 | tangent_sector_osculation | κ(1/m) × face-extent²/2 | m | FLAG F11 |
| chord_join.rs:710 | split_sphere_section_polar | sin(axes) × sphere radius | m | OK |
| chord_join.rs:1114 | split_tangent_chord_forward | dimensionless param diff × ‖dir‖ | m | OK |
| chord_join.rs:855 | split_arc_window (×5) | azimuth (rad) × chart radius | m | OK for cylinder; FLAG F8 for the sphere wall (arm R vs local R·cos lat) |
| chord_join.rs:926 | split_arc_chart_orientation | cos × semi-major (= r for the plane×cyl ellipse) | m | OK |
| chord_join.rs:1411 | split_conic_inplane_mid | plane residual at midpoint | m | OK |
| chord_join.rs:1468 | bool_between_arc_window | (cosΔ−cos h)·r_c — quadratic in the angular deviation for narrow windows | m | FLAG F8 |
| chord_join.rs:1490 | split_chart_azimuth_frame | radial·u_ref (m) — branch selection | m | OK (note N5) |
| chord_join.rs:1623/1639 | split_sphere_window_pole(_side) | radius − axial distance | m | OK |
| splitting/join.rs:377 | split_section_area | 2·\|A\|/P mean width | m | FIXED (factor-2 doc/code mismatch; dimension was already m) |
| splitting/finish.rs:414 | classify_dihedral arm | edge extents (m) | m | OK |

> **Anchors moved (2026-08-20).** The nine rows above that read
> `splitting/join.rs` now read `chord_join.rs`: the shared chord-join
> core moved to a top-level module, and the two `split_arc_window` /
> `split_arc_chart_orientation` / `split_sphere_section_polar` sites
> per rung became ONE each — the boolean planar-side chord was a
> hand-copy of the split lane's S9 block and now calls it. The
> dimensions and the verdicts are unchanged; only the address and the
> site count are. `split_section_area` stayed in `splitting/join.rs`,
> which is now the split sweep alone.

| ray_parity.rs (via `chart_region.rs`'s `ROWS`) | chart_region_segment | a closed segment's own length — the degeneracy gate, through the `Margin::norm2` door | m | OK (split off `chart_region_boundary` by #712, with the door corrected from `Margin::of` in the same pass) |
| ray_parity.rs (via `chart_region.rs`'s `ROWS`) | chart_region_boundary/side/advance | the point_in_loop rows on METRED chart coordinates (exact arms only: plane 1, cylinder r): distances; m²/m advance. Since #712 these decide in the SAME shared walk as the 3-D rows, under their own names. The 3-D `point_in_loop_arm` row is derived away — a fixed 2-D schedule member is in-plane by construction, so no projected-length predicate exists | m | OK (new in M9-2) |
| chart_region.rs (M9-2) | chart_region_parallel / collinear_offset | segment-pair 2×2 determinant / offset determinant over one segment's length — the perpendicular height across that segment's line | m | OK (new in M9-2) |
| chart_region.rs (M9-2) | chart_region_cross_span | crossing fraction (dimensionless) × its own segment's length — the crossing point's clearance from a segment endpoint | m | OK (new in M9-2) |
| chart_region.rs (M9-2) | chart_region_collinear_overlap | shared-span length of collinear segments (difference of metre projections) | m | OK (new in M9-2) |
| chart_region.rs (M9-2) | chart_region_cross_order | same-edge crossing-pair advance-fraction difference (dimensionless) × the edge's own length — the crossing points' separation along the boundary (the clip walk's order certificate, union fix U2) | m | OK (new in M9-2 fix pass) |
| chart_region.rs (M9-2) | chart_region_orientation / chart_region_area | signed loop shoelace 2A (m²) / perimeter — the loop's (resp. intersection region's) mean width, through the same `Margin::over_lever` door as `split_section_area` two dimensions up (separate accumulators — the reasons are at `chart_region.rs`'s area margin) | m | OK (new in M9-2) |
| chart_region.rs (M9-2) | chart_region_seam_span | azimuth-span excess over one period (rad) × the chart's azimuth arm r | m | OK (new in M9-2) |
| boolean/rest.rs (M9-2 PR-2) | tangent_locus_axis_parallel | sin(axis, plane / axis, axis) × the 1 m verification arm (carrier_pair_relation's own) | m | OK (new in M9-2 PR-2) |
| boolean/rest.rs (M9-2 PR-2) | tangent_locus_gap / tangent_locus_side | axis-to-plane (or axis-to-axis) distance minus radius sum/difference; signed height / radius difference — all metre data of the carriers | m | OK (new in M9-2 PR-2) |
| census.rs (M9-2 PR-2 fix pass) | census_backstop_gap | per-axis gap between two faces' SOUND reach boxes (plane hull ⊕ boundary-arc radius; cylinder axial span ⊕ radius; sphere ball — coordinate differences and radii, metres); only a DEFINITE positive clears the pair | m | OK (new in the union fix; boxes tightened to the face_box construction in the delta) |
| census.rs (M9-2 PR-2 fix pass) | census_backstop_containment | per-axis extent margin between two solids' vertex hulls (coordinate differences — metres); containment = all six definitely positive, clearance = any definitely negative | m | OK (new in the union fix) |

Funnel bypasses found: **boolean/ops.rs:634/649** (`sign_within`
called directly on volume margins — was FLAG F3, **FIXED**: the gates
now route through `k_stats::decide` under `volume_backstop_operand`,
`volume_backstop` and `volume_backstop_violation`). **This audit's
scope — geom-brep and topo — has no funnel bypass left:** every
shipped decision in the two crates goes through `k_stats::decide`, so
every margin the recorder sees is attributed to the predicate that
actually decided it. The claim is scoped on purpose. One shipped raw
`sign_within` exists elsewhere in the workspace — `editor-core`'s
expression evaluator — and is carried below as **F12** rather than
swept under the headline. Raw ε reads outside decisions: solver
tolerances and step-size control in ssi (documented structure
parameters), `props.rs` trig pad (ε/radius, an enclosure pad, not a
decision), test fixtures.

### Uncovered names (measured at `43e2998d`)

Twenty-three names reach the funnel from `geom-brep` or `topo` and
have **no row, no family cell and no mention** anywhere above this
section. They are enumerated rather than audited: each wants its
comparand read and a dimension verdict, which is a unit (§D's **D46**)
and not a side errand of the measurement that found them. Three of the
eight homes are files this document has never named at all —
`edge_nurbs.rs`, `boolean/carrier_eq.rs`, `boolean/contact_verify.rs` —
and the other five are named files whose rows predate these names.

**Where the 23 verdicts land when D46 does them**: one row each in the
`## geom-brep` or `## topo` table above, with `site`, `predicate`,
`comparand`, `dim` and `status` filled the way every other row is —
plus a disposition entry in *Findings* for any that come back FLAG, and
its `F`-number. A name leaves this section only by acquiring that row;
the section is empty when the two counts above meet at 246.

| home | names |
|---|---|
| `geom-brep/certify.rs` | `carrier_in_seam_halfplane`, `carrier_on_iso_curve`, `plane_nurbs_on_locus`, `plane_nurbs_hull_sup` |
| `geom-brep/edge_nurbs.rs` | `plane_nurbs_transversality` |
| `geom-brep/pcurve_cache.rs` | `pcurve_chart_polar_affine`, `pcurve_chart_polar_winding` (the polar twins of the azimuth pair one row up) |
| `geom-brep/props/curved.rs` | `props_band_coplanar` |
| `topo/pcurves.rs` | `pcurve_chart_u_closed`, `pcurve_iso_arc_direction`, `pcurve_iso_seam_column` |
| `topo/census.rs` | `pm_census_bound_end`, `pm_census_bound_vertex` |
| `topo/boolean/carrier_eq.rs` | `carrier_sphere_center`, `carrier_sphere_radius`, `carrier_cyl_axis_parallel`, `carrier_cyl_axis_offset`, `carrier_cyl_radius` (a `[(&'static str, Margin<T>)]` table — carried, so no literal at the funnel site) |
| `topo/boolean/contact_verify.rs` | `contact_tangent_on_1`, `contact_tangent_on_2`, `contact_tangent_opposed`, `contact_tangent_parallel`, `contact_tangent_second_order` |

## Findings (dispositions)

Fixed in this unit. Live-pin coverage, honestly (review MINOR-2):

- **F1 (the unit's trigger)** `props_rim_level_group`/`props_rim_side`
  — per-kind metering via the `RimLevel` enum; the #89 in-band landing
  retired (margin now the true rim separation, scale-linear). Pinned
  by `crates/geom-brep/tests/rim_dim_scale_twins.rs` and the
  grouping-flip probes (`rim_dim_review_probes.rs`, adopted by merge).
- `bool_join_facing` (×2 files), `pm_census_ee_parallel`,
  `point_in_loop_arm` — live in the boolean scale-twin pin
  (`crates/topo/tests/rim_dim_boolean_twins.rs`).
- `bool_plane_orient` (×2 rungs), `split_join_frame_arm`,
  `split_section_area` (factor-2 aligned to its documented spec) —
  live in the adopted review probes' flush-subtract + oblique-split
  linearity test (`crates/topo/tests/rim_dim_review_probes.rs`).
- `bool_strut_order` — CODE-READ + suites-green only; the rare
  germ-fan lane fires in none of the above configs.

Fixed by the **F3+F4 dimensional unit** (the follow-up unit this
audit's F3/F4 rows banked; both findings were EXECUTED, not
speculative — each row below states what it measured):

- **F3** `volume_backstop` (ops.rs): the tree's ONE funnel bypass is
  gone. Both gates decide through `k_stats::decide` — the bound check
  under `volume_backstop` (margin `ΔV / (A_got + A_bound)`: a boundary
  displaced by δ moves the volume it encloses by ≈ δ·A, so the summed
  surface area of the two compared bodies is the whole boundary that
  could have produced the defect, and the quotient is the mean boundary
  displacement the violation corresponds to) and the operand-bounded
  test under `volume_backstop_operand` (`V/A`, verbatim the
  validate.rs `positive_volume` precedent). Both quotients are exactly
  zero when the volumes agree exactly, so the gate's non-strict pass
  direction is unmoved. The attribution defect this closes was measured
  in `rim_dim_boolean_twins` at ε = 1e-12: the operand/result VOLUME
  set {1, 1, 3, 8, 8, 16} m³ logged under certify's
  `witness_at_mid_parameter` (cubic, ×1e-9 between the twins) and
  perturbing that predicate's sample COUNT (102 vs 103). Post-fix the
  volumes appear under their own names and scale ×1000, and
  `witness_at_mid_parameter`'s decisive list is EMPTY at both scales —
  its real samples are coincident residuals, exactly as the old
  allowlist comment claimed.
  **Third executed consequence, found by this unit and NOT previously
  known: the cubic comparand was silently switching the backstop
  OFF at mm scale.** `bounded` classified a raw m³ volume against the
  linear band, so a 2 mm cube's 8e-9 m³ landed inside
  `Band{1e-9, 1e-8}` at the default ε — indeterminate — and the code
  reads an indeterminate operand as "not certifiably bounded" and
  SKIPS its bound. The whole `vol(A∖B) ≤ vol(A)` check was therefore
  vacuous on that boolean. Measured as a one-line census delta on the
  twin fixtures (`witness_at_mid_parameter` 123 samples/5 nonzero →
  118/0, plus `volume_backstop` 2/2 and `volume_backstop_operand` 4/4;
  every one of the other 49 predicates byte-identical): five volume
  decisions became six, the sixth being the restored check, which
  passes. Metered as `V/A` the same operand answers 3.3e-4 m —
  decisively bounded at every ε in the matrix. No verdict flipped; a
  gate that had been skipping now runs.
  **The metering's WEAKENING direction, and the dual-arm answer (#200
  review MAJ-1).** Metering alone would not have been pure gain:
  `ΔV/(A_got + A_bound)` shrinks with the bodies' area, so a localized
  wrong component on a large body meters below ε even while the defect
  stays macroscopic. Executed by the reviewer: a wrongly-kept 3 mm cube
  on a 2 m × 2 m × 0.1 m plate is ΔV = 2.7e-8 m³ over ~17.6 m² →
  1.53e-9 m, inside the default band, where the raw-m³ comparand had
  refused decisively. Resolution: the backstop asks two questions and
  only one is about a magnitude. `volume_backstop_violation` decides
  the SIGN against the **exact bit-hairline band** (`splitting::order`'s
  device, note N6) — the bound is an inequality, and a sign-certain
  violation is a dimension-free fact no amount of boundary area can
  dilute; `volume_backstop` keeps the metered mean displacement against
  ε for the near-zero region the sign arm leaves open. Both arms
  consume the same metered comparand, since dividing by a
  certainly-positive lever cannot move a sign — so the K stream stays
  length-dimensioned and scale-linear (verified in the twins) while the
  refusal is scale-free. The gate is now strictly stronger than both
  its predecessors. Pinned end-to-end by
  `ops::tests::volume_backstop_refuses_a_wrong_component_hidden_by_a_large_area`
  (verified red with the sign arm removed) and at band level by the
  adopted `tests/probe_f34_review.rs`.
- **F4** `bool_ring_run_winding` (join.rs, merge_faces.rs,
  validate.rs — one predicate, three sites, all three moved together):
  the Newell AREA is divided by the region's boundary PERIMETER, giving
  `2A/P` — the ring's MEAN WIDTH, the distance the boundary would have
  to move to sweep the enclosed region away, and the same quantity
  `split_section_area` already meters. The canonical derivation lives
  at `boolean::join::ring_run_ccw`; the other two sites cross-reference
  it. In the join's ring-run lane the perimeter is arc-aware (conics
  contribute `|Δ|·semi-major` — exact for a circle, an upper bound for
  an ellipse, and an over-large P escalates rather than decides) and
  includes the chord that closes the open run. This retires an
  EXECUTED in-band refusal: at ε = 1e-6 the mm pocket-subtract twin
  refused typed on a 2e-6 m² margin inside Band{1e-6, 1e-5}; the same
  decisions now carry 5e-4 / 7.5e-4 / 1e-3 m and compute on every ε row
  in the hosted matrix. `rim_dim_boolean_twins`'s three-outcome F4
  signature match is deleted, and the predicate is pinned LINEAR.

Flagged, NOT fixed here (dispositions):

- **F2** `solid_contain.rs` ray-caster denominators (691/743/763/792 — refs refreshed and the fourth site relabeled cylinder hit-outward at the clause-(i) fix pass; the sphere-disc form at :850 is the model and took `over_lever`):
  dimensionless and 1/m comparands. The cylinder-disc site carries an
  in-tree admission earmarking a re-pin unit (PR 9c review F3). One
  coordinated unit should meter all four (the sphere-disc form (now :850)
  is the model). Reported, deferred to that unit.
- **F3** — **FIXED by the F3+F4 dimensional unit** (see the fixed
  list above).
- **F4** — **FIXED by the F3+F4 dimensional unit** (see the fixed
  list above).
- **F5** `pcurve_chart_radial_moving` — **FIXED by M6-3** (the
  loft-assembly unit, PR #192): the amplitude is compared BARE (it is
  already a displacement in metres; the ×radius factor made it an
  area). The predicted retirement executed exactly: the freecad
  `CORPUS_EPS_CEILING` moved 1e-8 → 1e-5 (the 1e-7/1e-6 refusals were
  this comparand's artifact; at 1e-4 the attachment/span gates refuse
  at the corpus's true feature scale — table re-measured in
  step-import/tests/freecad.rs, composing #197's F-row retirement).
  In-band amplitudes take the meridian arm as a D9 tie-break, the
  discarded drift carried by check 4's envelope in metres.
- **F6** pcurve chart arms — **RETIRED**. M6-3 closed most of it
  (`chart_arms` answers (r, r) for spheres and (R+r, r) for tori, and
  `pcurves.rs::azimuth_arm` is the LOCAL lever — r·cos v etc., zero at
  poles/apex, which the walk exploits). The residue closed with F7:
  `param_rate` answers a NURBS carrier's certified speed lower bound,
  so the fitted and iso `pcurve_interval_forward` spans cross to the
  band as arc lengths through the metered door, with the meter itself
  gated as a length (`pcurve_interval_meter`, the collapsed-arm
  idiom); and the fitted lane's azimuth headroom takes
  `chart_arms_at`'s lever, so the cone's arm is `v_sup·sin α` from the
  check's own boxes rather than 1. What remains is NOT this family:
  `pcurves.rs::azimuth_arm`'s non-plane `_ => 1` fallback (:644) and
  the `v_meter` fallbacks (:1027, :1273) are cross-crate — an honest
  arm needs geom-brep's `nurbs_stretch_bounds` from topo — and carry
  issue #501. The Plane case there is OK, not flagged:
  a plane chart's u/v ARE metres, so 1 is exactly right.
- **F7** `nurbs_span_meter` (certify.rs:1164) — **RETIRED**. The gate
  is a LENGTH: the net's knot-domain extent metered through the
  certified speed lower bound, a lower bound on its arc length. That
  comparand is reparametrization-invariant where the bare rate was
  not (`t → 2t` halves the rate and doubles the domain), and it is
  what D4's ε classifies. The collapsed-arm idiom keeps the two
  failure modes distinct: a collapsed or poison meter is `Invalid`,
  a backwards span is `IntervalNotForward`.
- **F8** window/cosine family: `bool_between_arc_window` (cosΔ−cos h,
  quadratic near narrow/full windows), `bool_wall_trim` cone term
  (same shape, conservative direction), sphere-wall `split_arc_window`
  arm (R vs local parallel radius R·cos lat — over-generous near
  poles, the anti-conservative direction). One family, one fix shape
  (linearized angular margin × honest local arm); own unit.
- **F9** `ssi_closure_tangent` (march.rs:484): cos × whole-branch arc
  length — an unbounded arm (nothing folds in `ctx.extent`).
  Arm-policy question for the design conversation.
- **F10** `transform_rigid_*` (transform.rs:139): dimensionless
  rigidity residuals of the linear map against the metre band; the
  natural arm is the model/session-box extent. (The transform.rs
  collision claim is STALE as of the F3+F4 unit — the loft-assembly
  lane merged. Deferred on the arm question alone, which is a design
  input, not a conflict.)
- **F11** `tangent_sector_osculation` (rules.rs:179): sagitta model
  κ·L²/2 metered at the WHOLE-FACE extent, squared, and invalid for
  κ·L ≳ 1 — over-refusal direction. Arm-policy question; own unit.
- **F13** (added by the clause-(i) migration)
  `geom-brep/pcurve_cache.rs`, the cone chart's ruling lane: the nappe
  fallback datum `hs = dir·axis` is a **cosine** (the line's direction
  is unit), classified against the metre band when the anchor height
  `h0` is coincident-with-zero. The primary datum `h0 = w·axis` IS a
  length (`of`); the fallback is dimensionless — the N5
  branch-selection family (it picks a nappe, and the Zero arm refuses
  typed). Carried as `decide_flagged(.., "F13")`; the honest lever (a
  slant/extent datum) is a design question for the
  structure-selection-funnel conversation N5 banks.
- **F15** (added by the clause-(i) fix pass, from the #213 review's
  MAJ-1 — the review's scale-blindness probe EXECUTED it)
  `editor-core/eval/wire.rs` `revolve_axis_dir_in_plane`: `dir·n̂` with
  BOTH vectors unit is a bare **sine** against the metre band — the
  audit's class-(c) shape, wrapped in `of` by the first migration pass
  with no argument (the sibling `revolve_axis_origin_in_plane`
  comparand `rel·n̂` IS metres and keeps `of`). Executed consequence
  (review probe, adopted on merge as this row's pin,
  `geom-core/tests/review_margin_probe.rs`): a tilt of θ = 5e-10
  classifies Zero at every model scale while the induced deviation θ·r
  crosses the band between a 1 mm and a 10 m profile. The honest form
  levers the sine at the profile's radial extent, which lives
  kernel-side (`revolve/mod.rs` computes exactly that arm for
  `revolve_angle`) — that fix is F15's own unit, byte-identity forbids
  it here. Carried as `decide_flagged(.., "F15")`.
- **F14** (added by the clause-(i) migration)
  `editor-core/eval/wire.rs` `revolve_full_vs_partial`: `|θ| − τ` is
  **radians** against the linear band — the full-circle coincidence
  check runs in the editor before the kernel's own metered
  `revolve_angle`/`revolve_angle_headroom` gates (which lever at the
  profile's radial extent, correctly). The honest lever lives
  kernel-side; duplicating it in the editor is a design question, not
  a same-day fix. Carried as `decide_flagged(.., "F14")`.
- **F12** (added by the F3+F4 unit, from the #200 review's MIN-3)
  `editor-core/src/expr.rs:656`: the expression evaluator's door-2
  finiteness probe is a shipped raw `sign_within` — its own comment
  calls it "a reified decision" — so it is UNATTRIBUTED in the K
  telemetry, the same structural defect F3 just retired one crate over.
  Three things make it a different disposition, not a same-day fix:
  the comparand is `value · 0`, which is exactly `0` for every finite
  value and poison otherwise, so it is a **finiteness probe, not a
  geometric margin** — no dimension, no length, nothing the ε
  semantics govern; it classifies against a synthetic
  `Band{1e-100, 1e-50}` where, by that construction, *any* valid band
  decides identically; and it lives in the expression layer, outside
  this document's props/predicate sweep (geom-brep + topo). The honest
  statement is therefore "not a dimensional defect, but an attribution
  hole": routing it through the funnel would give the recorder a name
  for it and cost nothing. Deferred to whoever owns the editor layer —
  NOT fixed here, because a K-telemetry row for the expression
  evaluator is a scope question for that crate, not a consequence of
  this audit. **Clause-(i) migration note:** re-examined against the
  door set — no door fits, by the row's own argument: the operand is
  unit-erased at the expression boundary (GQ5), so `value · 0` has no
  honest length reading, and wrapping it would launder exactly what
  this row records. The site stays a raw `sign_within` outside the
  typed seam (it is not a `decide` call), doubly visible now that the
  seam admits only `Margin<T>`. The attribution hole stands as
  documented; the funnel routing (which would also push editor verdict
  rows into the N5 verdict-log channel) remains the editor-layer
  owner's scope call.

**Every `props/curved.rs` row above is cited BY TARGET NAME, not by
line** (S176(a)). The line numbers they carried were written against a
2026-08 tree and had already rotted at #877's merge base; #877 moved
200+ lines of the file and would have rotted the rest. A row whose
citation cannot be resolved is a row nobody re-derives.

**The audited POPULATIONS move under #877** — recorded here because
this document's rows are about what is recorded, and all audited probe
suites are green:

- **sphere `props_rim_level_group`**: two decides per comparison became
  **one**. The pair is `(sin v, 0)`, so the old second component was
  `0 − 0` and always decided `Zero`; the chord folds it away. Same
  verdict, one fewer sample.
- **torus `props_rim_level_group`**: the recorded margin changes from a
  per-component value at `major` to the chord at `minor` (N1's
  retirement). Different number, and it is the exact one.
- **`props_rim_side`**: now records on faces the gate previously
  refused before reaching it — `boundary_material_sign`'s three
  linearly-leveled arms run the premise first, so the population loses
  the non-rectangular faces it used to include and the K stream stops
  carrying sides that were a property of loop-flattening order.

Notes (verified honest, kept for the design conversation):

- **N1 — RETIRED by S81.** Torus `props_rim_level_group` levered
  Δ(sin v, cos v) at `major` while the induced point deviation levers
  at `minor` (the sibling `props_rim_level` used × minor). It
  overstated by R/r — conservative in the sense that it escalates or
  splits a truly-coincident pair rather than merging a distinct one,
  but conservative is not exact, and the split it produced was a
  **refusal**: on a 1 m / 1 mm gasket, a rim arc whose split vertex sat
  **0.5 nm** off level — half of ε — was metered as 0.5 µm, grouped
  apart, and the face refused `props_du_consistent`
  (`geom-brep/tests/s81_one_rim_level_rule.rs` is that face).
  **The resolution is the one this note named**: `du_of_rims`' single
  `arm` was doing double duty for a minor-circle LEVEL difference and
  for an azimuthal angle difference, and it is now two fields
  (`RimArms { level, azimuth }`). The level rule itself is one function
  (`level_coincides`) for both call sites, so the arm cannot drift
  again without both moving. Deferring this to "when typed margins
  land" is what left the two spellings 90 lines apart for eight months;
  typed margins will still find one rule here rather than two.

- **N2** `props_rim_dir_group` compares a structural ±1 through the
  numeric funnel (margin 0 or ±2·arm). Guarded upstream: a rim with
  arm ≲ K·ε cannot reach it (`props_circle_axis_class` escalates
  first, cos·r_c in-band).
- **N3** The cone's `du_of_rims` arm is the FIRST rim's radius
  |v|·sinα (bounded below ≳ K·ε by the same axis-class guard). The
  `T::one()` fallback is REACHED — both callers compute the arm before
  they know whether there is a rim — but **never metered against**:
  every route from it to a margin refuses on the empty rim list first,
  by `du_of_rims`' opening `is_empty` on the flux lane and by
  `linear_rim_side`'s `rims.first()` at the material-side gate. "The
  empty-rims refusal precedes" named one of those two routes and was
  false at the other (S112(d)); the invariant is what both establish,
  and `s58_iso_rectangle::a_rim_free_cone_refuses_at_both_doors` is the
  row.
- **N4** `tangent_normal_parallel`'s arm 1/κ_rel is the ratified D4 ¶1
  tangency lever ("normal-parallel within θ ⟺ within ε of the locus");
  unbounded only in the refusing direction, and the second-order gate
  fires first.
- **N5** `ps_frame_seam`, `pcurve_sphere_chart_frame`,
  `split_chart_azimuth_frame`, and `split_conic_phase_frame` are
  deterministic BRANCH/frame selections whose arms are all documented
  as verdict-neutral; their margins pollute per-predicate K telemetry
  with non-coincidence semantics (frame threshold at 0.5, mixed
  dimensions). Candidate for a separate "structure-selection" funnel
  tag in the typed-margin design.
- **N6** `split_join_order_u/v` deliberately classify against the
  bit-level exact band (total-order device), not ε — documented
  contract, excluded from the length rule by design.
- **N7** (review MINOR-3) The sphere's `Unit(sin v, 0) × R` grouping
  margin meters the AXIAL separation `R·|Δsin v|`, which degenerates
  toward the poles (∝ cos v̄ → 0): two distinct near-polar latitude
  rims can group as coincident although their true point separation is
  ~`R·Δv`. Dimensionally honest (the margin IS a length, and it is the
  quantity the area formula consumes), but the LEVER understates the
  3-D deviation near the poles — the same lever-magnitude family as
  N1 (now retired), opposite direction: N1 escalated, this merges, and
  it merges in the ACCEPTING direction. Cone/cylinder bare levels and
  the torus two-component pair do not share this. Typed-margin
  conversation input, and **smell-scan S82** — Evan's to answer, not a
  lane's. S81's unification does not answer it and does not try to; it
  makes it **cheaper**, because the sphere's lever is now one field
  (`RimArms::uniform(radius)`'s `level`) at one site, feeding one rule,
  rather than a scalar passed to two functions that metered it two
  ways. Whatever the answer, it is a change to that field.
