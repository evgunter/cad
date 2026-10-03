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
hole in it. **(2) Not complete inside them.** Of **248** funnel-reaching
names in the two crates the tables and their prose reach **225**, carry
an individual `dim` verdict for **123**, and miss **23** entirely; the
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
RENAMED and re-scoped to `over_lever` when Ev's layering ruling moved
the consistency backstops out of the seam, see below), and each row's
"comparand" column is the door's justification (out-of-ledger crates —
profile, sweep, editor-core's eval/naming, geom-curves — argue their
doors inline; their comparands are the same length shapes). Rows whose
comparand this ledger FLAGS as not-a-length are carried through the
seam by `geom_core::k_stats::decide_flagged(name, margin, band, row)`
— the finding lane: no `Margin` is constructed, the row id is a
compile-time argument at the site, and grepping `decide_flagged`
enumerates the clause-(i) debt exactly (F10 ×1 — one loop over seven
rigidity residuals — F13 ×1, F14 ×2, F16 ×1 — 5 shipped sites, tracked as issue #214 and pinned by
`geom-core/tests/flagged_census.rs`: no new site ships without a row
here, and the count only moves together with this section).

**The metric door is two doors, and the rate carries its own bound
direction.** `Margin::metered(span, rate)` takes an
`InfSpeed<T>` and `Margin::metered_sup(span, rate)` a `SupSpeed<T>` —
metres per parameter unit, transparent newtypes beside `Margin` in
`geom-core`, minted at each producer (`speed_lower_bound`,
`param_rate`, `chart_stretch_sup`, `nurbs_stretch_bounds`, `v_meter`,
the split and conic meters, `PatchRegularity`'s speeds,
`plane_nurbs_ssi`'s local chart speed). The direction is the semantic
content, and the rule that says which a site needs has one home —
`crates/geom-core/src/predicate.rs`'s module docs, *The rate pair,
beside the doors*. Before the pair, three shipped sites passed a
certified sup through a door whose doc promised an inf — correct in each case, argued in prose, checked by
nothing; the rows below now cite the door that matches the tag. The
conversions are one operation each (`span * s`, `m / s`), so no
margin's bits moved when the sites were typed.

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

**The invariant lane (Ev's #213 layering ruling).** The consistency
backstops — `volume_backstop` / `volume_backstop_operand` /
`volume_backstop_violation`, inequalities between integral RESULTS
(wrong-component detectors, never accuracy gates) — are outside the
length seam by design: they decide on bare `T` through
`k_stats::decide_invariant` (no `Margin` minted — not a door, not
debt), keeping their predicate names and margin values byte-identical
in the K stream, and a certified violation surfaces as the
Corrupt-class `ResultVolumeImplausible` (which ends in the shared
kernel-defect ending), separated in type and voice
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

**Names — the deliverable.** **248** distinct predicate names: **212**
written as a literal at one of those spellings, and **36 carried** by a
module-private `const`, a struct field or a local table
(`sector_shape.rs`'s three consts, `ray_parity::ParityRows` twice over,
`transform.rs`'s two arrays, `carrier_eq.rs`'s margin tuples,
`contact_verify.rs`'s and `splitting/order.rs`'s loop tuples,
`pcurve_cache.rs`'s winding closure, `pcurves.rs`'s two closures).

| the document's relation to a name | count |
|---|---|
| carries an individual row with a `dim` verdict | **123** |
| named only in prose, no `dim` column | 3 |
| reached only through a family cell or slash-list | 99 |
| **reached, on the most generous reading** | **225** |
| **recorded nowhere, under any reading** | **23** |

**Reach is not verdict, and the asymmetry runs one way.** The 99 family
matches are a judgement: glosses like *"pm_census vv/ve/vf/ef gaps,
spans, residuals"* and *"sphere/torus meridian checks"* were read as
covering every name they plausibly reach, which is the reading most
favourable to this document. So **225 is a ceiling on REACH**, **23 a
floor on the hole** — a stricter reader moves names out of the 99 and
into the 23, and nothing can move one out of the 23, because those
names appear nowhere above this section in any form. And **reach is not
a dimensional verdict**: the number of names this document has actually
dimensioned, one row and one `dim` cell each, is **123**. The other 102
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
**83** of the 248 do not appear in the committed M7 baseline at all
(`props_rim_interior_side`, `props_rim_only_extent` and
`props_rim_only_closed` are new here and post-date it;
`props_rim_dir_group`, which it does carry, is retired),
and that baseline in turn still carries six spellings the tree has
retired (`bool_sector_*` / `split_sector_*`, unified to `sector_*` by
#652). Re-deriving:

```sh
# code half — every funnel site, all thirteen spellings, both crates.
# It also matches doc-comment prose and `fn` definitions; the ledger
# above says which, and they are subtractions, not sites.
# `decide_positive` / `decide_negative` / `decide_nonzero` are `decide` with the caller's
# sign requirement folded in, so their sites are rows of this table
# like any other; `gate_measured` is deliberately absent — it classifies
# no margin and has no comparand to dimension.
grep -rnE '\b(decide|decide_flagged|decide_invariant|decide_positive|decide_negative|decide_nonzero|check_residual|classify|classify_len|require_zero|require_extent|gap_is_zero|signed_is_zero)\s*(::<[^()]*>)?\s*\(' \
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

**Ten names carry the K vocabulary and never reach the funnel**, so
they are correctly outside the 248 and a reader who greps for one
should know why. They live only in an `Indeterminate.predicate`:
- six through `predicate: Some("…")`: `carrier_kind`,
  `contact_tangent_independent`, `contact_rest_senses_opposed`,
  `contact_rest_ladder_invariant`, `transversality`, `validate_probe`;
- three through `topo::invalid_margin::invalid`:
  - `bool_contact_arc_straddle` and `point_in_arc_loop_conic_straddle`,
    an ellipse's lower and upper bound on one distance straddling the
    whole band;
  - `point_in_arc_loop_boundary_disagreement`, the carrier walk meeting
    on an edge a point its caller's pass placed off it;
- one, `plane_nurbs_transversality_reported`, as the name argument of a
  `k_stats::gate_measured` call. That records the escalation on the
  open frame but classifies nothing, so it is outside this table for
  the same reason the other nine are.
None decides anything, none appears in the M7 baseline, and none has a
comparand to dimension.

**It was nine, and the ninth is why this paragraph is worth keeping.**
`pm_census_containment` was an `invalid(band, …)` tag the tier-3′
census minted when the point-in-face door refused for a reason that
metred nothing — an arc-bearing loop no walk expresses, an exhausted
parity schedule, unwalkable topology. A name that decides nothing is
outside the audit by this paragraph's own rule, but it still reached a
user, inside an `Indeterminate` claiming a predicate by that name had
been posed and come back poisoned. The census now carries the door's
own typed refusal instead, and the tag is gone from the tree.

**Why no gate on 123 / 225 / 248 / 302 — the third answer to Q6.**
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
| geom/src/curves.rs `Curve3::spiric` (CURVED-SPIRIC; the kind's deciding door, registered here because `topo`'s rim mint decides through it) | spiric_minor_positive | the minor radius | m | OK (new in CURVED-SPIRIC) |
| geom/src/curves.rs `Curve3::spiric` (CURVED-SPIRIC) | ring_torus_convention (`geom::ring_torus`, the convention's one home) | `major_radius − minor_radius` — the ring convention's length | m | OK (new in CURVED-SPIRIC) |
| geom/src/curves.rs `Curve3::spiric` (CURVED-SPIRIC) | spiric_two_ovals | `(major_radius − minor_radius) − \|offset\|` — the length the two-oval regime closes by, decided before any root (the `offset_axial_rim_torus_reach` comparand, re-decided at the kind's own door) | m | OK (new in CURVED-SPIRIC) |
| geom/src/curves.rs `Curve3::spiric` (CURVED-SPIRIC) | spiric_frame_orthogonal | `axis · u_ref`, a cosine of unit vectors, levered at `major_radius + minor_radius` (the farthest point the frame places) | m | OK (new in CURVED-SPIRIC) |
| dihedral.rs:140 | dihedral_arm | min(curvature arms, extent) | m | OK |
| dihedral.rs:151 | dihedral_wedge | sinθ (unit-gradient cross) × arm | m | OK |
| enters.rs:84 | enters_material_arm | caller arm (contract: m) | m | OK |
| enters.rs:95 | enters_material | cos(unit,unit) × arm | m | OK |
| enters.rs:141 | tangent_sector_order2_arm | caller arm | m | OK |
| enters.rs:153 | tangent_sector_order2 | normal curvature (1/m) × arm²/2 | m | OK |
| enters.rs `bends_into_material` | wall_bend_order2_arm | caller arm | m | OK |
| enters.rs `bends_into_material` | wall_bend_order2 | mean normal curvature (1/m), folded to the material side, × arm²/2; its one caller (`splitting/rules.rs` `wall_graze`) passes the face extent | m | FLAG F11 |
| newell.rs:165 | newell_plane_residual | (p−centroid)·n̂ | m | OK |
| certify.rs:849/858 | interval_span_forward/winding (Circle) | span × radius / (τ−span) × radius, through `Margin::metered` | m | OK. The radius IS the carrier's own parameter rate (`|dP/dθ| = r` exactly), the same number `pcurve_cache::param_rate` mints as an `InfSpeed` for a circle, so this crossing goes through the metric door like the Nurbs arm two rows down. Exact ⇒ inf, and both claims here are *definitely apart* (a forward span, headroom to one period), which is the inf side |
| certify.rs:872/877 | interval_span_forward/winding (Ellipse) | span × min(\|major\|, \|minor\|), through `Margin::metered` | m | OK. `|dP/dθ| ≥ min(|a|, |b|)` (the semi-axes carry no order and no sign; `minor` in the ordinary order), so the smaller semi-axis magnitude is a certified LOWER bound on the ellipse's own parameter rate — an `InfSpeed`, the same mint `param_rate` and `splitting::classify` make for the same kind. Conservative in the direction a forward claim needs |
| certify.rs:883 | interval_span_forward (Line) | span (t IS arc length) | m | OK |
| certify.rs:1164 | nurbs_span_meter | knot-domain length × speed_lower_bound() — the net's arc-length lower bound, reparametrization-invariant | m | OK (metered door; the collapsed-arm gate on the meter) |
| certify.rs:1175 | interval_span_forward (Nurbs) | span × (m/param) | m | OK |
| certify.rs:917/925 | carrier_endpoint_start/end | point distance | m | OK |
| certify.rs:950/958/992/1000 | carrier/tangent_on_surface_1/2 | implicit_residual (/2r-normalized) | m | OK |
| certify.rs:1015 | tangent_second_order | κ_rel·arm²/2 | m | OK |
| certify.rs:1982/1954 | tangent_normal_parallel | sinθ / κ_rel (arm = 1/κ_rel, the D4 ¶1 tangency lever) on a Positive second-order margin; sinθ × the folded lever arm (`dihedral::folded_lever_arm`) at a second-order refusal, where a definite reading only renames the refusal | m | OK (note N4) |
| certify.rs:1047 | carrier_matches_mapped_source | point distance | m | OK |
| certify.rs:1057/1068/1076 | carrier_on_seam_* | residual / radial·unit | m | OK |
| certify.rs:1103/1112 | tangent_hull_sup / tube_margin | m residual sums; κ·arm² | m | OK |
| certify.rs:1143/1151/1162 | witness_* | residuals / point distance | m | OK |
| intersect.rs:554/560/591 | pc_axis_plane_parallel / parallel_gap / rim_alignment | sin×extent; r−gap; sin×r | m | OK |
| intersect.rs:694 | ps_frame_seam | (sin−0.5)·r — deterministic frame tie-break, not a coincidence question | m | OK (note N5) |
| intersect.rs:705 | ps_center_gap | r − center-plane distance | m | OK |
| intersect.rs:834–882 | cc_* (radius eq, axes parallel, coaxial, gap, coplanar) | lengths / sin×extent / common-perpendicular | m | OK |
| intersect.rs:1000/1006/1043 | pn_apex_*, pn_axis_normal | m·unit; trig diff×extent; sin×rim r | m | OK |
| intersect.rs:1480/1484/1498/1504/1535/1544 | pt_tube_guard / ring_torus_convention (read from `geom::ring_torus`, the convention's one home) / axis_in_plane / axis_plane_gap / axis_normal / cap_gap | r; R−r; sin×extent; centre-plane gap; sin×R; r−station depth | m | OK — with the axis_in_plane lever CONDITION on record: just inside the Zero band the raw sine reaches ε/extent, so the minted meridian circles sit off their own PLANE by up to r·ε/extent (27ε measured at extent = 0.01; the torus residual stays machine-zero). Cannot bite with a real operand: extent ≥ R + r for any plane that reaches the torus, so the planarity error stays under ε |
| intersect.rs:2026/2047/2053/2068/2093/2111 | coc_cylinder_radius / coc_aperture_sin / coc_aperture_cos / coc_axes_parallel / coc_coaxial / coc_station_reach | R; sin α×extent; cos α×extent; sin×extent; axis-to-axis distance; extent − |R·cot α| | m | OK — the family prefix is `coc_`, not `cc_`: `cc_*` is cylinder×cylinder's glob two rows above and would over-match. `coc_station_reach` is the arm's ADMISSION criterion; the two aperture clauses are a division guard and a convention clause, not conditioning questions |
| pcurve_cache.rs:1225/1233 | pcurve_chart_azimuth_affine / winding | (rad coeff)×radius | m | OK |
| pcurve_cache.rs:1268 | pcurve_map_residual | mapped point distance | m | OK |
| pcurve_cache.rs:1964 | pcurve_interval_forward (harmonic) | span × param_rate | m | OK |
| pcurve_cache.rs:1988 | pcurve_azimuth_period (harmonic) | (τ−extent)·azimuth_lever | m | OK |
| pcurve_cache.rs:1894 | pcurve_interval_meter (fitted/iso gate) | carrier parameter extent × param_rate (a NURBS net's knot domain × its certified speed lower bound) | m | OK (metered door; the collapsed-arm gate) |
| pcurve_cache.rs:2382 / :2868 | pcurve_interval_forward (fitted / iso) | span × param_rate — a NURBS carrier's rate IS its certified speed lower bound | m | OK (metered door; the meter gated at :1894) |
| pcurve_cache.rs:2397 | pcurve_azimuth_period (fitted) | rad headroom × `chart_arms_at`'s azimuth lever (the cone's `v_sup·sin α`) | m | OK (levered door) |
| pcurve_cache.rs:1664 | pcurve_chart_radial_moving | Σ m-norms BARE (amplitude is metres) | m | FIXED (M6-3) |
| pcurve_cache.rs:1680/1772/1791 | pcurve_chart_orientation / sphere meridian | m² ÷ radius | m | OK |
| pcurve_cache.rs:1752 | pcurve_sphere_chart_frame | m at :1770, dimensionless at :1836 (tie-break) | mixed | FLAG (note N5) |
| pcurve_cache.rs:1759–1829 | pcurve_sphere_chart_* | m-scaled coefficients / rooted | m | OK |
| pcurve_cache.rs (iso lane, M7) | pcurve_iso_boundary / iso_axis_u/v / iso_domain | chart-param values/extents/overhangs × stretch bounds (m per chart unit); two definite non-zero `pcurve_iso_boundary` verdicts route the seam class through a `pcurve_iso_domain` decide of the fixed channel's overshoot (× `stretch_u`; a column outside the domain refuses typed) to the collapsed-row hull (an interior column, `nurbs_iso::interior_iso_u`), whose slack is the channel's drift alone — no snap term | m | OK (**`metered_sup` door**: `nurbs_stretch_bounds` answers a `SupSpeed` pair and every row here meters an overshoot or a snap slack; door added by the clause-(i) migration) |
| pcurve_cache.rs (ARC-RIM iso class, M8-3) | pcurve_interval_forward / pcurve_iso_boundary | span × `param_rate` = arc LENGTH (the class's carrier is a `Curve3::Circle` by construction, so the rate is the radius); the sub-arc weight residual `w − cos(h/2)` metered at the radius | m | OK (metered door, so an `InfSpeed`: the class's rate is a circle radius, an exact rate and therefore an inf bound, and it cannot be poison — which is why it carries no meter gate. The arc rim's chart-column knot deviation, metred through `stretch_u`, takes `metered_sup` with the rest of the iso lane) |
| pcurve_cache.rs (iso/fitted lanes) | pcurve_envelope | certified sup bound (m); on the seam class the traversed row's control-difference hull, a boundary row's copy or an interior column's de Boor collapse (each collapsed control point encloses the exact one at the interval scalar, so the hull's upper end still bounds the sup) | m | OK (added by the clause-(i) migration) |
| pcurve_cache.rs (fitted lane, `MapResidualHermite` statement) | pcurve_envelope_hermite | the sphere general-circle image's Hermite sup bound, re-derived by `certify_fitted` (m); refined to `ε/4` by `sphere_circle_image_lane`, so ε-coupled by construction and judged by `tools/k-lint`'s `CONSTRUCTION_COUPLED` | m | OK |
| pcurve_cache.rs (chart derivation, M6-3) | pcurve_cone/sphere/torus_chart_axial / _centered / chart_radial_moving | axial displacement sums; radial-offset norms; Σ m-norms | m | OK (added by the clause-(i) migration) |
| pcurve_cache.rs (chart derivation) | pcurve_chart_orientation / sphere/torus_chart_meridian | oriented area a×b·n̂ over its radius lever (m²/m) | m | OK (over_lever door; added by the clause-(i) migration) |
| pcurve_cache.rs (chart derivation) | pcurve_cone_chart_nappe (h0/h data) | axial heights (m) | m | OK; the hs COSINE fallback is FLAG F13 |
| pcurve_cache.rs (`spiric_chart_pcurve`, torus arm) | pcurve_spiric_chart_axis | `chart.axis · carrier.axis`, a cosine of unit vectors, levered at the chart's `major_radius` | m | OK (**`levered` door** — a dimensionless cosine is metered by MULTIPLYING an arm; `over_lever` divides a measure by one and is for an oriented area over a radius. Zero refuses — a torus whose axis is perpendicular to the carrier's is not the torus this spiric sections) |
| pcurve_cache.rs (`run_spiric_checks`, check 1) | pcurve_spiric_chart_center / _chart_major / _chart_minor | the CHART torus's centre, `R` and `r` against the CARRIER's, bare (all three are metres) | m | OK; Zero required. The identity is a statement about the map, so the chart being the carrier's own torus is its premise — and what the band admits is priced into check 4 rather than discarded |
| pcurve_cache.rs (`run_spiric_checks`, check 1) | pcurve_spiric_chart_tilt | `\|chart.axis × carrier.axis\|`, a sine of unit vectors, levered at the chart's `R + r` | m | OK (levered door; PARALLELISM only — which of the two directions the frame takes is `pcurve_spiric_chart_axis`'s decision at the mint, and a wrong sign is a whole-span displacement the schedule sees at every sample) |
| pcurve_cache.rs (`run_spiric_checks`, check 1) | pcurve_spiric_major / _minor / _offset | the image's stored scalar minus the CARRIER's own, bare (all three are metres) | m | OK; Zero required, and whatever the band admits is carried into check 4's envelope through the `f` drift bound rather than discarded |
| pcurve_cache.rs (`run_spiric_checks`, check 1) | pcurve_spiric_sense | `\|sense\| − 1`, dimensionless, levered at `azimuth_lever` | m | OK (**`levered` door**, so the Zero window is `\|η\| ≤ ε/(R + r)`; Zero required, and the residue is metered into the envelope on BOTH channels — `v` at the chart's minor radius over the parameter reach, `u` at `R + r` over `atan2`'s own `π` range) |
| pcurve_cache.rs (chart derivation) | pcurve_chart_azimuth_frame / sphere_chart_pole_frame / polar & meridional rates | metre projections/norms at six of the seven frame callers; on the CONE ruling lane's F13 fallback the frame input is a UNIT radial's projection — dimensionless. Tie-break-only either way (N5: the trilean picks between two formulas identical mod τ — verdict-neutral), and that lane is F13-flagged one decision earlier | m (mixed on the F13 lane) | OK as tie-break (N5; row corrected at the clause-(i) fix pass, review MIN-1) |
| props/curved.rs (`require_rim_incidence`) | props_rim_axis_parallel / props_rim_center_on_axis | sin×r_c; perpendicular offset | m | OK |
| props/curved.rs (`level_coincides`, `props_rim_level_group` call) | props_rim_level_group (Length) | level difference BARE (v is arc length) | m | FIXED (#89's unit) |
| props/curved.rs (`level_coincides`, `props_rim_level_group` call) | props_rim_level_group (Unit) | rooted (sin,cos) CHORD × `RimArms::level` (sphere ×R, torus ×minor) | m | **FIXED — N1 RETIRED** (S81: one rule, one arm. Was Δ(sin,cos) componentwise × `major` on the torus) |
| props/curved.rs (`du_of_rims`) | props_du_consistent | Δu (rad) × `RimArms::azimuth` | m | OK |
| props/curved.rs (`require_rim_interior_sides`) | props_rim_interior_side | `rim_offset_margin` pointed by σ: the same per-kind comparand as `props_rim_side`, bare (Length) / × `RimArms::level` (Unit), multiplied by an exact ±1 | m | OK (note N2; σ is a product of two discrete signs and reads no margin of its own) |
| props/curved.rs (`sphere_rim_only_pole_level`, `cone_apex_level`, and `boundary_material_sign`'s sphere arm) | props_rim_only_extent | per kind, `require_extent`'s OWN comparand asked one step earlier: `sphere_extent_margin` — `(hi − lo)·R` — on the sphere, from the one helper both read; the bare slant difference `hi − lo` on the cone, as `require_extent`'s cone call reads it | m | OK (note N9: ONE comparand under TWO names, so a meridian-free rim-bearing sphere face records both; note N8 applies verbatim — the sine extent shrinks by `cos v̄` near the poles, in the FOLDING direction here, which is the direction that serves the cap) |
| props/curved.rs (`require_rim_only_closed`) | props_rim_only_closed | `(Δu − τ)` × `RimArms::azimuth` — the arc a rim whose face folded a missing extreme fails to close by; the sphere's pole and the cone's apex share it, at each kind's own azimuthal arm (`R`, and the cone's first rim radius) | m | OK (the azimuthal arm `props_du_consistent` already meters) |
| props/curved.rs (`require_rim_only_closed`) | props_rim_only_join | `‖p_end(i) − p_start(i+1)‖` over the group's arcs, cyclically — a point deviation, bare (metres already), the comparand `require_rim_incidence` meters an incidence with | m | OK (the tiling half of the closure guard: the sum says the spans total a turn, this says the arcs actually chain, and a sum without a chain covered half a circle and answered a whole cap) |
| props/curved.rs (`linear_rim_side`'s nested `side`) | props_rim_side | per-kind: bare (Length) / × `RimArms::level` (Unit) | m | FIXED (#89's unit); note N8 open — the sphere margin reads the PRIMARY component (`lo + hi − 2·sin v`), an axial quantity that shrinks by `cos v̄` near the poles, refusing direction |
| props/curved.rs (`cylinder_chart`) | props_loop_closed | `‖traversal end(i) − traversal start(i+1)‖` per junction, bare | m | OK (the cylinder Green form's closure premise; TANG, PR 3851) |
| props/curved.rs (`cylinder_chart`) | props_chart_loops_closed | `Σ Δu` over every loop's rims (rad), levered at the radius | m | OK (the face's loops wind the cylinder zero times — what makes `−Σ∮ v du` anchor-free) |
| props/curved.rs (`cylinder_material_sign`) | props_chart_area_side | `2·R·A_chart / P` — the chart area in m² over the boundary length, the mean width (F4) | m | OK (`over_lever`; the planar loop-winding comparand on the wall's chart) |
| props/curved.rs (`cylinder_boundary`'s line arm / `cone_boundary`'s line arm) | props_meridian_axial / props_meridian_generator | sin (or cos-diff) × parameter span (m for lines) | m | OK |
| props/curved.rs (the four `*_boundary` parses) | props_meridian_on_surface / props_rim_fit (all kinds) | residuals; sphere/torus fits ROOTED before compare | m | OK |
| props/curved.rs (the four `*_boundary` parses) | props_circle_axis_class | cos × r_c | m | OK (note N3) |
| props/curved.rs (`require_extent`, called from all four flux lanes) | props_face_extent | m levels; sin-levels ×R; dt×minor | m | OK (note N8 open on the sphere arm — `(hi − lo)·R` is the AXIAL extent, which shrinks by `cos v̄` for a near-polar band, refusing direction) |
| props/curved.rs (`cone_boundary`'s line arm) | props_meridian_apex | apex-line distance | m | OK |
| props/curved.rs (`cone`'s single-nappe check) | props_cone_nappe | slant levels (m) bare | m | OK |
| props/curved.rs (`sphere_boundary`'s meridian arm, `torus_boundary`, `torus_meridian_orient`) | props_meridian_great / props_band_coplanar / props_meridian_orient | lengths / sin×R / cos×minor | m | OK |
| props/curved.rs (`sphere_tilted_circle`) | props_sphere_circle_tilt / props_sphere_circle_great | `‖n_c × â‖` levered by the circle radius; the centre offset and radius misfit (m) | m | OK |
| props/curved.rs (`sphere_circle_loop`) | props_sphere_circle_on / props_sphere_circle_fit / props_sphere_loop_closed | `‖(C − c) × n_c‖`; `√(‖C − c‖² + ρ²) − R`; the gap between one arc's end and the next's start (m) | m | OK |
| props/curved.rs (`sphere_circle_loop`, `require_cusp_free`) | props_sphere_loop_cusp / props_sphere_loop_area | the chord between a departing and a reversed arriving unit tangent, levered by R; the smaller of the face's and its complement's solid angle (sr), levered by R | m | OK |
| props/curved.rs (`side_on_meridian`) | props_sphere_side_meridian / props_sphere_side_plane / props_sphere_side_roots / props_sphere_side_half / props_sphere_side_order | a point's distance from the polar axis; `√(a² + b²)` and `k·(C − c)`; `√(a² + b²) − |d|`; a point's offset across the axis; a height difference (all m) | m | OK |
| props/curved.rs (`side_on_meridian`) | props_sphere_side_in_arc / props_sphere_side_enter | a parameter gap (rad) levered by the circle radius; the cosine of the loop's left direction against the walk, levered by R | m | OK |
| props/curved.rs (`torus_meridian_orient`: `UnitVec3::levered`, then `OrthoFrame::from_aim_and_reference`) | props_torus_axis / props_meridian_radial | the torus axis's norm (a unit-at-rest carrier field, a pure number) levered by the anchor meridian's reach from the torus centre, `\|c − centre\| + r` / `Margin::norm3` of the anchor meridian centre's offset from the axis (m) | m / m | FIXED (was the axis's bare norm against the length band, which reads differently at every model scale) |
| boolean/join.rs (germ-plane read: `UnitVec3::levered` of a plane germ's carrier normal) | bool_germ_plane_normal | the normal's norm (a unit-at-rest carrier field, a pure number) levered by a lower bound on the reach the section consumes it over: the ball through the two joined germ sites, from the plane's origin (`ExtentBall::lever_from`) | m | FIXED (was the bare norm against the length band; `sweep`'s `ray_wall_margin_twins` pins it on a plane × cylinder bore) |
| boolean/boxes.rs (`face_box_rule`'s cylinder read: `UnitVec3::levered` of the carrier's axis) | bool_box_cylinder_axis | the axis's norm (a unit-at-rest carrier field, a pure number) levered by the radius, the arm `slab_extent` swings it by | m | FIXED (was the bare norm against the length band) |
| props/curved.rs (`require_band_opposite`, the rimless arm's coplanar branch, after `props_band_coplanar` has put every meridian on one great circle) | props_band_opposite | at each junction of the loop, the chord between the unit traversal tangent arriving (arc i's traversal end) and the one departing (arc i+1's traversal start), × R — the distance between the two arcs' departure points scaled to the sphere radius | m | OK (added by the BOOL-5 fix pass, issue 542 / S-BOOL. Lever R, the run's linear band. Zero at every junction ⇒ the loop runs its great circle once, the two-band face, `Δu = π`; Positive ⇒ the loop reverses there — two arcs on one half-plane, a slit of no width or the ball less a slit — typed refusal; in-band escalates. Coplanarity alone cannot tell opposite half-planes from coincident ones, and the coincident pair is reachable through `revolve` because its angle door levers at the profile's `r_max` while the coplanar decide levers at the face's R. Stated at the junction rather than as the chord between the two arcs' departure directions so that a great circle split at ordinary points or into more than two arcs — CERT-1's rows — keeps measuring; for two pole-to-pole arcs the two readings coincide) |
| props/curved.rs (`sphere_wedge_azimuth`, the rimless arm's wedge branch, reached when `props_band_coplanar` is definitely nonzero) | props_wedge_azimuth | the signed azimuth `φ = atan2(d_B·I_A, d_B·d_A)` (rad) from meridian A's half-plane into the face's interior direction `I_A = ν·f_A·n_A` (sense bit × forward bit × carrier axis) to meridian B's half-plane, × R — the equatorial arc between the two meridian planes on the face's side, signed by whether it is the short one | m | OK (added with the rim-free spherical wedge arm, issue 542 / S-BOOL. Lever R, the run's linear band. Positive ⇒ `Δu = φ`, Negative ⇒ `Δu = φ + 2π`, Zero ⇒ `DegenerateFace` (coincident meridians), in-band escalates. The Zero/in-band outcomes are the arm's D2 floor rather than a door, unreachable by the factor K: the arm is entered only on `R·|sin φ| ≥ escalate = K·zero` under `props_band_coplanar` at the same band and lever, and `|φ| ≥ |sin φ|`, so `R·|φ|` is at least K coincidence widths above the Zero edge — a typed refusal kept because the inventory states every outcome, not a rounding window. The `atan2` is safe here where the pole helper forbids it: its branch cut is the opposite-half-plane pair, which the coplanar decide has just excluded by at least the escalate width at R) |
| props/curved.rs (`sphere_meridian_pole_margins`, decided by `sphere_meridian_span_levels` AND by `require_one_chart_branch`) | props_meridian_pole | chord from the pole's span-relative direction to the nearer span endpoint, carrying the membership sign (`copysign` of a midpoint dot test) × R — the point deviation of moving the pole onto the span boundary | m | OK (added with the span-derived sphere extent, issue 723 / S-CERT. ONE margin, TWO dispositions since issue 1571, which is why the arithmetic has its own home: the EXTENT FOLD takes everything but a definite Negative — **Positive, Zero and the indeterminate band alike fold the pole latitude** — while the BRANCH DOOR refuses only a definite Positive, so the gap between them is exactly the arc that ENDS at the pole, which both admit. Neither disposition escalates, so the note below holds for both. Near a span end the endpoint latitude is within band² of the pole's, so the fold choices agree far inside tolerance, the folded extent is continuous across the decision, and an in-band margin carries no information a refusal could report — refusing there flipped certify-exactly into an import escalation for a split vertex 1e-6 rad off the pole. The site still RECORDS through `decide` like any classify site; it just never escalates, so its in-band population is expected (issue 1251 schedules the K-baseline fold-in). **Stated for a span of at most one period, and decided so first** (issue 1601): the membership edge is the unclamped `cos(dt/2)`, whose zero set is the two span endpoints only while `dt ≤ 2π`; the helper itself decides `props_meridian_span_forward` and `props_meridian_span_winding` (next two rows) before this margin is formed, so an admitted span is forward and exceeds τ by at most `zero/R`, where the edge cosine is within `(zero/2R)²/2` of its half-turn value and can reclassify only a pole within `zero/2R` of the span endpoint — in-band on both dispositions. The retired half-turn clamp had a zero set at the direction antipodal to the span midpoint, an interior point of any longer span, where a rounding residual folded 36 of 400 `2π + 2δ` spans short. Direction arithmetic throughout — no `atan2`, no mod-2π `floor`: either is wide at its cut/step for an interval enclosure of an arc anchored at a pole, forcing an escalation the scalar lane does not have, live on the die-fillet corpus) |
| props/curved.rs (`require_meridian_span_within_period`, run by `sphere_meridian_pole_margins` before it forms a margin — so by `sphere_boundary`'s fold AND by `require_one_chart_branch`'s sphere arm) | props_meridian_span_forward | `Δt·R` — the stored span itself, levered at the sphere radius: certification's `interval_span_forward` re-decided at the parse | m | OK (added with the reversed-span refusal, issue 1601 / S-MESH fix pass. Same margin, band and lever as certification's, hence the same dispositions: only a definite `Positive` span admits; `Zero` refuses `NotIsoRectangle { what: "props_meridian_span_forward" }` as certification's `IntervalNotForward` does, definitely negative (a span stored reversed, `t1 < t0`) refuses the same way, the ambiguity band escalates. Decided before the winding headroom (next row), which cannot see a reversed span — its headroom `τ − Δt` is Positive for any `Δt < 0`. The torus decides the same half for a reconstructed span as `props_meridian_pieces_forward`. Reachable only from hand-built `LoopEdge`s, as the winding half is) |
| props/curved.rs (`require_meridian_span_within_period`, run by `sphere_meridian_pole_margins` before it forms a margin — so by `sphere_boundary`'s fold AND by `require_one_chart_branch`'s sphere arm) | props_meridian_span_winding | `(τ − Δt)·R` — the stored span's headroom to one period, levered at the sphere radius: certification's `interval_span_winding` re-decided at the parse | m | OK (added with the saturated-span refusal, issue 1601 / S-MESH. Same margin, band and lever as certification's, hence the same dispositions: Zero and Positive headroom admit — a span inside the coincidence band above τ is one certification admits too — the ambiguity band escalates, definitely negative headroom refuses `NotIsoRectangle { what: "props_meridian_span_winding" }` on every consumer of the sphere parse and at the branch door, which does not run the parse but reaches the pole helper, where the decide lives. The torus decides the same invariant for a reconstructed span as `props_meridian_pieces_winding`; the sphere has no fold and decides it per edge. Reachable only from hand-built `LoopEdge`s: every certified door hands the parse a span within the bound, and the import door normalises into `(0, τ]`) |
| props/curved.rs (`require_one_chart_branch`'s cone arm) | props_cone_apex | the apex's own line parameter against the nearer span end, bare (a line's `t` IS arc length on the unit `dir` the parse certifies — the same argument `props_meridian_apex` makes one row up) | m | OK (added with the arc-branch door, issue 1571. Positive = the apex strictly inside the generator's stored span, where the chart `u` jumps to the mirror nappe and a walk reading one `u` per edge cannot read the edge; Zero and the indeterminate band ADMIT, so a generator ENDING at the apex — every `revolve` cone cap — passes. Signed as `min(t_apex − t0, t1 − t_apex)`: positive exactly on interior membership, and its magnitude is the metres to the nearer end, i.e. the point deviation of moving the apex onto the span boundary) |
| props/curved.rs (`require_rims_at_extremes`, through `level_coincides`) | props_rim_level | per-kind: bare level difference (cylinder/cone `Length`) / rooted (sin,cos) chord × `RimArms::level` (sphere ×R, torus ×minor) | m | **FIXED — N7 RETIRED** (N1 RETIRED earlier. Generalised from the torus-only site to all four kinds by S58/#649, and unified with its sibling `props_rim_level_group` by S81 — ONE rule (`level_coincides`), one metric (the chord), one arm (`RimArms::level`), one fail direction; the two names are the funnel's recording channels, not two rules, and the metering is still carried by [`RimLevel`]. N7's near-polar sphere understatement — an axial-only `(sin v, 0)` pair whose chord collapsed by `cos v̄`, merging distinct near-polar rims in the ACCEPTING direction on both names, this refusing one included — is retired by the full `(sin v, cos v)` pair (issue 893 / S-CERT; this verdict column previously said `OK` while N7's own prose recorded the collapse). Pinned as scale twins by `geom-brep/tests/rim_dim_scale_twins.rs` and, in suites CI runs, by `geom-brep/tests/s81_one_rim_level_rule.rs` and the near-polar rows of `geom-brep/tests/cert1_sphere_polar.rs`.) |
| props/quad.rs:453 | props_quad_converged | ε·F − flux-width(m³)/(3·area(m²)) | m | OK |
| props/quad.rs:461 | props_quad_face_extent | area/perimeter (mean width) | m | OK |
| props/quad.rs (`piece_monotone`, asked once per trim piece by `trim_cells`) | props_trim_piece_monotone | `Margin::metered(span, rate)` — `span` is the control polygon's LEAST advance along the unit chord direction, a chart-parameter length (a convexity fact on the Bézier differences, which are the derivative's own coefficients up to the positive `p/h`); `rate` is the chart's metric rate along that same direction, `|S_u·ĉ_u + S_v·ĉ_v|` bounded BELOW over the piece box | m | OK (added with the trimmed-region quadrature, TRIM-2 PR-1. Clause (iii)'s metric door: a parameter-space span crosses to model space through a per-kind rate, and the rate here is the chart's own directional derivative rather than a carrier speed. Both factors round toward REFUSING — the span is the `lo()` of the bracketed dot product, and a derivative hull that straddles zero in every component answers rate `0`, hence margin `0`, hence Zero. Only a definite `Positive` admits the piece; `Zero` and the ambiguity band alike bisect the piece and re-ask, to `TRIM_MONOTONE_DEPTH`, and then refuse `QuadratureUnsupported` naming this row — the lane never pads a piece it cannot certify is a graph over its chord. **The margin carries the ROUND's lever as well as the image's, and a reader of this row has to know it**: `span` is the REFINED block's least advance, so it halves with every uniform cut and every bisection AND scales with the face. Measured (the v6 dual, both arms independently): the same smooth parabolic arc certifies at every round on a 10 µm face, refuses at round 4 on a 1 µm face, and refuses at every round on a 0.1 µm face — that is the lane's resolution speaking, not the image's shape, which is why the refusal names an UNRESOLVED margin rather than a cusp. Nothing on today's corpus is near the band (the fixture's scale is 1e-3 and round 0 is decades clear). An in-band verdict ESCALATES, as it does from every other lane in this file: bisection halves the very span the margin meters, so it cannot resolve one. Pinned by `geom-brep`'s `q5_a_fold_over_its_chord_refuses_typed` (a fold that survives every bisection) and `q5b_a_piece_that_only_looks_folded_resolves_by_bisection` (the curvature artefact the ladder is FOR — a refusal is cheap to get right by refusing everything).) |
| ssi.rs:645 | ssi_cs_tangency | radius/axis distance differences | m | OK |
| ssi/certify.rs:366–524 | ssi_on_locus / hull_sup / foot / chart | residuals, /2R linearizations, foot distances | m | OK |
| ssi/certify.rs:836 | ssi_tube_transversality | sin (unit triple product) × arm | m | OK |
| ssi/march.rs:295/310 | ssi_transversality_arm / ssi_transversality | arm (m); sin × arm | m | OK |
| ssi/march.rs:420/447/478 | ssi_step_progress / branch_open_end / closure_return | state × (m/state); scaled domain margins | m | OK |
| ssi/march.rs:484 | ssi_closure_tangent | cos(unit tangents) × whole-branch arc length | m | FLAG F9 |
| locus.rs (M9-2 PR-2) | pc_axis_plane_parallel / cc_axes_parallel (the witness lane's reads of intersect.rs's rows) | sin(axis, plane / axis, axis) × the declared pair's consumed extent read from the foot of its centre on the (second) cylinder's axis, where the gap row is read (`ExtentBall::lever_from`, the extent topo's carrier-pair doors lever their ladder at) | m | FIXED (TANG; was a 1 m `T::one()` arm, which bridged a tilt standing more than Kε off across a face longer than a metre; the lane's own `tangent_locus_axis_parallel` row retired into the section's) |
| locus.rs (M9-2 PR-2) | pc_parallel_gap / cc_parallel_gap / tangent_locus_internal_gap / tangent_locus_side | r − axis-to-plane distance at the foot; r1 + r2 − axis-to-axis distance; \|r1 − r2\| − axis-to-axis distance; axis offset / radius difference — all metre data of the carriers | m | OK (the plane×cylinder and external-cylinder gaps are the section classifiers' rows; the internal gap and side rows are the lane's own) |

## topo

| site | predicate | comparand | dim | status |
|---|---|---|---|---|
| boolean/contain.rs (`boundary_pre_pass`) | bool_contact_vertex / bool_contact_edge_length / bool_contact_edge | point distance; a straight edge's own length (the degeneracy gate) and the distance from the point to its closed segment, through `ray_parity::on_segment` | m | OK (CONTACT-4) |
| boolean/contain.rs (`boundary_pre_pass`, a conic's `End`) | bool_contact_arc_end_vertex | the distance from a conic edge's carrier end to a stored vertex — any in-band value escalates, and a definite one escalates too (a body certified at a coarser band carries up to that band's ε) | m | OK (CONTACT-4) |
| boolean/contain.rs (`boundary_pre_pass`, through `splitting::containment::LoopEdge::contact`) | bool_contact_arc_span / bool_contact_arc / bool_contact_arc_end / bool_contact_arc_trim | `(τ − w)` levered by the smaller semi-axis. A CIRCLE: the distance from the circle `√(((ρ − 1)·r)² + axial²)` (the quantity `point_on_circle` meters under this name), the unit-circle chord to either end and the chordal-defect sum, each levered by the radius — exact. An ELLIPSE: the distance bounded on both sides, `on` decided on the upper bound (a point of the ellipse one Newton step and a radial snap from `q`) for ON and on the lower bound `2|F|/(g + √(g² + 4|F|/b²))` for OFF; `end` the exact distance from `q` to the end point; `trim` the chordal defect of the foot levered by the LARGER semi-axis, so it bounds the arc length to the nearer end from above | m | OK (CONTACT-4) |
| boolean/insert.rs (`strut_order`) | bool_strut_side | `n̂·(ĝ × ê)`: the sine of a strut germ's angle from the arrival edge, × min sector arm — its distance at that arm from the arrival edge's line; a decided zero is placed by `bool_dir_same` (`sectors::direction_sense`) | m | OK (JOIN reflex corner) |
| boolean/insert.rs (`strut_order`) | bool_strut_order | `n̂·(ĝ₀ × ĝ₁)`: the sine of the angle between two strut germs in one half-turn, × min sector arm; a decided zero refuses (`decide_nonzero_reported`) | m | FIXED (JOIN reflex corner: was the cosine difference `(ĝ₀ − ĝ₁)·ê` × arm, second order in the spacing beside 0 and π, with a decided zero read as an order; before that, dimensionless) |
| boolean/insert.rs (`walks_after`) | bool_shared_cut_order | (unit cut dir × unit cut dir)·(unit sector normal) × sector arm — two crossing pairs' cuts in one corner of a vertex both cut, ordered along the corner; a decided zero (the cuts along one direction, checked one ray by `bool_dir_same`) is placed by the runs: held only when the other pair's run leaves the direction the same way and is the same arc (`tied_held`), and not when the two runs are struts one of which holds the other whole (`holds_whole`); and an in-band reading refuses as a `Coincide::Sectors` coincidence, as `bool_strut_order` does | m | OK |
| boolean/insert.rs (`germ_dir`) | bool_germ_line | sin(n̂_a,n̂_b) × min sector arm — the margin `pair_search` read definite before it recorded the pair as a crossing | m | OK |
| boolean/join.rs:567/803 | bool_join_chord | germ-site chord LENGTH (the degeneracy gate: Zero ⇒ coincident sites, no polygon edge) | m | OK |
| boolean/join.rs:603/817 | bool_join_nearest | a DIFFERENCE of two chord lengths (nearest-candidate selection) | m | OK |
| boolean/join.rs:743/744 | bool_join_facing | unit germ dir · chord (cos × separation) | m | FIXED (was bare cosine, `/dist`) |
| boolean/join.rs:750/751 | bool_join_arc_facing | axis·((p−c)×dir) — radius-metered sine | m | OK |
| boolean/join.rs:1093 | bool_ring_run_winding | (n̂ · Newell sum) / run perimeter — 2A/P, the run's mean width | m | FIXED (F4; was a bare **m² AREA**) |
| boolean/ops.rs (`bounded`) | volume_backstop_operand | V/A — the operand's mean thickness | m | FIXED (F3); on the INVARIANT LANE since Ev's #213 layering ruling — bare `T`, outside the length seam by design |
| boolean/ops.rs (`bound_holds`, arm 2) | volume_backstop | ΔV over the summed area of the bodies the inequality compares — mean boundary displacement | m | FIXED (F3); INVARIANT LANE (see above) |
| boolean/ops.rs (`Posture::read`, arm 1) | volume_backstop_violation | the same length, against the EXACT bit-hairline band — a sign question, not a magnitude one | m (band-free) | OK by design (note N6's category; #200 review MAJ-1); INVARIANT LANE (see above) |
| boolean/ops.rs (`Posture::read`, the +V arm) | positive_volume / positive_volume_enclosure | V/A — tier 3's check-7 reading (`validate::plus_v_read`), shared | m | OK (one home with tier 3's check 7) |
| boolean/ops.rs:1194–1480 | bool_sphere_* | radius/gap differences; sin × radius | m | OK |
| boolean/plane_eq.rs:174/233 | bool_plane_parallel | sin(n̂1,n̂2) × arm | m | OK |
| boolean/plane_eq.rs:190/252 | bool_plane_orient | cos(n̂1,n̂2) × arm | m | FIXED (was bare cosine) |
| boolean/plane_eq.rs:203/265 | bool_plane_offset | signed-offset difference | m | OK |
| boolean/carrier_eq.rs (`at_consumed_extent`, the undeclared posture through `rest::carrier_pair_verdict`) | bool_plane_parallel / bool_plane_orient / carrier_cyl_axis_parallel / carrier_torus_axis_parallel at the carrier-pair door | sin (cos) of the two normals or axes × the farthest reach of a ball enclosing both faces from the pivot the kind's position datum is read at, re-anchored nearest the ball's centre: the centre itself (plane, offsets read there), the foot of the centre on the second axis (cylinder), the second torus centre | m | FIXED (TANG; was a 1 m `T::one()` arm) |
| boolean/carrier_eq.rs (`declared_reading`) | bool_plane_reach / carrier_sphere_reach / carrier_cyl_reach / carrier_torus_reach | the declared pair's displacement over its consumed extent, bounded above at every point of the ball: the position datum read at the pivot + the axes' or normals' chord `\|a₁ − σa₂\|` × the reach from it (a levered value, through `Margin::levered`) + the radius differences, summed; a sum of lengths | m | OK (TANG; replaces deciding each datum on its own, which bridged nearly twice the band) |
| boolean/carrier_eq.rs (`declared_reading`) | bool_plane_reach_floor / carrier_*_reach_floor | the larger of the radius difference less the rest of the upper bound (a bound across the whole ball) and each face vertex's distance from the other carrier less its own; floored at zero | m | OK (TANG) |
| boolean/recl.rs:224–748 | side_code / bool_dir_same / bool_ee_collinear | side_code as in the sectors.rs row (`flank_key`, edge-edge membership); cos/sin × sector arms for the rest | m | OK |
| boolean/recl.rs (`resolve_bisector_graze`) | bool_sector_bisector_side | refusal only: a grazing bisector between keys definitely on one side, reachable only at K ≤ 2 (argument at `vtxfac`'s on-edge resolution) | — | OK (CONTACT-9) |
| boolean/reduce.rs:548–802 | bool_vertex_face_side / circle & line clearances | plane residuals, /2r residual extremes, sagitta dips | m | OK |
| boolean/reduce.rs (`bool_conic_curved_clearance`'s ARC half) | bool_conic_curved_clearance | `geom_brep::conic_arc_residual_range`: a hull of `implicit_residual` samples (m) widened by the chord-dip charge `f2·h²/8`, `f2` in **m/rad²** and `h` in rad — a residual's second derivative with respect to an ANGLE, so the product is a length and the comparand stays a length | m | OK |
| geom-brep/implicit.rs (`circle_residual_curvature_bound`, `circle_arc_residual_range`) | — (no funnel call; the comparand is built here and decided at the row above) | `f2` is `|(d²)″|/2r`: `(d²)″` is m²/rad² over the `2r` linearization, giving **m/rad²**; the returned range is m | m/rad² and m | OK |
| boolean/circle_cylinder.rs (`circle_cylinder_roots`, the arm switch) | bool_circle_cylinder_tilt | `ρ·\|â × ŵ\|` — the carrier's tilt to the wall axis levered at its own radius, the height its points swing by | m | OK |
| boolean/circle_cylinder.rs (square arm, through `circle_roots::first_harmonic_roots`) | bool_circle_cylinder_square_noise / _square_coaxial / _square_extreme / _square_root_slack | the harmonics are the `/2r` residual itself (`geom_brep::conic_cylinder_harmonics`): noise = rounding of the m² term bound over `2r`, plus the dropped second harmonic (m); `A₁`; the extremes `c₀ ∓ A₁`, and a constant residual's `c ∓ s` with its spread `s = A₁ + noise` (`constant_residual_roots`); the root's arc slack `ρ·noise/√(A₁² − c₀²)`, m·m/m | m | OK |
| boolean/circle_sphere.rs (through `circle_roots::first_harmonic_roots`) | bool_circle_sphere_noise / _coaxial / _extreme / _root_slack | the extremes are the `/2r` residual's own (`geom_brep::circle_sphere_harmonic`): noise = the larger of the extremes' error bounds, each a running rounding bound plus the frame's orthonormality charge `(|e|² + ρ²)·‖GᵀG − I‖/2r` (m); `A₁ = (hi − lo)/2`; the factored extremes `(D∓ − r)(D∓ + r)/2r`, and a constant residual's `c ∓ s` with `s = A₁ + noise`; the root's arc slack `ρ·(δR/√(−lo·hi) + δφ + τ·NOISE_ULPS·u/2)`, `δR = (hi·δlo − lo·δhi)/(hi − lo)` (m) and `δφ` the phase's error (rad) | m | OK |
| boolean/circle_cylinder.rs (tilted arm, through `circle_roots::half_angle_roots`) | bool_circle_cylinder_ladder_noise / _pole / _pole_conditioning | `noise / f_per_metre` with `F` the residual itself (`f_per_metre = 1`); the residual at the pole; `ρ·(\|F(pole)\| − κA)/A`, a radius times a ratio of residuals | m | OK |
| boolean/circle_cylinder.rs (tilted arm's quartic, `solid_contain::depressed_quartic_roots`) | bool_circle_cylinder_disc / _shape / _depth / _odd / _split / _split_lead | the ladder's coefficients in the root variable `τ = 2ρ·t` (a length), each over the power of the lever `2ρ` that makes it a length (`Δ`/lever¹¹, `p`/lever, `D`/lever³, `q̂`/lever², factor discriminants/lever) | m | OK |
| boolean/circle_torus.rs (coaxial arm, through `circle_roots::constant_residual_roots`) | bool_circle_torus_coaxial_residual | the torus `implicit_residual` at `θ = 0`, `(g² − r²)/2r`, `∓` its spread `2δ(g₀ + 2δ)/r` with `δ = offset + √2·tilt` (both m), a length times a ratio of lengths | m | OK |
| boolean/arcs.rs (`arcs_along`) | bool_arc_along / bool_arc_ahead | `\|t̂ × d̂\|` and `t̂ · d̂` of two unit tangents, each levered at the arc's radius (`Margin::levered`): sin and cos of the angle between them × radius, the offset the arc's far side swings by | m | OK |
| boolean/reduce.rs (`arc_chain_reaches`, `boundary_meets_circle_only_at`) | bool_arc_chain_on_circle / bool_arc_boundary_off_circle | a point's distance from a circle, `√(h² + (ρ − r)²)` (`arcs::circle_miss`): its height over the circle's plane and its axial distance minus the radius, both m | m | OK |
| boolean/reduce.rs (`boundary_meets_circle_only_at`) | bool_arc_plane_side | a vertex's signed height over the circle's plane, `(p − c) · â` with `â` unit; a conic's own plane offset from it (`ConicPlaneMeet::Parallel`) | m | OK |
| boolean/rest.rs:401 | bool_join_chord | germ-site chord LENGTH | m | OK |
| boolean/rest.rs:411/413 | bool_join_facing | unit dir · chord | m | FIXED (was bare cosine) |
| boolean/rest.rs:421 | bool_join_nearest | a DIFFERENCE of two chord lengths | m | OK |
| boolean/sectors.rs:342–433 | bool_sector_within / bool_dir_* / bool_faces_parallel | sin/cos × sector arm (arm = shorter bounding chord, m; every caller passes unit dirs — verified); a pair `bool_faces_parallel` reads Zero is a near-coincidence and goes to the carrier ladder with every code On (its arm-setting bound reads On or in band) | m | OK |
| boolean/sectors.rs (`side_code`) | bool_chord_side / enters_material / bool_pierce_sector_side_curved | a LINE bound: its far vertex's signed distance from the plane through the base vertex (`sector_shape::plane_offset`); a curved bound: cos × its own extent; a bisector: cos × its sector's arm; the curvature charge: the bound's tangent RAY's least separation from the face past the sagitta, `slope·l − l²/lever` at `l = min(slope·lever/2, reach)`, where it peaks within the bound's own reach (a statement about the ray for every reach kind, not a point of a curved edge or a bisector) | m | FIXED (CONTACT-9; every bound was cos × the shorter sector arm, so a long line edge read On while its far end stood hundreds of bands off) |
| boolean/solid_contain.rs:438 | bool_wall_trim_period | (τ−width)·radius | m | OK |
| boolean/solid_contain.rs:462 | bool_wall_trim (cone term) | (cosΔ−cos h)·radius — effective arm sin(h)·r, collapses for narrow windows | m | FLAG F8 |
| boolean/solid_contain.rs (`wall_outline`) | bool_wall_iso_meridian / bool_wall_iso_rim / bool_wall_section_tilt | sin or cos of unit vectors × radius; radius and off-axis differences | m | OK |
| boolean/solid_contain.rs (`wall_outline`) | bool_wall_section_seat | off-axis distance; `minor − r`; `major·|n̂·â| − r`; the major axis's minor-direction component × `(major − minor)`, the displacement a rotation in the plane causes | m | OK |
| boolean/solid_contain.rs (`wall_outline`) | bool_wall_piece_span | azimuth extent × radius | m | OK |
| boolean/solid_contain.rs (`rim_levels`) | bool_wall_rim_level | offset of one rim plane from another along the axis | m | OK |
| boolean/solid_contain.rs (`point_on_chart_wall`) | bool_wall_trim (piece side) | perpendicular distance from a unit-normal plane | m | OK |
| boolean/solid_contain.rs (`point_on_chart_wall`) | bool_wall_junction | cos and sin of the angle between two unit radial directions × radius | m | OK |
| boolean/solid_contain.rs (`wall_hit_outside_reach`) | bool_wall_outline_reach | distance to the ball's centre − its radius | m | OK |
| boolean/solid_contain.rs:538/562/587 | bool_point_in_solid_plane | plane residual; /2r linearizations | m | OK |
| boolean/solid_contain.rs:645/655 | bool_point_in_solid_advance/order | ray parameters (m, unit dir) | m | OK |
| boolean/solid_contain.rs (`cast_ray`, plane arm) | bool_point_in_solid_denom (plane) | cos(unit,unit) levered by the selection's reach (`selection_reach`): the ray's rise off the plane over every length at which it could meet the face | m | FIXED (F2) |
| boolean/solid_contain.rs (`line_wall_roots`) | bool_point_in_solid_denom (cylinder) | `|d⊥|` levered by the run the caller reads (the ray: the selection's reach + 2r; an edge: its own span): the line's drift off its distance from the axis | m | FIXED (F2; was sin²/2r, 1/m) |
| boolean/solid_contain.rs (`line_wall_roots`) | bool_ray_cylinder_disc | disc/|d⊥|² over 2r (over_lever), the sphere arm's form | m | FIXED (F2; was disc/(2r)², dimensionless) |
| boolean/solid_contain.rs (`cast_ray`, wall arm) | (cylinder hit-outward) | read off the decided discriminant's root order, as the sphere lane reads it: no decision | — | FIXED (F2; was (unit·radial)/radius, dimensionless) |
| boolean/solid_contain.rs:850/903 | bool_ray_sphere_disc / at_infinity | disc/2r (over_lever); volume/area (V/A mean thickness, over_lever — a genuine containment decision, not a backstop) | m | OK |
| boolean/vtxfac.rs:106/113/453 | side_code / bool_sector_coplanar / bool_germ_line | side_code as in the sectors.rs row; `bool_sector_coplanar`: sin × sector arm, which only proposes coplanar (both bounds must also read On, in band too); `bool_germ_line` (`pierce_germ_dir`): sin × the sector's farther reach (`BoolSector::span`) | m | FIXED (CONTACT-9; the germ line was levered at the sector arm) |
| boolean/vtxfac.rs (on-edge resolution) | bool_sector_bisector_side | refusal only: a bisector reading On between two readings definitely on one side, reachable only at K ≤ 2 | — | OK (CONTACT-9) |
| census.rs:313–599 | pm_census_vv/ve/vf/ef gaps, spans, residuals | point/line/plane distances and spans (unit dirs verified) | m | OK |
| census.rs:614–746 | pm_census_span_* / ee_gap / ee_span / ee_overlap | span arithmetic (m) | m | OK |
| census.rs:666 | pm_census_ee_parallel | sin(unit dirs) × min(edge lengths) | m | FIXED (was bare sine) |
| census.rs:812/831 | pm_census_confirm_* | distances / residuals | m | OK |
| merge_faces.rs:924 | bool_ring_run_winding | (n̂ · Newell sum) / loop perimeter | m | FIXED (F4) |
| pcurves.rs (`spline_gap_closes`: `chart_u_arm`, `v_meter`) | pcurve_loop_continuity | a SPLINE chart's (`Nurbs`/`Approx`) joint gap: Δu × `chart_u_arm` and Δv × `v_meter`, `geom_brep::chart_stretch_sup`'s `(sup \|S_u\|, sup \|S_v\|)`, the chart's own metre stretch | m | OK (`metered_sup` door: an escape metred through a certified upper bound can only refuse) |
| pcurves.rs (`near_pole_gap_closes`: `joint_arm`) | pcurve_loop_pole_gap | at a joint whose singular incidence AND orbit marks are undecided: (gap − m·step) × the vertex's own distance from the axis, for the orbit points m = 0 (and ±1 where the image has a sphere twin) | m | OK (levered door; the lever is the vertex's own, so an escape it admits is ≤ ε in metres at that vertex) |
| pcurves.rs (`lift_joint`: `joint_arm`) | pcurve_loop_branch | (gap − (m+½)·step) × the joint vertex's distance from the chart axis (analytic; `step` the orbit's: half a period on a sphere, whose twin sits half a period over, a whole one elsewhere) or `chart_u_arm` (spline); the polar channel (gap − (k+½)·τ) × its polar radius | m | OK, and NOT by the escape argument: a mark decision is not an escape claim, and a larger arm decides marks more readily. It is sound because the arm is positive, so a decided sign is the sign of `gap − mark` itself, and the cell between two decided marks holds exactly one orbit point; the arm's size only sets how near a mark a gap may sit and still decide. The decided integer is the joint's true deck element because the true gap lies within the joint bound (`r·\|Δu\| ≤ 2π·ε`, `lift_joint`'s docs) of its orbit point, under the margin `step/2 × lever` to the nearest mark wherever the incidence reads `Off` (the vertex at least `K·ε` from the singular set). Nearer than that the incidence is undecided or `On`; any integer decided there still names a lift of the same point (a deck transformation moves no point), and `chart_boundary`'s fence refuses such a joint before a polygon reads its azimuth |
| pcurves.rs | pcurve_iso_side / pcurve_loop_pole_joint | chart-image point distance; the joint vertex's distance to the chart's singular set (a sphere's nearer pole, a cone's apex), or a spline net's `sup \|S_u\|` | m | OK |
| split.rs:197 | split_edge_param_interior | param spans × per-kind rate (1 / radius / minor / speed bound) | m | OK (metered door; the rate is an `InfSpeed` on every kind — the three closed forms by being exact, the net by `speed_lower_bound`'s derivation — which is what an interiority claim needs) |
| transform.rs:139 | transform_rigid_* (7 residuals) | unit-column/orthogonality/det residuals, no arm | dimensionless | FLAG F10 |
| transform.rs:155 | transform_rigid_trans_finite_* | t·0 poison probe (0 or NaN by construction) | — | OK |
| validate.rs:1662/1847 | planar_face/boundary_residual | plane residuals | m | OK |
| validate.rs:1795 | tangent_second_order | κ_rel × arm²/2 | m | OK |
| validate.rs:2030 | bool_ring_run_winding | (outward · Newell sum) / loop perimeter | m | FIXED (F4) |
| validate.rs:2014 | positive_volume | volume/surface-area (the documented dimensional fix) | m | OK |
| sector_shape.rs (the three rungs) | sector_arm / sector_reflex / sector_straight | arm = shorter bounding chord (m); sin/cos × arm | m | OK — ONE implementation since the S5 sector-predicate unit, and since #652 ONE name set: the former `bool_sector_*` / `split_sector_*` pairs were the same computation on the same quantity, which is why this was already one row |
| splitting/classify.rs:81–286 | split_vertex_side / conic lane | plane residual; rooted amplitude; (rad)×minor semi-axis | m | OK |
| splitting/classify.rs `box_clears` | split_gate_box_side | a padded reach box's distance from the split plane: the centre's plane residual less the box's support `Σ|nᵢ|·hᵢ` (unit normal, half-extents in metres) | m | OK |
| splitting/classify.rs `gate_face_reach` | split_gate_sphere_axis | a sphere's polar axis read as a unit direction (`UnitVec3::new`'s own length decision) | m | OK |
| ray_parity.rs (via `containment.rs`'s `ROWS`) | point_in_loop_segment | a loop segment's own length — the degeneracy gate, through the `Margin::norm3` door | m | OK (split off `point_in_loop_boundary` by #712, which was deciding two questions under one name) |
| ray_parity.rs (via `containment.rs`'s `ROWS`) | point_in_loop boundary/side/advance | distances; m²/m advance | m | OK |
| splitting/containment.rs (`certify_plane`) | point_in_loop_normal / point_in_loop_plane / point_in_loop_query | (\|n\| − 1) levered by the loop's reach from its first vertex; a vertex's, conic centre's or control point's offset `(p − o)·n̂` off the plane, and a conic's or spiric's plane tilt `\|axis × n̂\|` levered by its larger semi-axis (a spiric's by `R + r + \|offset\|`); the query's offset `(q − o)·n̂` | m | OK (CLEAVE) |
| splitting/containment.rs (the frame gate) | point_in_loop_arm | sin(member, plane normal) × loop extent (the member's in-plane fraction) | m | FIXED (was dimensionless schedule norm) |
| ray_parity.rs (via `containment.rs`'s `ARC_LOOP_ROWS`) | point_in_arc_loop_segment/boundary/side/advance | the point_in_loop rows over an arc-bearing loop's STRAIGHT edges (`on_segment` per chord edge, `ray_crossings` masked to chord edges): distances; m²/m advance | m | OK (ATREST-9) |
| splitting/containment.rs (`point_in_loop`) | point_in_arc_loop_arm / point_in_arc_loop_reach | the frame gate, as point_in_loop_arm with the conics' reach in the extent; a ray's distance from an uncrossable edge's ball less its reach, `|w − d·max(w·d, 0)| − reach` (a length) | m | OK (ATREST-9; reach row CONTACT-4) |
| splitting/containment.rs (`ConicArc`, `conic_crossings`) | point_in_arc_loop_conic_span / _window / _disc / _advance | unit-circle quantities levered by the conic's SMALLER semi-axis: (τ − width), read only for a window wound past a period; `arc_trim`'s chordal-defect sum on a ray's crossing (with `_end` its step 1); (1 − h²)/2 — exact lengths for a circle (the last is (r² − h²)/2r, the perpendicular-offset form), a lower bound for an ellipse, where a `Zero` or an in-band margin only abandons the ray; the root's advance t is metres along a unit ray | m | OK (ATREST-9; window ATREST-12) |
| splitting/spiric_arc.rs (`SpiricArc::contact`, through `LoopEdge::contact`) | point_in_arc_loop_spiric_end / _clear / _on / _leaf; bool_contact_spiric_end / _clear / bool_contact_spiric / _leaf | the distance from the point to an end of the arc; `|q − P(v_m)| − S·h`, a piece's ball's clearance from the point; `|q − P(v_m)|`; `S·h`, the ball's own radius | m | OK (CLEAVE) |
| splitting/spiric_arc.rs (`SpiricArc::crossings`) | point_in_arc_loop_spiric_side / _turn / _advance (and point_in_arc_loop_reach) | `(P(v) − q)·n`, a piece end's offset from the ray line; `|s(v_b) − s(v_a)| − 2·A·h²` (A in m/rad², h in rad; `S` and `A` over the piece's own window, `geom::spiric_rate_bounds`); `(c − q)·d ∓ S·h`; the ball's in-plane clearance from the ray less `S·h` | m | OK (CLEAVE) |
| splitting/containment.rs (`LoopEdge::contact`, the boundary pre-pass) | point_in_arc_loop_conic_on / point_in_arc_loop_conic_end / point_in_arc_loop_conic_trim | the same distances as `bool_contact_arc/_end/_trim`, under the carrier walk's own names (the `on` row is a distance, not the signed `ρ − 1`) | m | OK (CONTACT-4) |
| splitting/neighborhood.rs:228–309 | split_conic_departure / split_bisector_side | tangent×extent projections; bisector·n̂ × arm | m | OK |
| splitting/order.rs:73 | split_join_frame_arm | sin(member, plane normal) × points' spread (the member's in-plane fraction) | m | FIXED (was dimensionless schedule norm) |
| splitting/order.rs (`lex_cmp`) | split_join_order_u/v | coordinate difference (m) vs the EXACT bit-level band (deliberate total-order device, documented) | m | OK (note N6) |
| splitting/order.rs (`sort_along`) | split_join_line_order | how far one of a face's crossings lies past another along its section (m) vs the run's band: a real distance apart or one point. A planar face's is the difference of along-line coordinates; a curved face's is the eccentric-anomaly difference times the conic's speed halfway, the arc length to second order | m | OK (CLEAVE) |
| splitting/order.rs (`group_coincident`), splitting/join.rs (`conic_pairs`) | split_join_line_gap | the same gap between successive sorted crossings of one face (m), and across a conic's branch cut, vs the run's band: Zero is one point, Positive two | m | OK (CLEAVE) |
| splitting/join.rs (`line_pairs`) | split_join_face_line | `|n_face × n_plane|` levered by the face's crossings' spread (m): the face's section line exists | m | OK (CLEAVE) |
| splitting/join.rs (`conic_pairs`) | split_join_conic_heading | the sine `(n_plane × n_out)·Ĉ′` between the section's heading into the face and the conic's unit tangent, levered by the wall's `curvature_lever_arm` (the cylinder's or sphere's radius) (m): how far the section runs from tangent to the wall, and so which way along the conic enters the face | m | OK (CLEAVE) |
| splitting/finish.rs (`line_clears_conic`) | split_nest_line_conic | a conic carrier's centre offset from a line less its amplitude across it, `|m·(c − p)| − √((m·a)² + (m·b)²)` (m) | m | OK (CLEAVE) |
| splitting/finish.rs (`conics_clear`) | split_nest_conic_conic | in one conic's unit coordinates, `1 − |c′| − σ` or `|c′| − σ − 1` with `σ` the largest singular value of the other's semi-axis matrix, levered by the first's smaller semi-axis (m) | m | OK (CLEAVE) |
| splitting/rules.rs:132/151/202 | split_sector_extent / coplanar / enters arm | extent; sin×extent | m | OK |
| splitting/rules.rs:179 | tangent_sector_osculation | κ(1/m) × face-extent²/2 | m | FLAG F11 |
| chord_join.rs (`select_arc_by_run_side`) | split_arc_run_end | the difference of a run end's distances to the chord's two ends (m) | m | OK |
| chord_join.rs (`select_arc_by_run_side`) | split_arc_run_side / split_arc_run_along | the cosine of the candidate's departure against the run's left (resp. its travel), levered by the section radius | m | OK |
| chord_join.rs (`run_corner_opens`) | split_arc_run_corner / split_arc_run_cusp | `n̂·(t̂_in × t̂_out)` and `t̂_in·t̂_out`, levered by the section radius | m | OK |
| chord_join.rs (`run_is_section_arc`) | split_arc_run_on_section_plane / split_arc_run_on_section_conic | a run edge's midpoint offset from the section plane (m); its conic residual in unit coordinates, levered by the semi-major axis | m | OK |
| chord_join.rs:710 | split_sphere_section_polar | sin(axes) × sphere radius | m | OK |
| chord_join.rs:1114 | split_tangent_chord_forward | dimensionless param diff × ‖dir‖ | m | OK (metered door; a line's ‖dir‖ is its exact speed and so an `InfSpeed`) |
| chord_join.rs:855 | split_arc_window (×5) | azimuth (rad) × chart radius | m | OK for cylinder; FLAG F8 for the sphere wall (arm R vs local R·cos lat) |
| chord_join.rs:926 | split_arc_chart_orientation | cos × semi-major (= r for the plane×cyl ellipse) | m | OK |
| chord_join.rs:1411 | split_conic_inplane_mid | plane residual at midpoint | m | OK |
| chord_join.rs (`between_edge_is_section`, boolean planar side) | bool_between_line_on_wall | a line's midpoint offset from the wall, `geom_brep::implicit_residual` (cylinder: (ρ² − r²)/2r; sphere: (‖p − c‖² − r²)/2r), the signed distance to first order | m | OK |
| chord_join.rs (`chart_v_du`) | split_chart_azimuth_linear | harmonic azimuth amplitude `\|pa.x\| + \|pb.x\|` (rad), levered at the radius | m | OK (a precondition: the cylinder chart writes the azimuth linear; TANG, PR 3851) |
| chord_join.rs (`chart_island_winding`) | split_ring_closure_ruling | `n·â` (cosine), levered at the radius | m | OK |
| chord_join.rs (`chart_island_winding`) | bool_ring_run_winding (wall chart) | `2·R·A_chart / P`, `P` an upper bound in metres (`R·\|Δu\|` plus axial variation per piece) | m | OK (the planar arm's F4 comparand on the wall's chart) |
| chord_join.rs (`chart_ring_side`) | split_ring_chart_window | `τ − Δu` (rad) levered at the radius | m | OK |
| chord_join.rs (`chart_ring_side`) | split_ring_chart_ray_azimuth | azimuth difference (rad) levered at the radius | m | OK (cylinder only; a sphere refuses before it) |
| chord_join.rs (`chart_ring_side`) | split_ring_chart_ray_height | axial height difference, bare | m | OK |
| chord_join.rs:1468 | bool_between_arc_window | (cosΔ−cos h)·r_c — quadratic in the angular deviation for narrow windows | m | FLAG F8 |
| chord_join.rs:1490 | split_chart_azimuth_frame | radial·u_ref (m) — branch selection | m | OK (note N5) |
| chord_join.rs:1623/1639 | split_sphere_window_pole(_side) | radius − axial distance | m | OK |
| splitting/join.rs:377 | split_section_area | 2·\|A\|/P mean width | m | FIXED (factor-2 doc/code mismatch; dimension was already m) |
| splitting/join.rs:463 | split_section_spur | distance between a spur tip's two neighbours, through the `Margin::norm3` door | m | OK (new) |
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
| ray_parity.rs (via `chart_region.rs`'s `ROWS`) | chart_region_boundary/side/advance | the point_in_loop rows on METRED chart coordinates (plane 1 and cylinder r exactly; every other kind's certified INF arm, `chart_region_arm_inf`): distances; m²/m advance. Since #712 these decide in the SAME shared walk as the 3-D rows, under their own names. The 3-D `point_in_loop_arm` row is derived away — a fixed 2-D schedule member is in-plane by construction, so no projected-length predicate exists | m | OK (new in M9-2) |
| chart_region.rs (M9-2) | chart_region_parallel / collinear_offset | segment-pair 2×2 determinant / offset determinant over one segment's length — the perpendicular height across that segment's line | m | OK (new in M9-2) |
| chart_region.rs (M9-2) | chart_region_cross_span | crossing fraction (dimensionless) × its own segment's length — the crossing point's clearance from a segment endpoint | m | OK (new in M9-2) |
| chart_region.rs (M9-2) | chart_region_collinear_overlap | shared-span length of collinear segments (difference of metre projections) | m | OK (new in M9-2) |
| chart_region.rs (M9-2) | chart_region_cross_order | same-edge crossing-pair advance-fraction difference (dimensionless) × the edge's own length — the crossing points' separation along the boundary (the clip walk's order certificate, union fix U2) | m | OK (new in M9-2 fix pass) |
| chart_region.rs (M9-2) | chart_region_orientation / chart_region_area | signed loop shoelace 2A (m²) / perimeter — the loop's (resp. intersection region's) mean width, through the same `Margin::over_lever` door as `split_section_area` two dimensions up (separate accumulators — the reasons are at `chart_region.rs`'s area margin) | m | OK (new in M9-2) |
| chart_region.rs | chart_region_arm_inf | a chart lever arm — metres per chart unit, gated as a length (the collapsed-arm idiom, `pcurve_interval_meter`'s shape) | m | OK. The arm is a certified LOWER stretch bound (`certified_arms`), which is the side a positive-extent claim needs: an over-stated arm would inflate the metred region and certify a sliver, so the sup bounds (`geom_brep::chart_stretch_sup`) are deliberately not reachable from this lane. Definite-positive walks on; a collapsed arm is `ArmUnbounded`; an in-band arm escalates |
| chart_region.rs (M9-2) | chart_region_seam_span | azimuth-span excess over one period (rad) × the chart's azimuth arm r | m | OK (new in M9-2) |
| chart_bound.rs (TRIM-3) | chart_bound_gap | the separating-axis gap between a metred cell and a boundary edge, five axes under one name: the cell's four side axes take a coordinate difference of METRED chart quantities (`Margin::of`), and the segment's own normal axis takes the corner's signed offset `n·(c−A)` — an area (m²) — over `|n|`, the perpendicular distance the corner stands off the segment's line (`Margin::over_lever`) | m | OK (new in TRIM-3). One name, one population by construction: the five axes ask the same question (metres of clear space between a cell and one edge) and a positive on ANY of them separates, so splitting them would meter one decision as five |
| chart_bound.rs (TRIM-3) | chart_bound_outer_span | the outer loop's own chart `u`-extent minus the chart's period — a chart-unit quantity (radians on an azimuth chart, knot units on a spline chart) through the chart's FIRST-channel lever arm (`Margin::levered`), so the margin is the metre excess by which the face wraps its own chart | m | OK (new in TRIM-3). Refusal-only and one-sided: a DEFINITE positive refuses the description, `Zero`/in-band/poison let it stand, and an inexact arm on a torus moves only where the refusal fires |
| ray_parity.rs (via `chart_bound.rs`'s `ROWS`) | chart_bound_segment | a metred chord's own length — the degeneracy gate, through the `Margin::norm2` door | m | OK (new in TRIM-3) |
| ray_parity.rs (via `chart_bound.rs`'s `ROWS`) | chart_bound_boundary | the cell centre's distance to a closed metred chord — perpendicular at an interior foot, endpoint otherwise | m | OK (new in TRIM-3) |
| ray_parity.rs (via `chart_bound.rs`'s `ROWS`) | chart_bound_side | a polygon vertex's signed offset from the ray line, in metred chart coordinates | m | OK (new in TRIM-3) |
| ray_parity.rs (via `chart_bound.rs`'s `ROWS`) | chart_bound_advance | a straddling chord's crossing advance along the ray: a 2×2 determinant (m²) over the straddle height (m) | m | OK (new in TRIM-3) |
| census.rs (M9-2 PR-2 fix pass) | census_backstop_gap | per-axis gap between two faces' SOUND reach boxes (plane hull ⊕ boundary-arc radius; cylinder axial span ⊕ radius; sphere ball — coordinate differences and radii, metres); only a DEFINITE positive clears the pair | m | OK (new in the union fix; boxes tightened to the face_box construction in the delta) |
| census.rs (M9-2 PR-2 fix pass) | census_backstop_containment | per-axis extent margin between two solids' vertex hulls (coordinate differences — metres); clearance = any definitely negative; anything else sends the pair to the material test, which decides through `point_in_solid`'s own predicates (the invariant the clear rests on is stated at `census.rs` arm 2) | m | OK (new in the union fix) |
| census.rs (CONTACT-7) | census_touch_side | `n·(q − p)`: a star piece vertex's signed distance from a candidate plane through the touch point `p` (`n` unit) — a projection of a metre vector onto a unit direction, through `Margin::of`. Each star face is read through its piece at `p` (the part visible from `p`, star-shaped from it), so the vertices' distances bound every point of the piece | m | OK (levered in CONTACT-1; a vertex distance since CONTACT-7) |
| census.rs (CONTACT-7) | census_touch_dihedral | an edge's convexity: the far face's piece vertices' signed distances from the near face's plane through `p`, in both orders — `Margin::of` | m | OK (levered in CONTACT-1; a vertex distance since CONTACT-7) |
| census.rs (CONTACT-7, `face_piece`) | census_touch_piece_side | building a face's piece at `p`: a point's signed distance from the plane through a line through `p` in the face, normal to the face (`(m × û)·(q − p)`, `û` unit in the face) — through `census::metric::Distance`, `Margin::of` | m | OK (new in CONTACT-7) |
| census.rs (CONTACT-7, `face_piece`) | census_touch_piece_turn / census_touch_piece_front | building a face's piece: a point read along a unit ray from `p` (`û·(q − p)`) — the reference ray for a vertex's turn, a probe ray for an edge's end or crossing — through `census::metric::Distance`, `Margin::of` | m | OK (new in CONTACT-7; split by quantity in the fix pass) |
| census.rs (CONTACT-7, `face_piece`) | census_touch_piece_reach / census_touch_piece_meet | building a face's piece: one crossing point read against another along a unit probe ray, and a meeting point read against an edge's end along the edge's unit direction — through `census::metric::Distance`, `Margin::of`. An undecided choice builds no piece (a refusal, never a rest) | m | OK (new in CONTACT-7) |
| census.rs (CONTACT-7) | census_touch_normal | an On face's outward normal against a candidate plane's, through `geom_brep::classify_material_pairing_as` under this name: the dot of unit normals levered by the folded arm, a sign of magnitude about 1 wherever it is asked | m | OK (new in CONTACT-7; `material_wedge_side`'s construction under its own population) |
| census.rs (CONTACT-7) | census_touch_fold | an edge's two faces' outward normals where neither order of its convexity decides — aligned a flat seam, opposed a fold — through `geom_brep::classify_material_pairing_as` under this name: the dot of unit normals levered by the folded arm, a sign of magnitude about 1 | m | OK (new in CONTACT-7's fix pass; was `material_wedge_side`) |
| census.rs (CONTACT-1, kept) | census_touch_span | whether two unit generators span a candidate plane — the norm of their cross product, levered at the two stars' reach (`Margin::levered`). Candidate generation only: a Zero skips a candidate and decides nothing; the one levered door the touch analysis's source row admits | m | OK (CONTACT-1; its verdict siblings became vertex distances in CONTACT-7) |
| offset_axial.rs (VERBS-RIMCAP) | offset_axial_cap_pair | sine between the two moved meridian caps' cross-section normals × each of the corner's own edge arc lengths — the length a near-parallel pair's solve error would move the corner by (the `offset_axial_corner` idiom on the derived pair) | m | OK (new in VERBS-RIMCAP) |
| offset_axial.rs (VERBS-RIMCAP) | offset_axial_cap_line | the moved caps' meeting line's own distance from the axis (`ρ_L`, a norm of metre coordinates) — `Zero` routes to the pole arm rather than refusing | m | OK (new in VERBS-RIMCAP) |
| offset_axial.rs (VERBS-RIMCAP) | offset_axial_datum_arm | the old corner's distance from its own profile circle's centre (2-D norm in the meridian half-plane, metres) — the carried-datum direction's lever, dead exactly when no direction exists | m | OK (new in VERBS-RIMCAP) |
| offset_axial.rs (VERBS-RIMCAP) | offset_axial_rim_concentric / offset_axial_rim_great | rim-carrier centre to sphere centre distance; carrier radius minus operand sphere radius — both metre data of the stored geometry | m | OK (new in VERBS-RIMCAP) |
| offset_axial.rs (VERBS-RIMCAP, reused by CURVED-SPIRIC's torus rim arm) | offset_axial_rim_plane | sin(carrier axis, moved cap normal) × the body's radial extent — the length the tilt would move a rim point by (the `offset_axial_latitude_tilt` idiom) | m | OK (new in VERBS-RIMCAP) |
| offset_axial.rs (VERBS-RIMCAP) | offset_axial_rim_reach | moved sphere radius minus the moved cap's stand-off `\|t\|` — the length the section circle dies by at tangency, decided before the root `√(r² − t²)` is taken | m | OK (new in VERBS-RIMCAP) |
| offset_axial.rs (CURVED-SPIRIC) | offset_axial_rim_torus_reach | inner-equator radius of the moved torus minus the moved cap's stand-off, `(R − r′) − \|d\|` — the length the two-oval regime closes by, decided before the root `√((R − r′)² − d²)` (the `offset_axial_rim_reach` idiom) | m | OK (new in CURVED-SPIRIC) |
| offset_axial.rs (CURVED-SPIRIC) | offset_axial_rim_side | `m·(q_mid − c)` for the OLD rim's midpoint, `m = axis × cap normal`: a projection of a metre vector, equal to `±ρ(q_mid) ∈ ±[R − r, R + r]` since the midpoint lies in the meridian plane — unlevered | m | OK (new in CURVED-SPIRIC) |
| offset_axial.rs (CURVED-SPIRIC) | offset_axial_rim_sense | `old.axis · cap normal`, a cosine of unit vectors, levered at the body's radial extent | m | OK (new in CURVED-SPIRIC) |
| offset_axial.rs (CURVED-SPIRIC) | offset_axial_rim_meridian | the old rim circle's centre's distance from the tube-centre circle in `(ρ, h)` — the `offset_axial_seam_meridian` comparand on a distinct-charts edge | m | OK (new in CURVED-SPIRIC) |
| offset_axial.rs (CURVED-SPIRIC) | offset_axial_rim_tube | the old rim circle's radius minus the operand tube's minor radius | m | OK (new in CURVED-SPIRIC) |
| offset_axial.rs (`forward_window`) | offset_axial_rim_window | a kind-changed spiric rim's window span `t₁ − t₀` (radians of the minor angle) levered at the moved tube's minor radius — an arc of the tube in metres. Routing: `Positive` keeps the read end, `Negative` takes it a period on, `Zero` refuses | m | OK (new in the equator-seam unit) |
| offset_axial.rs (`reauthor`) | offset_axial_reauthor_plane / offset_axial_reauthor_end | a revolved point's moved start (end) corner's out-of-plane coordinate in its declaration's start (end) sketch plane — a rigid placement's inverse applied to a metre point, so a metre coordinate. Routing only: `Zero` keeps that end's azimuth, a definite side turns the declaration to the corner's own azimuth | m | OK (`_plane` routes since the equator-seam unit, where it had refused; `_end` new there) |
| offset_axial.rs (`reauthor`) | offset_axial_reauthor_azimuth | the same coordinate after the sketch plane has been turned about the axis onto the moved start corner's azimuth — zero to rounding when the plane contains the axis; a definite side refuses | m | OK (new in the equator-seam unit) |
| offset_axial.rs (SHELL-7) | offset_axial_centre | a point's distance from the axis (a norm of metre coordinates) — a surface's centre, origin or apex at classification and a same-surface or two-chart circle's centre at the carrier mint, one helper (`centre_on_axis`) and one name for every site; unlevered, since a length carries its own scale and the extent levers only the sines here | m | OK (one name since SHELL-7; `offset_axial_latitude` and `offset_axial_seam_latitude` folded into it) |

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
the section is empty when the two counts above meet at 248.

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
- **F4** `bool_ring_run_winding` (one predicate, three sites — the
  boolean join's ring lane, the merge's role assigner, tier 3's check
  6 — and one arithmetic home, `crate::loop_winding`, which all three
  read): the Newell AREA is divided by the region's boundary PERIMETER,
  giving `2A/P` — the ring's MEAN WIDTH, the distance the boundary
  would have to move to sweep the enclosed region away, and the same
  quantity `split_section_area` already meters. The canonical
  derivation lives in `crate::loop_winding`'s module docs; the sites
  cross-reference it. The perimeter is arc-aware (conics contribute
  `|Δ|` times the larger semi-axis magnitude — exact for a circle, an
  upper bound for an ellipse, and an over-large P escalates rather
  than decides) and, for the join's open run, includes the chord that
  closes it. This retires an
  EXECUTED in-band refusal: at ε = 1e-6 the mm pocket-subtract twin
  refused typed on a 2e-6 m² margin inside Band{1e-6, 1e-5}; the same
  decisions now carry 5e-4 / 7.5e-4 / 1e-3 m and compute on every ε row
  in the hosted matrix. `rim_dim_boolean_twins`'s three-outcome F4
  signature match is deleted, and the predicate is pinned LINEAR.

Flagged, NOT fixed here (dispositions):

- **F2** `solid_contain.rs` ray-caster denominators — **FIXED** (REACH,
  `work/reach/an-open-sign-row-reds-main-at-1e-6-with-section-loop-mixed`).
  The two skip questions (the plane arm's `d·n̂`, the wall's axis-parallel
  rung) are levered by how far from the query the selection reaches, so
  a ray is skipped only where it drifts less than the band over every
  length at which it could meet the face; the wall's discriminant takes
  the sphere arm's `over_lever` form; the wall's hit-outward sign is read
  off the discriminant's root order and no longer decided. Measured
  before the fix at ε = 1e-6: `sin²/2r` cannot exceed ε on a wall of
  radius 5e5 or more, so every ray skipped the rod's wall and a point on
  its axis read `Out`.
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
  (`chart_stretch_sup` answers (r, r) for spheres and (R+r, r) for tori, and
  `pcurves.rs::chart_u_arm` is the LOCAL lever — r·cos v etc., zero at
  poles/apex, which the walk exploits). The residue closed with F7:
  `param_rate` answers a NURBS carrier's certified speed lower bound,
  so the fitted and iso `pcurve_interval_forward` spans cross to the
  band as arc lengths through the metered door, with the meter itself
  gated as a length (`pcurve_interval_meter`, the collapsed-arm
  idiom); and the fitted lane's azimuth headroom takes
  `chart_arms_at`'s lever, so the cone's arm is `v_sup·sin α` from the
  check's own boxes rather than 1. The last residue — the spline
  charts' `1` at `pcurves.rs::chart_u_arm` and at the two `v_meter`
  fallbacks — is closed too: `geom_brep::chart_stretch_sup` is the
  exported sup-side bound, `topo` meters both channels through it,
  and a spline chart's arms are now the net's own stretch rather
  than a default. The Plane case there was never flagged and did not
  move: a plane chart's u/v ARE metres, so 1 is exactly right, and
  the exported bound answers exactly 1 for it.
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
- **F11** `tangent_sector_osculation` (rules.rs `apply_rule_a`) and
  `wall_bend_order2` (rules.rs `wall_graze`, through
  `geom_brep::bends_into_material`): sagitta model κ·L²/2 metered at
  the WHOLE-FACE extent, squared, and invalid for κ·L ≳ 1. The face
  extent is longer than the contact's own arm, so the margin is
  overstated and a bend decides more readily than it would there — the
  permissive direction, not the over-refusal one. Arm-policy question;
  own unit.
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
- **F15 — RETIRED (the predicate is gone, not levered).**
  (added by the clause-(i) fix pass, from the #213 review's MAJ-1 —
  the review's scale-blindness probe EXECUTED it)
  `editor-core/eval/wire.rs` `revolve_axis_dir_in_plane`: `dir·n̂` with
  BOTH vectors unit was a bare **sine** against the metre band — the
  audit's class-(c) shape, wrapped in `of` by the first migration pass
  with no argument (the sibling `revolve_axis_origin_in_plane`
  comparand `rel·n̂` IS metres and kept `of`). Executed consequence
  (review probe, adopted on merge as this row's pin,
  `geom-core/tests/review_margin_probe.rs`): a tilt of θ = 5e-10
  classified Zero at every model scale while the induced deviation θ·r
  crossed the band between a 1 mm and a 10 m profile.
  
  This row proposed levering the sine at the profile's radial extent,
  kernel-side. What happened instead is that the QUESTION was deleted.
  A revolve's axis is now a `Datum::AxisInPlane` — written in the
  profile's own frame, in that frame's two coordinates — so it cannot
  have an out-of-plane component to classify, and the only thing left
  to decide is whether it is the same frame the profile is drawn on:
  an equality of node ids, with no band and no scale. Both in-plane
  predicates went with it, this one and the metre-valued
  `revolve_axis_origin_in_plane` beside it.
  
  The row stays here, struck through rather than deleted, because it is
  the executed evidence for why the shape mattered — and because
  `flagged_census.rs` reads these headings and would call a citation to
  a vanished row a string. Nothing cites it now: the count above went
  from 8 to 7, and `LEDGER_FLAGGED_SITES` with it.
- **F14** (added by the clause-(i) migration)
  `editor-core/eval/wire.rs` `turns_off`: `|θ| − k·τ` is **radians**
  against the linear band. The revolve's full-circle coincidence check
  (`revolve_full_vs_partial`, at `k = 1`) runs it in the editor before
  the kernel's own metered `revolve_angle`/`revolve_angle_headroom`
  gates (which lever at the profile's radial extent, correctly). The
  honest lever lives kernel-side; duplicating it in the editor is a
  design question, not a same-day fix. Carried as
  `decide_flagged(.., "F14")`. The circular pattern's step reads the
  same helper (`pattern_step_full_turn`: whether the step reaches a
  turn, and how many whole turns it holds), and one more site of the
  same comparand, `editor-core/eval/wire/stepped.rs`
  `SteppedOperands::circular`'s `pattern_step` (the step, radians,
  against the band: a zero step lands every copy on the master,
  carried as `decide_flagged_reported(.., "F14")` because a driven
  step's refusal quotes it). The honest lever is the master's radial
  extent about the axis, which the mate solve's derived offset (the
  same constructor) does not have in hand.
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

- **F16** (added by M10-2, the measurement vocabulary)
  `editor-core/src/measure.rs` `assert_bound`: an assertion's
  comparison is `measured − bound` (negated for `AtMost`) in the
  MEASURE's own dimension, classified against the linear band. For a
  `Length` measure that comparand is honest metres and the site would
  take `of`; for an `Angle` measure it is **radians** against a metre
  band — the audit's class-(c) shape, and it is the same site, so the
  site is flagged rather than split into an honest lane and a
  dishonest one under one predicate name.
  The obvious repair is foreclosed BY RATIFIED DESIGN, which is why
  this row is a disposition rather than a to-do: ERROR-DESIGN E3
  rejects lever-arm unification of angle with distance by name —
  "it requires a chosen length scale — an ad-hoc constant, exactly the
  class this project refuses". An angular assertion's honest arm is
  therefore a design question (what extent is an angular tolerance
  consumed over?) and not something one unit may invent. Two things
  bound the exposure meanwhile: the comparison is REPORT-ONLY (E10 v1
  — no gate, no geometry, no downstream outcome reads it), and its
  band arm is `Unevaluated`, which reports the indeterminacy rather
  than picking a side. Carried as `decide_flagged(.., "F16")`.

- **F17** (added by M10-5, the clearance engine)
  `editor-core/src/clearance.rs` `clearance_margin` and
  `self_intersection_gap`: the two compares the E7 inner subdivision
  makes. Both comparands are `d − c`, the interval enclosure of the
  Euclidean separation between two geometry-domain cells minus the
  requested clearance, in METRES against the linear band; the second is
  the same shape at `c = 0`. Honest metres on both sides, so **neither
  site is flagged** and neither takes `decide_flagged` — this row is
  the disposition, and `LEDGER_FLAGGED_SITES` does not move. It is
  written here rather than left implicit because the two sites are new
  names in the K roster and a reader asking "what dimension is a
  clearance margin" should find the answer in the ledger.

  The two names exist rather than one because they read a definite
  `Sign::Zero` differently, and each reading is a claim about a
  different question: under `clearance_margin` the bound is non-strict
  (`min-clearance ≥ c`, the assertion lane's convention) so a
  separation equal to `c` at the run's tolerance DISCHARGES, while
  under `self_intersection_gap` E7 asks for a strictly positive
  distance between non-adjacent faces, so a coincidence at the run's
  tolerance is the violation the check exists to find. One name with
  two readings would put two populations under one K row.

- **F18** (added by M10-8's fix pass, found by scoping the recorder's
  name) `editor-core/src/expr.rs` `refuse_non_finite`: the ruled
  door-2 finiteness check on a FINAL evaluated value, `value · 0`
  against the band `(1e-100, 1e-50)` — exactly zero for every finite
  value, poison otherwise. The comparand carries `value`'s dimension,
  whatever the expression's was (a length, an angle, a count, a ratio),
  so no `Margin` door fits, and the check is not a geometric margin at
  all: it is the evaluator's refusal of a non-finite result. **Carried
  as `k_stats::check_unlogged(.., "F18")`** — the recorder's named
  evaluator door, which classifies through the one funnel body and
  stays OUT of the verdict log (the check fires once per expression
  evaluation, a count the witness and a leaf do not share; logged, it
  refused every M10-6 min-clearance box on a vector mismatch). It is
  therefore not a `decide_flagged` site and `LEDGER_FLAGGED_SITES`
  does not move. The finding that put it here:
  this site called `sign_within` directly, outside any named
  `classify`, and every K sample it recorded was charged to whichever
  predicate had classified LAST — 1,054 samples in the corpus sweep at
  ε = 1e-6 (`m4_pr8_k_probe`'s `<unnamed>` guard went red the moment
  the name was scoped). Every one of those samples is a `Definite(Zero)`
  at margin 0 and never a rule sample, so no K claim moves; what moves
  is the per-predicate attribution of 1,054 rows, now under their own
  name.

- **F19** (added by TRIM-3, the chart-boundary outside test)
  `topo/src/chart_bound.rs` `certifies_outside` and `assembled`: the
  six rows above.
  Every comparand is metres by construction, because the value they
  decide on is a `MetredBound` — the description scaled by the chart's
  arms (plane `(1, 1)`, cylinder `(r, 1)`, both exact) BEFORE any
  predicate runs, so no chart-space quantity ever reaches the funnel.
  Honest metres throughout, so **no site is flagged** and
  `LEDGER_FLAGGED_SITES` does not move; this row is the disposition,
  written here because the five names are new in the K roster and a
  reader asking "what dimension is a chart-boundary cell margin"
  should find the answer in the ledger.

  **The claim has a guard, which is what a construction-based claim
  owes.** "Metres by construction" rested on `metred` running before
  any predicate and on `Margin::over_lever(n·(c−A), |n|)` dividing the
  segment-normal area by the normal's length. The second half is the
  fragile one: replacing that door with `Margin::of` decides a signed
  AREA against the linear band and changes no verdict on any short
  edge, so nothing stood against it. `t10_a_long_edge_guards_the_dimensional_claim`
  is what does — a 10³ m edge and a gap of half the
  coincidence threshold, where the honest margin is `ε/2` and the area
  is `500·ε`, so the two doors disagree about dropping a cell that
  meets the boundary.

  Two readings of a definite `Sign::Zero` are deliberately the SAME
  here, which is why the six names are not six questions but two.
  `chart_bound_gap` and the four parity rows all read `Zero` as *not
  certified* — the cell is kept — and that is the invariant
  `certifies_outside` states: a drop needs a definite sign at `≥ K·ε`,
  so `Zero`, an in-band margin and poison are one outcome. There is
  therefore no strict/non-strict split of the F17 kind to name, and
  the names exist only to keep their populations (a cell-to-edge gap,
  a chord length, a point-to-chord distance, a side offset, a crossing
  advance, and — asked once per description rather than once per cell
  — an outer loop's period excess) from pooling.

  **The `pcurves.rs` branch row takes a second call site under this
  unit.** `chart_boundary` re-decides its loops' closing deck element
  under the walk's own `pcurve_loop_branch` name and requires the
  identity (winding 0), because the description needs the closure the
  walk's `±1` winding deliberately admits to be excluded. Same quantity,
  same arm, same dimension — the row two tables up covers it, and the
  population grows rather than splitting.

- **F20** (added by PCERT's incidence-and-fidelity unit, PR 3812)
  `geom-brep/src/pcurve_cache.rs` `schedule_residuals` under
  `Record::CrossCheck`: check 3's samples on a `Harmonic` row,
  `|S(P(tᵢ)) − C(tᵢ)|` in metres. The comparand is a length, so the
  `Margin::of` door fits it dimensionally; what keeps it off the logged
  doors is WHERE it runs. Since C4 (Ev, PR 3781) the envelope is the
  whole certified statement on a harmonic row, and the schedule is its
  cross-check, run on the witness lane (`f64`, `Sym<f64>`) and not at an
  exact-witness scalar, whether a point or a box. The
  driver replays a leaf at the point witness and at the box scalar and
  compares the two verdict vectors row for row, so a logged
  cross-check would put rows in the witness's vector that the leaf's
  cannot have (measured: every probe refused). **Carried as
  `k_stats::check_unlogged(.., "F20")`**, the same door as F18; a
  sample over the band still refuses the certificate, and the witness
  build with it. Not a `decide_flagged` site; `LEDGER_FLAGGED_SITES`
  does not move.

**Every `props/curved.rs` row above is cited BY TARGET NAME, not by
line** (S176(a)). The line numbers they carried were written against a
2026-08 tree and had already rotted at #877's merge base; #877 moved
200+ lines of the file and would have rotted the rest. A row whose
citation cannot be resolved is a row nobody re-derives.

**The audited POPULATIONS move under #877** — recorded here because
this document's rows are about what is recorded, and all audited probe
suites are green:

- **sphere `props_rim_level_group`**: two decides per comparison became
  **one**. The pair was then `(sin v, 0)` (it has carried `cos v` since
  the issue-893 fix — N7's retirement), so the old second component was
  `0 − 0` and always decided `Zero`; the chord folds it away. Same
  verdict, one fewer sample.
- **torus `props_rim_level_group`**: the recorded margin changes from a
  per-component value at `major` to the chord at `minor` (N1's
  retirement). Different number, and it is the exact one.
- **`props_rim_side`**: now records on faces the gate previously
  refused before reaching it — `boundary_material_sign`'s three
  linearly-leveled arms run the premise first, so the population loses
  the non-rectangular faces it used to include and the K stream stops
  carrying sides that were a property of loop-flattening order. It also
  records at the SHAPE door now, once per rim of every rim-bearing
  linearly-leveled face `require_iso_rectangle` is asked about
  (`linear_rims_at_extremes`, which takes the sense-free unanimity
  residue): a population the door contributed nothing to before, on the
  same comparand and lever as the two lanes above.

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

- **N2** `props_rim_dir_group` compared a structural ±1 through the
  numeric funnel (margin 0 or ±2·arm), guarded upstream by
  `props_circle_axis_class`. **RETIRED**: the traversal direction has
  one representation now (`Rim::d_u_sign`, a discrete `Sign`) and
  `du_of_rims` compares it as a sign, so no margin is formed and the
  name reaches no funnel site. What the note guarded against is the
  live shape at `props_rim_interior_side`, one row up, and it is
  guarded differently: σ there is a product of two DISCRETE signs and
  never becomes a comparand — the margin it points is
  `props_rim_side`'s own `lo + hi − 2v`, at `props_rim_side`'s lever,
  so the structural sign steers a length rather than being banded as
  one.
- **N9** `props_rim_only_extent` and `props_face_extent` are ONE
  comparand under two names on the sphere —
  `sphere_extent_margin(lo, hi, R)`, from one helper both call — asked
  at two moments about two level lists: before a pole is folded in and
  after. A meridian-free rim-bearing sphere face therefore records both,
  which is intended and is what tells "the levels alone carry no
  extent" apart from "the face has none". Two names over one comparand
  is the thing the head matter warns about in the other direction (one
  name over two comparands); it is safe here only because the helper is
  shared, and a second spelling of either margin is the defect to watch
  for.
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
  fires first. The fallback lever (the folded arm, where the
  second-order margin refused and `1/κ_rel` does not exist) reads only
  on a path that already refuses.
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
- **N7 — RETIRED (issue 893 / S-CERT).** As filed (review MINOR-3):
  the sphere's `Unit(sin v, 0) × R` grouping margin metered the AXIAL
  separation `R·|Δsin v|`, which degenerates toward the poles
  (∝ cos v̄ → 0): two distinct near-polar latitude rims grouped as
  coincident although their true point separation is ~`R·Δv` —
  dimensionally honest, but the lever understated the 3-D deviation,
  the same lever-magnitude family as N1 with the opposite direction:
  N1 escalated, this merged, and it merged in the ACCEPTING direction
  (a non-rectangular near-polar domain PASSED `props_rim_level`).
  **The resolution is the torus's own representation**: sphere rims
  now mint the full `Unit(sin v, cos v)` pair (`sphere_boundary`'s rim
  arm reads `cos v = r_c/R` off stored data; the scalar-extreme lift
  completes a sine with its nonnegative cosine), so the chord
  `level_coincides` meters IS the point deviation everywhere on the
  sphere, exactly as on the torus. S81's one-rule/one-arm unification
  is what made this a one-site change. The near-polar refusal row is
  `geom-brep/tests/cert1_sphere_polar.rs`
  (`two_distinct_near_polar_rims_are_not_one_level`, with its
  within-band accepting control beside it); the chord pin is
  `rim_dim_scale_twins.rs`'s sphere twin. **The retirement is scoped
  to the chord** (`level_coincides`, both recording names): the
  sphere's other polar-shrinking margins are note N8's, still open.

- **N8 — open, refusing direction only (recorded at the issue-893
  close-out; outside that issue's accepting-direction ask).** Two
  sphere margins still meter AXIAL sine differences that shrink by
  `cos v̄` toward the poles: `props_rim_side`'s side margin
  (`lo + hi − 2·sin v`, the `Unit` primary component × R) and
  `props_face_extent`'s sphere margin (`(hi − lo) × R`). Neither can
  ACCEPT wrongly — an understated side margin escalates or refuses a
  genuine side, and an understated extent refuses a genuinely thin
  near-polar band as degenerate (a `DegenerateFace` where a serving
  lane could measure) — so both sit in the refusing direction, the
  direction a precondition may err. Retiring them means giving the
  side test a direction-pair comparand and the extent a geodesic
  `Δv`-scaled one; both are lever changes at one site each, N1/N7's
  family. No arithmetic moved here.
