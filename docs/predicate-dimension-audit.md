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
hole in it. **(2) Not complete inside them.** At `43e2998d`, of
**255** funnel-reaching names in the two crates the tables and their
prose reached **230**, carried an individual `dim` verdict for **128**,
and missed **25** entirely, the 25 §D's **D46**. Those figures are
dated, and the instrument that took them read calls line by line. At
`9b882992`, read across lines, **475** names are written as a literal
at a funnel call and **188** of them are recorded nowhere (*Uncovered
names*, below the tables). The word *every* was this document's for a year and it was
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
rigidity residuals — F13 ×1, F14 ×1, F16 ×1, F22 ×1 — 5 shipped sites, tracked as issue #214 and pinned by
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
`of` argument — seven shipped sites do (each dimensionally verified):
`props/curved.rs` sphere `props_rim_fit` (inline root), cone
`props_rim_fit` (inline lever `|v|·sinα`), cylinder and cone
`props_meridian_on_surface` (inline norms, and the radial sag's root
and quotient), `fillet/battery.rs`
`fillet3_radius_headroom` (inline quotient `r²/arm`) and
`fillet3_spine_regularity` (inline lever `r²·κ`), and
`pcurve_cache.rs`'s sphere `polar_rate` fallback (inline `·aa/radius`).
Structural debt to keep named, not a defect: full door-composition
would need Margin arithmetic, which the convention refuses. The
recorded margin stream is bit-identical by construction (each door
performs exactly the operation the bare site performed); the
probe-census diff row is the executed proof. F12 stays OUTSIDE the
seam by its unchanged disposition (below).

**The invariant lane (retired).** A consistency backstop between
integral results once decided on bare `T` through
`k_stats::decide_invariant` (Ev's #213 layering ruling). Its one member,
the boolean's volume backstop, left the operation path when Ev made its
relations a test oracle (`[ev]` PR 5780, PR 5842;
`topo::test_support::VolumeOracle`), and `[ev]` PR 6161 retired the
lane's DESIGN sentence; `decide_invariant` is deleted. Its three
predicates (`volume_backstop`, `volume_backstop_operand`,
`volume_backstop_violation`) are no longer decided. The former
`per_boundary` door is renamed `over_lever` and re-scoped to the
genuine geometric decisions (mean-width 2A/P, mean-thickness V/A
containment, chart-orientation areas, the crossing advance).

Factor conventions used throughout (verified against definitions):
`Curve3::Line.dir` unit ⇒ line parameter is arc length (m);
`Circle`/`Ellipse` parameters are radians with radii/semi-axes in m;
all stored surface axes/normals/`u_ref` unit; `implicit_residual` is
`/2r`-normalized to meters; `implicit_gradient` unit on-locus;
`curvature_lever_arm` meters; `TangentJet.kappa_rel` 1/m;
`speed_lower_bound()` meters per parameter unit.

## Coverage of the two crates, measured

**Against main at `43e2998d`, and dated** — carried explicitly, because
this is a survey written into the tree it surveys (§D's D23). Every
figure in this section is that tree's, taken with the line-shaped grep
below; the re-measurement at `9b882992` is in *Uncovered names*, and
the figures here have not been re-derived on its reading. *(Taken three
times: `f87b203`, then `a0a6e1a5` after main moved `census.rs` and
`splitting/rules.rs` under it, then here after `topo` gained `live.rs`
and rewrote `split.rs`. Every figure reproduced at all three.)*

**What is in scope.** K-REPORT's restated rule: a name is in scope if
it reaches the `geom_core::k_stats` funnel *however it is spelled at
the call site*. In these two crates that is nine spellings — `decide`,
`decide_flagged`, and the `check_residual` /
`classify` / `classify_len` / `require_zero` / `require_extent` /
`gap_is_zero` / `signed_is_zero` wrappers.

**Names — the deliverable.** **255** distinct predicate names: **217**
written as a literal at one of those spellings, and **38 carried** by a
module-private `const`, a struct field or a local table
(`sector_shape.rs`'s three consts, `ray_walk::ParityRows` twice over,
`transform.rs`'s two arrays, `carrier_eq.rs`'s margin tuples,
`contact_verify.rs`'s and `splitting/order.rs`'s loop tuples,
`pcurve_cache.rs`'s winding closure, `pcurves.rs`'s two closures).

| the document's relation to a name | count |
|---|---|
| carries an individual row with a `dim` verdict | **128** |
| named only in prose, no `dim` column | 3 |
| reached only through a family cell or slash-list | 99 |
| **reached, on the most generous reading** | **230** |
| **recorded nowhere, under any reading** | **25** |

**Reach is not verdict, and the asymmetry runs one way.** The 99 family
matches are a judgement: glosses like *"pm_census vv/ve/vf/ef gaps,
spans, residuals"* and *"sphere/torus meridian checks"* were read as
covering every name they plausibly reach, which is the reading most
favourable to this document. So **230 is a ceiling on REACH**, **25 a
floor on the hole** — a stricter reader moves names out of the 99 and
into the 25, and nothing can move one out of the 25, because those
names appear nowhere above this section in any form. And **reach is not
a dimensional verdict**: the number of names this document has actually
dimensioned, one row and one `dim` cell each, is **128**. The other 102
are covered by a family gloss or a sentence, which is a claim about a
family and not a check on a comparand.

**Sites are NOT the deliverable, and this is why.** Counting them is
still useful as a cross-check, so the ledger is below — but D19's
lesson (*"a count of SITES was never the right measure of a NAME
roster"*) has a second edge here: **about thirty helpers in these two
crates fix a name internally and have more than one caller** —
`require_extent`, `require_rim_incidence`, `du_of_rims`,
`classify_dihedral`, `enters_material`, `point_in_loop`,
`tangent_locus_relation` and the rest — so *"where a
decision is posed"* and *"where the funnel is called"* differ by a wide,
convention-dependent margin. The roster is unharmed (every one of those
names is a literal at the wrapper's own funnel call, so all are in the
210), which is exactly the point: the name count is stable under the
convention and the site count is not.

| | |
|---|---|
| raw matches of the ten spellings | 327 |
| − prose inside doc comments (`boolean/ops.rs:3`, `chord_join.rs:188`, `sector_shape.rs:412`, `:456`) | 4 |
| − a `format!("decide({rung}"` string inside a test (`chord_join.rs:2481`) | 1 |
| − two calls to an unrelated local `classify` closure (`splitting/join.rs:278`) | 2 |
| **= funnel call sites** | **320** |
| − forwarding hops (first argument is a name *parameter* of the enclosing fn or closure, so the name is chosen by its caller) | 13 |
| **= sites at which a name is fixed** | **307** |

**Both halves of the measurement have a blind spot, and neither is a
roster alone** (K-REPORT's framing; it reproduces here). A code scan
misses names not written at a funnel site — the 38 carried ones, 15% of
the roster. A corpus column misses names the corpus never exercises —
**83** of the 248 measured then do not appear in the committed M7 baseline at all
(`props_rim_interior_side`, since retired with the sphere's iso flux
arm, `props_rim_only_extent` and `props_rim_only_closed` are new here
and post-date it;
`props_rim_dir_group`, which it does carry, is retired),
and that baseline in turn still carries six spellings the tree has
retired (`bool_sector_*` / `split_sector_*`, unified to `sector_*` by
#652). Re-deriving:

```sh
# code half — every funnel site, all nineteen spellings, both crates.
# It also matches doc-comment prose and `fn` definitions; the ledger
# above says which, and they are subtractions, not sites.
# `decide_positive` / `decide_negative` / `decide_nonzero` / `decide_magnitude` are `decide`
# with the caller's sign requirement folded in, and `decide_reported` is
# `decide` keeping the verdict for the refusal's words, so their sites
# are rows of this table like any other; `gate_measured` is deliberately
# absent — it classifies no margin and has no comparand to dimension.
# The `_reported` doors and `check_unlogged` (the F18–F25 door, the
# one-margin case of `unlogged`, F28's) are the same decisions keeping
# their verdict or leaving the log.
grep -rnE '\b(decide|decide_reported|decide_flagged|decide_flagged_reported|decide_positive|decide_negative|decide_nonzero|decide_nonzero_reported|decide_magnitude|decide_magnitude_reported|check_unlogged|check_residual|classify|classify_len|require_zero|require_extent|gap_is_zero|signed_is_zero)\s*(::<[^()]*>)?\s*\(' \
  crates/geom-brep/src crates/topo/src
# behavioural half — what the committed baseline emitted
zcat docs/k-report-data/m7-eps-1e-9.csv.gz | tail -n +2 | cut -d, -f2 | sort -u
# coverage — compare against THIS FILE WITH ITS TWO SELF-DESCRIBING
# SECTIONS REMOVED. `Uncovered names` lists the uncovered and this section
# names helpers that share spellings with predicates, so a re-derivation
# that reads the whole file finds a hole of zero and calls it closed.
sed -e '/^## Coverage of the two crates/,/^## geom-brep/d' \
    -e '/^### Uncovered names/,/^## Findings/d' \
    docs/predicate-dimension-audit.md
```

**The `grep` is line-shaped, and the tree is not.** rustfmt puts a
long call's first argument on the line after the `(`, and at
`9b882992` a literal-first-argument reading of the line-shaped grep
sees 283 names where a scan that lets the argument wrap sees 475. The
grep also reads every `#[cfg(test)]` module and every test-only file.
A re-derivation reads calls across lines, cuts each `#[cfg(test)] mod
… { … }` by brace matching (an attribute may sit between the two),
and drops each file a `#[cfg(test)] mod x;` declares; the figures
dated `9b882992` in this document were taken that way.

The first command is a **starting set, not an answer**: a site whose
first argument is not a literal has to be read, and a spelling not in
the alternation is invisible to it — `require_extent` was, until this
measurement, absent from the alternation while being named in this
file's own first paragraph. That residue is this table's standing cost,
disclosed rather than discovered.

**These names carry the K vocabulary and never reach the funnel**, so
they are correctly outside the 255 and a reader who greps for one
should know why. They live only in an `Indeterminate.predicate`:
- eight through `predicate: Some("…")`: `carrier_unglued_coincidence`,
  `contact_rest_ladder_invariant`, `contact_rest_senses_opposed`,
  `contact_tangent_independent`, `merge_pair_extent`,
  `seam_cover_unread`, `seam_traversal_unread`, `tangency_sides_mixed`;
- four through a helper that fills that field for its caller:
  `carrier_kind` (`carrier_eq::definite`),
  `continuation_senses_aligned` (`boolean::sense_contradiction`, which
  names `contact_rest_senses_opposed` too), `tangency_conformal`
  (`verify_tangency`'s `label`), and
  `bool_join_torus_section_touching_under_a_germ`
  (`invalid_margin::invalid`, through a `const`);
- the name argument of each shipped `k_stats::straddle` call, two
  decided bounds on one quantity straddling the band (the call sites
  are the list: `grep -rn 'straddle(' crates/ --include=*.rs`, outside
  `#[cfg(test)]`, whose `lane_straddle`, `finish_straddle` and
  `rh_straddle` are fixtures):
  - an ellipse's lower and upper bound on one distance, or on its span's
    overlap, under each caller's conic rows: `bool_contact_arc_straddle`,
    `point_in_arc_loop_conic_straddle`, `pm_census_ef_cross_arc_straddle`,
    `bool_ruled_region_arc_straddle`, and
    `bool_sphere_region_arc_straddle` and
    `bool_torus_region_arc_straddle` (each a circle edge's rows, whose
    two bounds are one, so neither is raised);
  - `pc_axis_plane_parallel_straddle`, a plane×cylinder section on the
    declared-tangency path read in the rulings' lane over a wall face's
    reach measured over each edge's span and in the conic lane over it
    measured round each edge's whole turn (`chord_join::agreed_section`);
  - `pm_census_ee_span_straddle` and `pm_census_ee_gap_straddle`, the
    lower and upper bound of a reach or of the lines' gap that the
    census's crossing lane reads off the closed form's rounding
    (`crossing_reading`);
  - `pm_census_span_rounding`, the lower and upper bound of a charged
    span comparison (`pm_census_span_order_charged`,
    `pm_census_ee_overlap_charged`) read apart (`charged_cmp`);
  - `pm_census_ef_pierce_lever_straddle`,
    `pm_census_ef_pierce_straddle`, `pm_census_ef_pierce_span_straddle`
    and `pm_census_ef_pierce_height_straddle`, the lower and upper
    bound of the lever the census's edge–face pierce is placed by, of
    the pierce's distance from the face's boundary where its
    placement's two ends read the face apart, of the placement's
    half-span, and of its reach to a boundary point under it
    (`pierce_span`, `pierce_inside`, `span_clear`);
  - `curve_carrier_straddle`, `geom`'s on-carrier reading of a point
    whose lower bound on the distance from a conic and the gap at its
    foot straddle (`curves::on_carrier`);
  - `curve_pair_span_end_straddle`, a span end's side read Zero against
    the point's chord to that end, and `curve_pair_span_past_straddle`,
    a kept piece's clearance read near against that chord
    (`curve_pair::span`);
  - `curve_pair_meet_stretch`, two meetings of one stretch read apart
    with a point between them read on (`curve_pair::meet`);
  - and a blend's battery straddle, which carries its decision's own
    funnel row, so it adds no name here;
- four as the name argument of a `k_stats::gate_measured` call:
  `plane_nurbs_transversality_reported` and
  `nurbs_pair_transversality_reported` (`edge_nurbs`), and
  `shell_walls_extent` and `shell_moved_walls_extent` (`shell`).

The two `k_stats` doors record the escalation on the open frame but
classify nothing, so their names are outside this table for the same
reason the twelve `predicate` names are.
The list is `predicate: Some("…")`, `with_predicate("…")`,
`straddle("…"` and `gate_measured("…"` over every crate's production
code, minus every name a funnel call takes; that scan cannot see a name
passed through a variable, a `const` or a row struct's field, so each
such site (a helper whose `predicate` is a parameter, a `straddle` whose
row is not a literal) is read to its literals by hand.
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

**Why no gate on 128 / 230 / 255 / 307 — the third answer to Q6.**
Not "it is guarded" and not "dating it is enough": a gate would have to
fix the family-matching convention in code, and that convention is the
judgement this section is careful to expose rather than freeze. A green
gate would assert a reading, not a fact. What *is* mechanical is
already elsewhere — `geom-core/tests/flagged_census.rs` pins the
`decide_flagged` count, and the K sweep recomputes the emitted-name set
on every merge. The 25 are scheduled as their own unit (§D's **D46**),
which is what actually moves the number.

## geom-brep

| site | predicate | comparand | dim | status |
|---|---|---|---|---|
| geom/src/curves.rs `Curve3::spiric` (CURVED-SPIRIC; the kind's deciding door, registered here because `topo`'s rim mint decides through it) | spiric_minor_positive | the minor radius | m | OK (new in CURVED-SPIRIC) |
| geom/src/curves.rs `Curve3::spiric` (CURVED-SPIRIC) | ring_torus_convention (`geom::ring_torus`, the convention's one home) | `major_radius − minor_radius` — the ring convention's length | m | OK (new in CURVED-SPIRIC) |
| geom/src/curves.rs `Curve3::spiric` (SHELL axial-closed) | spiric_off_axis | `offset`, the plane's signed stand-off from the axis — a length; `Zero` refuses, the oval being a meridian `Circle` (C1, most exact kind first) | m | OK (new in SHELL axial-closed) |
| geom/src/curves.rs `Curve3::spiric` (CURVED-SPIRIC) | spiric_two_ovals | `(major_radius − minor_radius) − \|offset\|` — the length the two-oval regime closes by, decided before any root (the `offset_axial_rim_torus_reach` comparand, re-decided at the kind's own door) | m | OK (new in CURVED-SPIRIC) |
| geom-brep/src/intersect.rs `plane_torus_section` (SHELL axial-closed) | pt_spiric_two_ovals / pt_spiric_two_ovals_floor | `(R − r) − \|d\|` for the axis-parallel plane's stand-off `d` — the length the two-oval regime closes by, decided before the spiric is minted (`spiric_two_ovals`' comparand at the section's own arm) — read with `pt_axis_in_plane`'s sin×extent beside it (`decide_across`) | m | OK (new in SHELL axial-closed) |
| geom/src/curves.rs `Curve3::spiric` (CURVED-SPIRIC) | spiric_frame_orthogonal | `axis · u_ref`, a cosine of unit vectors, levered at `major_radius + minor_radius` (the farthest point the frame places) | m | OK (new in CURVED-SPIRIC) |
| dihedral.rs:140 | dihedral_arm | min(curvature arms, extent) | m | OK |
| dihedral.rs:151 | dihedral_wedge | sinθ (unit-gradient cross) × arm | m | OK |
| enters.rs:84 | enters_material_arm | caller arm (contract: m) | m | OK |
| enters.rs:95 | enters_material | cos(unit,unit) × arm | m | OK |
| enters.rs:141 | tangent_sector_order2_arm | caller arm | m | OK |
| enters.rs:153 | tangent_sector_order2 | normal curvature (1/m) × arm²/2 | m | OK |
| enters.rs `bends_into_material` | wall_bend_order2_arm | caller arm | m | OK |
| enters.rs `bends_into_material` | wall_bend_order2 | mean normal curvature (1/m), folded to the material side, × arm²/2; its one caller (`splitting/rules.rs` `apply_rule_a`) passes the face extent | m | FLAG F11 |
| newell.rs:165 | newell_plane_residual | (p−centroid)·n̂ | m | OK |
| geom-core/src/period.rs (`Period::turn`, **the turn rule**; asked by every row that cites it, each under its own `TurnRows`) | the caller's `TurnRows::turn` / `TurnRows::straddle` | the headroom `(period − extent)` crossed at the rate: through `Margin::metered` at an `Exact` rate and at a `Bounded` rate's inf, through `Margin::metered_sup` at its sup — at a `Bounded` site `TurnRows::turn` carries both readings, two bounds on one quantity; a straddle carries the hull of the two readings | m | OK. Each question has one home — `Period::not_over` (with the one admission of an undecided reading, `TurnUndecided::not_over_at_sup`: the sup reads the headroom definitely positive), `Period::strictly_partial`, `Period::whole_turn` — and the rows citing this one ask through them. The extent is a parameter span (an `Extent`, never reversed), so it crosses through the rate pair (`[tolerance.margins]` (iii)), never `Margin::levered`. `Over` and `Short` read the inf, the "definitely apart" side; `Whole` reads the sup, which certifies smallness; a pair whose readings disagree escalates (`k_stats::straddle`, on the straddle row). At an `Exact` rate the two reads are one |
| certify.rs:849/858 | interval_span_forward/winding (Circle) | span × radius / (τ−span) × radius, through `Margin::metered`; the winding half is the turn rule (row above) at `Exact(radius)` | m | OK. The radius IS the carrier's own parameter rate (`|dP/dθ| = r` exactly), the same number `pcurve_cache::param_rate` mints as an `InfSpeed` for a circle, so this crossing goes through the metric door like the Nurbs arm two rows down. Exact, so it is the inf and the sup at once: the forward claim is *definitely apart* (the inf side), and the winding half admits `Whole`, a smallness claim (the sup side), which an exact rate certifies too |
| certify.rs:872/877 | interval_span_forward/winding / interval_span_winding_straddle (Ellipse, and Spiric at its minor radius) | span × min(\|major\|, \|minor\|), through `Margin::metered`; the winding half is the turn rule (two rows up) at `Bounded(geom::carrier_speed::turn_defect_speed_lo, geom::carrier_speed::turn_defect_speed_hi)` — the inf and the sup over the gap or overlap itself, which `ConicArc` reads too: on an ellipse the speed at its middle less or plus the larger semi-axis times half its width, the inf floored at the smaller semi-axis; on a spiric the minor radius below and `spiric_rate_bounds` over the defect above — whose straddle row is `interval_span_winding_straddle` | m | OK. `|dP/dθ| ≥ min(|a|, |b|)` (the semi-axes carry no order and no sign; `minor` in the ordinary order), so the smaller semi-axis magnitude is a certified LOWER bound on the ellipse's own parameter rate — an `InfSpeed`, the same mint `param_rate` and `splitting::classify` make for the same kind. Conservative in the direction a forward claim needs |
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
| certify.rs (`gap_orientation`, the tangent arm) | tangent_gap_orientation | n̂₁·n̂₂ (unit implicit gradients) levered by the faces' `reach`; a decided zero refuses (`decide_nonzero`), renamed by a definite `tangent_normal_parallel` at the folded arm | m | OK |
| certify.rs (the tangent arm) | tangent_gap / tangent_gap_hull_sup | `r₁ − σ·r₂`, two implicit residuals (/2r-normalized) under the decided orientation σ = ±1; its sampled max over both ends' σ per span plus `TangentSpanBounds::gap_sag`, `(Δt²/8)(sup\|f₁″\| + sup\|f₂″\|)` | m | OK |
| certify.rs:1143/1151/1162 | witness_* | residuals / point distance | m | OK |
| intersect.rs:949/953/892 (`plane_cylinder_ruled`, `plane_cylinder_section`) | pc_axis_plane_parallel / pc_parallel_gap / pc_parallel_gap_floor / pc_rim_alignment | sin×lever, the lever over the reach from the rulings' hinge: the consumed region's axial distance from the hinge's station (`Reach::hinge_lever`) plus its distance across the wall from the hinge at second order, `abs(sin)/(1 + cos)` × a length (`Reach::turn_lever`: the face's reach from `at`, a conic edge read over the span it holds, `Reach::span_reach_from` (a circle arc exactly, an ellipse arc by its quarters), and `at`'s offset across the wall from the normal's own off-axis part; the germ frame and chord_join's cylinder lane serve only where that reach and the whole-turn one agree, `chord_join::agreed_section`); \|r−gap\| + \|sin×lever\| and r−gap shrunk toward zero by \|sin×lever\| (`decide_across`: the gap and the tilt as one sum, the floor deciding the definite side); sin×r | m | OK |
| intersect.rs:694 | ps_frame_seam | (sin−0.5)·r — deterministic frame tie-break, not a coincidence question | m | OK (note N5) |
| intersect.rs `sphere_sphere_section` | ss_carrier_identity / ss_carrier_external / ss_carrier_internal | `d + \|r₁ − r₂\|`, `d − (r₁ + r₂)`, `d − \|r₁ − r₂\|`, `d` the centres' distance — lengths of the two carriers, bare | m | OK |
| intersect.rs `ss_frame_seam` | ss_frame_seam | `(‖n̂ × û‖ − ½)·r`, a sine levered by operand A's radius — deterministic frame tie-break, not a coincidence question | m | OK (note N5, `ps_frame_seam`'s twin) |
| pair_section.rs (`CylinderSpherePose::decide`, read by `cylinder_sphere_section`, `cylinder_sphere_ssi`, the boolean's section table and its guard) | cs_cylinder_radius / cs_sphere_radius / cs_coaxial / cs_wall_reach / cs_near_reach / cs_far_reach | `r`; `R` (the two radii a `(name, margin, …)` table decides); the sphere centre's distance `d` from the cylinder's (unit) axis; coaxial `R − r`; off the axis `R − \|d − r\|` and `R − (d + r)` | m | OK |
| intersect.rs:705 | ps_center_gap | r − center-plane distance | m | OK |
| intersect.rs:834–882 | cc_* (`cc_radius_equality`, `cc_axes_parallel`, `cc_coaxial` and `cc_coaxial_floor`, `cc_parallel_gap` and `cc_parallel_gap_floor`, `cc_axes_coplanar`) | lengths / sin×extent / common-perpendicular; the coaxial distance and the gap each summed with the axes' sin×lever (`decide_across`) | m | OK |
| intersect.rs:2232/2246/2276/2327/2337 (`plane_cone_section`) | pn_apex_on_plane, pn_apex_section (and `pn_apex_section_floor`), pn_apex_point_reach / pn_apex_pair_reach / pn_apex_tangent_reach, pn_axis_normal, pn_conic_type | m·unit, the apex gap (`pn_apex_on_plane`); trig diff×extent, summed with the apex gap (`pn_apex_section` and its floor, `decide_across`), and alone off the apex (`pn_conic_type`); the apex gap with its own rounding, \|δ̂\| + 2.5·EPSILON·Σ\|(apex − origin)ᵢ·nᵢ\| (m), levered by 1/\|D\|, the off-apex ellipse's farthest point from the apex (`pn_apex_point_reach`) and where the hyperbola's far branch starts (`pn_apex_pair_reach`, the lesser of that and a sum of lengths over the reach's window along the axis, `pair_bound`: a nappe's start less the window's least distance from the apex, clamped at zero, plus the apex gap levered by 2/(σ + √max(σ² − 2s\|δ\|/(ρ·sin α), 0)), plus the served lines' rounding, EPSILON/2·((3 cos α + 4 sin α + 6s·sin α)/σ + 5 sin α/s + 12 sin α + 3) levered by the window's farthest distance from the apex), and by 1/(sin α·s + cos α·\|c\|), where the section starts nearest the apex (`pn_apex_tangent_reach`); sin×rim r | m | OK |
| intersect.rs `plane_cone_section` (the aperture guards, a `(name, margin, what)` table decided before any pose trilean) | pn_aperture_sin / pn_aperture_cos / pn_seam_off_axis | `sin α`, `cos α` and the seam's sine off the axis (`geom::square_frame`'s `width`), each levered by the operand extent from the apex (`Reach::lever_from`) | m | OK (levered door; division guards and a convention clause, `coc_`'s three at this arm's lever) |
| intersect/conic_arc.rs (`OpenConicSection::branch_at`) | pn_conic_branch | the seed's signed distance from the hyperbola's conjugate axis in the plane, `((P − F)·û − cδs/K)·sign(δσ)` | m | OK (new in OFFSET conic arc) |
| intersect/conic_arc.rs (`ConicBranch::arc`) | pn_conic_window_on | a window end's distance from the branch's point at its height | m | OK (new in OFFSET conic arc) |
| intersect/conic_arc.rs (`ConicBranch::arc`) | pn_conic_window_span | the distance between the window's two minted ends | m | OK (new in OFFSET conic arc) |
| intersect.rs `plane_torus_section` | torus_tube_positive / ring_torus_convention (read from `geom::ring_torus`, the convention's one home) / pt_axis_in_plane / pt_axis_plane_gap (and `pt_axis_plane_gap_floor`) / pt_axis_normal / pt_cap_gap (and `pt_cap_gap_floor`) | r; R−r; sin×extent; the stand-off `n·(q − c)`, summed with the meridian circles' lean `r·(a·n)` (`decide_across`): each circle stands at most that sum off the plane, at any extent; sin×(R + r); r − the depth `d/(a·n)` of the plane's crossing of the axis (independent of where its origin is stored), summed with sin×(R + r) (`decide_across`): each circle is minted at that crossing and stands at most sin×(R + r) off the plane | m | OK |
| intersect.rs:2026/2047/2053/2068/2093/2111 | coc_cylinder_radius / coc_aperture_sin / coc_aperture_cos / coc_seam_off_axis / coc_axes_parallel / coc_coaxial / coc_coaxial_floor / coc_station_reach | R; sin α×extent; cos α×extent; the seam's sine off the axis (`geom::square_frame`'s `width`)×extent; sin×extent; the apex's distance from the cylinder's axis plus sin×extent, and less it for the floor (`decide_across`); extent − |R·cot α| (extent measured from the apex) | m | OK — the family prefix is `coc_`, not `cc_`: `cc_*` is cylinder×cylinder's glob two rows above and would over-match. `coc_station_reach` is the arm's ADMISSION criterion; the two aperture clauses are a division guard and a convention clause, not conditioning questions |
| edge_nurbs.rs (`plane_nurbs_limbs`, the transversality hook on `chart_image`'s interior samples; through `dihedral::decide_reported`) | plane_nurbs_transversality | `sin θ` of the plane's normal against the wall's `du × dv` (`normal_angle_sine`, both normalised), levered by `ssi::point_arm`'s `extent / max(1, κ·extent)` = min(extent, 1/κ), κ the wall's largest principal curvature at the foot (1/m) | m | OK (levered door; `ssi_transversality`'s comparand at the plane × NURBS lane's own samples. The reported minimum is `plane_nurbs_transversality_reported`, a `gate_measured` name outside this table) |
| dihedral_side.rs (`spline_normal`, a spline face's foot of a shared edge's point; through `dihedral::decide_magnitude_reported`) | dihedral_side_foot | `‖S(u*, v*) − p‖`, the projection's own distance at `T` | m | OK (a nonnegative length on the magnitude door: zero puts the point on the spline face, positive refuses it off the face. The side itself is decided by `enters_material` over the folded arm) |
| dihedral_side.rs (`spline_normal` through `near`, a spline face's foot against the chart's irregular points; through `dihedral::decide_magnitude_reported`) | dihedral_side_regular | `‖m‖·‖S_x‖/‖∂_x m‖` for `m = S_u × S_v`, least over the two directions and folded with the reach: the arc along `x` to where `m` vanishes (twice it at a kink) | m | OK (a nonnegative length on the magnitude door: positive reads the foot as regular; zero, in band or poison (`m` and its change both zero) escalates `Crease`) |
| dihedral_side.rs (`on_a_crease`, a spline face's foot against each line its normal is not continuous across; through `dihedral::decide_magnitude_reported`) | dihedral_side_crease | `‖S(c, v*) − S(u*, v*)‖` (or the `v` twin), the foot's distance from a crease run or a closed chart's end along the iso line through it | m | OK (a nonnegative length on the magnitude door: positive reads the foot's span, zero or in band escalates `Crease`) |
| edge_nurbs.rs (`probe_pick` through `first_within`; the label is `pick_arc_rim`'s) | pcurve_iso_arc_row_direction | the largest of the carrier's three 3-D distances from the chart's point at a candidate image's own parameter — at the edge's two ends and a quarter span in — over the four closed-form arc-rim images (two rows × two `u` directions), then the measured ones | m | OK (`Margin::of` a distance; as `pcurve_iso_line_rim`: only a decided `Zero` selects, an escalation is deferred, and the arc-rim certification follows the selection) |
| pcurve_cache.rs (`Placement::turns`) | pcurve_chart_placement | (angle − mark)×the angle's lever through `Margin::metered` (the lever an `InfSpeed`: for the azimuth, the farthest of three sampled reaches, a floor on the span's largest azimuth rate and not on the rate everywhere, which the placement needs since every outcome of its marks is a placement of the same curve (`Levers`); the tube angle's minor radius and the latitude's sphere radius, each the exact rate), the half-period marks about the window's centre (`Period::decide`); one expression, the read's one cut being the window's (`pcurve_chart_cut_side`) | m | OK (metric door, clause (iii)) |
| pcurve_cache.rs:1268 | pcurve_map_residual | mapped point distance | m | OK |
| pcurve_cache.rs:1964 | pcurve_interval_forward (harmonic) | span × param_rate | m | OK |
| pcurve_cache.rs:1894 | pcurve_interval_meter (fitted/iso gate) | carrier parameter extent × param_rate (a NURBS net's knot domain × its certified speed lower bound) | m | OK (metered door; the collapsed-arm gate) |
| pcurve_cache.rs:2382 / :2868 | pcurve_interval_forward (fitted / iso) | span × param_rate — a NURBS carrier's rate IS its certified speed lower bound | m | OK (metered door; the meter gated at :1894) |
| pcurve_cache/projected.rs (`run_projected_checks`, check 2) | pcurve_azimuth_period / pcurve_azimuth_period_straddle (projected; its only site, the closed-form lanes' gone with the edge certificate's `interval_span_winding`, PCERT B2a) | each channel's sweep over the swept box against one turn through the turn rule (`Period::turn`'s row): a cylinder's azimuth at `Exact(r)`; the sphere's, torus's and cone's at `Bounded(azimuth_lever_inf, chart_sup_arms_at)` — `r·cos v` at the box's widest latitude, `R − r`, the cone's least `|v|·sin α` below, `r`, `R + r`, the cone's `v_sup·sin α` above; the tube's at `Exact(minor)` | m | OK (asked "not over", `Period::not_over`: `Short` and `Whole` admit, `Over` refuses; an undecided reading the sup reads definitely positive is short and its escalation superseded (`TurnUndecided::not_over_at_sup`); any other escalates, an overshoot only the sup reads past the band included) |
| pcurve_cache.rs:1664 | pcurve_chart_radial_moving | Σ m-norms BARE (amplitude is metres) | m | FIXED (M6-3) |
| pcurve_cache.rs:1680/1772/1791 | pcurve_chart_orientation / sphere meridian | m² ÷ radius | m | OK |
| pcurve_cache.rs:1752 | pcurve_sphere_chart_frame | m at :1770, dimensionless at :1836 (tie-break) | mixed | FLAG (note N5) |
| pcurve_cache.rs:1759–1829 | pcurve_sphere_chart_* | m-scaled coefficients / rooted | m | OK |
| pcurve_cache.rs (iso lane, M7) | pcurve_iso_boundary / iso_axis_u/v / iso_domain | chart-param values/extents/overhangs × stretch bounds (m per chart unit); two definite non-zero `pcurve_iso_boundary` verdicts route the seam class through a `pcurve_iso_domain` decide of the fixed channel's overshoot (× `stretch_u`; a column outside the domain refuses typed) to the collapsed-row hull (an interior column, `nurbs_iso::interior_iso_u`), whose slack is the channel's drift alone — no snap term | m | OK (**`metered_sup` door**: `nurbs_stretch_bounds` answers a `SupSpeed` pair and every row here meters an overshoot or a snap slack; door added by the clause-(i) migration) |
| pcurve_cache.rs (iso lane, seam class) | pcurve_iso_seam_sense | the image's `v` travel `pl.y · span`, `metered_sup` by `stretch_v` | m | OK (which way a seam-class image runs its column: a wrap edge laid against the column is compared against the column run back) |
| pcurve_cache.rs (ARC-RIM iso class, M8-3) | pcurve_interval_forward / pcurve_iso_boundary | span × `param_rate` = arc LENGTH (the class's carrier is a `Curve3::Circle` by construction, so the rate is the radius); the sub-arc weight residual `w − cos(h/2)` metered at the radius | m | OK (metered door, so an `InfSpeed`: the class's rate is a circle radius, an exact rate and therefore an inf bound, and it cannot be poison — which is why it carries no meter gate. The arc rim's chart-column knot deviation, metred through `stretch_u`, takes `metered_sup` with the rest of the iso lane) |
| pcurve_cache.rs (iso/fitted lanes) | pcurve_envelope | certified sup bound (m); on the seam class the traversed row's control-difference hull, a boundary row's copy or an interior column's de Boor collapse (each collapsed control point encloses the exact one at the interval scalar, so the hull's upper end still bounds the sup) | m | OK (added by the clause-(i) migration) |
| pcurve_cache.rs (fitted lane, `MapResidualHermite` statement) | pcurve_envelope_hermite | the sphere general-circle image's Hermite sup bound, re-derived by `certify_fitted` (m); refined to `ε/4` by `sphere_circle_image_lane`, so ε-coupled by construction and judged by `tools/k-lint`'s `CONSTRUCTION_COUPLED` | m | OK |
| pcurve_cache.rs (chart derivation, M6-3) | pcurve_cone/sphere/torus_chart_axial / _centered / chart_radial_moving | axial displacement sums; radial-offset norms; Σ m-norms | m | OK (added by the clause-(i) migration) |
| pcurve_cache.rs (chart derivation) | pcurve_chart_orientation / sphere/torus_chart_meridian | oriented area a×b·n̂ over its radius lever (m²/m) | m | OK (over_lever door; added by the clause-(i) migration) |
| pcurve_cache.rs (chart derivation, CAVITYDIAG, `cylinder_rim_axial`) | pcurve_cylinder_chart_axial | a moving carrier's axial `cos t` and `sin t` terms on a cylinder's chart, `√(a² + b²)` (`harmonic_sup_scalar`; the terms are displacement components along the unit axis, metres) | m | OK (new in CAVITYDIAG; the cylinder's twin of the cone arm's `pcurve_cone_chart_axial`). Asked only where both terms are enclosures holding zero and not both exactly zero, so `f64` never reaches it; only a definite `Zero` writes them as zero, the rim's height then a constant; anything else keeps them as computed |
| pcurve_cache.rs (chart derivation) | pcurve_cone_chart_nappe (h0/h data) | axial heights (m) | m | OK; the hs COSINE fallback is FLAG F13 |
| pcurve_cache.rs (`spiric_chart_pcurve`, torus arm) | pcurve_spiric_chart_axis | `chart.axis · carrier.axis`, a cosine of unit vectors, levered at the chart's `major_radius` | m | OK (**`levered` door** — a dimensionless cosine is metered by MULTIPLYING an arm; `over_lever` divides a measure by one and is for an oriented area over a radius. Zero refuses — a torus whose axis is perpendicular to the carrier's is not the torus this spiric sections) |
| pcurve_cache.rs (`spiric_chart_pcurve`, torus arm) | pcurve_spiric_mint_center | `‖chart.center − carrier.center‖`, bare (metres) | m | OK (decides the image's `WallTorus` tag: Zero is the carrier's own torus, either sign the mirror through the cutting plane. Not a certificate — check 1's `pcurve_spiric_chart_center` then gates the chart against the tagged torus, so a chart that is neither refuses there) |
| pcurve_cache.rs (`run_spiric_checks`, check 1) | pcurve_spiric_chart_center / _chart_major / _chart_minor | the CHART torus's centre, `R` and `r` against the torus the image's `WallTorus` tag names (the carrier's own, or its mirror through the cutting plane, centre `WallTorus::center`), bare (all three are metres) | m | OK; Zero required. The identity is a statement about the map, so the chart being that torus is its premise — and what the band admits is priced into check 4 rather than discarded |
| pcurve_cache.rs (`run_spiric_checks`, check 1) | pcurve_spiric_chart_other | `‖chart.center − c_other‖`, the chart's centre against the OTHER of the two tori that hold the oval, bare (metres) | m | OK (read only after `pcurve_spiric_chart_center` fails and the radii and tilt gates hold: Zero is a chart that is the other torus, refused as a mismatched image rather than as off the chart; anything else refuses off the chart) |
| pcurve_cache.rs (`run_spiric_checks`, check 1) | pcurve_spiric_chart_tilt | `\|chart.axis × carrier.axis\|`, a sine of unit vectors, levered at the chart's `R + r` | m | OK (levered door; PARALLELISM only — which of the two directions the frame takes is `pcurve_spiric_chart_axis`'s decision at the mint, and a wrong sign is a whole-span displacement the schedule sees at every sample) |
| pcurve_cache.rs (`run_spiric_checks`, check 1) | pcurve_spiric_major / _minor / _offset | the image's stored scalar minus the CARRIER's own, bare (all three are metres) | m | OK; Zero required, and whatever the band admits is carried into check 4's envelope through the `f` drift bound rather than discarded |
| pcurve_cache.rs (`run_spiric_checks`, check 1) | pcurve_spiric_sense | `\|sense\| − 1`, dimensionless, levered at `azimuth_lever` | m | OK (**`levered` door**, so the Zero window is `\|η\| ≤ ε/(R + r)`; Zero required, and the residue is metered into the envelope on BOTH channels — `v` at the chart's minor radius over the parameter reach, `u` at `R + r` over `atan2`'s own `π` range) |
| pcurve_cache.rs (chart derivation, through `geom::Azimuth`: the decision itself is `stable_azimuth`'s, in `crates/geom/src/azimuth.rs`) | pcurve_chart_cut_side (the direction's signed distance across the principal window's cut line, `y·cos s − x·sin s` with `s` the window's start: which `atan2` formula keeps its cut off the direction; it was `pcurve_chart_azimuth_frame`, decided on `x`'s sign, a second cut at the landmark −π/2) / sphere_chart_pole_frame / polar & meridional rates | metres where the caller passes a radial at its length; dimensionless where it passes a unit direction (the Villarceau `t0`, the sphere meridian's `α`, the cone ruling, whose F13 fallback is a UNIT radial's projection). Tie-break-only either way (N5: the trilean picks between two formulas identical mod τ — verdict-neutral), and that lane is F13-flagged one decision earlier | m (mixed on the F13 lane) | OK as tie-break (N5; row corrected at the clause-(i) fix pass, review MIN-1) |
| props/curved.rs (`require_rim_incidence`; the iso parses) | props_rim_axis_parallel / props_rim_center_on_axis | sin×r_c; perpendicular offset | m | OK (the circle's deviation from the iso rim, not its distance from the surface, so a miss is the inventory's, `NotIsoRectangle`, on every kind: each parse decides incidence at the true distance first, the cylinder under `props_cylinder_circle_*` and the cone under `props_cone_circle_*`. Pinned in `tests/iso_parse_incidence.rs`) |
| props/curved.rs (`cylinder_boundary`) | props_cylinder_circle_tilt / props_cylinder_circle_off_axis | the circle's TRUE distance from the cylinder, `props_rim_fit` and the definite axis class held: tilted by `θ` (`cos θ = \|n_c·â\|`), `r_c·(1 − cos θ)`; centre `o` off the axis, `o`. Both attained | m | OK (the cone's pair at `α = 0`. Refused as `OffSurface`; pinned in `tests/iso_parse_incidence.rs`) |
| props/curved.rs (`require_on_cone`: the cone's flux lane and material sign, on every circle of every loop; the iso parse `cone_boundary`, before the iso pair) | props_cone_circle_tilt / props_cone_circle_off_axis | the circle's TRUE distance from the cone, the other two circle premises (`props_rim_fit`, the definite axis class) held: tilted by `θ` (`sin θ = ‖n_c × â‖`, `cos θ = \|n_c·â\|`), `r_c·(sin θ·sin α + (1 − cos θ)·cos α)`; centre `o` off the axis line, `o·cos α`. Both attained, by the point opposite the tilt and the point on the offset's side | m | OK (the lane reads a circle by its vector area alone, so it asks incidence, not whether the circle is the iso rim the iso parses' `props_rim_axis_parallel` / `props_rim_center_on_axis` read a level off. Those meter the deviation from that rim, `r·sin θ` and `o`, which the distance from the cone undershoots by `1/sin α` toward a needle and `1/cos α` toward the flat, and by the second-order `r·(1 − cos θ)` alone at the needle's limit, which a first-order `r·θ·sin α` would drop, admitting any tilt there. Refused as `OffSurface`; pinned at both ends of the half-angle range in `tests/props_cone_incidence.rs`) |
| props/curved.rs (`require_cone_ellipse`, from `require_on_cone`, on every ellipse of every loop) | props_cone_ellipse_nappe | a lower bound on the arc's axial height `|h|` above the apex over its span, in 128 equal pieces: on each, the larger of the midpoint's `|h_m|` less what the turned axis vectors can move it (`|A·axis|·X + |B·axis|·Y`) and the whole carrier's least `|h_m − A·axis| − ‖(A·axis, B·axis)‖`; the least over the pieces | m | OK (decided `Positive` before the incidence bound divides by it: an ellipse on the cone lies on one nappe clear of the apex, so a carrier that reaches the apex's plane is off the cone, refused `OffSurface`) |
| props/curved.rs (`require_cone_ellipse`, as above) | props_cone_ellipse_on_cone | an upper bound on the arc's distance from the cone over its whole span: on each of 128 equal pieces, `|Q|` bounded through its five coefficients about the piece's midpoint, over a lower bound on the second factor `ρ·cos α + |h|·sin α` of `Q = (ρ·cos α − |h|·sin α)(ρ·cos α + |h|·sin α)`; the largest over the pieces | m | OK (`Q = ρ²·cos²α − h²·sin²α` along a conic carrier is a trigonometric polynomial of degree 2; about a midpoint, in `x = cos δ − 1`, `y = sin δ` with `x² + 2x + y² = 0`, it has the unique form `k₀ + k_x·x + k_y·y + k_xx·x² + k_xy·x·y`, every coefficient zero on the cone, and `|x| ≤ 1 − cos δ_max`, `|y| ≤ sin δ_max` bound it — a bound over the span, not samples of it: the five-sample decide this replaced read the elevation at five span points and passed rings up to 584 bands off the cone between them on eccentric sections (PR 4744 review). `k₀` is the midpoint's own elevation times the second factor, and the quadratic form is taken from radial and axial parts apart, so the coefficients carry the rounding of points near the face — not of a carrier reaching far past it, nor of `cos²α·|w|² − h²`'s cancellation toward a needle. Pieces are many because an eccentric section's parameter is slow where it is sharp. Over-reads by the spread of `ρ`, `|h|` across a piece and the triangle inequality. On genuine sections the interval bound escalates only where `64·u·reach` is within about an order of magnitude of the band, and where it starts depends on where the pieces fall: at ε = 1e-12 a whole section at α = 0.5, far/near 1000, reach 570 m (`64·u·reach/zero` ≈ 8, u = 2⁻⁵²) answers stored over `[0, τ]` and escalates over `[π, π + τ]` (PR 4744 verifier); the review's sweep first escalated at ≈ 81 (1e-12) and ≈ 4 (1e-9), and a span starting elsewhere can escalate lower, as that ring does. At rest, tier 2 samples an `Intersection` or cone-`Chart` edge onto the cone over its span; an ellipse whose `Chart` description names the plane is placed on the cone by nothing at rest, the curved-face containment `validate_geometric` lists as unchecked. Refused as `OffSurface`) |
| props/curved.rs (`level_coincides`, `props_rim_level_group` call) | props_rim_level_group (Length) | level difference BARE (v is arc length) | m | RETIRED — the site is gone: the grouping lived in `du_of_rims`, deleted when its last caller, the cone's iso premise, retired (FLUX `cone-face-closed-form-runs-an-iso-premise-its-formula-does-not-need`). FIXED by #89's unit while it stood |
| props/curved.rs (`level_coincides`, `props_rim_level_group` call) | props_rim_level_group (Unit) | rooted (sin,cos) CHORD × `RimArms::level` (sphere ×R, torus ×minor) | m | RETIRED — the site is gone with `du_of_rims` (above). **N1 RETIRED** by S81 while it stood |
| props/curved.rs (`du_of_rims`) | props_du_consistent | Δu (rad) × `RimArms::azimuth` | m | RETIRED — the site is gone with `du_of_rims` (above) |
| props/curved.rs (`cone_apex_level`) | props_rim_only_extent | per kind, `require_extent`'s OWN comparand asked one step earlier: `sphere_extent_margin` — `(hi − lo)·R` — on the sphere, from the one helper both read; the bare slant difference `hi − lo` on the cone, as `require_extent`'s cone call reads it | m | OK (note N9: ONE comparand under TWO names, so a meridian-free rim-bearing sphere face records both; note N8 applies verbatim — the sine extent shrinks by `cos v̄` near the poles, in the FOLDING direction here, which is the direction that serves the cap) |
| props/curved.rs (`require_rim_only_closed`) | props_rim_only_closed | the rim spans' sum (an `Extent` sum) against one turn through the turn rule (`Period::turn`'s row) at `Exact(RimArms::azimuth)`, a whole turn admitted — the headroom `(τ − Δu)`, the arc a rim whose face folded a missing extreme fails to close by; the sphere's pole and the cone's apex share it, at each kind's own azimuthal arm (`R`, and the cone's first rim radius) | m | OK (the shape door's apex-fold guard, `require_iso_rectangle`; the flux arms read a closed loop by `props_loop_closed` and, on the cone, `props_cone_loop_winding`) |
| props/curved.rs (`require_rim_only_closed`) | props_rim_only_join | `‖p_end(i) − p_start(i+1)‖` over the group's arcs, cyclically — a point deviation, bare (metres already), the comparand `require_rim_incidence` meters an incidence with | m | OK (the tiling half of the closure guard: the sum says the spans total a turn, this says the arcs actually chain, and a sum without a chain covered half a circle and answered a whole cap) |
| props/curved.rs (`rim_side`, through `unanimous_rim_side` — the shape door's, `require_iso_rectangle`) | props_rim_side | per-kind: bare (Length) / × `RimArms::level` (Unit) | m | FIXED (#89's unit); note N8 open — the sphere margin reads the PRIMARY component (`lo + hi − 2·sin v`), an axial quantity that shrinks by `cos v̄` near the poles, refusing direction |
| props/curved.rs (`cylinder_windings`, `torus_chart_loop`: the closed forms' input to `closure::require_joints_closed`; `cone_loops`, through `closure::require_gap_closed`) | props_chart_junction_closed / props_chart_junction_open | `max(G − S, 0)` per junction, one value read twice — `G = Σ lever·\|turn\| + \|flat\|`, the turn about each periodic direction from one edge's traversal end to the next's start the short way round (rad: about the axis, levered at the radius on the cylinder and at the after-point's distance from the axis on the torus; about the tube, levered at the minor radius), and the level gap on the cylinder (m); on the cone the gap in space, `‖end − start‖`, which has no period to take off and reads the apex with every other point (m); `S = 3ε`, the two carriers' ends about their vertex | m | OK (the periodic-chart closure decision on the closed forms' carriers: closed Zero, `ChartLoopOpen` on a definite Positive, escalated in band — the quadrature's rule below, with no end slack because a closed form's carrier ends where it is evaluated) |
| props/closure.rs (`loop_turns`, every closed-form loop on a cylinder, torus or cone chart; on the cone every loop off the apex) | props_chart_loop_turns / props_chart_loop_sense | `π·h − \|W\|` for `h = 1, 3`, `W` the loop's total turn (its edges' sweep plus its junctions' short turns, rad), levered at the loop's largest junction radius; then `W` itself | m | OK (whole-number structure read at half-turn thresholds a closed loop's `W = 2πk` is clear of by `π·lever`: `0`, `±1`, or `±2` standing for any more. `\|k\| ≤ 1` and `Σk = 0` over the face, per direction, and at most two winding loops on one connected face, are then integer facts, refused `LoopsWindChart`; a ring is a cylinder hole iff its `k` is `0`. A `W` in band of a half turn is no whole number, `ChartLoopOpen`) |
| props/closure.rs (`closed_loop_turns`, every closed-form loop on a cylinder, torus or cone chart with `\|k\| ≤ 1`) | props_chart_loop_whole | `excess(Σ_e lever_e·\|miss_e\|, 3ε)` — each edge's miss, the principal angle from its start's direction turned by its sweep to its end's (`EdgeTurn::new`, off the two ends' dot and cross arms, so read clear of the branch cut whatever the sweep), metered at the least radius the edge is read at, summed over the loop, against the vertex slack once for the loop | m | OK (every sweep is its edge's turn about the surface's axis (core circle) between its own ends — a rim's (a meridian's) stored turn about its own centre, carried to the axis at each end, an ellipse's thirds about the cone's axis, and a ruling's, a generator's, a meridian's azimuth and a parallel's tube angle as the short turn itself — so each term is rounding alone, charged at its own radius. What `W` holds beyond the terms is the short turns through the loop's vertices, a whole number of turns by construction. The loop-level `\|W − k·τ\|` at the largest junction radius it replaced charged an azimuth read within `ρ` of the axis, which rounds by `≈ u·\|p\|/ρ`, at millimetres, and escalated spindle and horn tori with a parallel within `10⁻⁸` m of the axis at the interval scalar (`circles_just_past_the_axis_class_threshold_measure`). Pinned also by `rims_in_many_arcs_off_the_axis_measure`, `a_seam_cut_wall_with_arcs_off_the_axis_measures` and `a_cone_rim_in_many_arcs_off_the_axis_measures`, arcs centred within `ε` of the axis. Positive refuses `LoopsWindChart`, a sweep that disagrees with its edge's ends; in band escalates. Pinned by `tests/periodic_chart_closure.rs::a_loop_of_many_rulings_turns_by_their_own_turns`, `a_window_from_near_the_apex_measures_its_closed_form` and `a_metre_spindle_torus_near_its_axis_measures_its_closed_form` (each edge charged at its own radius; the torus meridians carried in their own plane), and `props::closure`'s unit rows. A miss is stated where it is exact: none for a cone generator's short turn or an ellipse's thirds, which agree with their ends by construction, and a rim's read about its own centre, the carry cancelling (`EdgeTurn::exact`, `EdgeTurn::carried`); read back about the axis against its own ends, a sweep charged the interval scalar's width of those points as a miss and escalated cone faces at `ε = 1e-12` (`reach_cone_root_lane`'s ring, the steep frustum)) |
| props/closure.rs (`require_axis_inside`: the cylinder once at its least rim's radius; the torus once at its minor radius and at each parallel's own) | props_chart_axis_inside | `radius`, bare (metres) | m | OK (the premise `curved::about_axis` and the tube's analogue are read under: a circle whose centre the parse holds within `ε` of the axis has the axis inside it once its radius is definite, at least `Kε` — the least radius the parse's axis class admits a rim at — so the turn carrying its stored turn to the axis stays under a quarter turn, clear of the branch cut. Positive passes; Zero or Negative refuses `NotIsoRectangle`, a circle within its own band; in band escalates. Each call reads a circle's own radius, so the axis class subsumes it on every face the parse admits; the cylinder's read the surface radius until `flux/e-bundle-lane-honesty` and escalated a cylinder narrower than `Kε` with rims the class admits (`tests/periodic_chart_closure.rs::a_cylinder_under_k_eps_with_admitted_rims_measures`)) |
| props/curved.rs (`cylinder_material_sign`) | props_chart_area_side | `2·R·A_chart / P` — the chart area in m² over the boundary length, the mean width (F4) | m | OK (`over_lever`; the planar loop-winding comparand on the wall's chart) |
| props/curved.rs (`torus_chart`) | props_torus_lift_branch | `cos v` of a loop's first rim, bare: which `atan2` branch keeps the lift's start off the cut | — | OK (a branch selection, not a verdict: either branch is exact, the lift moved by a period shifts nothing the Green form sums; `pcurve_chart_azimuth_frame`'s shape) |
| props/curved.rs (`torus_lift`, per rim of a torus loop) | props_torus_lift_turns | `Margin::metered(g − (k ± ½)·τ, InfSpeed(minor))` at the half-turn marks (`Period::decide`), `g` the running tube angle less the rim's own | m | OK (a branch decided as structure, an integer: every junction is decided closed and every edge's turn whole first (`closure::closed_form_windings`), so `g` is a whole number of turns to within `≈ Kε/minor`, half a turn clear of each mark, and the marks decide; the lift's shift `k·τ` is a literal at every scalar. Replaces `(at − angle).periodic_branch(τ)`, a floor opaque at `Sym`. Zero (on a mark) refuses `NotIsoRectangle`; in band escalates. Pinned by `props::curved::torus_lift_rows` at `Sym<Interval>` over a box, `k ∈ {−1, 0, 1, 2}`, red under a floor-restoring and a constant-branch mutant) |
| props/curved.rs (`torus_chart`, `torus_material_sign`) | props_chart_area_side / props_ring_winding | `2·A / P` — the torus chart area (m², `−Σ∮ G(v) du`) over the boundary length | m | OK (`over_lever`; the cylinder's comparand on the torus chart) |
| props/curved.rs (`cone_loops`) | props_loop_closed | — | m | RETIRED — the cone's junctions are the periodic-chart closure's (`props_chart_junction_closed`, against its `3ε` slack), on every loop of every cone face, at the flux arm and the gate (FLUX `the-cone-closure-decide-is-a-fifth-spelling-of-the-periodic-chart-question`) |
| props/curved.rs (`cone_loops`, signed by the nappe — the flux arm and the gate; `require_holes_against` for rings) | props_cone_area_side / props_ring_winding | `2·A / P` — the cone area `−ν·axis·A⃗ / sin α` (m², signed by the traversal and the nappe) over an upper bound on the boundary length | m | OK (`over_lever`; the hole meter the cylinder and torus share. A face of no width encodes no side and is degenerate) |
| props/curved.rs (`cone_loop_winding`, a ring's) | props_cone_ring_off_apex | the radial distance from the axis of an edge's nearer end, bare | m | OK (where the azimuth is defined) |
| props/curved.rs (`cone_loop_winding`, an outer loop's) | props_cone_loop_off_apex | the radial distance from the axis of each edge end, bare | m | OK (an end not decided off the axis is on the apex: a generator's azimuth there is read off its other end, any other edge refuses) |
| props/curved.rs (`cone_loop_winding`) | props_cone_arc_sense / props_cone_arc_turn | `n·axis` (a cosine), levered by the arc's smallest radial distance; and, on an ELLIPSE only, each third of its principal turn (rad) times its sense, levered by the smallest radial distance of its ends and third-points | m | OK (a rim's parameter is its azimuth, so it turns through its stored span with no turn decided — no margin read off an arc's length, which escalated rims a few `K·ε` long. An ellipse's thirds: a whole turn closed by one edge is no principal angle of zero, and the long-way arm is reachable, a third of a tilted section's parameter near the vertex the axis pierces closest to subtending more than half a turn) |
| props/curved.rs (`require_cone_ring_contractible`) | props_cone_ring_contractible | — | m | RETIRED — a ring's winding is the periodic-chart closure's (`props_chart_loop_turns`); a ring that winds is a band's rim, which measures beside the other, and the face's windings are held to a boundary's, the apex winding back the balance of a face that holds it |
| props/curved.rs (`cone_face_closed_form`) | props_cone_loop_winding | — | m | RETIRED — an outer loop off the apex winds as the periodic charts' do (`props_chart_loop_turns`, `props_chart_loop_whole`); one that winds twice is `LoopsWindChart` |
| props/curved.rs (`cone_face_closed_form`) | props_cone_apex_loop_net / props_cone_apex_loop_sweep | the loop's net sweep, decided nonzero at its widest radius (`Extent::decide_signed`; none refuses `ConeLoop`), then against one turn through the turn rule (`Period::turn`'s row) at `Exact(widest radius)` — a loop through the apex sums its edges' turn, the sector it sweeps | m | OK (a whole turn ADMITS: the slit cap sweeps exactly a turn. Past a turn the loop covers part of the nappe twice; no whole-turn quantisation here, so the bound is the turn itself and not a half-turn margin) |
| props/curved.rs (`require_cone_loop_vertices_distinct`) | props_cone_loop_vertex | `‖v_i − v_j‖` over a loop's non-adjacent traversal starts, bare | m | OK (Positive = distinct; a coincidence is a pinch or a restatement unless the sheet decide says the loop went round between them) |
| props/curved.rs (`require_cone_loop_vertices_distinct`) | props_cone_loop_vertex_sheet | `\|Δθ\| − π` (rad), the azimuth the loop turns between two coincident vertices, levered by their radius | m | OK (a whole number of turns: a seam's two copies of one vertex are a turn apart, a pinch none) |
| props/curved.rs (`require_cone_holes`) | props_cone_outer_side | `2·A / P` of the outer loop alone, the cone's area meter | m | OK (`over_lever`; the side holes are wound against, and the face's side must keep it — else `HolesExceedFace`) |
| props/curved.rs (`cylinder_boundary`'s line arm / `cone_boundary`'s line arm / `require_on_cone`) | props_meridian_axial / props_meridian_generator | the cylinder: sin × parameter span; the cone: `sin(β − α) = \|dir × axis\|·cos α − \|dir·axis\|·sin α`, the line's angle off the generator, × the edge's REACH from the apex (its farther end's distance) | m | OK (the line's deviation from a ruling or generator, not its distance from the surface, so a miss is the inventory's, `NotIsoRectangle`, after `props_meridian_on_surface` has decided incidence: a line tipped in the tangent plane leaves the ruling, and misses the apex, to first order while it leaves the surface to second. A line through the apex at angle `β` is off the cone by `s·sin(β − α)` at distance `s` from it, so the reach, not the span, is the lever: the span understated a short edge far from the apex by their ratio, and closure caught it under its own name. The cone's line meters its angle off the generator directly: the cosine difference `\|dir·axis\| − cos α` it replaced is that angle times `sin α`, understating it toward a needle cone; `tests/props_cone_incidence.rs` pinned both while these were incidence; at the inventory they are the true distance again only for a line through the apex, which `props_meridian_on_surface` reads first, so no row separates the generator decide from it. The tangential tips are pinned in `tests/iso_parse_incidence.rs`) |
| props/curved.rs (`cylinder_boundary`'s line arm; `require_cone_generator`, from `cone_boundary` and `require_on_cone`) | props_meridian_on_surface | an upper bound on the segment's distance from the surface: its ends' residuals (`ρ − R`; `ρ·cos α − \|h\|·sin α`), and how far it sags inside their chord, the radial sag `N / (√ρ0 + √ρ1)²` with `N = ‖q1 − q0‖² − (ρ1 − ρ0)²` (times `cos α` on the cone); on the cone the segment is split where it crosses the apex's plane, whose point is a piece end at `ρ·cos α` | m | OK (`ρ` is convex along a line and `h` affine, so the residual is convex on each piece: exact at the ends and the crossing, and the sag within a factor 2 of the greatest, read as `min(N/(√ρ0 + √ρ1)², 2N/(ℓ + δ))` (`ℓ = ‖q1 − q0‖`, `δ = |ρ1 − ρ0|`) with each denominator floored at the least positive `f64`, and `N` formed without the cancellation `ℓ − δ` carries: `2·(ρ0ρ1 − q0·q1)`, or `2·‖q0 × q1‖²/(ρ0ρ1 + q0·q1)` where `q0·q1 ≥ 0`, so a generator's (radially parallel ends) is rounding squared rather than two norms' width — at the interval scalar that width escalated `reach_cone_split`'s 410 m steep frustum at `ε = 1e-12` (`tests/periodic_chart_closure.rs::a_steep_wide_frustum_wall_off_the_origin_measures_at_every_eps`), so the piece of a generator ending at the apex — both ends on the axis, an enclosure of zero at the interval scalar — reads its sag, not poison. Decided before the line is asked to be a ruling or generator, so a line within band that is neither is the inventory's; it was the ruling's or generator's deviation, which a line tipped in the tangent plane reaches to first order and the surface to second, refused `OffSurface` within band. Pinned in `tests/iso_parse_incidence.rs` (tangential and normal tips, a chord, a line through both nappes off the apex) and `tests/props_cone_incidence.rs`; the apex-crossing generator of `tests/mesh11_arc_branch.rs` reads zero)
| props/curved.rs (the four `*_boundary` parses; `require_on_cone`) | props_rim_fit (all kinds) | residuals; sphere/torus fits ROOTED before compare; the cone's rim fit is the rim's distance from the cone in its meridian plane, `r·cos α − \|h\|·sin α` with `h` the centre's axial height | m | OK (the cone's was `r − \|v\|·sin α` with `v = h / cos α`, the distance over `cos α`: a near-flat cone at `α = π/2 − 10⁻⁶` read a rim stored to the last bit as 10⁻⁹ m off and escalated every such face) |
| props/curved.rs (the four `*_boundary` parses; `require_on_cone`, where Zero is `OffSurface`) | props_circle_axis_class | cos × r_c | m | OK (note N3) |
| props/curved.rs (`require_extent`, called from all four flux lanes) | props_face_extent | m levels; sin-levels ×R; dt×minor | m | OK (note N8 open on the sphere arm — `(hi − lo)·R` is the AXIAL extent, which shrinks by `cos v̄` for a near-polar band, refusing direction) |
| props/curved.rs (`cone_boundary`'s line arm / `require_on_cone`) | props_meridian_apex | apex-line distance | m | OK (inventory, after `props_meridian_on_surface`: the row above) |
| props/curved.rs (`cone_loops`) | props_cone_nappe | slant levels (m) bare | m | OK |
| props/curved.rs (`sphere_boundary`'s meridian arm, `torus_boundary`, `torus_meridian_orient`) | props_meridian_great / props_meridian_orient | lengths / cos×minor | m | OK |
| props/curved.rs (`sphere_loop`) | props_sphere_circle_on / props_sphere_circle_fit / props_loop_closed | `‖(C − c) × n_c‖`; `√(‖C − c‖² + ρ²) − R`; the gap between one arc's end and the next's start (m) | m | OK (`props_loop_closed` is the cylinder's and cone's closure decide on the same gap; the sphere refuses it as `SphereLoop`) |
| props/curved.rs (`require_circle_span`, run by `sphere_loop` on every arc) | props_sphere_arc_span_forward / props_sphere_arc_span_winding | `Δt·ρ` through `Extent::forward` / the span against one turn through the turn rule (`Period::turn`'s row) at `Exact(ρ)`, the arc's own circle radius: certification's `interval_span_forward` / `interval_span_winding` re-decided at the flux lane | m | OK (the doors' `props_meridian_span_*` pair, two rows down, under the flux lane's names and lever, since Gauss–Bonnet integrates every arc's span and the lever is the arc's; refused as `SphereLoop`) |
| props/curved.rs (`require_cusp_free`, `sphere_face`) | props_sphere_loop_cusp / props_sphere_loop_area / props_sphere_hole_area | the chord between a departing and a reversed arriving unit tangent, levered by R; the smaller of the outer region's and its complement's solid angle (sr); a hole's solid angle (sr); both levered by R | m | OK (solid angles from `solid_angle`: Gauss–Bonnet, corrected by a fan of triangles inside the region where the two agree, so a small region keeps its digits) |
| props/curved.rs (`side_of_sphere_loops`) | props_sphere_face_area | a face with holes: its solid angle under each reading of its loops (sr), levered by R | m | OK (the one home of "the face's area is positive": at most one reading is, `4π(1 − r)` less the other; neither is when holes overlap, refused as `SphereLoop`) |
| props/curved.rs (`pole_side`) | props_sphere_pole_plane / props_sphere_pole_on_arc | a pole's offset from an arc's carrier plane, bare (m); the chord from the pole to the arc's nearer span end, membership-signed, levered by the circle radius | m | OK (`arc_point_margin` is `sphere_meridian_pole_margins`' direction arithmetic on any circle) |
| props/curved.rs (`pole_side`) | props_sphere_pole_arc_full / props_sphere_pole_winding | the arc's extent against one turn through the turn rule (`Period::turn`'s row) at `Exact(radius)`, a whole turn (`Period::whole_turn`) read as a full circle; the stereographic image's total turning (rad, ±2π up to rounding) levered by R | m | OK (the turning's two values sit 4π apart, so its sign is decided far from the band) |
| props/curved.rs (`torus_meridian_orient`: `UnitVec3::levered`, then `RigidMotion::from_aim_and_reference`) | props_torus_axis / props_meridian_radial | the torus axis's norm (a unit-at-rest carrier field, a pure number) levered by the anchor meridian's reach from the torus centre, `\|c − centre\| + r` / `Margin::norm3` of the anchor meridian centre's offset from the axis (m) | m / m | FIXED (was the axis's bare norm against the length band, which reads differently at every model scale) |
| boolean/join.rs (germ-plane read: `UnitVec3::levered` of a plane germ's carrier normal) | bool_germ_plane_normal | the normal's norm (a unit-at-rest carrier field, a pure number) levered by a lower bound on the reach the section consumes it over: the ball through the two joined germ sites, from the plane's origin (`ExtentBall::lever_from`) | m | FIXED (was the bare norm against the length band; `sweep`'s `ray_wall_margin_twins` pins it on a plane × cylinder bore) |
| boolean/boxes.rs (`face_box_rule`'s cylinder read: `UnitVec3::levered` of the carrier's axis) | bool_box_cylinder_axis | the axis's norm (a unit-at-rest carrier field, a pure number) levered by the radius, the arm `slab_extent` swings it by | m | FIXED (was the bare norm against the length band) |
| props/curved.rs (`sphere_meridian_pole_margins`, decided by `sphere_meridian_span_levels` AND by `require_one_chart_branch`) | props_meridian_pole | chord from the pole's span-relative direction to the nearer span endpoint, carrying the membership sign (`copysign` of a midpoint dot test) × R — the point deviation of moving the pole onto the span boundary | m | OK (added with the span-derived sphere extent, issue 723 / S-CERT. ONE margin, TWO dispositions since issue 1571, which is why the arithmetic has its own home: the EXTENT FOLD takes everything but a definite Negative — **Positive, Zero and the indeterminate band alike fold the pole latitude** — while the BRANCH DOOR refuses only a definite Positive, so the gap between them is exactly the arc that ENDS at the pole, which both admit. Neither disposition escalates, so the note below holds for both. Near a span end the endpoint latitude is within band² of the pole's, so the fold choices agree far inside tolerance, the folded extent is continuous across the decision, and an in-band margin carries no information a refusal could report — refusing there flipped certify-exactly into an import escalation for a split vertex 1e-6 rad off the pole. The site still RECORDS through `decide` like any classify site; it just never escalates, so its in-band population is expected (issue 1251 schedules the K-baseline fold-in). **Stated for a span of at most one period, and decided so first** (issue 1601): the membership edge is the unclamped `cos(dt/2)`, whose zero set is the two span endpoints only while `dt ≤ 2π`; the helper itself decides `props_meridian_span_forward` and `props_meridian_span_winding` (next two rows) before this margin is formed, so an admitted span is forward and exceeds τ by at most `zero/R`, where the edge cosine is within `(zero/2R)²/2` of its half-turn value and can reclassify only a pole within `zero/2R` of the span endpoint — in-band on both dispositions. The retired half-turn clamp had a zero set at the direction antipodal to the span midpoint, an interior point of any longer span, where a rounding residual folded 36 of 400 `2π + 2δ` spans short. Direction arithmetic throughout — no `atan2`, no mod-2π `floor`: either is wide at its cut/step for an interval enclosure of an arc anchored at a pole, forcing an escalation the scalar lane does not have, live on the die-fillet corpus) |
| props/curved.rs (`require_meridian_span`, via `require_span_within_period`, run by `sphere_meridian_pole_margins` before it forms a margin — so by `sphere_boundary`'s fold AND by `require_one_chart_branch`'s sphere arm — and by `torus_classify` on every torus meridian edge, at the minor radius) | props_meridian_span_forward | `Δt·R` — the stored span itself at the sphere radius (the minor radius on the torus), through `Extent::forward`: certification's `interval_span_forward` re-decided at the parse | m | OK (added with the reversed-span refusal, issue 1601 / S-MESH fix pass. Same margin, band and lever as certification's, hence the same dispositions: only a definite `Positive` span admits; `Zero` refuses `NotIsoRectangle { what: "props_meridian_span_forward" }` as certification's `IntervalNotForward` does, definitely negative (a span stored reversed, `t1 < t0`) refuses the same way, the ambiguity band escalates. Decided before the winding headroom (next row), which cannot see a reversed span — its headroom `τ − Δt` is Positive for any `Δt < 0`. The torus decides it per meridian edge at classification (so its chart's lift, its fold and every consumer of its parse) and the same half for a span reconstructed across pieces as `props_meridian_pieces_forward`. Reachable only from hand-built `LoopEdge`s, as the winding half is) |
| props/curved.rs (`require_meridian_span`, via `require_span_within_period`, run by `sphere_meridian_pole_margins` before it forms a margin — so by `sphere_boundary`'s fold AND by `require_one_chart_branch`'s sphere arm — and by `torus_classify` on every torus meridian edge, at the minor radius) | props_meridian_span_winding | `(τ − Δt)·R` — the stored span's headroom to one period, through the turn rule (`Period::turn`'s row) at `Exact(R)` (`Exact(minor)` on the torus): certification's `interval_span_winding` re-decided at the parse | m | OK (added with the saturated-span refusal, issue 1601 / S-MESH. Same margin, band and lever as certification's, hence the same dispositions: Zero and Positive headroom admit — a span inside the coincidence band above τ is one certification admits too — the ambiguity band escalates, definitely negative headroom refuses `NotIsoRectangle { what: "props_meridian_span_winding" }` on every consumer of the sphere parse and at the branch door, which does not run the parse but reaches the pole helper, where the decide lives. The torus decides it per meridian edge at classification, before its chart lifts `v` through the span and before the fold reads it through `sin`/`cos` (a one-edge chain re-decides nothing), and decides the same invariant for a span reconstructed across pieces as `props_meridian_pieces_winding`. Reachable only from hand-built `LoopEdge`s: every certified door hands the parse a span within the bound, and the import door normalises into `(0, τ]`) |
| props/curved.rs (`require_rim_span`, via `require_span_within_period` — asked by `Rim::decided`, the one door every rim of every kind is minted through (`cylinder_boundary`, `cone_boundary`, `sphere_boundary`, `torus_classify`), and by `require_cone_conic_spans` on every circle of every loop the cone's flux lane and material sign integrate) | props_rim_span_forward | `Δt·r_c` — the rim's stored span itself at the rim circle's own radius, through `Extent::forward`: certification's `interval_span_forward` re-decided at the parse | m | OK (same margin, band and lever as certification's, hence the same dispositions: only a definite `Positive` span admits; `Zero` or a span stored reversed refuses `NotIsoRectangle { what: "props_rim_span_forward" }`; the ambiguity band escalates. Decided before the winding headroom (next row), which cannot see a reversed span. What rests on it: every reader of a minted rim's `dt` (the cylinder chart's and the torus chart's `Δu` sums, `span_sum`), and the cone's `loop_vector_area`, `cone_edge_length` and `cone_loop_winding` reads of a circle's span on every loop, rims and rings alike — a circle on a cone is a coaxial section, so its span is a rim's. The cone's flux lane and material sign mint no rim, so each decides a circle's span once, in `cone_loops`. An ellipse arc's span on a cone is decided beside it under its own pair (`props_cone_ellipse_span_*`, below). A distinct key from the meridian pair: a rim's span is the face's `Δu`, not its level extent. On the sphere's flux lane and material sign the arc pair `props_sphere_arc_span_*` fires first on the same span. Reachable only from hand-built `LoopEdge`s; pinned in `tests/stored_span_winding_bound.rs`) |
| props/curved.rs (`require_rim_span`, as the row above) | props_rim_span_winding | `(τ − Δt)·r_c` — the rim's stored span's headroom to one period, through the turn rule (`Period::turn`'s row) at `Exact(r_c)`: certification's `interval_span_winding` re-decided at the parse | m | OK (Zero and Positive headroom admit — a full-period rim, the scaffolding every revolve mints, sits AT the bound and is admitted, and so is a span inside the coincidence band above τ; the ambiguity band escalates; definitely negative headroom refuses `NotIsoRectangle { what: "props_rim_span_winding" }` at every consumer of every kind's parse. Unrelated to the meridian pair's decide, which never reads a rim. Reachable only from hand-built `LoopEdge`s: every certified door bounds the span at τ + zero/r) |
| props/curved.rs (`require_cone_conic_spans`, via `require_span_within_period`, on every ellipse of every loop the cone's flux lane and material sign read, before `loop_vector_area`, `cone_edge_length` and `cone_loop_winding` take its span) | props_cone_ellipse_span_forward / props_cone_ellipse_span_winding / props_cone_ellipse_span_winding_straddle | `Δt·m` with `m = min(\|major\|, minor)`, certification's speed floor (`geom::carrier_speed::speed_floor`), through `Extent::forward` / the span against one turn through the turn rule (`Period::turn`'s row) at `Bounded(turn_defect_speed_lo, turn_defect_speed_hi)`: `interval_span_forward` / `interval_span_winding` re-decided at the flux lane | m | OK (certification's rates and band, hence its dispositions: only a definite `Positive` span admits forward; `Short` and `Whole` admit, `Over` refuses, a straddle of the two readings escalates. Refused as `ConeLoop`: an ellipse arc is no rim, so not under the rim's pair. Before it, an arc stored over `τ + θ` closed a band with a rim and two generators, turned once round the axis, and counted the whole section twice in the vector area. Reachable only from hand-built `LoopEdge`s; pinned in `tests/props_cone_incidence.rs`) |
| props/curved.rs (`require_one_chart_branch`'s cone arm) | props_cone_apex | the apex's own line parameter against the nearer span end, bare (a line's `t` IS arc length on the unit `dir` the parse certifies — the same argument `props_meridian_apex` makes one row up) | m | OK (added with the arc-branch door, issue 1571. Positive = the apex strictly inside the generator's stored span, where the chart `u` jumps to the mirror nappe and a walk reading one `u` per edge cannot read the edge; Zero and the indeterminate band ADMIT, so a generator ENDING at the apex — every `revolve` cone cap — passes. Signed as `min(t_apex − t0, t1 − t_apex)`: positive exactly on interior membership, and its magnitude is the metres to the nearer end, i.e. the point deviation of moving the apex onto the span boundary) |
| props/curved.rs (`require_rims_at_extremes`, through `level_coincides`) | props_rim_level | per-kind: bare level difference (cylinder/cone `Length`) / rooted (sin,cos) chord × `RimArms::level` (sphere ×R, torus ×minor) | m | **FIXED — N7 RETIRED** (N1 RETIRED earlier. Generalised from the torus-only site to all four kinds by S58/#649, and unified with its former sibling `props_rim_level_group` by S81 — ONE rule (`level_coincides`), one metric (the chord), one arm (`RimArms::level`), one fail direction; the sibling went with `du_of_rims`, so `level_coincides` now records under this one name, the shape door's (`require_iso_rectangle`), and the metering is still carried by [`RimLevel`]. N7's near-polar sphere understatement — an axial-only `(sin v, 0)` pair whose chord collapsed by `cos v̄`, merging distinct near-polar rims in the ACCEPTING direction on both names, this refusing one included — is retired by the full `(sin v, cos v)` pair (issue 893 / S-CERT; this verdict column previously said `OK` while N7's own prose recorded the collapse). Pinned as scale twins by `geom-brep/tests/rim_dim_scale_twins.rs` and, in suites CI runs, by `geom-brep/tests/s81_one_rim_level_rule.rs` and the near-polar rows of `geom-brep/tests/cert1_sphere_polar.rs`.) |
| props/quad/ (every quadrature lane's round, the spiric lane's too, each read through `quad::converged`) | props_quad_converged | ε·F − flux-width(m³)/(3·area(m²)) | m | **RETIRED** (QUAD measure door units B and C: a face's width is compared with the caller's stated resolution, `Settle::Resolution`, and not decided, so this mint is gone; the reporting target `QUAD_TARGET_LEN_FACTOR·ε` it read and `QuadratureBudget` are gone with it. The row is kept for the committed eras' samples, `k_lint::ERA_EPS_COUPLED_PREDICATES`.) |
| props/quad/run.rs | props_quad_face_extent | area/perimeter (mean width) | m | OK |
| props/quad/cylinder.rs (`loop_windings`, per junction of every loop of a cylinder quadrature face, through `closure::interval_loop_turns`) | props_chart_junction_closed | `excess(hi(G), S)` — `G = R·\|Δu − 2πm\| + \|Δv\|`, the chart gap between one edge's end and the next edge's start (each read through its own vertex's trig bracket), a whole number `m` of azimuth turns taken off, its UPPER bound in excess of the slack `S` (the two ends' `end_slack`: `3/2·env` plus the parameter bracket's half-width times the edge's speed bound; and `3ε` for the two carriers' ends about their vertex); `0` within it | m | OK (Zero, an excess under ε: every gap the enclosure admits is within `S + ε`, and the junction closes. In band escalates. Positive asks `props_chart_junction_open` below. The `3ε` vertex term is measured necessary: at ε = 1e-12 on the geom-brep, topo and sweep suites, 3428 real junctions read up to 1.99ε beyond their end slack alone, and none reads beyond `S + ε`. The loop's turns are then the integer `Σm`; `\|turns\| ≤ 1` and `Σ turns = 0` over the face are integer facts that decide nothing, refused `LoopsWindChart`) |
| props/quad/cylinder.rs (`loop_windings`, a junction whose upper excess is definitely positive) | props_chart_junction_open | `excess(lo(G), S)` — the same gap's LOWER bound in excess of the same slack, through `decide_positive` | m | OK (only a definite Positive refuses `ChartLoopOpen`: no gap the enclosure admits is within `S + Kε`. Zero or in band escalates, so a junction the enclosure cannot decide is never measured) |
| props/spiric.rs (`wall_windings`, per junction of a spiric torus wall's loop on its lifted images, through `closure::interval_loop_turns`) | props_chart_junction_closed / props_chart_junction_open | `excess(hi(G), S)` / `excess(lo(G), S)` — `G` the chart gap less whole turns in `u` and `v`, the azimuth's levered at the junction's own distance from the axis, `ρ = R + r·cos v` enclosed over the after edge's start, the tube angle's at `r`; `S` the two images' certificate envelopes and `3ε` | m | OK (the quadrature's rule on the torus chart. A wall carries one loop, so its turns `Σm` per direction must be `0`, refused `LoopsWindChart`) |
| props/quad/trimmed.rs (`piece_monotone`, asked once per trim piece by `trim_cells`) | props_trim_piece_monotone | `Margin::metered(span, rate)` — `span` is the control polygon's LEAST advance along the unit chord direction, a chart-parameter length (a convexity fact on the Bézier differences, which are the derivative's own coefficients up to the positive `p/h`); `rate` is the chart's metric rate along that same direction, `|S_u·ĉ_u + S_v·ĉ_v|` bounded BELOW over the piece box | m | OK (added with the trimmed-region quadrature, TRIM-2 PR-1. Clause (iii)'s metric door: a parameter-space span crosses to model space through a per-kind rate, and the rate here is the chart's own directional derivative rather than a carrier speed. Both factors round toward REFUSING — the span is the `lo()` of the bracketed dot product, and a derivative hull that straddles zero in every component answers rate `0`, hence margin `0`, hence Zero. Only a definite `Positive` admits the piece; `Zero` and the ambiguity band alike bisect the piece and re-ask, to `TRIM_MONOTONE_DEPTH`, and then refuse `QuadratureUnsupported` naming this row — the lane never pads a piece it cannot certify is a graph over its chord. **The margin carries the ROUND's lever as well as the image's, and a reader of this row has to know it**: `span` is the REFINED block's least advance, so it halves with every uniform cut and every bisection AND scales with the face. Measured (the v6 dual, both arms independently): the same smooth parabolic arc certifies at every round on a 10 µm face, refuses at round 4 on a 1 µm face, and refuses at every round on a 0.1 µm face — that is the lane's resolution speaking, not the image's shape, which is why the refusal names an UNRESOLVED margin rather than a cusp. Nothing on today's corpus is near the band (the fixture's scale is 1e-3 and round 0 is decades clear). An in-band verdict ESCALATES, as it does from every other lane in this file: bisection halves the very span the margin meters, so it cannot resolve one. Pinned by `geom-brep`'s `q5_a_fold_over_its_chord_refuses_typed` (a fold that survives every bisection) and `q5b_a_piece_that_only_looks_folded_resolves_by_bisection` (the curvature artefact the ladder is FOR — a refusal is cheap to get right by refusing everything).) |
| ssi/certify.rs (`analytic_limbs`, `nurbs_limbs`) | ssi_on_locus / hull_sup / foot / chart | residuals, /2R linearizations, foot distances | m | OK |
| ssi/certify.rs (`tube_transversality`) | ssi_tube_transversality | sin (unit triple product) × arm | m | OK |
| ssi/march.rs (`march`) | ssi_transversality_arm / ssi_transversality | arm (m); sin × arm | m | OK |
| ssi/section.rs (`boundary_section`) | ssi_boundary_on_plane | sup over a boundary curve of `|n·(C − p₀)|`, from its control net (the rational hull) | m | OK |
| ssi/section.rs (`decide_crossing`) | ssi_boundary_crossing | `|dφ/ds|` at a root (a sine: metres of plane distance per metre along the curve) × arm | m | OK |
| ssi/boundary.rs (`Pass::run`) | ssi_boundary_corner | `|n·(P_corner − p₀)|`, the corner control point's plane distance | m | OK |
| ssi/boundary.rs (`side_region`) | ssi_boundary_strip | inf of the wall's slope across a side (metres per chart unit) over the sup chart speed (a sine) × arm | m | OK |
| ssi/refine.rs (`halve`) | ssi_refine_halving | half a sample gap's chord, the step a halving would leave; where it falls in the band before a polyline has the fit's samples (`fit_minimum`), the sized `ShortBranchUncertified` | m | OK |
| ssi/ends.rs (`neither`) | ssi_short_branch | the distance between a branch's known ends over `SHORT_BRANCH_STEPS`, the step it would be marched at, where the march refused for want of step | m | OK |
| ssi/march.rs (`march`; `SlabExit`, the ℝ³ lane, for the open end) | ssi_step_progress / branch_open_end / closure_return | state × (m/state); scaled domain margins | m | OK |
| ssi/march.rs (`march`) | ssi_closure_tangent | cos(unit tangents) × whole-branch arc length | m | FLAG F9 |
| locus.rs (M9-2 PR-2) | pc_axis_plane_parallel / cc_axes_parallel (the witness lane's reads of intersect.rs's rows) | sin(axis, plane / axis, axis) × the declared pair's consumed extent read from the foot of its centre on the cylinder's axis, where the gap row is read (`ExtentBall::lever_from`); for the cylinder pair, the lesser of the two feet's levers (`ExtentBall::lever_between`, through `cylinder_axes_parallel`) | m | FIXED (TANG; was a 1 m `T::one()` arm, which bridged a tilt standing more than Kε off across a face longer than a metre; the lane's own `tangent_locus_axis_parallel` row retired into the section's) |
| locus.rs (M9-2 PR-2) | pc_parallel_gap / cc_parallel_gap / tangent_locus_internal_gap (each with its `_floor`) / tangent_locus_side | r − axis-to-plane distance at the foot; r1 + r2 − axis-to-axis distance; \|r1 − r2\| − axis-to-axis distance — each summed with its tilt row's sin×lever (`decide_across`); axis offset / radius difference — all metre data of the carriers | m | OK (the plane×cylinder and external-cylinder gaps are the section classifiers' rows; the internal gap and side rows are the lane's own) |
| locus.rs (`Section`) | tangent_locus_across / tangent_locus_face_clear | the cross-section's direction: `r·\|a × n\|` (plane×cylinder) or the axes' offset `\|d_vec\|` (parallel cylinders); a face's least distance to the other carrier across the locus, from its padded extent along that direction, less the tilt's displacement across the reach — metre data of the carriers and the faces | m | OK |

## topo

| site | predicate | comparand | dim | status |
|---|---|---|---|---|
| boolean/contact_verify.rs (`touching`, `tangent_locus_reading`) | contact_tangent_gap / contact_tangent_touch | the pair's separation `ε₁r₁ + ε₂r₂` (each implicit residual, /2r-normalized, read as the witness's height above its face); its sampled max plus `TangentSpanBounds::gap_sag`, as `tangent_gap_hull_sup` | m | OK |
| boolean/contain.rs (`boundary_pre_pass`) | bool_contact_vertex / bool_contact_edge_length / bool_contact_edge | point distance; a straight edge's own length (the degeneracy gate) and the distance from the point to its closed segment, through `ray_walk::on_segment` | m | OK (CONTACT-4) |
| boolean/contain.rs (`boundary_pre_pass`, a conic's `End`) | bool_contact_arc_end_vertex | the distance from a conic edge's carrier end to a stored vertex, a norm (`decide_magnitude_reported`) — any in-band value escalates; with all four decided, the point at the end is the finding `ContainError::ArcEndClearOfCorners`, carrying them. The ends are read through `Curve3::eval`, as certification reads them, so at the run's band the finding is the band's own parting (`K < 2`); an end off both corners is read at a caller's band finer than the run's, or where the certifier's distance and this reading part by rounding across a band narrower than it (`K` near one), which the census reports as a defect (`ValidationError::ArcEndOffCorner`) | m | OK (CONTACT-4) |
| boolean/contain.rs (`boundary_pre_pass`, through `splitting::containment::LoopEdge::contact`) | bool_contact_arc_span | `(τ − w)` through the turn rule (`Period::turn`), as `ConicArc::of` reads it (`point_in_arc_loop_conic_span`'s row). Where the point sits against the arc is `ConicArc::hit`: `geom`'s on-carrier reading at the point's foot (`curve_carrier_gap`, the gap at the foot, for ON; `curve_carrier_clear` for OFF, a lower bound on the distance from the whole conic, exact on a circle and the Hessian-bounded root `2|F|/(g + √(g² + 4|F|/b²))` on an ellipse, the two straddling on `curve_carrier_straddle`), and the span's reading of its ends (`curve_pair_span_end_chord`, the raw chord to an end's point; `_side`, the foot's parameter distance from the end metered by the greatest speed) | m | OK (CONTACT-4) |
| boolean/sphere_region.rs (`SphereFaceRegion::contains`'s boundary pass, through `splitting::containment::ConicArc::hit` on the arc's circle) | bool_sphere_region_arc_span | `(τ − w)` through the turn rule (`Period::turn`), as `ConicArc::of` reads it (`point_in_arc_loop_conic_span`'s row); where the point sits against the arc is `ConicArc::hit`'s two readings (the row above) | m | OK |
| boolean/sphere_region.rs (the rays) | bool_sphere_region_arm / bool_sphere_region_span / bool_sphere_region_at / bool_sphere_region_order / bool_sphere_region_cross | a ray direction's tangential fraction `\|raw\|/\|toward\|` × the sphere's radius; a root's parameter less a span end × the arc's radius (arc length); a crossing's angle along the great circle × the sphere's radius (geodesic length), and two such lengths' difference; the heading's sine against the face side × the sphere's radius | m | OK |
| boolean/sphere_region.rs (`ray_roots`, through `circle_roots::first_harmonic_roots_in_band`) | bool_sphere_region_roots_noise / _coaxial / _extreme / _slack | the first-harmonic door's rows, as `bool_conic_quadric_first_*`'s: the arc's signed distance off the ray's plane through the centre, `ĝ·(C − c) + ρ(ĝ·û) cos θ + ρ(ĝ·v̂) sin θ`, its rounding charge, its extremes, and a root's arc slack at the arc's radius | m | OK |
| boolean/torus_region.rs (`TorusFaceRegion::contains`'s boundary pass, through `splitting::containment::ConicArc::hit` on a circle edge and `PiecedArc::contact` on an oval arc) | bool_torus_region_arc_span / bool_torus_region_spiric_end / _clear / _on / _leaf | `(τ − w)` through the turn rule (`Period::turn`), as `ConicArc::of` reads it (`point_in_arc_loop_conic_span`'s row); an oval arc's end distance, a piece ball's clearance, a point distance and a ball radius | m | OK |
| boolean/torus_region.rs (the paths) | bool_torus_frame_radius / bool_torus_region_family / bool_torus_region_component / bool_torus_region_span / bool_torus_region_at / bool_torus_region_corner / bool_torus_region_order / bool_torus_region_cross | the point's distance from the axis; a circle edge's axis sine × its radius and its radius less `R` (class selection); a root's separation from the plane's other curve (`(y − c)·ρ̂` on a meridian plane, `ρ − R` on a parallel plane, a circle's distance from its mirror and from itself, `(y − c)·m` on an oval's plane), and an edge in a leg's plane against the leg's own circle (a meridian's centre against `ρ̂`, a parallel's radius less the leg's); a root's parameter less a span end × the circle's radius or the tube's (arc length, or a lower bound on it); a crossing's angle along its leg × the leg's radius, against the leg's start, its corner, or another crossing; the heading's sine against the face side × the tube's radius | m | OK |
| boolean/torus_region.rs (`circle_roots`, `plane_roots`, through `circle_roots::first_harmonic_roots_in_band`) | bool_torus_region_roots_noise / _coaxial / _extreme / _slack | the first-harmonic door's rows: a circle edge's signed distance off a meridian leg's plane in its own parameter, or a leg's signed distance off an edge's plane in the leg's parameter; its rounding charge, its extremes, and a root's arc slack at the circle's radius | m | OK |
| boolean/insert.rs (`strut_order`) | bool_strut_side | `n̂·(ĝ × ê)`: the sine of a strut germ's angle from the corner's arrival bound (the arrival edge's departure: a line's chord, a curved edge's tangent at the vertex), × that bound's reach (a line's length, a curved edge's extent from below) — its distance at that reach from the bound's line; a decided zero is placed by `bool_dir_same` (`sectors::direction_sense`) | m | OK (JOIN reflex corner) |
| boolean/insert.rs (`strut_order`) | bool_strut_order | `n̂·(ĝ₀ × ĝ₁)`: the sine of the angle between two strut germs in one half-turn, × the shared entry's arm; a decided zero is the caller's tie, read one order down where the caller can (`geom_brep::tied_germ_turn`, rows `tangent_sector_order2{,_arm}`) and refused (`decide_nonzero`'s reading) where it cannot or they curve alike | m | FIXED (JOIN reflex corner: was the cosine difference `(ĝ₀ − ĝ₁)·ê` × arm, second order in the spacing beside 0 and π, with a decided zero read as an order; before that, dimensionless) |
| boolean/vtxfac.rs (`germ_order`) | bool_strut_side | `n̂·(ĝ × ĝ₀)`: the sine of a pierce germ's angle from the ring's first germ, × each arm a reading of it uses (its own pair's, the shorter of the two arms, and every comparison's, the shortest of three) — its distance at that arm from the first germ's line; in band with its sense along (`bool_dir_same`) at any of them, the two germs coincide and it refuses as a `Coincide::Sectors` coincidence, unless the two are of one pierce: then, as two germs of one pierce along any one direction, they are ordered one order down by their sections' bends in the face (`geom_brep::implicit_section_acceleration`, `tied_germ_turn`), and two of different pierces by their chords' nesting; the order itself is `strut_order`'s rows above | m | OK |
| boolean/insert.rs (`walks_after`) | bool_shared_cut_order | (unit cut dir × cut dir)·(unit sector normal) × sector arm, the second direction unit or, from `beside_order`, the difference `oj − oi` of two unit flanker directions (magnitude up to 2, so the reading is the first-order term of `d + δ oi` against `d + δ oj`, up to that bounded factor) — two crossing pairs' cuts in one corner of a vertex both cut, ordered along the corner; a decided zero (the cuts along one direction, checked one ray by `bool_dir_same`) is placed by the runs: held only when the other pair's run leaves the direction the same way and is the same arc (`tied_held`), and not when the two runs are struts one of which holds the other whole (`holds_whole`); and an in-band reading refuses as a `Coincide::Sectors` coincidence, as `bool_strut_order` does | m | OK |
| boolean/insert.rs (`beside_order`) | bool_beside_flush | `n̂·b̂`: the sine of a unit flanker direction's tilt out of the sector's plane, × the sector arm — the flanker's far bound's distance at that arm from the plane; a decided zero is the flanker flush with the sector, the side a germ along the other solid's edge lies on, and two germs along one direction are then ordered by `bool_shared_cut_order` of `d` against the difference of their sides; an in-band reading refuses as a `Coincide::Sectors` coincidence | m | OK |
| boolean/insert.rs (`germ_dir`) | bool_germ_line | sin(n̂_a,n̂_b) × min sector arm — the margin `pair_search` read definite before it recorded the pair as a crossing | m | OK |
| boolean/join.rs:567/803 | bool_join_chord | germ-site chord LENGTH (the degeneracy gate: Zero ⇒ coincident sites, no polygon edge) | m | OK |
| boolean/join.rs (`nearer_along`) | bool_join_chord | the chord LENGTH between two candidate partner sites of one germ (Zero ⇒ one site: the two records take the chord order, not the travel) | m | OK |
| boolean/join.rs (`nearer`) | bool_join_nearest | a DIFFERENCE of two chord lengths (which germ's nearest pair joins first; within one germ, the tie-break of two records at one site) | m | OK |
| boolean/join.rs (`germ_arm`, through `travel`) | bool_join_arc_ahead | the side of the plane through the section's axis and the germ's site the partner site lies on, in the germ's sense, read as the arc along the conic: `turn_of(p − site)` over `\|turn_of(t̂)\|`, `t̂` the conic's unit tangent at the site (a metre vector projected across a metre radius, over a unit tangent projected the same way) | m | OK |
| boolean/join.rs (`nearer_along`, through `travel`) | bool_join_arc_travel | the same arc read from the incumbent partner's site: which of two sites in one half-turn the germ reaches first | m | OK |
| boolean/join.rs (`travel`) | bool_join_arc_slack | `off_conic_slack`: `2ε cot ψ` at the site, two sites each up to ε across the conic (`Margin::levered`, `cot ψ` dimensionless, `ε` the arm) | m | OK |
| boolean/join.rs (`travel`) | bool_join_arc_clear | the travel margin's size less that slack: a difference of two lengths | m | OK |
| boolean/join.rs:743/744 | bool_join_facing | unit germ dir · chord (cos × separation) | m | FIXED (was bare cosine, `/dist`) |
| boolean/join.rs (`rotational_sense`) | bool_join_arc_facing | axis·((p−c)×dir) — radius-metered sine | m | OK |
| boolean/join.rs (`SectionLoops::branch_at`) | bool_join_section_loop | which loop of a plane×torus section's two a germ site lies on, decided where the germ is minted: `(p − o)·d̂` (the side of a plane between the loops, `d̂` unit) or `‖(p − o) × â‖ − R` (the distance from an axis against a radius, `â` unit) | m | OK |
| boolean/join.rs (`SectionLoops::branch_at`) | bool_germ_bisector_branch | which bisector-plane ellipse of an equal cylinder pair with meeting axes a germ site lies on: `|(p − o)·ŝ| − |(p − o)·d̂|`, `ŝ = unit(â₁ + â₂)` and `d̂ = unit(â₁ − â₂)` the two planes' normals, so each term is the site's distance from a plane; Zero only near the nodes, where the ellipses cross | m | OK |
| boolean/vtxfac.rs (`mint_node`) | bool_node_wall_sense | which side of its wall a face's material is at a node: the face's unit outward normal against the radial `node − p`, `±r` | m | OK |
| boolean/vtxfac.rs (`mint_node`) | bool_node_tangent_toward_axis | which way a node's arc turns toward the axis its runs are about: `t̂·â`, levered at the walls' radius `r` (`r·cos(θ/2)` or `r·sin(θ/2)`); Zero refuses `NodeUnbuilt` | m (levered) | OK |
| boolean/vtxfac.rs (`mint_node`) | bool_node_run_turn | which arc a node's run is entered by, turning about B's outward normal: `(ĝ₁ × ĝ₂)·n̂`, levered at `r`; the arcs cross square, so `±r` | m (levered) | OK |
| chord_join.rs (`section_loop`) | chord_section_loop | which loop of a plane×torus section's two a split-lane chord's sites lie on, by the split both lanes build for the section (`LoopSplit::plane_torus`): `(p − o)·d̂` or `‖(p − o) × â‖ − R`; a boolean chord takes its germ's branch and reads no margin | m | OK |
| boolean/join.rs (`cylinder_pair_axes`: the frame dispatch `pair_section_frame` and the kind arm `segment_arm`, cylinder pair) | cc_axes_parallel (retired `bool_germ_frame_axes_parallel`) | the section table's own row, asked through `geom_brep::cylinder_axes_parallel`: `‖a₁ × a₂‖` levered at the larger radius or the germ walls' span, whichever is longer, both callers from the same point and span (`frame_lever`, `Reach::Measured`) | m | FIXED (TANG; one name for one fact) |
| boolean/join.rs (`pair_section_frame`, cylinder pair) | cc_axes_coplanar (retired `bool_germ_frame_axes_coplanar`) | the section table's own row, asked through `geom_brep::cylinder_axes_coplanar`: the signed axis-to-axis gap along the common perpendicular, `w·(a₁ × a₂) / ‖a₁ × a₂‖`, `w` between the axes' feet at the reach | m | FIXED (TANG; was read between the stored origins, `o₂ − o₁`, under a second name) |
| boolean/join.rs (`along_facing`) | bool_join_foot_heading | unit germ dir · the carrier's unit tangent at the site's foot, levered by the section's own scale (`CarriedSection::lever`, the cylinder's radius for cylinder × sphere), so a germ's heading is its own and read once per operation (`SectionTable::germ_foot`) | m (levered) | OK |
| boolean/join.rs (`along_facing`) | bool_join_foot_ahead | the carrier parameter from one site's foot to the other's, the way the germ leaves, metered by the carrier's speed at the germ's foot | m | OK |
| boolean/join.rs (`nearer_along`) | bool_join_foot_travel | a DIFFERENCE of two such parameters, metered by that speed: which partner's foot the germ reaches first | m | OK |
| boolean/join.rs (`parallel_radical_plane`) | bool_join_cc_axis_offset | the axis-to-axis offset of a parallel cylinder pair, `‖Δ − a₁(Δ·a₁)‖` with `Δ` between the axes' feet at the germ sites (`parallel_axes_at`) and `a₁` unit — the radical plane normal's length before it is normalized (`UnitVec3::new`); only a definite offset names a plane | m | OK |
| boolean/join.rs:1093 | bool_ring_run_winding | (n̂ · Newell sum) / run perimeter — 2A/P, the run's mean width | m | FIXED (F4; was a bare **m² AREA**) |
| boolean/sectors.rs (`cone_side`, `arc_side`, `in_sector`, `apart`, `cone_read`) | bool_cone_arc / bool_cone_arc_span / bool_cone_within / bool_cone_facing / bool_cone_apart / bool_cone_pointed | a polygon cone's reading of a direction: `bool_cone_arc` is `det(d̂,p̂,b̂)` over `\|p̂×b̂\|/L_d + \|d̂×p̂\|/L_b + \|d̂×b̂\|/arm₀` — the least joint displacement of the direction's far point, the bound's far point and the reference levered at its arm that flips it (`least_lever`); `bool_cone_arc_span` the sine between `p` and that bound × the shorter of the two sectors' arms, which gates that plane's normal; `bool_cone_within` the sine of a direction past each of a sector's bound planes, and `bool_cone_facing` its cosine to the sector's middle, each levered at the least joint deviation of the points it reads (`least_lever`, `1/Σ(rate/L)`): the read point at its reach (the direction's, or for a crossing point the shorter of it and how well the point is located, `\|x\|·arm₀·L_d/(arm₀ + L_d)`) and each bound compared against at its own reach; `bool_cone_apart` the sine `(a×b)·n` of two of a sector's bounds, the direction and the reference, in that sector's plane, less eight ulp of rounding, levered at the least joint deviation of the two points at their reaches (the reference at its arm); `bool_cone_pointed` the length of the link's mean unit bound × the shortest sector arm, which gates reading `−c` | m | OK (TANG polygon cones) |
| boolean/ops.rs:1194–1480 | bool_sphere_* | radius/gap differences; sin × radius | m | OK |
| boolean/plane_eq.rs:174/233 | bool_plane_parallel | sin(n̂1,n̂2) × arm | m | OK |
| boolean/plane_eq.rs:190/252 | bool_plane_orient | cos(n̂1,n̂2) × arm | m | FIXED (was bare cosine) |
| boolean/plane_eq.rs:203/265 | bool_plane_offset | signed-offset difference | m | OK |
| boolean/carrier_eq.rs (`at_consumed_extent`, the undeclared posture through `rest::carrier_pair_verdict`) | bool_plane_parallel / bool_plane_orient / carrier_cyl_axis_parallel at the carrier-pair door | sin (cos) of the two normals or axes × the farthest reach of a ball enclosing both faces from the pivot the kind's position datum is read at, re-anchored nearest the ball's centre: the centre itself (plane, offsets read there), the foot of the centre on the second axis (cylinder) | m | FIXED (TANG; was a 1 m `T::one()` arm) |
| boolean/carrier_eq.rs (`declared_reading`) | bool_plane_reach / carrier_sphere_reach / carrier_cyl_reach / tt_coincident (the torus pair's bound, `geom_brep::TorusDisplacement`) | the declared pair's displacement over its consumed extent, bounded above at every point of the ball: the position datum read at the pivot + the axes' or normals' chord `\|a₁ − σa₂\|` × the reach from it (a levered value, through `Margin::levered`) + the radius differences, summed; a sum of lengths | m | OK (TANG; replaces deciding each datum on its own, which bridged nearly twice the band) |
| boolean/carrier_eq.rs (`declared_reading`) | bool_plane_reach_floor / carrier_*_reach_floor / tt_coincident_floor | the larger of the radius difference less the rest of the upper bound (a bound across the whole ball) and each face vertex's distance from the other carrier less its own; floored at zero | m | OK (TANG) |
| geom-brep/src/pair_section/torus_coincidence.rs (`tori_coincide`) | tt_coincident / carrier_torus_axis_parallel / carrier_torus_center / carrier_torus_major_radius / carrier_torus_minor_radius / tt_coincident_straddle | the bound on two ring tori's distance (centre separation + major-radius difference + the axes' chord × the larger major radius + minor-radius difference), and each datum alone (the tilt as `\|a₁ × a₂\|` × the larger major radius), read by the torus pose and the carrier ladder | m | OK |
| boolean/recl.rs:224–748 | side_code / bool_dir_same / bool_ee_collinear | side_code as in the sectors.rs row (`flank_key`, edge-edge membership); cos/sin × sector arms for the rest | m | OK |
| boolean/recl.rs (`wedge_is_reflex`) | bool_wedge_reflex | `n̂₀·r̂₁`: the sine of an edge-edge site's dihedral wedge angle (the second flanker's projected representative against the first flanker's outward normal), × min sector arm — the second face's distance at that arm from the first face's plane; asked only when a representative splits the two planes' verdicts, and a decided zero refuses (`decide_nonzero`) | m | OK (JOIN reflex wedge) |
| boolean/recl.rs (`flank_rep`) | bool_flank_offset | `\|v̂ − â(v̂·â)\|`: a flanking bound's component perpendicular to the edge-edge site's common line (the sine of its angle off the line), × the bound's own reach (`Reach::length`: a line edge's chord, a curved edge's extent, a bisector's arm) — the bound's far end's distance off the line, where `side_code` reads it; a decided zero refuses (`decide_positive`) | m | OK (CLEAVE flanker representative) |
| boolean/recl.rs (`resolve_bisector_graze`) | bool_sector_bisector_side | refusal only: a grazing bisector between keys definitely on one side, reachable only at K ≤ 2 (argument at `vtxfac`'s on-edge resolution); the nonzero gate's rejection (`k_stats::gate_rejection`) of the `On` code's Zero, quoting `±zero` | — | OK (CONTACT-9) |
| boolean/reduce.rs:548–802 | bool_vertex_face_side / circle & line clearances | plane residuals, /2r residual extremes, sagitta dips | m | OK |
| boolean/reduce.rs (`bool_conic_curved_clearance`'s ARC half) | bool_conic_curved_clearance | `geom_brep::conic_arc_residual_range`: a hull of `implicit_residual` samples (m) widened by the chord-dip charge `f2·h²/8`, `f2` in **m/rad²** and `h` in rad — a residual's second derivative with respect to an ANGLE, so the product is a length and the comparand stays a length | m | OK |
| geom-brep/implicit.rs (`circle_residual_curvature_bound`, `circle_arc_residual_range`) | — (no funnel call; the comparand is built here and decided at the row above) | `f2` is `|(d²)″|/2r`: `(d²)″` is m²/rad² over the `2r` linearization, giving **m/rad²**; the returned range is m | m/rad² and m | OK |
| boolean/conic_quadric (`conic_quadric_roots`, the arm switch) | bool_conic_quadric_second_harmonic | `A₂ = \|(c₂, s₂)\|`, the second harmonic's amplitude of the `/2r` residual (`geom_brep::ConicHarmonics`); on a circle against a wall `(ρ·sin α)²/4r` | m | OK |
| boolean/conic_quadric (first-harmonic arm, through `circle_roots::first_harmonic_roots`; not a circle against a sphere) | bool_conic_quadric_first_noise / _first_constant / _first_extreme / _first_root_slack | the harmonics are the `/2r` residual itself (`geom_brep::conic_cylinder_harmonics`, `conic_sphere_harmonics`): noise = rounding of the m² term bound over `2r`, plus the dropped second harmonic `A₂` (m); `A₁`; the extremes `c₀ ∓ A₁`, and a constant residual's `c ∓ s` with its spread `s = A₁ + noise` (`constant_residual_roots`); the root's arc slack `speed_hi·noise/√(A₁² − c₀²)`, m·m/m | m | OK |
| boolean/conic_quadric (first-harmonic arm, a circle against a sphere, `A₂ ≡ 0`) | bool_conic_quadric_first_noise / _first_constant / _first_extreme / _first_root_slack | the extremes are the `/2r` residual's own (`geom_brep::circle_sphere_harmonic`): noise = the larger of the extremes' error bounds, each a running rounding bound plus the frame's orthonormality charge `(|e|² + ρ²)·‖GᵀG − I‖/2r` (m); `A₁ = (hi − lo)/2`; the factored extremes `(D∓ − r)(D∓ + r)/2r`, and a constant residual's `c ∓ s` with `s = A₁ + noise`; the root's arc slack `ρ·(δR/√(−lo·hi) + δφ + τ·NOISE_ULPS·u/2)`, `δR = (hi·δlo − lo·δhi)/(hi − lo)` (m) and `δφ` the phase's error (rad) | m | OK |
| boolean/conic_quadric (ladder, through `geom_brep::half_angle::half_angle_roots`) | bool_conic_quadric_ladder_noise / _pole / _pole_conditioning / _sub_clear / _sub_monotone / _sub_side / _sub_width | `noise / f_per_metre` with `F` the residual itself (`f_per_metre = 1`); the residual at the pole; `b·(\|F(pole)\| − κA)/A`, the semi-minor speed times a ratio of residuals; the subdivision's residual bounds and piece widths at the top speed (m) | m | OK |
| boolean/conic_quadric (ladder's quartic, `geom_brep::quartic::depressed_quartic_roots`) | bool_conic_quadric_disc / _shape / _depth / _odd / _split / _split_lead | the ladder's coefficients in the root variable `τ = 2b·t` (a length; a circle's `2ρ·t`), each over the power of the lever `2b` that makes it a length (`Δ`/lever¹¹, `p`/lever, `D`/lever³, `q̂`/lever², factor discriminants/lever) | m | OK |
| boolean/circle_torus.rs (coaxial arm, through `circle_roots::constant_residual_roots`) | bool_circle_torus_coaxial_residual | the torus `implicit_residual` at `θ = 0`, `(g² − r²)/2r`, `∓` its spread `2δ(g₀ + 2δ)/r` with `δ = offset + √2·tilt` (both m), a length times a ratio of lengths | m | OK |
| boolean/ellipse_torus.rs (through `geom_brep::half_angle::certified_subdivision` at degree 4) | bool_ellipse_torus_sub_clear / _sub_monotone / _sub_side / _sub_width | the torus implicit `F` (m⁴, `geom_brep::ConicTorusHarmonics`) less its Taylor bounds, over the near-surface ceiling `2r((2R + 2r)² + 3r²)` (m³ per metre of residual) and capped at `r/2` (m): a lower bound on the residual in metres; the least slope over the most bend (rad) at the carrier's top speed `max(\|a\|, \|b\|)` (m/rad); the torus `implicit_residual` at a piece end or a root; a piece's width (rad) at the top speed | m | OK |
| boolean/ellipse_torus.rs (`geom_brep::half_angle::RootSlack`) | bool_ellipse_torus_root_slack | `speed_hi · f_hi · (\|res\| + δres) / L`: the residual at the root and its running rounding bound (m, `geom_brep::conic_torus_residual`), times the smaller of the carrier's ceiling `2r(\|C₀ − c\| + ‖C − C₀‖ + R)²` and the surface's `2r((2R + 2r)² + 3r²)` on `\|F\|` per metre of residual (m³), over `F`'s least slope near the root `L` (m⁴/rad), at the top speed (m/rad) | m | OK |
| boolean/circle_torus.rs (`geom_brep::half_angle::RootSlack`, the general arm) | bool_circle_torus_sub_root_slack | `speed_hi · (\|F\| + δF) / L`: the torus's quartic `F = ((ρ − R)² + h² − r²)((ρ + R)² + h² − r²)` at the root and its running rounding bound (m⁴, `geom_brep::conic_torus_implicit`), read directly (ceiling `1`), over `F`'s least slope near the root `L` (m⁴/rad), at the top speed `ρ` (m/rad) | m | OK |
| boolean/conic_quadric (the cone arm, both arms through `circle_roots`) | bool_conic_quadric_* (as for a wall) | the cone's quadric form over the carrier's reach from the apex, `F = (cos²α·ρ² − sin²α·h²)/R` (`geom_brep::conic_cone_harmonics`), `\|F\| ≤ \|res\|`: every definite reading is a lower bound on the residual's (m); the ladder's noise meter reads `noise / floor`, `floor = min(sin α, cos α)·d/R` | m | OK |
| boolean/conic_quadric (`geom_brep::half_angle::RootSlack`, the cone arm) | bool_conic_cone_root_slack | `speed_hi · (\|res\| + δres) / L`: the cone's residual at the root and its running rounding bound (m, `geom_brep::conic_cone_residual`), the ceiling on `\|F\|` per metre of residual being `1`, over `F`'s least slope near the root `L` (m/rad), at the top speed (m/rad) | m | OK |
| boolean/conic_quadric (`geom_brep::half_angle::RootSlack`, the ladder arm on a sphere or a wall) | bool_conic_quadric_sub_root_slack | `speed_hi · (\|res\| + δres) / L`: the `/2r` residual at the root and its running rounding bound (m, `geom_brep::conic_quadric_residual`), `F` being that residual (ceiling `1`), over `F`'s least slope near the root `L` (m/rad), at the top speed (m/rad) | m | OK |
| boolean/conic_quadric (the cone arm, an on-surface answer) | bool_conic_cone_on_surface | `(max(\|lo\|, \|hi\|) + noise) / floor`: a constant `F`'s reach read through the floor on `\|F\|` per metre of residual, an upper bound on the residual's magnitude (m) | m | OK |
| boolean/conic_quadric (the cone arm, each certified root) | bool_conic_cone_apex | `\|C(θ) − apex\|`, the root's distance from the apex (m, `Margin::norm3`) | m | OK |
| boolean/reduce.rs (`line_cone_roots`) | bool_line_cone_apex | the edge segment's distance from the apex: the line's foot on the apex clamped to the span (m, `Margin::norm3`) | m | OK |
| boolean/reduce.rs (`line_cone_roots`) | bool_line_cone_lead | `A/\|dir\|² = cos²(∠(dir, â)) − cos²α` levered by the segment's length (dimensionless × m) | m | OK |
| boolean/reduce.rs (`line_cone_roots`) | bool_line_cone_depth | `disc/(\|A\|·R)`: `−disc/A` is the negated quadric form `G` at the vertex (m²), over `R ≥ \|q(t*)\|`, a ceiling on the form's factor `ρ cos α + \|h\| sin α` there: a lower bound on the residual's magnitude at the vertex (m) | m | OK |
| boolean/reduce.rs (`line_cone_roots`) | bool_line_cone_root_slack | `\|dir\|·NOISE_ULPS·u/2·(\|w₀\| + \|dir\|\|t\|)²/(2√disc)` less the root's arc distance outside the span: `G`'s rounding (m²) over `\|G′\|` (m²/param) at the line's speed (m/param) | m | OK |
| boolean/reduce.rs (`wall_crossing`, a cone face's roots) | bool_cone_root_nappe | the root's elevation off the face's own nappe (`geom_brep::cone_elevation` with that nappe), a length | m | OK |
| boolean/arcs.rs (`arcs_along`) | bool_arc_along / bool_arc_ahead | `\|t̂ × d̂\|` and `t̂ · d̂` of two unit tangents, each levered at the arc's radius (`Margin::levered`): sin and cos of the angle between them × radius, the offset the arc's far side swings by | m | OK |
| boolean/reduce.rs (`arc_chain_reaches`, `boundary_meets_circle_only_at`) | bool_arc_chain_on_circle / bool_arc_boundary_off_circle | a point's distance from a circle, `√(h² + (ρ − r)²)` (`arcs::circle_miss`): its height over the circle's plane and its axial distance minus the radius, both m | m | OK |
| boolean/reduce.rs (`boundary_meets_circle_only_at`) | bool_arc_plane_side | a vertex's signed height over the circle's plane, `(p − c) · â` with `â` unit; a conic's own plane offset from it (`ConicPlaneMeet::Parallel`) | m | OK |
| boolean/sectors.rs (`on_planar_face`, read by `runs_in`) | bool_germ_tie_plane | a point's signed height over a face's plane, `(p − o) · n̂` with `n̂` unit | m | OK |
| boolean/sectors.rs, fate.rs, slit.rs (`geom_brep::parting`: read by `runs_into`, `crescent`, fate.rs's `corner_holds` and slit.rs's `walls_inside`) | bool_sector_runs_into_bend / bool_sector_crescent / bool_fate_corner_bend / bool_fate_corner_bend_straddle / bool_rider_wall_bend_across | `(κ − κ_b)·û·l²/2`: two curvature vectors at a vertex where their curves leave along one tangent, across it toward the side read, at a length `l` the site sets: `runs_into` an edge against a sector bound, at the shorter of their exact reaches; `crescent` two sectors' bounds, at the shorter bound's reach; `walls_inside` the floor's and the wall's normal sections across the line an edge rides, at the wall's reach; `corner_holds` a kept edge against a dropped face's corner bound, at the lever that bound's first-order side is read at, at both its bounds where a reach is known only from both sides and their straddle under `bool_fate_corner_bend_straddle` | m | OK (one reading, one name per site: each is its own decision with its own lever, and a refusal's diagnosis and `k_stats` name the site) |
| boolean/fate.rs (`corner_holds`, `tip_wraps`, through `sign_at`; `bisector` and `tip_wraps`' half-turn count, through `sign`) | bool_fate_corner / bool_fate_corner_straddle | `n̂ · (a × b)` and `a · b` of two unit directions at a fate cut's corner (the sine and cosine of their angle), levered at the shorter of their reaches (`sectors::reading_lever`), at both bounds where a reach is known only from both sides and their straddle under `bool_fate_corner_straddle`; a sector's bisector sum's length, and a loop's half-turn count against its corners', dimensionless, each levered at 1 m | m | OK |
| boolean/reduce.rs (`sheets_part`) | bool_touch_sheets_part | `û · v̂` of two unit directions into two tangent faces at a touch of their boundaries (`stands::inward`, each about its face's outward normal), a cosine levered at the longer of the two faces' reaches from their edges (`wedge_reach`, m) | m | OK |
| boolean/join.rs (`stands_on`) | bool_rim_face_opposed | `n̂ · n̂_f` of a closed rim's coplanar planar face and the face it meets at a boundary vertex, unit outward normals, a cosine levered at the rim face's reach from the rim (`wedge_reach`, m) | m | OK |
| boolean/sectors.rs:342–433 | bool_sector_within / bool_dir_* / bool_faces_parallel | sin/cos × sector arm (arm = shorter bounding chord, m; every caller passes unit dirs — verified); a pair `bool_faces_parallel` reads Zero is a near-coincidence and goes to the carrier ladder with every code On (its arm-setting bound reads On or in band) | m | OK |
| boolean/sectors.rs (`wedge_classes`) | bool_corner_one_plane | the chord `\|n̂ − n̂′\|` between two faces' unit outward normals at the vertex × the farthest reach the reading classes (the classed edges' reaches and the corner's sectors' spans, m): how far apart the two planes stand there, opposed normals two apart; a Zero lets a bound On or in band of that plane vote nothing on the corner's convexity, and anything else reads the corner as a polygon cone | m | OK |
| boolean/sectors.rs (`side_code`) | bool_chord_side / enters_material / bool_pierce_sector_side_curved | a LINE bound: its far vertex's signed distance from the plane through the base vertex (`sector_shape::plane_offset`); a curved bound: cos × its own extent; a bisector: cos × its sector's arm; the curvature charge: the bound's tangent RAY's least separation from the face past the sagitta, `slope·l − l²/lever` at `l = min(slope·lever/2, reach)`, where it peaks within the bound's own reach (a statement about the ray for every reach kind, not a point of a curved edge or a bisector) | m | FIXED (CONTACT-9; every bound was cos × the shorter sector arm, so a long line edge read On while its far end stood hundreds of bands off) |
| geom-brep/src/conic_bend.rs (`conic_bend_side`; the one bound-side reading's descent, `topo::bound_side::Tie::descend`, reads it for the boolean's vertex-on-face pass and the splitting lane's `neighborhood` alike) | conic_flat / conic_bend / conic_bend_side | a conic bound tied with a plane at first order: the whole edge's height bound `|t̂·n̂|·reach + |N̂·n̂|·rise` (sines × the carrier's point-set radius from the vertex and its rise off the tangent line there); the bend's certified lift less the first-order residue, `|N̂·n̂|·2·sin²(κ_min·l/2)/κ_min − |t̂·n̂|·l` at `l = min(max(extent, arc), lifts_to)` (`arc` a lower bound on the edge's arc length; `lifts_to` a half turn on a circle, a quarter turn at `κ_max` on an ellipse); and its side, that signed lift | m | OK |
| boolean/solid_contain.rs (`FaceGeo::PartialWall`, `FaceGeo::PartialCone`) | bool_wall_outline_reach / bool_cone_partial_reach | distance to the ball's centre − its radius | m | OK |
| boolean/ruled_region.rs (`RuledFaceRegion::contains`'s boundary pass, through `splitting::containment::ConicArc::hit` on a circle or ellipse edge and `ray_walk::on_segment` on a line edge) | bool_ruled_region_arc_span / bool_ruled_region_edge_length / bool_ruled_region_edge | `(τ − w)` through the turn rule (`Period::turn`), as `ConicArc::of` reads it (`point_in_arc_loop_conic_span`'s row); a segment's length; a point's distance from a closed segment | m | OK |
| boolean/ruled_region.rs (the paths) | bool_ruled_region_frame / bool_ruled_region_nappe / bool_ruled_region_component / bool_ruled_region_collinear / bool_ruled_region_span / bool_ruled_region_at / bool_ruled_region_end / bool_ruled_region_corner / bool_ruled_region_order / bool_ruled_region_cross | the point's distance from a cylinder's axis or a cone's apex; a point's height over the apex along the axis; a root's offset from the axis along the leg's radial (`2ρ` from the meridian plane's other point); a line edge's ends' larger distance from the leg's line; a root's parameter less a span end × the conic's smaller semi-axis or the line's unit speed; a crossing's place along its leg (arc length on a parallel, metres on a ruling) against the leg's start, its end, or another crossing; an aimed ruling's run to its corner; the heading's sine against the face side × the face's scale (the radius, or a bound on the cone face's reach from its apex, never under it) | m | OK |
| boolean/ruled_region.rs (`landings`, through `circle_roots::first_harmonic_roots_in_band`) | bool_ruled_region_roots_noise / _coaxial / _extreme / _slack | the first-harmonic door's rows: a parallel leg's signed distance off an edge's plane in the leg's parameter, or a conic edge's off a ruling leg's meridian plane in its own; its rounding charge, its extremes, and a root's arc slack at the leg's radius or the conic's larger semi-axis | m | OK |
| boolean/solid_contain.rs:538/562/587 | bool_point_in_solid_plane | plane residual; /2r linearizations | m | OK |
| boolean/solid_contain.rs:645/655 (both `RayRows`) | bool_point_in_solid_advance/order / side_at_infinity_advance / side_at_infinity_order | ray parameters (m, unit dir) | m | OK |
| boolean/solid_contain.rs (`ray_crossings`, plane arm; both `RayRows`) | bool_point_in_solid_denom / side_at_infinity_denom (plane) | cos(unit,unit) levered by the selection's reach (`selection_reach`): the ray's rise off the plane over every length at which it could meet the face | m | FIXED (F2) |
| boolean/solid_contain.rs (`line_wall_roots`) | bool_point_in_solid_denom (cylinder) | `|d⊥|` levered by the run the caller reads (the ray: the selection's reach + 2r; an edge: its own span): the line's drift off its distance from the axis | m | FIXED (F2; was sin²/2r, 1/m) |
| boolean/solid_contain.rs (`line_wall_roots`) | bool_ray_cylinder_disc | disc/|d⊥|² over 2r (over_lever), the sphere arm's form | m | FIXED (F2; was disc/(2r)², dimensionless) |
| boolean/solid_contain.rs (`cast_ray`, wall arm) | (cylinder hit-outward) | read off the decided discriminant's root order, as the sphere lane reads it: no decision | — | FIXED (F2; was (unit·radial)/radius, dimensionless) |
| boolean/solid_contain.rs (`sphere_chart_trim`, `trim_window_short`) | bool_sphere_trim_period | a face's azimuth hull against one turn through the turn rule (`Period::turn`'s row) at `Exact(R)`; only `Short` keeps a window. A meridian edge's own turn is its certificate's (`CertifiedTurn`, check 2's), read rather than re-decided | m | OK |
| boolean/solid_contain.rs (`sphere_chart_trim`, through its `zero` closure) | bool_sphere_iso_meridian | `n_c·â`, a cosine levered by the circle's radius; then `‖c_c − c‖` and `r_c − R`, bare | m | OK (a meridian great circle: square to the polar axis, centred at the sphere's centre, at its radius) |
| boolean/solid_contain.rs (`sphere_chart_trim`, through its `zero` closure) | bool_sphere_iso_rim | `surface_group::coaxial_margins`: `‖n_c × â‖` levered by `r_c`, and the centre's distance off the axis; then `√(‖w‖² + r_c²) − R`, the rim's seat on the sphere | m | OK |
| boolean/solid_contain.rs (`sphere_chart_trim`, a meridian edge whose turn is `Short`) | bool_sphere_trim_pole_interior | `p̂·m̂ − cos(w/2)`, the pole's direction against the edge's mid-direction less the half-span's cosine, levered by `R` — the cosine-window construction | m | FLAG F26 |
| boolean/solid_contain.rs (`latitude_after`, `latitude_extremes`) | bool_sphere_trim_latitude / bool_sphere_trim_antipode | `R·sin(v_b − v_a)` (`latitude_sine`, levered by `R`: the arc between the two latitudes to first order, which does not collapse at a pole); on its `Zero`, the axial difference `h_a − h_b`, bare | m | OK (the fold's in-band arm is a deterministic tie-break, D9; the axial tie-break is asked only where the sine is Zero, at two equal latitudes or the two poles, where it does not cancel) |
| boolean/solid_contain.rs (`sphere_chart_trim`'s wrapping arm; `latitude_extremes`' `resolve`) | bool_sphere_trim_pole_end | a level's radial distance `ρ` from the axis, then its axial height `h`, bare | m | OK |
| boolean/solid_contain.rs (`point_in_faces`, `FaceGeo::Sphere` and `FaceGeo::SpherePatch`) | bool_point_in_solid_sphere | `(‖q − c‖² − R²)/2R` | m | OK (the /2r linearization) |
| boolean/solid_contain.rs (`carrier_roots`' cone arm; `ray_crossings`' `PartialCone` arm; both through `line_cone_quadratic`) | bool_ray_cone_lead | `a₂ = (d·â)² − cos²α` (`d` unit), levered by a bound on the face's reach from its apex (`region.reach()`; `PartialCone`'s `reach`) | m | OK (`cos²β − cos²α ≈ sin 2α·(α − β)`: the ray's angle off the generator cone, to first order, times `sin 2α` — an under-read on a needle or near-flat cone, whose only consequence is a graze) |
| boolean/solid_contain.rs (as `bool_ray_cone_lead`) | bool_ray_cone_disc | `(b₂² − a₂c₂)/reach` (`over_lever`, m²/m) | m | FLAG F27 |
| boolean/solid_contain.rs (`root_outward`, cone) | bool_ray_cone_nappe / bool_ray_cone_incidence | the root's axial height above the apex, bare; `d·n̂`, the ray's cosine against the unit chart normal `r̂·cos α ∓ â·sin α`, levered by the root's radius `ρ` off the axis | m | OK (D4's θ·r; the nappe has put `ρ` definitely clear of the apex) |
| boolean/solid_contain.rs (`root_outward`, torus) | bool_ray_torus_incidence | `d·∇̂F`, the ray's cosine against the tube's unit implicit gradient, levered by the minor radius | m | OK |
| boolean/solid_contain.rs (`line_torus_roots`, the `RAY_TORUS_ROWS` const, through `geom_brep::quartic::depressed_quartic_roots`) | bool_ray_torus_disc / bool_ray_torus_shape / bool_ray_torus_depth / bool_ray_torus_odd / bool_ray_torus_split / bool_ray_torus_split_lead | the ladder's coefficients in `y = t + b` (a length: `d` is unit), each over the power of `ext = R + r` that makes it a length (`Δ`/ext¹¹, `p`/ext, `D`/ext³, `q̂`/ext², the factors' discriminants over ext or ext³, the resolvent root `z`/ext) | m | OK (the conic-quadric ladder's comparand, at the ray's own lever). `z` is `α²`, a squared length over a length, so `bool_ray_torus_split_lead` under-reads where `α ≪ ext`: decidedness, not correctness, and `work/inside/torus-split-lead-escalates-a-legitimately-small-resolvent-root.md`'s |
| boolean/solid_contain.rs (`clear_of_carrier`, under the `RayRows` consts `SCHEDULE_ROWS` and `side_reader::AIMED_ROWS`) | bool_point_in_solid_clearance / side_at_infinity_clearance | `\|elev\|`: `q`'s signed distance from a plane, or a cylinder wall's `(ρ² − r²)/2r` (`wall_elevation`) | m | OK |
| boolean/solid_contain.rs (`ray_passes_beside`, both `RayRows`) | bool_point_in_solid_beside / side_at_infinity_beside | the larger of the two sides' least signed distances from the plane through the ray spanned by `d` and the face normal, over the loop hull's vertices and its curved edges' balls less their radii (`splitting::containment::loop_hull`) | m | OK |
| boolean/solid_contain.rs (`edge_meets`, both `RayRows`) | bool_point_in_solid_edge / bool_point_in_solid_edge_span / side_at_infinity_edge / side_at_infinity_edge_span | the ray line's enclosed distance from a line edge's line where they come closest (`census::LinePass::gap`); that closest point's distance along the edge from each of its ends (`LinePass::reach`) | m | OK |
| boolean/solid_contain.rs (`EdgeMeets::vertex`, both `RayRows`) | bool_point_in_solid_vertex / side_at_infinity_vertex | a vertex's distance from the ray segment, less its rounding charge (`geom_core::rounding_charge`), clamped at zero | m | OK |
| boolean/solid_contain.rs (`ray_crossings`' met-edge pass, both `RayRows`) | bool_point_in_solid_met_edge_represents / side_at_infinity_met_edge_represents | a crossing's advance off the met edge's span along the ray (`t`, `d` unit) | m | OK |
| boolean/solid_contain.rs (`reaches_root_in_band`, both `RayRows`) | bool_point_in_solid_met_edge_reaches / side_at_infinity_met_edge_reaches | the ray's rise off a curved carrier from a met edge's passage to a root `off` along it, `Margin::levered(\|d·n̂\| + tilt + rounding, off)` (`n̂` the chart normal, `tilt` the root point's residual over the least radius of curvature), less the passage's clearance (`implicit_residual`, metres), plus a short chain's rounding | m | OK |
| boolean/solid_contain.rs (`crosses_by_turn`, both `RayRows`) | bool_point_in_solid_winding / side_at_infinity_winding | `(\|turn\| − π)·reach`, through `k_stats::check_unlogged(.., "F23")` | m | OK (F23) |
| boolean/solid_contain.rs (`at_infinity_side` → `props::role_walk` → `props::read_sign`, through the `AT_INFINITY` / `AT_INFINITY_ENCLOSURE` consts) | bool_point_in_solid_infinity / bool_point_in_solid_infinity_enclosure | the selection's volume, its certified enclosure's sign alone: `Margin::sign_of` against `Band::enclosure_sign()` (D10's bandless sign read) | — | OK (no length is held against the band, so there is no dimension to convert; the V/A reading this row's predecessor named is gone from the path) |
| boolean/solid_contain.rs (`line_sphere_roots`) | bool_ray_sphere_disc | disc/2r (over_lever) | m | OK |
| boolean/vtxfac.rs:106/113/453 | side_code / bool_sector_coplanar / bool_germ_line | side_code as in the sectors.rs row; `bool_sector_coplanar`: sin × sector arm, which only proposes coplanar (both bounds must also read On, in band too); `bool_germ_line` (`pierce_germ_dir`): sin × the sector's farther reach (`BoolSector::span`) | m | FIXED (CONTACT-9; the germ line was levered at the sector arm) |
| boolean/vtxfac.rs (`faces_holding_pierce`) | bool_held_pierce_curved | the held run's curvature charge, `sectors::sagitta_charge` at the sine `|n × n_p|`, the curved face's bend `ρ` (one face of the two is a plane; both curved refuses) and the pierce's shortest arm: the half above the pierced face's least separation from it past the sagitta | m | OK |
| boolean/vtxfac.rs (on-edge resolution) | bool_sector_bisector_side | refusal only: a bisector reading On between two readings definitely on one side, reachable only at K ≤ 2; the nonzero gate's rejection, as at `recl` | — | OK (CONTACT-9) |
| census.rs:313–599 | pm_census_vv/ve/vf/ef gaps, spans, residuals | point/line/plane distances and spans (unit dirs verified) | m | OK |
| census.rs:614–746 | pm_census_span_* / ee_gap / ee_span / ee_overlap | span arithmetic (m) | m | OK |
| census.rs:666 | pm_census_ee_parallel | sin(unit dirs) × min(edge lengths) | m | FIXED (was bare sine) |
| census.rs:812/831 | pm_census_confirm_* | distances / residuals | m | OK |
| census.rs (`edge_vertex_at`, from the edge-on-face and collinear-overlap bound backings) | pm_census_bound_end | `s` and `len − s`: a bound's position along the edge's unit direction from `v0`, and its distance from `v1` (`len` the chord length the direction was normalised by) | m | OK |
| census.rs (`any_boundary_vertex_at`) | pm_census_bound_vertex | `‖p_w − q‖`, a boundary vertex's distance from the bound's point (`Margin::norm3`) | m | OK |
| merge_faces.rs:924 | bool_ring_run_winding | (n̂ · Newell sum) / loop perimeter | m | FIXED (F4) |
| pcurves.rs (`spline_gap_closes`: `chart_u_arm`, `v_meter`) | pcurve_loop_continuity | a SPLINE chart's (`Nurbs`/`Approx`) joint gap, on the sheet lifted by the joint's `u` and `v` periods where the net closes in each: Δu × `chart_u_arm` and Δv × `v_meter`, `geom_brep::chart_stretch_sup`'s `(sup \|S_u\|, sup \|S_v\|)`, the chart's own metre stretch | m | OK (`metered_sup` door: an escape metred through a certified upper bound can only refuse) |
| pcurves.rs (`near_pole_gap_closes`: `joint_arm`) | pcurve_loop_pole_gap | at a joint whose orbit marks missed (undecided, or the gap on one) at a band-sized lever (`lever_periods`; a vertex near a pole, or off a narrow cone's apex by more than the band with its lever `|p − apex|·sin α` under it): (gap − m·step) × the vertex's own distance from the axis, for the offsets m of the joint's `Orbit` (`Periods`: 0, ±1 whole periods; `HalfPeriodsWithTwin` where the image has a sphere twin: 0, ±1, ±2 half periods), then the two branches astride the mark the marks' walk stopped on (`Period::decide`), each a lift of the one point. Zero takes m; an offset in band, where none closes, escalates over the marks' decided on-mark miss | m | OK (`metered_sup` door; the rate is the vertex's own distance from the axis, exact there, so an escape it admits is ≤ ε in metres at that vertex) |
| pcurves.rs (`joint_has_room`: `axis_distance`) | loop_windings_joint_room | `loop_windings`' room fence, per joint off the singular set: a quarter of the chord two of the joint's chart ends would span half the orbit's step apart, `sin(step/4)/2` (`sin(π/4)/2` on a sphere, `1/2` on a cylinder, cone or torus) × the ends' least distance from the axis, `d − 2ε` at the vertex's own `d` | m | OK (levered door). It is an escape claim on the conservative side: only a definite `Positive` admits, so the chord `2·(d − 2ε)·sin(step/4)` is past four times the band's escalation, `4·K·ε > 4ε`, the joint bound, at every `K > 1`: no two chart ends of the joint lie half a step apart, and the decided integer is the joint's own. A spline chart is exempt: its joints decided their lifted chart-space gap |
| pcurves.rs (`turn_element`) | pcurve_turn_closes | a killed half's two vertex points' 3-D distance: whether its carrier is closed, so the kill crosses it by a turn | m | OK (`Margin::of` a distance; only a decided `Zero` reads the ends as one point, and the turn is then decided at that point by the walk's own joint decision, which takes the coincidence as given) |
| pcurves.rs (`decide_joint`: `joint_arm`) | pcurve_loop_branch | (gap − (m+½)·step) × the joint vertex's distance from the chart axis (analytic; `step` the orbit's: half a period on a sphere, whose twin sits half a period over, a whole one elsewhere) or `chart_u_arm` (spline); the second channel where the chart closes in it: (gap − (k+½)·τ) × its polar radius on a sphere or torus, (gap − (k+½)·p_v) × `v_meter` (the net's `sup \|S_v\|`) on a net closed in `v`, `p_v` its knot domain's length | m | OK (`metered_sup` door, both channels: the analytic charts' exact vertex lever and polar radius serve as both members of the pair, a net's stretch as its sup). NOT by the escape argument: a mark decision is not an escape claim, and a larger arm decides marks more readily. It is sound because the arm is positive, so a decided sign is the sign of `gap − mark` itself, and the cell between two decided marks holds exactly one orbit point; the arm's size only sets how near a mark a gap may sit and still decide. Where the margin `step/2 × lever` exceeds the joint bound (the chart ends `≤ 4ε` apart in metres, `decide_joint`'s docs), the decided integer is the joint's true deck element. It does not everywhere the incidence reads `Off`: `Off` only puts the vertex `K·ε` from the singular set, so near a pole at a small `K` the lever is about `K·ε`, and near a narrow cone's apex it is `p·sin α`, small even at `K = 10`. There a mark may decide a different orbit point, but only one equally near the joint; every orbit point lifts the same 3-D point, so the lift stays continuous in metres to the joint bound. Where a mark misses there, the near-pole read (`pcurve_loop_pole_gap`) tries the orbit's offsets about the image and the two branches astride the mark the walk stopped on, and takes one the gap meets within ε at the lever; one in band escalates; only a gap decided off every offset it tries refuses (on a mark): with both branches astride the mark decided off, `π·lever ≥ escalate` and every other orbit point is at least `3π·lever − ε` off, so none is within the band, at any `K > 1`. No decided integer names a different point |
| pcurves.rs | pcurve_loop_pole_joint | a spline net's `u` reach (`net_reach`): its `u` knot domain's length × `sup \|S_u\|` (`Margin::metered_sup`), the metres a whole `u` sweep moves a point at most; the stretch alone is a rate (m per parameter unit) and is not compared with the band. An analytic chart's joint reads `chart_singularity_at_vertex` instead | m | OK |
| chart_singularity.rs (`vertex_at_singular_point`) | chart_singularity_at_vertex | a vertex's stored point's 3-D distance to one singular point of its chart (a sphere's pole, a cone's apex): the one incidence reading, which `locate`'s at-vertex placement, the joint's reset (`singular_at`) and tier 3's wrap-tip rule read | m | OK (`Margin::of` a distance) |
| pcurves.rs (`decide_joint`) | pcurve_loop_v_stretch | a spline net closed in `v`: its `v` reach (`net_reach`), the `v` knot domain's length × `sup \|S_v\|` (`v_meter`, `Margin::metered_sup`), the `v` companion of `pcurve_loop_pole_joint`'s net-level read; a reach decided past the band reads the `v` marks, one decided under it is `kv = 0` on an ordinary joint, one in band refuses the joint (`PcurveMintError::NetStretchUndecided`) | m | OK |
| pcurves.rs (`iso_schedule`, which names the label) → geom-brep edge_nurbs.rs `probe_pick` | pcurve_iso_line_rim | a straight edge's carrier points at both ends and a quarter span in, against the chart's point at a candidate iso image's own parameter there: the largest of the three 3-D distances, for a `Line` carrier described as a chart image or a scaffold | m | OK (`Margin::of` a distance; only a decided `Zero` selects a candidate, and the full iso certification follows the selection; an escalation is deferred and answered only when no candidate holds) |
| pcurves.rs (`iso_schedule`, which names the label) → geom-brep edge_nurbs.rs `probe_pick` | pcurve_iso_wall_seam | a seam's carrier points at both ends and a quarter span in, against the chart's point at a candidate image's own parameter there: the largest of the three 3-D distances, for a seam stated as an iso line in another chart. The label decides every candidate the wall-seam arm hands the boundary-iso schedule: the stated columns (each chart boundary and the column the start's foot names), the closed-form columns and rows, and the measured maps | m | OK (as `pcurve_iso_line_rim`: `Margin::of` a distance, only a decided `Zero` selects, an escalation is deferred and answered only when no candidate holds, and the full iso certification follows the selection) |
| pcurves.rs (`iso_schedule`, which names the label) → geom-brep edge_nurbs.rs `probe_pick` | pcurve_iso_seam_column | as `pcurve_iso_line_rim`, for an `Intersection` or `OnPair` seam on a spline chart: the largest of the carrier's 3-D distances from the chart at a candidate column's or row's own parameter, at both ends and a quarter span in, over the closed-form columns and rows and the measured maps | m | OK (as `pcurve_iso_line_rim`) |
| pcurves.rs (`net_closes`, for `chart_u_period` and `chart_v_period`; the name rides a `(name, pairs)` tuple) | pcurve_chart_u_closed / pcurve_chart_v_closed | `‖P_first − P_last‖`, each control point of the net's first boundary column (row) against the matching one of its last, bare | m | OK (only a decided `Zero` at every pair closes the channel; the weights the test does not compare, and the in-band fold, are `work/pcert/chart-u-period-folds-an-in-band-net-closure-into-no-period.md`'s, neither a dimension question) |
| chart_singularity.rs (`spline_collapsed_row`, read by `spline_side_spans`; the one reading of a collapsed side: `pcurves::chart_side_bounds`, `section_cert`'s W1′, and the plane × spline door) | pcurve_chart_side_spans | `‖P_first − P_k‖`, the first control point of the net's row along a chart side against each other one, bare | m | OK (`Margin::of` a distance; only a decided `Positive` at some pair reads the side as spanning, and spanning is the side's condition to clear: a decided `Zero` or an escalation keeps it collapsed, the refusing side) |
| pair_section/plane_spline.rs (`PlaneSplinePose::hull` and `door`) and plane_spline/sweep.rs (the regularity sweep, C5 stage 3) | section_nurbs_plane_coincident / section_nurbs_plane_pole / plane_spline_sweep_excluded / plane_spline_sweep_regular | the net's largest control-point height over the plane (`decide_magnitude`); each point of a collapsed side's row, its height over the plane; the plane's height over a cell on the accounting's first-order box, over the normal's length; the wall's slope off the plane along `u` or `v` over a cell, `n·S_d / (‖n‖·sup‖S_d‖)`, levered by the feature extent | m | OK (heights are lengths; the slope is a sine levered by the extent, as `ssi_transversality` is) |
| split.rs:197 | split_edge_param_interior | param spans × per-kind rate (1 / radius / minor / speed bound) | m | OK (metered door; the rate is an `InfSpeed` on every kind — the three closed forms by being exact, the net by `speed_lower_bound`'s derivation — which is what an interiority claim needs) |
| `geom-core` `linalg/rigid_motion.rs` (`RigidMotion::check_rigid`, the decided door a raw map is admitted at; a motion minted by construction asks none; the translation's finiteness is `Real::is_bounded`, as at the mints, and not a band decision) | transform_rigid_* (7 residuals) | unit-column/orthogonality/det residuals, no arm | dimensionless | FLAG F10 |
| validate.rs:1662/1847 | planar_face/boundary_residual | plane residuals | m | OK |
| validate.rs:1795 | tangent_second_order | κ_rel × arm²/2 | m | OK |
| validate.rs:2030 | bool_ring_run_winding | (outward · Newell sum) / loop perimeter | m | FIXED (F4) |
| validate.rs:2014 | positive_volume | volume/surface-area (the documented dimensional fix) | m | OK |
| sector_shape.rs (the three rungs) | sector_arm / sector_reflex / sector_straight | arm = shorter bounding chord (m); sin/cos × arm | m | OK — ONE implementation since the S5 sector-predicate unit, and since #652 ONE name set: the former `bool_sector_*` / `split_sector_*` pairs were the same computation on the same quantity, which is why this was already one row |
| sector_shape.rs (`apex_shape`) | sector_apex_tilt / sector_apex_window | `sin α·‖n_r‖` levered by the arm (m): how far the plane's side varies round a cone's apex rulings; `2 sin α·sin(Δφ/2)` levered by the arm (m): the chord between two rulings' points a lever out, the window's width and an extremal ruling's distance in from each bound | m | OK (SPLITDOOR) |
| face_normal.rs (`cone_point`) | cone_off_axis / cone_at_apex | a point's distance off a cone's axis, and from its apex (m), through `Margin::norm3` | m | OK (SPLITDOOR; `cone_off_axis` was `bool_cone_off_axis`) |
| face_normal.rs (`face_outward_normal_at_foot`) | foot_off_locus | a point's distance off a cylinder's axis or from a sphere's centre (`Margin::norm3`), and its height over a cone's apex plane along the axis (`Margin::of`), in m | m | OK (CLEAVE) |
| splitting/classify.rs:81–286 | split_vertex_side / conic lane | plane residual; rooted amplitude; (rad)×minor semi-axis | m | OK |
| splitting/classify.rs `reach_clears` | split_gate_box_side | an unarmed entity's reach, read in the plane's own frame, against the plane: its gap from the plane's offset along the unit normal, less the sweep pad (metres) | m | OK |
| boolean/boxes.rs `sphere_window` | bool_box_sphere_axis | a sphere's polar axis, read as a unit direction (`UnitVec3::new`'s own length decision) | m | OK |
| boolean/boxes.rs `sphere_window` | bool_box_sphere_seam_unit | a sphere's seam direction, read as a unit direction (`UnitVec3::new`'s own length decision) | m | OK |
| boolean/boxes.rs `sphere_window` | bool_box_sphere_seam | a sphere's seam direction against its polar axis: their cosine, levered by the radius | m | OK |
| splitting/classify.rs `torus_window_reach` | split_gate_torus_ring | a torus's major radius less its minor, the ring convention `R > r` its chart rectangle's closed form reads | m | OK |
| ray_walk.rs (via `containment.rs`'s `ROWS`) | point_in_loop_segment | a loop segment's own length — the degeneracy gate, through the `Margin::norm3` door | m | OK (split off `point_in_loop_boundary` by #712, which was deciding two questions under one name) |
| ray_walk.rs (via `containment.rs`'s `ROWS`) | point_in_loop boundary/side/advance | distances; m²/m advance | m | OK |
| splitting/containment.rs (`certify_plane`) | point_in_loop_normal / point_in_loop_plane / point_in_loop_query | (\|n\| − 1) levered by the loop's reach from its first vertex; a vertex's, conic centre's or control point's offset `(p − o)·n̂` off the plane, and a conic's or spiric's plane tilt `\|axis × n̂\|` levered by its larger semi-axis (a spiric's by `R + r + \|offset\|`); the query's offset `(q − o)·n̂` | m | OK (CLEAVE) |
| splitting/containment.rs (the frame gate) | point_in_loop_arm | sin(member, plane normal) × loop extent (the member's in-plane fraction) | m | FIXED (was dimensionless schedule norm) |
| ray_walk.rs (via `containment.rs`'s `ARC_LOOP_ROWS`) | point_in_arc_loop_segment/boundary/side/advance | the point_in_loop rows over an arc-bearing loop's STRAIGHT edges (`on_segment` per chord edge, `ray_crossings` masked to chord edges): distances; m²/m advance | m | OK (ATREST-9) |
| splitting/containment.rs (`point_in_loop`) | point_in_arc_loop_arm / point_in_arc_loop_reach | the frame gate, as point_in_loop_arm with the curved edges' reach in the extent; the reach row is a spiric piece's (below) | m | OK (ATREST-9; reach row CONTACT-4) |
| splitting/containment.rs (`ConicArc`, `conic_crossings`) | point_in_arc_loop_conic_span / _straddle / _window / _disc / _advance | `_span` / `_straddle`: (τ − width) through the turn rule (geom-brep's table, `Period::turn`) at `Exact(radius)` on a circle and `Bounded(turn_defect_speed_lo, turn_defect_speed_hi)` on an ellipse, read for "not over" (an undecided turn is kept as an arc only where `TurnUndecided::not_over_at_sup` admits it; any other escalates); the rest are unit-circle quantities levered by the conic's SMALLER semi-axis: `arc_trim`'s chordal-defect sum on a ray's crossing (with `_end` its step 1); (1 − h²)/2 — exact lengths for a circle (the last is (r² − h²)/2r, the perpendicular-offset form), a lower bound for an ellipse, where a `Zero` or an in-band margin only abandons the ray; the root's advance t is metres along a unit ray | m | OK (ATREST-9; window ATREST-12) |
| splitting/spiric_arc.rs (`PiecedArc::contact` on a `SpiricArc`, through `LoopEdge::contact`) | point_in_arc_loop_spiric_end / _clear / _on / _leaf; bool_contact_spiric_end / _clear / bool_contact_spiric / _leaf | the distance from the point to an end of the arc; `|q − P(v_m)| − S·h`, a piece's ball's clearance from the point; `|q − P(v_m)|`; `S·h`, the ball's own radius | m | OK (CLEAVE) |
| splitting/spiric_arc.rs (`PiecedArc::crossings` on a `SpiricArc`) | point_in_arc_loop_spiric_side / _turn / _advance (and point_in_arc_loop_reach) | `(P(v) − q)·n`, a piece end's offset from the ray line; `|s(v_b) − s(v_a)| − 2·A·h²` (A in m/rad², h in rad; `S` and `A` over the piece's own window, `geom::spiric_rate_bounds`); `(c − q)·d ∓ S·h`; the ball's in-plane clearance from the ray less `S·h` | m | OK (CLEAVE) |
| splitting/spline_arc.rs (`PiecedArc::contact` on a `SplineArc`, through `LoopEdge::contact`) | point_in_arc_loop_spline_end / _clear / _on / _leaf; bool_contact_spline_end / _clear / bool_contact_spline / _leaf | the distance from the point to an end of the arc; `|q − c| − ρ`, a piece's control ball's clearance from the point (`c` its Bézier net's bounding-box centre, `ρ` its farthest control point's distance from `c`); `|q − c_0|`, the distance to the piece's start point; `ρ`, the ball's own radius | m | OK (SPLICE) |
| splitting/spline_arc.rs (`PiecedArc::crossings` on a `SplineArc`) | point_in_arc_loop_spline_side / _turn / _advance | `(c_k − q)·n`, a control point's offset from the ray line; `s_{k+1} − s_k`, a step of those offsets along the control polygon; `(c_k − q)·d`, a control point's advance along the ray | m | OK (SPLICE) |
| splitting/containment.rs (`LoopEdge::contact`, the boundary pre-pass) | curve_carrier_gap / curve_carrier_clear / curve_carrier_straddle / curve_pair_span_end_chord / curve_pair_span_end_side / curve_pair_span_end_straddle | a conic arc's boundary reading, `geom`'s on-carrier reading and the span's reading of its ends at the point's foot (the `bool_contact_arc_span` row above); every distance in metres, the side metered by the carrier's greatest speed | m | OK (CONTACT-4) |
| splitting/neighborhood.rs (`wide_entries`) | split_bisector_side | bisector·n̂ × arm (a conic edge's departure is the one bound-side reading's, `topo::bound_side`: `enters_material{,_arm}`, then `conic_flat` / `conic_bend` / `conic_bend_side`) | m | OK |
| splitting/insert.rs (`section_chords`) | split_germ_line | `\|(\|ŵ·n̂\| û + \|û·n̂\| ŵ)\|`, the in-plane combination of a transition sector's unit bounds, levered by the shorter bound's reach (m): the sector meets the plane along a line. Not read where one bound lies in the plane (the germ is that bound) | m | OK (CLEAVE) |
| splitting/insert.rs (`apex_germ`) | split_germ_apex | the signed chord between two rulings' points a unit out from a cone's apex (`ConeApex::turn`), the lesser from each bound of the cut's sub-sector, levered by its arm (m): a section ruling lies inside it | m | OK (SPLITDOOR) |
| splitting/insert.rs (`section_chords`) | split_germ_side / split_germ_order / split_germ_sense | `n̂·(ĝ₀ × ĝ)` and `n̂·(ĝ × ĥ)`, the sines between unit germs about the plane's normal, and `ĝ₀·ĝ` for a germ on the first germ's line, each levered by the shortest reach of the germs it reads (m): its distance at that reach from the other's line. A decided zero is two germs of different sectors on one line to the tolerance, ordered one order down by their sections' bends (`geom_brep::tied_germ_turn`, rows `tangent_sector_order2{,_arm}`; a section at a cone's apex is a ruling and bends by 0) and refused where they curve alike (`SectionGermsUnordered { Coincident }`); the two germs either side of one in-plane entry are never read | m | OK (CLEAVE) |
| splitting/order.rs:73 | split_join_frame_arm | sin(member, plane normal) × points' spread (the member's in-plane fraction) | m | FIXED (was dimensionless schedule norm) |
| splitting/order.rs (`axis_plane`) | split_join_frame_axis | a component of the unit plane normal — the cosine between the normal and an axis — levered by the points' spread (`Margin::levered`), vs the EXACT bit-level band: whether it is exactly zero, which picks the exact axis frame for an axis plane | m | OK (CLEAVE; levered like `split_join_frame_arm`) |
| splitting/order.rs (`lex_cmp`) | split_join_order_u/v | coordinate difference (m) vs the EXACT bit-level band (deliberate total-order device, documented) | m | OK (note N6) |
| splitting/order.rs (`sort_along`) | split_join_line_order | how far one of a face's crossings lies past another along its section (m) vs the run's band: a real distance apart or one point. A planar face's is the difference of along-line coordinates; a curved face's is the eccentric-anomaly difference times the conic's speed halfway, the arc length to second order | m | OK (CLEAVE) |
| splitting/order.rs (`group_coincident`), splitting/join.rs (`conic_pairs`) | split_join_line_gap | the same gap between successive sorted crossings of one face (m), and across a conic's branch cut, vs the run's band: Zero is one point, Positive two | m | OK (CLEAVE) |
| splitting/join.rs (`line_pairs`) | split_join_face_line | `|n_face × n_plane|` levered by the face's crossings' spread (m): the face's section line exists | m | OK (CLEAVE) |
| splitting/join.rs (`ruling_pairs`) | split_join_ruling_side | a crossing's distance from one section ruling less its distance from the other (m): which ruling it lies on, Zero where the rulings meet (a cone's apex) | m | OK (CLEAVE) |
| splitting/join.rs (`ruling_pairs`) | split_join_ruling_germ | a germ's sine from one section ruling less its sine from the other, levered by the face's farthest crossing from the apex (m): which ruling an apex crossing leaves along | m | OK (SPLITDOOR) |
| splitting/join.rs (`conic_pairs`) | split_join_conic_heading | the sine `(n_plane × n_out)·Ĉ′` between the section's heading into the face and the conic's unit tangent, levered by the wall's `curvature_lever_arm` (the cylinder's or sphere's radius) (m): how far the section runs from tangent to the wall, and so which way along the conic enters the face | m | OK (CLEAVE) |
| splitting/finish.rs (`line_clears_conic`) | split_nest_line_conic | a conic carrier's centre offset from a line less its amplitude across it, `|m·(c − p)| − √((m·a)² + (m·b)²)` (m) | m | OK (CLEAVE) |
| splitting/finish.rs (`conics_clear`) | split_nest_conic_conic | in one conic's unit coordinates, `1 − |c′| − σ` or `|c′| − σ − 1` with `σ` the largest singular value of the other's semi-axis matrix, levered by the first's smaller semi-axis (m) | m | OK (CLEAVE) |
| splitting/rules.rs (`apply_rule_a`; the coplanar read through `geom_brep::normal_alignment`) | split_sector_extent / coplanar / enters arm | extent; sin×extent, each face over its own face extent (D4 ¶1's parallelism arm). At a smooth edge in the plane the two faces read one tilt over two arms, so one can decide zero and the other past the band: `rules::wall_graze` refuses that as two decided readings that disagree (`SectorReading::SmoothEdgeDisagrees`), carrying both | m | OK |
| splitting/rules.rs:179 | tangent_sector_osculation | κ(1/m) × face-extent²/2 | m | FLAG F11 |
| chord_join.rs:1114 | split_tangent_chord_forward | dimensionless param diff × ‖dir‖ | m | OK (metered door; a line's ‖dir‖ is its exact speed and so an `InfSpeed`) |
| chord_join.rs (`arc_leaving`) | chord_arc_leave_germ | a boolean germ's unit direction against the conic's unit tangent at the chord's start, `dir·Ĉ′(θ₁)` (the cosine between them), levered by the semi-major axis | m | OK (REACH) |
| chord_join.rs (`arc_leaving`) | chord_arc_leave_section | the split's `±(n_plane × n_out)` against the same unit tangent (the sine between plane and wall times that cosine), levered by the wall's `curvature_lever_arm` at the chord's start: the quantity and the lever `split_join_conic_heading` decides it on | m | OK (REACH) |
| chord_join.rs (`arc_leaving`) | chord_arc_leave_section_aligned | where `chord_arc_leave_section` reads Zero: the split datum's unit direction against the unit tangent (the cosine between them), levered by the semi-major axis, as `chord_arc_leave_germ` is; Zero names the broken invariant, a sign the graze | m | OK (REACH) |
| chord_join.rs:1411 | split_conic_inplane_mid | plane residual at midpoint | m | OK |
| chord_join.rs (`chart_island_winding`) | split_ring_closure_ruling | `n·â` (cosine), levered at the radius | m | OK |
| chord_join.rs (`chart_island_winding`) | bool_ring_run_winding (wall chart) | `2·R·A_chart / P`, `P` an upper bound in metres (`R·\|Δu\|` plus axial variation per piece) | m | OK (the planar arm's F4 comparand on the wall's chart) |
| chord_join.rs (`path_island_winding`) | split_ring_path_lean | `−a·(N × t)/‖N × t‖` (the cosine between a path's arrival and the closing chord's left normal), levered at `Quadric::lever` (the sphere's radius, the radius of the cone's parallel through the chord's midpoint, or the smaller of the torus's tube radius and its parallel's there) | m | OK (TANG) |
| ring_path.rs (`Quadric::off_apex`) | split_ring_path_off_apex | a point's distance from a cone's or a torus's axis, shrunk toward zero by the coordinates' resolution (`charge`, `resolution`) | m | OK (TANG) |
| ring_path.rs (`Quadric::paths`) | split_ring_path_span | `‖â × b̂‖` (the sine between a sphere path's ends), levered at the sphere's radius, shrunk toward zero by the coordinates' resolution (`charge`, `resolution`) | m | OK (TANG) |
| ring_path.rs (`Quadric::paths`) | split_ring_path_nappe | a cone path end's height along the axis from the apex, shrunk toward zero by the coordinates' resolution (`charge`, `resolution`) | m | OK (TANG) |
| ring_path.rs (`Quadric::ruled_paths`) | split_ring_path_rise | the length along the end's ruling from the turn to the end: the difference of a cone path's ends' slant distances from the apex, or of a cylinder path's ends' heights, shrunk toward zero by the coordinates' resolution (`charge`, `resolution`) | m | OK (TANG; SECT) |
| ring_path.rs (`Quadric::ruled_paths`) | split_ring_path_turn | the length of a ruled path's arc round the end's parallel, its sweep times that parallel's radius, shrunk toward zero by the coordinates' resolution (`charge`, `resolution`) | m | OK (SECT) |
| ring_path.rs (`path_parity`) | split_ring_path_rulings_distinct | the distance between a cylinder ruling piece's line and a loop ruling's, shrunk toward zero by the path's resolution (`charge`, `path_resolution`) | m | OK (SECT) |
| ring_path.rs (`spline_crossings`) | split_ring_path_spline_piece | a path piece's length, shrunk toward zero by the path's resolution (`charge`, `path_resolution`); zero, the piece crosses no spline arc | m | OK (SECT) |
| ring_path.rs (`spline_crossings`) | split_ring_path_spline_across | `‖τ̂ₚ × τ̂ₐ‖` (the sine between a path piece and a spline loop arc where the curve-pair table finds them meeting), levered at the piece's length and charged the path's resolution over it | m | OK (SECT) |
| chord_join.rs (`carried_chord_spec`) | chord_carried_at_seat | a carried chord end's distance from its seated component's start, `‖p − C(t₀)‖` | m | OK (SECT) |
| chord_join.rs (`carried_chord_spec`) | chord_carried_leave | a boolean germ's unit direction against the component's tangent at the chord's start, `dir·Ĉ′(t₁)` (the cosine between them), levered at the section's scale (`CarriedSection::lever`, the cylinder's radius) | m | OK (SECT) |
| chord_join.rs (`carried_chord_spec`) | chord_carried_forward | the parameter travel from the chord's start to its end the way it leaves, metered by the carrier's speed at the start; only its sign is consumed | m | OK (SECT) |
| pair_section.rs (`seat_at`) | ssi_seat_at_the_start | where the knot edit opening a loop at `at` refuses: the seat's distance from the fitted loop's start (or end, one point), `‖C(at) − C(t₀)‖`; decided within the band the loop is kept, undecided the seat escalates | m | OK (SECT) |
| pair_section.rs (`CarriedComponent::holds`, the counted section's cross-check) | curve_carrier_gap / curve_carrier_clear / curve_carrier_straddle | a listed component's closed-form witness against a marched branch's fitted carrier, `geom`'s on-carrier reading at the witness's foot, in a band widened by the branch's tube radius (the section lies within it of the carrier) | m | OK (SECT) |
| ring_path.rs (`Quadric::paths`) | split_ring_path_off_core | a torus path end's distance from the core circle, shrunk toward zero by the coordinates' resolution (`charge`, `resolution`) | m | OK (GERM) |
| ring_path.rs (`Quadric::torus_circle_arc`) | split_ring_torus_meridian | `n̂·â` (the cosine between a loop circle's axis and the torus's), levered at the circle's radius | m | OK (GERM) |
| ring_path.rs (`Quadric::torus_circle_arc`) | split_ring_torus_parallel | `‖n̂ × â‖` (the sine between a loop circle's axis and the torus's), levered at the circle's radius | m | OK (GERM) |
| ring_path.rs (`Quadric::torus_circle_arc`) | split_ring_torus_parallel_side | a parallel's radius less the torus's major radius | m | OK (GERM) |
| ring_path.rs (`path_parity`) | split_ring_path_torus_loop | a crossing's side of the split between its plane's two loops on a torus: `(p − c)·ê` for a meridian, `ρ − R` (signed outward) for a parallel, shrunk toward zero by the coordinates' resolution (`charge`, `resolution`) | m | OK (GERM) |
| ring_path.rs (`Quadric::torus_circle_arc`) | split_ring_torus_crest | the height along the axis of the centre of the tube's top or bottom circle (a parallel of radius `R`), `±r` on the torus | m | OK (GERM) |
| ring_path.rs (`path_parity`) | split_ring_path_crest_graze | a parallel piece's radius less `R`: whether it runs along the cylinder `ρ = R` the top and bottom circles are read across, shrunk toward zero by the coordinates' resolution (`charge`, `resolution`) | m | OK (GERM) |
| ring_path.rs (`path_parity`) | split_ring_path_crest_side | `ρ − R` at a segment piece's end: its side of the cylinder `ρ = R`, shrunk toward zero by the coordinates' resolution (`charge`, `resolution`) | m | OK (GERM) |
| ring_path.rs (`path_parity`) | split_ring_path_meets_plane | the gap `r·√(A² + B²) − \|D\|`: a circle piece's reach across a loop conic's plane less its centre's offset from it, the plane's displacement that flips the meeting, shrunk toward zero by the coordinates' resolution (`charge`, `resolution`) | m | OK (TANG) |
| ring_path.rs (`path_parity`) | split_ring_path_segment_side | a ruling segment's end's offset from a loop conic's plane, `n̂·(p − c)`, shrunk toward zero by the coordinates' resolution (`charge`, `resolution`) | m | OK (TANG) |
| ring_path.rs (`segment_misses`) | split_ring_path_segment_apart | where a segment's end lies on a loop conic's plane: the segment's midpoint's distance from the arc (from the arc of the circle of radius `b` over the conic's span, less `\|a − b\|`), less half its length, shrunk toward zero by the path's resolution (`charge`, `path_resolution`); decided positive, the segment misses the arc | m | OK (GERM) |
| ring_path.rs (`misses_in_plane`) | split_ring_path_coplanar_apart | where a circle piece's meeting with a loop conic's plane is in the band: the gap between the piece's circle and the annulus `min(a, b)..max(a, b)` about the conic's centre, by the centres' distance (`max(d − r − max, min − r − d, r − max − d)`), shrunk toward zero by the path's resolution (`charge`, `path_resolution`); decided positive, the two miss whatever their planes | m | OK (GERM) |
| ring_path.rs (`misses_in_plane`) | split_ring_path_same_circle | the larger of the piece's and the conic's centres' distance and each of the piece's radius's differences from `a` and `b`: decided zero, the piece runs on the arc's own circle, and the arcs' ends are read against each other's spans (`split_ring_path_in_span`, the angle levered at the radius, each reading charged the coordinates' resolution) | m | OK (GERM) |
| ring_path.rs (`path_parity`) | split_ring_path_line_across | `m̂·d̂` (the cosine between a circle piece's plane normal and a loop ruling), levered at the piece's radius | m | OK (TANG) |
| ring_path.rs (`path_parity`) | split_ring_path_rulings_apart | `‖ê × d̂‖` (the sine between a segment's ruling and a loop ruling), levered at the segment's distance from the apex, shrunk toward zero by each direction's rounding: the coordinates' resolution over that distance and over the loop ruling's length (`charge`, `resolution`) | m | OK (TANG) |
| ring_path.rs (`path_parity`) | split_ring_path_in_span | a crossing's distance to a span end (a circle's parameter levered at its radius, a conic's at its minor semi-axis, a line's length) times the slope `\|n̂·τ̂\|` the piece crosses the other's plane at: the carrier displacement that moves the root past the end, shrunk toward zero by the coordinates' resolution (`charge`, `resolution`: `slack / arm` in the parameter); a crossing any of whose readings is decided out of span is passed over, even where another escalates | m | OK (TANG) |
| chord_join.rs (`chart_ring_side`, retired) | split_ring_chart_window, split_ring_chart_ray_azimuth, split_ring_chart_ray_height (retired: a cylinder face re-homes its rings by a path, `path_ring_side`, read by the `split_ring_path_*` rows) | — | — | retired |
| chord_join.rs (`run_azimuth_images`' ring re-homing) | split_chart_azimuth_frame (retired: the read goes through `geom_brep::Azimuth::principal`, decided as `pcurve_chart_cut_side`) | — | — | retired |
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

| ray_walk.rs (via `chart_region.rs`'s `ROWS`) | chart_region_segment | a closed segment's own length — the degeneracy gate, through the `Margin::norm2` door | m | OK (split off `chart_region_boundary` by #712, with the door corrected from `Margin::of` in the same pass) |
| ray_walk.rs (via `chart_region.rs`'s `ROWS`) | chart_region_boundary/side/advance | the point_in_loop rows on METRED chart coordinates (plane 1 and cylinder r exactly; every other kind's certified INF arm, `chart_region_arm_inf`): distances; m²/m advance. Since #712 these decide in the SAME shared walk as the 3-D rows, under their own names. The 3-D `point_in_loop_arm` row is derived away — a fixed 2-D schedule member is in-plane by construction, so no projected-length predicate exists | m | OK (new in M9-2) |
| chart_region.rs (M9-2) | chart_region_parallel / collinear_offset | segment-pair 2×2 determinant / offset determinant over one segment's length — the perpendicular height across that segment's line | m | OK (new in M9-2) |
| chart_region.rs (M9-2) | chart_region_cross_span | crossing fraction (dimensionless) × its own segment's length — the crossing point's clearance from a segment endpoint | m | OK (new in M9-2) |
| chart_region.rs (M9-2) | chart_region_collinear_overlap | shared-span length of collinear segments (difference of metre projections) | m | OK (new in M9-2) |
| chart_region.rs (M9-2) | chart_region_cross_order | same-edge crossing-pair advance-fraction difference (dimensionless) × the edge's own length — the crossing points' separation along the boundary (the clip walk's order certificate, union fix U2) | m | OK (new in M9-2 fix pass) |
| chart_region.rs (M9-2) | chart_region_orientation / chart_region_area | signed loop shoelace 2A (m²) / perimeter — the loop's (resp. intersection region's) mean width, through the same `Margin::over_lever` door as `split_section_area` two dimensions up (separate accumulators — the reasons are at `chart_region.rs`'s area margin) | m | OK (new in M9-2) |
| chart_region.rs | chart_region_arm_inf | a chart lever arm — metres per chart unit, gated as a length (the collapsed-arm idiom, `pcurve_interval_meter`'s shape) | m | OK. The arm is a certified LOWER stretch bound (`certified_arms`), which is the side a positive-extent claim needs: an over-stated arm would inflate the metred region and certify a sliver, so the sup bounds (`geom_brep::chart_stretch_sup`) are deliberately not reachable from this lane. Definite-positive walks on; a collapsed arm is `ArmUnbounded`; an in-band arm escalates |
| chart_region.rs (M9-2, `seam_span_over`) | chart_region_seam_span | the pair's azimuth hull against one turn through the turn rule (`Period::turn`'s row) at `Exact(r)`; `Over` refuses (the margin is the headroom, the negation of the excess it was) | m | OK (new in M9-2) |
| chart_region.rs (`wrap_band`, `cyl_wrap_whole`) | chart_region_cyl_wrap_forward / chart_region_cyl_wrap | the rectangle's azimuth span from its least to its greatest corner (ordered by the exact reads) at `r`, through `Extent::forward` (a span not forward is no band); then that span against one turn through the turn rule (`Period::turn`'s row) at `Exact(r)`; `Whole` is a full wrap (the margin is the headroom, the negation of the excess it was) | m | OK |
| cell_region.rs (CHART) | cell_region_vertex_gap | the distance from the station to one vertex of the face, less the radius of the ball about the cell's centre station that holds the cell's patch enclosure | m | OK (new in CHART). Only a definite `Positive` clears; `Zero`, in-band and poison keep the cell |
| cell_region.rs (CHART) | cell_region_segment_gap | the distance from the station to a line edge's closed segment (`sector_shape::point_segment_distance`), less the radius of the ball about the cell's centre station that holds the cell's patch enclosure | m | OK (new in CHART). Only a definite `Positive` clears; `Zero`, in-band and poison keep the cell |
| cell_region.rs (CHART) | cell_region_conic_gap | a lower bound on the distance from the station to a circular or elliptic edge: its whole annulus between the semi-axes, then its arc's floor (`ring_path::Conic::distance_floor`), each asked under this name, either clearing, less the radius of the ball about the cell's centre station that holds the cell's patch enclosure | m | OK (new in CHART). Only a definite `Positive` clears; `Zero`, in-band and poison keep the cell |
| cell_region.rs (CHART) | cell_region_ball_gap | the distance from the station to the ball a spiric or spline edge's arc lies in (`splitting::containment::carrier_ball`), less the radius of the ball about the cell's centre station that holds the cell's patch enclosure | m | OK (new in CHART). Only a definite `Positive` clears; `Zero`, in-band and poison keep the cell |
| cell_region.rs (CHART) | cell_region_singular_gap | the distance from the station to the carrier's singular set: a cone's apex, a torus's axis, less the radius of the ball about the cell's centre station that holds the cell's patch enclosure | m | OK (new in CHART). Only a definite `Positive` clears; `Zero`, in-band and poison keep the cell |
| injective.rs (SELFX, `by_kind`) | self_intersection_ring_torus | `R − r`, the torus's major radius less its minor radius | m | OK (new in SELFX). Only a definite `Positive` discharges the face by kind; anything else refuses `TorusNotRing` |
| injective.rs (SELFX, `is_zero`, read by `sign_on`) | self_intersection_on_shared | a presented side's (or corner's) separator value, a signed distance (below), in its centred form over the side; on a curved side (CURVEDEP), a bound of it along the curve: the linear separator's value in `{1, cos t, sin t}` coefficients, a quadric carrier's squared form `Q = ψ·D` over a lower bound of `D` (`r`, or `ν·h·sin α` on a cone), the graph separator's Lipschitz bound times the side's distance from the edge's carrier plus its overhang past the graph's piece; on a spiric's wall image (SPIRIC), the linear separator's value along the torus's image of the chart curve in `{1, f(t), cos t, sin t}` coefficients, the torus's drift from the carrier's radii carried as a closed-form term over `ρ = R + r·cos t` | m | OK (new in SELFX). Only a definite `Zero` reads the side as lying on the separator; anything else fails the arm |
| injective.rs (SELFX, `present_chart`; SPIRIC, `present_line_corner`) | self_intersection_sliver | a rectangle clipped to a shared line: its extent across, `Δ × sup \|S_∂\|` (`rect_jet`'s enclosure of the partial, so a metred sup) | m | OK (new in SELFX's fix pass, split off `self_intersection_on_shared`, which decided this and the side value under one name). Only a definite `Zero` reads the strip as holding no point of the face off the line, and only once the rectangle's sides and corners are proven the face's own — one of the presentation's ε concessions, stated at the site and listed in `injective.rs`'s module docs |
| injective.rs (SELFX, `sign_on`) | self_intersection_departure | the separator `ψ`, a signed distance in metres (`n̂·(p − q)` with `n̂` a unit normal; a cylinder's `ρ − r`; a sphere's `\|p − c\| − r`; a cone's `ρ·cos α − h·sin α`; the graph arm's `h − γ̃(s)`), as its centred form over the rectangle, or its derivative across a shared side (m per chart unit) times the rectangle's extent across it (chart units) | m | OK (new in SELFX). Only a definite `Positive`/`Negative` certifies; `Zero`, in-band and poison fail the arm and the pair splits |
| injective.rs (CURVEDEP, `present_curved`) | self_intersection_column | a curved edge's chart curve read as a graph over a chart axis: its advance along that axis over one piece of its parameters (chart units), times `sup \|S_axis\|` over the cell (`rect_jet`'s enclosure, m per chart unit) | m | OK (new in CURVEDEP). Only a definite sign, the same over every piece of the run, reads the curve as a graph over that axis; anything else leaves the cell unpresented and the pair splits |
| injective.rs (CORNER, `leaves_graph`, `leaves_line`) | self_intersection_corner_departs | a curved corner's other edge leaving the edge read through the vertex, toward that edge's face side: off a curve read as a graph, the edge's height over the graph's rate along it, `(d_b·τ_a − d_a·τ_b)·sign τ_a` (chart units per parameter, `d` the edge's chart direction scaled to unit length), times the columns' parameter span and `max sup \|S_axis\|` over the rectangle (`rect_jet`'s enclosure, m per chart unit); off a coordinate line, the other curve's rate on the held axis over one of its pieces, times the piece's width and `sup \|S_held\|` | m | OK (new in CORNER). Only a definite `Positive` reads the corner convex; a definite `Negative` at the vertex is a reflex corner, which no presentation reads (`Structural`); anything else leaves the cell unpresented and the pair splits |
| injective.rs (CORNER, `on_vertex`) | self_intersection_corner_vertex | the chart distance between a corner's other edge's end and the read edge's point at the vertex, `√((Δu·sup \|S_u\|)² + (Δv·sup \|S_v\|)²)` (`rect_jet`'s enclosure over the rectangle, exact on a plane's or a cylinder's constant-speed chart) | m | OK (new in CORNER). Only a definite `Zero` reads the two as one point — the presentation's vertex ε concession, stated at the site and listed in `injective.rs`'s module docs; anything else leaves the cell unpresented and the pair splits |
| injective.rs (SELFX, `graph_of`) | self_intersection_graph | `t̂·C′` over the edge's piece (m per parameter unit) times the piece's parameter span: how far the edge advances along the projection's first axis | m | OK (new in SELFX). Only a definite `Positive` admits the graph |
| injective.rs (SELFX, `present_plane`) | self_intersection_span | how far inside a line edge's two ends the cell's extent stays: the fraction's distance from the ends (dimensionless) times the edge's length | m | OK (new in SELFX). Only a definite `Positive` presents the rectangle |
| injective.rs (SELFX, `present_plane`) | self_intersection_corner | the far end of one edge's offset from the other edge's line, `n·w/\|n\|` in the plane's chart (chart units are metres on a plane) | m | OK (new in SELFX). Only a definite `Positive` reads the corner convex |
| injective.rs (SELFX, `present_chart`) | self_intersection_apex | a vertex's distance from the cone's apex | m | OK (new in SELFX). `Zero` is the apex |
| injective.rs (SELFX, `nappe`) | self_intersection_nappe | a vertex's signed height along the cone's axis from its apex | m | OK (new in SELFX's fix pass, split off `self_intersection_apex`, which decided this and the distance under one name). Every decided sign must agree, and one must be decided; the sign is the face's nappe |
| injective.rs (SELFX, `cusp`) | self_intersection_cusp | `\|n̂_x + n̂_y\| · reach`: the chord between one face's unit outward normal and the other's reverse, levered by the cell ball's reach | m | OK (new in SELFX). Only a definite `Zero` reads the edge as wedge 0/2π, which splits and then refuses naming the second-order arm; it never certifies |
| injective.rs (SELFX, `monotone`) | self_intersection_monotone | the least eigenvalue of the symmetric part of `L·J`, `L` the orthogonal polar factor of the centre Jacobian and each column of `J` scaled by the rectangle's span on its axis (m per chart unit × chart units) | m | OK (new in SELFX). Only a definite `Positive` certifies the strongly monotone projection |
| census.rs (M9-2 PR-2 fix pass) | census_backstop_gap | per-axis gap between two faces' SOUND reach boxes (plane hull ⊕ boundary-arc radius; cylinder axial span ⊕ radius; sphere ball — coordinate differences and radii, metres); only a DEFINITE positive clears the pair | m | OK (new in the union fix; boxes tightened to the face_box construction in the delta) |
| census/depth_gap.rs (RESTREAD) | census_backstop_depth_gap | one face's least depth (distance from an axis line, a centre or a circle) less the other's greatest, less both faces' pads: closed-form norms and radii of the faces' covering rings and segments (metres); only a DEFINITE positive clears the pair | m | OK (new) |
| census/depth_gap.rs (RESTREAD) | census_backstop_depth_foot | the depth's least point on a planar face's plane (an axis's pierce, a centre's foot) against the plane, `(q − o)·n̂`, the point placement's on-plane read before it places the point; only a placement `Out` gives the face a least depth | m | OK (new) |
| census/depth_gap.rs (RESTREAD) | census_backstop_depth_toward | the offset from a depth's axis (its component off the axis) or centre to the other face's nearest boundary segment point, read as a unit direction for arm 1's turned-direction gap; it proposes the direction and decides nothing (the gap along it is `bool_pair_gap`) | m | OK (new) |
| census/depth_gap.rs (RESTREAD) | census_backstop_depth_across | an arc's chord normal (its conic's unit normal crossed with the chord between the arc's ends, in the conic's plane), or an offset between two carriers' axes or centres (an origin off the other's axis, a centre off an axis, two centres' difference), read as a unit direction for arm 1's turned-direction gap; it proposes the direction and decides nothing (the gap along it is `bool_pair_gap`) | m | OK (new) |
| census.rs (M9-2 PR-2 fix pass) | census_backstop_containment | per-axis extent margin between two solids' vertex hulls (coordinate differences — metres); clearance = any definitely negative; anything else sends the pair to the material test, which decides through `point_in_solid`'s own predicates (the invariant the clear rests on is stated at `census.rs` arm 2) | m | OK (new in the union fix) |
| census/curved.rs (FUSE curved join) | pm_census_curved_speed_floor / pm_census_curved_speed_hi | the span's own length metered by the carrier's least speed (`Margin::metered`, `carrier_speed::speed_range`), and its reach by its greatest (`Margin::metered_sup`); a point against the span is `geom_brep::curve_pair::Span::contact`'s rows (`curve_pair_span_*`) | m | OK |
| census/curved.rs (FUSE curved join) | pm_census_ve_curved_clear / pm_census_ve_curved_piece | the foot search's bisection (`geom_core::bisect`): a piece's middle point's distance from a point less the piece's reach (half its span metered by the carrier's greatest speed, `Margin::metered_sup`), and a piece's reach alone, the piece stopped when it decides negative or zero (`stops`); two edges' meetings are `geom_brep::curve_pair::meet`'s rows | m | OK |
| census.rs (FUSE pass 2′) | pm_census_ve_curved_sweep_end | the vertex-on-curved-edge sweep's skip of a pair pass 1 owns: a vertex's distance from each end of the edge (`Margin::norm3`); only a definite Positive or Negative reads on, a Zero or an in-band reading leaves the pair to pass 1. Its walk shares the curved lane's own rows (`pm_census_ve_curved_clear` / `pm_census_ve_curved_piece`, `census::curved::near_pieces`) | m | OK (new) |
| census/curved.rs (FUSE curved join) | pm_census_curved_polish_step | a Gauss–Newton step's length in metres against its `16·slack` bound, decided on the exact-order band (`splitting::order::exact_band`): a heuristic's step kept or dropped, no verdict on geometry; a poison (singular) step escalates and is dropped | m | OK (exact band) |
| census/point.rs (CONTACTHOLD point touch) | pm_census_point_kappa / pm_census_point_gap / pm_census_point_centre / pm_census_point_plane / pm_census_point_radii / pm_census_point_on_plane | a point touch's relative curvature through the sagitta door (`κ_rel` levered by the curved carrier's radius squared, halved); a carrier's nearest distance less the other's reach (`|s| − r·sin θ` for a circle on a plane, `d − r` for a sphere, centre distance less the radii's sum or difference); a foot's or two centres' distance; a centre's signed offset from a plane (`n̂·(c − o)`); two radii's difference; the touch point's offset from a planar face's plane (the containment door's own) | m | OK (new) |
| boolean/edge_join.rs (FUSE curved join) | join_regular_point | a valence-2 vertex's distance from its chart's singular set: a cone's apex, a sphere's or torus's axis | m | OK (new) |
| link.rs (RESTREAD link) | census_touch_plane | `n·(q − p)`: a point's signed distance from the plane of a link's face through the touch point `p` (`n` the face's unit outward normal) — a piece's end, a probe inside a piece, or a parity arc's end against a face of the other link; and for `Link::offset`, a point on a sector within the band, and a ray one sector shares with another — through `link::metric::Distance`, `Margin::of`. Read at the pieces' own real points for a rest, and again inside the ball both links fill for a crossing; a definite sign is taken over the links' tie (`metric::sign_over`: the distance moved by the slack toward zero must read the same) | m | OK (new; replaces `census_touch_side`) |
| link.rs (RESTREAD link) | census_touch_wedge | `(m × û)·(x − p)`: a point's signed distance from the plane through a ray from `p` (a sector's ray, or a face vertex's while the face's piece is built), normal to its face (`û` unit along the ray, `m` the face's normal) — which side of the ray, in the face's plane: a wedge's membership, a cut's place among others, a vertex's turn about `p`, and, for `Link::offset`, which side of a wedge's ray its point stands on, read at the point's own distance rather than drawn in, so that a reading in band puts the point within the band of the ray's line (both branches then taken). The point is drawn in to the ray's own point first, so the ray is never read past what fixes it; decided over the links' tie (`metric::sign_over`) | m | OK (new; takes in `census_touch_piece_side`) |
| link.rs (RESTREAD link) | census_touch_ahead / census_touch_arc | a point on a ray's line read along the ray (`û·(x − p)`), ahead of `p` or behind, drawn in to the ray's point first — a sector bound, a face's reference ray or a piece's probe ray; a parity arc's two ends read against the plane through `p` normal to their bisector, the arc under half a turn — through `link::metric::Distance`, `Margin::of`. An arc any reading of which is not decided is passed over for another reference | m | OK (new; `census_touch_ahead` takes in `census_touch_piece_turn` and `census_touch_piece_front`) |
| link.rs (CONTACT-7, `face_piece`) | census_touch_piece_reach / census_touch_piece_meet | building a face's piece: one crossing point read against another along a unit probe ray, and a meeting point read against an edge's end along the edge's unit direction — through `link::metric::Distance`, `Margin::of`. An undecided choice builds no piece (a refusal, never a rest) | m | OK (new in CONTACT-7) |
| link.rs (CONTACT-7) | census_touch_normal | two coplanar faces' outward normals, one of each link, where a piece's probe lies on the other link's face — aligned (both materials behind one plane: a crossing) or opposed — through `geom_brep::classify_material_pairing_as` under this name: the dot of unit normals levered by the folded arm, a sign of magnitude about 1 | m | OK (CONTACT-7; its candidate-plane use retired with the plane certificate) |
| link.rs (`Link::lowest_on`, `Link::touches`) | link_offset_tie | a point against the piece of a link's boundary it is nearest (a plane's foot, a ray's foot), along the unit line from the foot to it — whether a point stands within the band of a closed ray (`touches`), and whether a segment's line meets a piece — and two points of a segment against each other along it: whether two pieces meet the line at one point, where their tie is that point, and whether two roots of a tie are one double root, taken in closed form — through `link::metric::Distance`, `Margin::of`. For a segment of points an in-band reading joins (the roots it joins lie within the band of a candidate read); for an enclosure only a decided Zero does, since an enclosure read in band stands for points the band does not join | m | OK (new) |
| link.rs (`Link::lowest_on`) | link_offset_ball | a read candidate against the ball a refused candidate's distance from the boundary clears: its side of the plane tangent to the ball at the point toward it (unit normal along the line between them) — inside, the two points share membership, nothing of the boundary lying between — through `link::metric::Distance`, `Margin::of`. Only a decided inside lends a membership; otherwise the segment refuses | m | OK (new) |
| link.rs (`rest::by_round`) | link_round_offset | a point of a link's boundary, or the lowest point of a segment of it, against a sphere's or a cylinder's carrier: its signed distance from the carrier, positive away from the face's material (`Round::lowest_on`, a norm of metre coordinates less the radius) — through `link::metric::Distance::off`, `Margin::of`. A decided below confirms a crossing; in band refuses | m | OK (new) |
| offset_axial.rs (VERBS-RIMCAP) | offset_axial_cap_pair | sine between the two moved meridian caps' cross-section normals × each of the corner's own edge arc lengths — the length a near-parallel pair's solve error would move the corner by (the `offset_axial_corner` idiom on the derived pair) | m | OK (new in VERBS-RIMCAP) |
| offset_axial.rs (VERBS-RIMCAP) | offset_axial_cap_line | the moved caps' meeting line's own distance from the axis (`ρ_L`, a norm of metre coordinates) — `Zero` routes to the pole arm rather than refusing | m | OK (new in VERBS-RIMCAP) |
| offset_axial.rs (VERBS-RIMCAP) | offset_axial_datum_arm | the old corner's distance from its own profile circle's centre (2-D norm in the meridian half-plane, metres) — the carried-datum direction's lever, dead exactly when no direction exists | m | OK (new in VERBS-RIMCAP) |
| offset_axial.rs (`tangent_foot`) | offset_axial_tangency | a line–circle profile pair's gap `r − \|d\|`, the circle's radius less its centre's distance from the line (metres). `Zero` is tangent and names the foot, taken only where no root is determined; a definite side leaves the pair to the transversal route | m | OK (new in SHELL apex-and-tangent) |
| offset_axial.rs (`branch`) | offset_axial_branch_side | the old corner's offset from the pair's foot projected onto the unit direction to the upper root, `along / reach` (metres). Read only where the two roots tie: a definite side picks the root on that side, `Zero` leaves the foot to `tangent_foot` or the tie refusal | m | OK (new in SHELL apex-and-tangent) |
| offset_axial.rs (SHELL axial-closed) | offset_axial_rim_side | `(a × n)·(q_mid − c)` for the OLD rim's midpoint, the torus axis `a`, the cap normal `n` and the torus centre `c`: a projection of a metre vector — names which of the section's two torus curves is the rim | m | OK (re-spelled in SHELL axial-closed; new in CURVED-SPIRIC) |
| offset_axial.rs (SHELL axial-closed) | offset_axial_rim_sense | the old carrier's plane normal (a circle's `axis`, a spiric's `u_ref`) against the section curve's, a cosine of unit vectors levered at the body's radial extent | m | OK (re-spelled in SHELL axial-closed; new in CURVED-SPIRIC) |
| offset_axial.rs (cone × axis-parallel cap) | offset_axial_generator_side | the old generator's midpoint's distance from the moved pair's second line less its distance from the first | m | OK (new in OFFSET conic arc) |
| offset_axial.rs (cone × axis-parallel cap) | offset_axial_generator_sense | the old line's unit direction against the chosen one's, a cosine levered at the body's radial extent | m | OK (new in OFFSET conic arc) |
| offset_axial.rs (`forward_window`) | offset_axial_edge_turn | the turn-kept span less the read one, `keep_turn(s_old, t₁ − t₀) − (t₁ − t₀)` (radians, zero or whole turns) levered at the carrier's speed ceiling (`param_speeds`: a circle's radius; a spiric's `r(R − r)/√((R − r)² − d²)`). Routing only: `Zero` keeps the read end | m | OK (new in SHELL axial-closed) |
| offset_axial.rs (`forward_window`) | offset_axial_edge_window | a moved edge's window span `t₁ − t₀`, levered at the carrier's speed floor on a periodic carrier (`param_speeds`: a circle's radius, a spiric's tube radius — an arc in metres at least) and unlevered on a line (arc length). `Positive` required; else the edge has no forward window | m | **GAP** on an ellipse or a spline: `param_speeds` answers `None` there, so the window decides unlevered, an ellipse's radians as metres (`work/offset/an-ellipse-edges-offset-window-decides-radians-as-metres.md`). OK on a circle, a spiric and a line (SHELL axial-closed; was `offset_axial_rim_window`, spiric rims only) |
| offset_axial.rs (`param_on`) | offset_axial_edge_anchor | the distance from `q` to the new carrier's point a half turn on from the old parameter, less its distance to the old parameter's point — metres. `Negative` starts the read a half turn on, `Zero` or `Positive` at the old parameter; an escalation escalates | m | OK (new in SHELL axial-closed) |
| offset_axial.rs (`param_on`) | offset_axial_edge_carried | the old point's read on the new carrier less its old parameter (radians) levered at the carrier's speed ceiling (`param_speeds`) — an arc in metres at most. Routing only: `Zero` keeps the old parameter verbatim (a carried frame), a definite read is the old point's place in a new frame | m | OK (new in SHELL axial-closed) |
| offset_axial.rs (`reauthor`) | offset_axial_reauthor_plane / offset_axial_reauthor_end | a revolved point's moved start (end) corner's out-of-plane coordinate in its declaration's start (end) sketch plane — a rigid placement's inverse applied to a metre point, so a metre coordinate. Routing only: `Zero` keeps that end's azimuth, a definite side turns the declaration to the corner's own azimuth | m | OK (`_plane` routes since the equator-seam unit, where it had refused; `_end` new there) |
| offset_axial.rs (`reauthor`) | offset_axial_reauthor_azimuth | the same coordinate after the sketch plane has been turned about the axis onto the moved start corner's azimuth — zero to rounding when the plane contains the axis; a definite side refuses | m | OK (new in the equator-seam unit) |
| offset_axial.rs (`reauthor`) | offset_axial_reauthor_rise | an extruded point's extrusion vector's height off its own sketch plane (the placement's inverse applied to a metre vector). `Zero` refuses: no station along it is read | m | OK (new in SHELL axial-closed) |
| offset_axial.rs (`reauthor`) | offset_axial_reauthor_extrude_start / offset_axial_reauthor_extrude_end | a moved end's height off its sketch plane less the rise times its rest station (`0` start, `1` end) — a metre coordinate. Routing only: `Zero` keeps the rest station, a definite side re-reads it | m | OK (new in SHELL axial-closed) |
| offset_axial.rs (SHELL-7) | offset_axial_centre | a point's distance from the axis (a norm of metre coordinates) — a surface's centre, origin or apex at classification and a same-surface or two-chart circle's centre at the carrier mint, one helper (`centre_on_axis`) and one name for every site; unlevered, since a length carries its own scale and the extent levers only the sines here | m | OK (one name since SHELL-7; `offset_axial_latitude` and `offset_axial_seam_latitude` folded into it) |
| shell.rs (`wall_clearance`) | shell_walls_antiparallel | the drift `\|n̂_a + n̂_b\| · L` less one wall `t`: `\|n̂_a + n̂_b\|` is the chord between one outward normal and the other's reverse (`2·sin(δ/2)` for a tilt `δ` off antiparallel), `L = reach(a) + reach(b) + \|o_b − o_a\|` bounds the distance between two points of the pair (each reach the farthest corner of the face's footprint box, arcs included, from its plane origin), so the product is how far one plane drifts from the other across the pair. Not Positive reads the pair; Positive defers to `shell_walls_antiparallel_cosine`. A read pair's gap is taken short by the drift at `shell_wall_clearance`. `L` passes `gate_measured` (`shell_walls_extent`), so a poisoned footprint escalates | m | FIXED (was `−(n̂_a·n̂_b) − 1` alone, a bare cosine against the metre band: second order in `δ`, so the facing window was `δ ≤ √(2ε)` at every model size and a tilted pair inside it was measured as parallel) |
| shell.rs (`wall_clearance`) | shell_walls_antiparallel_cosine | `−(n̂_a·n̂_b) − 1`, a cosine of unit vectors, unlevered, read only when the drift row is Positive | rad² | OK as a window, not a coincidence decision: it keeps every pair the gate read before the lever, the wider window when `t/L < √(2ε)` (a thin wall on a long part), so the levered row only ever adds pairs. Zero reads the pair, and its gap is still taken short by the drift, so a pair read through this window is not measured as parallel either |
| shell.rs (`footprints_may_overlap`) | shell_footprint_separation | the gap between two grown footprint boxes along one in-plane axis of `a`'s frame (coordinates of metre vectors on unit directions) | m | OK |
| shell.rs (`wall_clearance`) | shell_wall_clearance | `\|(o_b − o_a)·n̂_a\| − drift − 2t`: the plane gap at the origins, less the drift above (every point of `b` stands within it of that gap from `a`'s plane), less the two walls | m | OK |
| shell.rs (`moved_walls_cross`) | shell_moved_walls_apart | the gap between two moved planar faces' boxes along one world axis (each box holds the face's boundary vertices and the ball of every edge off a line), metres. Positive on any axis skips the pair; anything else reads it | m | OK |
| shell.rs (`moved_walls_cross`) | shell_moved_walls_transversal | `\|n̂_a × n̂_b\| · L`: the sine between the two moved planes levered at `L`, the two boxes' diagonals plus the distance between their low corners (passed through `gate_measured` as `shell_moved_walls_extent`), so how far the planes diverge across the pair. Positive reads the pair; Zero is a parallel pair, `wall_clearance`'s when it faces and uncrossable when it does not | m | OK |
| shell.rs (`MovedWall::cut`) | shell_moved_wall_side | a boundary vertex's offset from `L`, the line the two moved planes share, along the face's in-plane normal to `L` (a metre vector on a unit direction). Negative is one side; Zero counts with Positive, so each vertex has one side and an edge ending on `L` is counted once. Read twice, Zero counted with Positive and then with Negative, and the two cuts united, so a point on `L` is a contact whichever side the face lies on. Escalates the pair at a vertex; at a point the arc refinement sampled, an escalation counts with Zero's side, and the pieces either side of the point reach `L` and keep their balls' cut | m | OK |
| shell.rs (`MovedWall::cut`) | shell_moved_wall_ball_reach | a spiric or spline piece's ball radius less the ball centre's distance from `L`, metres. Not Negative, an escalation included, adds the chord of `L` through the ball to the face's set or halves the piece (a superset read: the region between the arc and its chord lies in the ball) | m | OK |
| shell.rs (`MovedWall::cut`) | shell_moved_wall_ball_radius | a reaching spiric or spline piece's ball radius, metres. Positive halves the piece while the split budget lasts; anything else adds the ball's chord of `L` to the set as it stands, which then lies within twice that radius of the arc | m | OK (new in SHELL tilted-read-gaps) |
| shell.rs (`moved_walls_cross`) | shell_moved_walls_overlap | `min(hi_a, hi_b) − max(lo_a, lo_b)` for one interval of each face's cut of `L`, less each moved common edge's interval and each shared vertex widened to twice the escalation width either side, positions along the unit direction of `L` in metres. Positive refuses `OffsetsCross`; Zero is a touch away from where the faces are joined and refuses `OffsetsCross` too; an escalation refuses `Escalated` | m | OK |
| shell.rs (`moved_walls_cross`) | shell_moved_walls_beside_joint | the length of one piece of an overlap as a joint (a moved common edge or a widened shared vertex) is taken out of it, metres. Negative drops the piece; anything else keeps it for `shell_moved_walls_overlap` to decide | m | OK (new in SHELL tilted-read-gaps) |
| shell.rs (`MovedWall::cut`) | shell_moved_wall_order | the difference of two crossing positions along `L`'s unit direction, metres: orders a face's crossings before they pair into intervals (a selection, so a tie never breaks an ordering invariant). Only Positive reorders; Zero and an escalation are a tie that keeps walk order, moving an interval end by less than the escalation width, which the overlap row then decides with the same band | m | OK |
| shell.rs (`MovedWall::cut`) | shell_moved_wall_arc_split | a conic edge's parameter gap (radians) between a side extreme and an end of the edge, levered at the larger semi-axis — an arc length in metres at least. Positive makes the extreme an interior split of the edge; anything else leaves it out, so a split within the band of an end is that end | m | OK |
| stands.rs (`across_edges`) | stands_across_meeting | `s`, where the inward line `m + s·w` from an edge's midpoint meets a line or conic carrier of the face's loops; `w` is a unit vector, so `s` is the distance along the line | m | OK (JOIN, `join/annular-tube-roles`) |

Funnel bypasses found: **boolean/ops.rs:634/649** (`sign_within`
called directly on volume margins — was FLAG F3, **FIXED**: the gates
routed through `k_stats::decide` under `volume_backstop_operand`,
`volume_backstop` and `volume_backstop_violation`, and have since left
the operation path with the backstop). **This audit's
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

### Uncovered names (the D10 hold's twelve, of the 24 measured at `43e2998d`; re-measured at `9b882992`)

Of the 24 names this section listed at `43e2998d` (with
`contact_tangent_off_1`/`_2` added since), five reach no funnel site
any more (`carrier_in_seam_halfplane`, `carrier_on_iso_curve`,
`plane_nurbs_on_locus`, `plane_nurbs_hull_sup`, `props_band_coplanar`)
and seven have rows above (`plane_nurbs_transversality`,
`pcurve_iso_arc_row_direction`, `pcurve_chart_u_closed`,
`pcurve_chart_v_closed`, `pcurve_iso_seam_column`,
`pm_census_bound_end`, `pm_census_bound_vertex`). The twelve left sit
on D10 stage 4's ground and wait for it:

| home | names |
|---|---|
| `topo/boolean/carrier_eq.rs` | `carrier_sphere_center`, `carrier_sphere_radius`, `carrier_cyl_axis_parallel`, `carrier_cyl_axis_offset`, `carrier_cyl_radius` (a `[(&'static str, Margin<T>)]` table — carried, so no literal at the funnel site) |
| `topo/boolean/contact_verify.rs` | `contact_tangent_on_1`, `contact_tangent_on_2`, `contact_tangent_off_1`, `contact_tangent_off_2`, `contact_tangent_opposed`, `contact_tangent_parallel`, `contact_tangent_second_order` |

**The hole is wider than this table.** At `9b882992` the code half of
the instrument (above, read as multi-line calls, nineteen spellings,
test modules cut) finds **475** names written as a literal at a funnel
call, and **188** of them appear nowhere above this section. `work/predtail/D46.md`
carries them by file. A name leaves the hole only by acquiring a row in
the `## geom-brep` or `## topo` table, with `site`, `predicate`,
`comparand`, `dim` and `status` filled, plus a disposition entry in
*Findings* for any that comes back FLAG.

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
  adopted `tests/probe_f34_review.rs`. The backstop has since left the
  operation path (`[ev]` PR 5780), and those pins with it. The test
  oracle that replaced it forgives any wrong volume up to ε·(area A +
  area B), so it flags the plate's 3 mm cube at 1e-9 and 1e-12 and not
  at 1e-6 (`test_support_volume`'s module docs; its
  `the_slack_is_eps_over_the_operands_boundary` pins the slack).
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
  `an-open-sign-row-reds-main-at-1e-6-with-section-loop-mixed` (REACH, closed by PR 3716)).
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
  `pcurves.rs::joint_arm` reads the LOCAL lever — the vertex's distance
  from the axis, zero at poles/apex, which the walk exploits). The residue closed with F7:
  `param_rate` answers a NURBS carrier's certified speed lower bound,
  so the fitted and iso `pcurve_interval_forward` spans cross to the
  band as arc lengths through the metered door, with the meter itself
  gated as a length (`pcurve_interval_meter`, the collapsed-arm
  idiom); and the fitted lane's azimuth headroom takes
  `chart_sup_arms_at`'s lever, so the cone's arm is `v_sup·sin α` from the
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
- **F8** window/cosine family — **RETIRED**. `bool_wall_trim`'s cone
  term (cosΔ−cos h, quadratic near narrow/full windows) left with the
  cylinder and cone chart trims: those faces are read from their
  boundary edges (`boolean::ruled_region`), with no window. Its two
  chord-side members, `bool_between_arc_window` and the sphere-wall
  `split_arc_window` arm (R vs local parallel radius R·cos lat —
  over-generous near poles), are retired with the window selection: a
  chord takes the arc its pairing leaves along.
- **F9** `ssi_closure_tangent` (march.rs:484): cos × whole-branch arc
  length — an unbounded arm (nothing folds in `ctx.extent`).
  Arm-policy question for the design conversation.
- **F10** `transform_rigid_*` (`RigidMotion::check_rigid`): dimensionless
  rigidity residuals of the linear map against the metre band; the
  natural arm is the model/session-box extent. (The transform.rs
  collision claim is STALE as of the F3+F4 unit — the loft-assembly
  lane merged. Deferred on the arm question alone, which is a design
  input, not a conflict.)
- **F11** `tangent_sector_osculation` (rules.rs `apply_rule_a`) and
  `wall_bend_order2` (rules.rs `apply_rule_a`, through
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
  A revolve's axis is now written on the revolve itself, in its
  profile's own two coordinates (`Revolve`'s `axis_through` and
  `axis_heading`), so it cannot have an out-of-plane component to
  classify and there is no second frame to compare: nothing is left to
  decide, with no band and no scale. Both in-plane
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
  `decide_flagged(.., "F14")`, its one site. A ring's step is a value
  of a placement over an index (D10 `[repetition]`), decided by the
  mate's chart, and no pattern node reads this helper.
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
  `topo/src/clearance.rs` `clearance_margin` and
  `self_intersection_gap`: the two compares the E7 inner subdivision
  makes on a pair decided by DISTANCE. SELFX added the injectivity
  question's local certificates beside them, each its own row in the
  `injective.rs` block above; this entry is about the distance test,
  which decides every pair that does not touch. Both comparands are `d − c`, the interval enclosure of the
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
  distance between cells that do not touch in M (*Self-intersection
  scope*), so a coincidence at the run's tolerance is the violation the
  check exists to find. One name with
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

  **Retired (2026-10-16, `expr-non-finite-asks-value-times-zero`).**
  `value · 0` reads `[0, 0]` for an unbounded enclosure (`[MAX, ∞]`,
  `[0, ∞]`, `[−∞, ∞]`), so the check passed an expression whose value
  ran to infinity. `refuse_non_finite` now asks `Real::is_bounded`,
  which is not a band decision, so this site decides nothing and
  records no `expr_non_finite` K sample.

- **F19** (added by TRIM-3, the chart-boundary outside test)

  **Retired (2026-10-24, `clearance-cells-are-decided-in-model-space`).**
  `topo/src/chart_bound.rs` left the tree with its six rows: a cell is
  decided off its face's edges in model space (the `cell_region_*` rows above), so no chart quantity is metred and no
  chord polygon is walked. `loop_windings_joint_room` stays, as the room
  fence of `pcurves::loop_windings`.

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

- **F21** (added by PCERT's limb-2 wording unit, PR 5160)
  `geom-brep/src/ssi/certify.rs` `nurbs_limbs`, the image-miss tell
  (`ssi_image_miss_tell`): at the break where limb 2's composite
  `|S(P(t)) − C(t)|` reads past the band against a spline operand, the
  carrier's own foot distance from the surface, in metres. It is a
  length, so `Margin::of` would fit it dimensionally. What keeps it off
  the logged doors is WHAT it decides:
  - It states nothing. Limb 2 has already refused at that break.
  - The tell only names the refusal: `SsiLimb::ImageMiss` where the
    foot decides Zero, `HullValueChart` otherwise.
  - Logged, it would put a verdict, or in band an escalation, beside the
    certificate's own. A reader that lets the first escalation speak
    for a leaf (`editor_core`'s drive `log_read`) would then charge
    limb 2's refusal to the tell (executed by PR 5160's review: two
    escalations on the 2ε-off row).

  **Carried as `k_stats::check_unlogged(.., "F21")`**, the same door as
  F18 and F20. Not a `decide_flagged` site; `LEDGER_FLAGGED_SITES` does
  not move.

- **F22** (added by PATHS' sketch-plane frame unit)
  `geom-core/src/linalg/rigid_motion.rs` `RigidMotion::turned_between`,
  under the caller's site name (`sweep`'s `sweep_path_turn`, a path
  sweep's step turn from station `i`'s tangent mirrored in the chord
  onto station `i + 1`'s, the second reflection of the double
  reflection, `crates/sweep/README.md` `[sweep.frame]`): `1 + a·b`,
  the scalar part of the quaternion `(1 + a·b, a × b)` whose matrix
  the turn is, for `a`, `b` direction witnesses, evaluated as
  `|a + b|²/2`. Dimensionless — a defect of the pair, zero exactly when
  they are opposed — decided against the metre band, which is the
  clause-(i) debt this row records; the turn has no lever arm of its
  own (it acts on the section, whose extent is the caller's). The
  margin is where the turn stops being THE minimal rotation, so the
  decision sits on that rather than on an axis `a × b`, which would
  escalate on a nearly straight step where any axis serves. It is not
  also a conditioning threshold: the quaternion matrix is orthogonal to
  rounding at every angle, and with both parts evaluated through
  `a + b` it carries `a` onto `b` to a few ulps at every margin the
  band decides positive (the residue `(|a| − |b|)²/(2|a × b|)` is of
  order `u²/√ε`), so rigidity and carry are rounding-level wherever the
  turn builds. Same arm question as F10; one site,
  `LEDGER_FLAGGED_SITES` 5 → 6.

- **F23** (added by FLUSH's edge-meeting ray, PR 5422)
  `topo/src/boolean/solid_contain.rs` `crosses_by_turn`, under the
  ray's `winding` row (`bool_point_in_solid_winding`,
  `side_at_infinity_winding`): `(|turn| − π)·reach`, the turn of a
  planar face's loops about a ray's hit less `π`, levered by the
  selection's reach. It is read only where the ray was decided
  definitely clear of every edge of the face, so the turn is `±2π` or `0`
  to rounding and the margin `±π·reach`, definite by construction: it
  can never escalate, and as a logged verdict it would be a margin no
  value of the variables moves, counted by the flip lane and the tie
  census as if it could. **Carried as `k_stats::check_unlogged(..,
  "F23")`**, the same door as F18, F20 and F21. Not a `decide_flagged`
  site; `LEDGER_FLAGGED_SITES` does not move.

- **F24** (added by PATHS' stored-arc read-back unit)
  `profile/src/seg.rs` `read_back_arc` and its `read_back`, reached from
  the path door before a chain stores its arcs (`path::arc_reads_back`):
  a constructed arc read back against what was authored, all margins
  in metres:
  - `path_arc_start_on_carrier`: ‖a − centre‖ − r;
  - `path_arc_landing`: ‖landing(a) − b‖; for a `Facts::Decided` arc,
    the landing against `b` moved along its ray onto the landing's
    circle (the sweep's own error, without the distance from the centre
    its inline decision admitted);
  - `path_arc_through_point`: a `Via` arc's ‖q − centre‖ − r;
  - `path_arc_branch`: r·Δθ signed by the stated turn sense, an angle
    levered by its radius;
  - and, behind a read that is off, `<read>_resolution`: the lesser of
    the authored data's headroom `K·ε/(ulps·2⁻⁵²) − scale` and the
    miss's clearance `miss − ulps·2⁻⁵²·scale` of the read's own
    rounding at its stored magnitude (16 ulps for every read; the rim
    reads' derivation is at `seg::READ_BACK_RIM_ULPS`).

  A `Bulge` leg's authored bulge is first asked validation's own
  `segment_straightness` question (the signed sagitta,
  `seg::sagitta`, the one spelling both use), unlogged, and a line is
  not read.

  All are lengths, so `Margin::of` fits them dimensionally; what keeps
  them off the logged doors is WHERE they run. Each fact is a theorem
  of its construction over the reals, or holds to an inline decision
  (D1), and this reads the arithmetic that delivered it on the witness
  lane (`Witness::Inexact`) only: an exact scalar's stored arc encloses
  the real one, so it has no miss to read, and a row there would
  re-decide a registered identity on the enclosure's width. A logged
  point-scalar row would be one a box build cannot have.

  **Carried as `k_stats::check_unlogged(.., "F24")`**, the door F18, F20,
  F21 and F23 use. Zero passes (Positive, for the branch). Off with
  that gate Positive refuses `PathError::ArcMissesAuthoredPoint`; any
  other definite miss, and any miss that is not finite, refuses
  `PathError::ArcMissBelowSceneResolution`; in the band or poisoned
  escalates, for every arc.

  A sample under a `_resolution` name is a refused arc, so those
  populations are the probe lane's count of the read-back's refusals.
  Not a `decide_flagged` site; `LEDGER_FLAGGED_SITES` does not move.

- **F25** (added by NURBS' census-onto-one-line unit, PR 5513)
  `topo/src/census.rs` `span_charge`, under `pm_census_span_precision`:
  `((K − 1)·ε − noise) / rounding_charge(1)`, the room the projections'
  charge (`geom_core::one_line`) leaves below `(K − 1)·ε`, read back in
  metres of the two spans' summed length through the charge's own rate
  (the room itself never passes `Kε`, so the band could not read it
  Positive). A length, so `Margin::of` would fit it dimensionally.
  What keeps it off the logged doors is WHAT it decides: how precisely
  the census reads two collinear spans' ends, not anything about the
  body.
  - Definitely positive, the ends read uncharged (`pm_census_span_order`,
    `pm_census_ee_overlap`), sound inside the room.
  - Anything else, in band included, reads them charged: ends that are
    one point of the body (one point key) read equal without a
    coordinate, and every other comparison is decided at its value less
    and plus the charge (`pm_census_span_order_charged`,
    `pm_census_ee_overlap_charged`, their straddle
    `pm_census_span_rounding`), sound at every room, so the threshold
    may fall either way in its band.
  - Logged, an in-band threshold would record an escalation beside a
    reading that went on to decide.

  **Carried as `k_stats::check_unlogged(.., "F25")`**, the door F18, F20,
  F21, F23 and F24 use. Not a `decide_flagged` site;
  `LEDGER_FLAGGED_SITES` does not move.

- **F26** (added by D46's first slice) `bool_sphere_trim_pole_interior`
  — `topo/src/boolean/solid_contain.rs` `sphere_chart_trim`, on a
  boundary meridian edge of a sphere face: `(p̂·m̂ − cos(w/2))·R`, the
  cosine-window construction, asking whether a pole lies strictly inside
  the edge's span `w`. A length, but levered wrongly: for a pole `δ`
  inside an end the margin is about `R·sin(w/2)·δ`, not the arc `R·δ`,
  so on a narrow edge (`w/2 ≲ √(Kε/R)`) or a near-whole one it reads
  Zero for a pole definitely inside, the sibling F8 never named.
  `Zero` is taken as "at an end or outside", so the latitude fold would
  miss an interior pole and the window would stop short of it by up to
  `R·w/2`. By reading, today's `chord_join::face_azimuth_window` refuses
  a meridian edge through a pole (no closed-form chart image across it),
  which sends the face out of the class first; unverified by a run. The
  margin `geom_brep::props`' `span_membership` takes (the signed chord to
  the nearer end) is the shape a fix would read. Filed as
  `work/inside/sphere-trim-pole-interior-reads-a-collapsing-cosine-window.md`.
- **F27** (added by D46's first slice) `bool_ray_cone_disc` —
  `topo/src/boolean/solid_contain.rs` `carrier_roots` and the
  `PartialCone` arm of `ray_crossings`: the ray × cone quadratic's
  discriminant `b₂² − a₂c₂` over the face's reach (m²/m). A length, but
  near a tangency it is about `|a₂|·sin 2α·(ρ/reach)·δ` for a ray `δ`
  clear of the cone at radius `ρ`, so it under-reads by `|a₂|`, which
  the lead row only holds above `Kε/reach`. Measured: `α = π/4`,
  `ρ = reach = 1`, a ray 0.01 rad off the generator and 1e-6 m clear
  reads 5e-11. The cylinder's discriminant was corrected for the same
  missing division (F2, `bool_ray_cylinder_disc`'s `disc/|d⊥|²`); the
  cone's was not. The under-read only grazes a ray that is definitely
  clear or definitely crossing — decidedness, not correctness: a
  definite sign is the discriminant's own. Where the tangency lies past
  the reach the margin over-reads by `ρ/reach`, and every root there
  is outside the face. Filed as
  `work/inside/ray-cone-discriminant-under-reads-by-its-lead-coefficient.md`.
- **F28** (added by CLEAR's TIGHTEN unit) `topo/src/clearance.rs`
  `min_separation`, the region readings its walk takes
  (`Bracket::outside`, `Bracket::witness`): a cell's region
  (`cell_region`, the `cell_region_*` rows above) and a centre station's
  place on its face (the face's point door, the `bool_contact_*` and
  point-in-face rows). Every row keeps its own dimension; what this row
  argues is why their outcomes stay out of the verdict log.
  - Every outcome of every reading leaves a bracket true to the band.
    A cell read `Outside` holds no point of its face, so the pair drops
    out of the lower end; a cell the door cannot decide (`Zero`, in
    band, poison) stays in it. A station placed `In` is a point of its
    face; one placed `OnEdge` or `OnVertex` is an in-band decision and
    may lie up to the band outside the face, and either bounds the upper
    end; a station the door does not place stays out of it. So the upper
    end bounds the faces' separation up to the band, and an `AtLeast c`
    over faces within the band above `c` can read `Violated`: the
    kernel's ε model, not a defect. The readings choose which pairs feed
    the bracket and decide nothing a census must see.
  - `min_separation` runs inside an evaluation, under the
    `min_clearance` measure, at the interval lane only: the `f64`
    witness answers `MeasureUnavailable`. Logged, its readings would be
    rows in the leaf's vector that the witness's cannot have, and every
    leaf would refuse `flip_crossing`; its in-band escalations would
    hand the driver a cue to bisect that no smaller parameter box
    answers (measured on M10-6's neck and notch fixtures: every call
    carries 137–186 of them).
  - What the scope does not hide. The symbolic tier's decision counts
    (`SymCounts`) still count the in-scope decisions, and they reach
    the drive verdict's `content_key` through its serialized receipt,
    which is where a receipt belongs. And the readings' K samples keep their names, which
    is vacuous at this site today: `Probe` never runs `min_separation`,
    since only a certifying scalar holds the engine's door. If a
    `min_clearance` document enters the driver's K population at a
    certifying scalar, its 137–186 in-band samples per call would fire
    k-lint rule 1; that is a finding to re-derive then, per the K-REPORT
    runbook, not a reason to log them.

  **Carried as `k_stats::unlogged("F28", ..)`**, the scoped door whose
  one-margin case is `check_unlogged`: its verdicts and escalations are
  discarded. `MinSeparation::decided` counts the readings as a receipt
  line. Not a `decide_flagged` site; `LEDGER_FLAGGED_SITES` does not
  move. `geom-core/tests/flagged_census.rs` checks that every
  `unlogged` and `check_unlogged` site cites a row this ledger has.

- **F29** (added by CLEAR's DIRSUPPORT unit) `topo/src/clearance.rs`
  `clearance_support_gap`, at `apart_along`: the distance test's second
  compare, asked of a cell pair whose box margin at F17's site is
  indeterminate, and by the injectivity question's choice between a
  touching pair's certificate and the distance test. The comparand is
  `L − c`, where `L` is the gap between the two cells' supports along
  one direction `n`, over `|n|`, and `c` the requested clearance (`0`
  under the strict question). Each support is the hull of `⟨P, n⟩` over
  the cell's restricted, dehomogenized control points
  (`NurbsSurface::rect_net`), a plane's own affine form, or the cell's
  box. `L` is a lower bound on the cells' separation, so the comparand
  is honest METRES against the linear band, and **the site is not
  flagged**; this row is the disposition, and `LEDGER_FLAGGED_SITES`
  does not move.
  - Only `Sign::Positive` is read, and it discharges the pair under
    either bound. `L` bounds the separation from below and from
    nowhere above, so a `Zero` or a `Negative` here is no coincidence
    and no violation: the pair goes on to F17's indeterminate arm as
    before. One name serves both bounds for that reason; F17 needs two
    because its `Zero` is read two ways.
  - `n` only chooses which bound is decided. The bound holds along any
    nonzero `n`, so `n` is a raw `f64` read of the cells' centre
    normals and centres, not a certified quantity, and the choice
    among three directions is a raw compare that decides nothing.
  - Logged, like F17: it is a decision of the bound sweep, and its
    population is the pairs F17 leaves indeterminate.

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
  name reaches no funnel site. What the note guarded against was live
  at `props_rim_interior_side` until the sphere's iso flux arm retired
  that name, and was guarded differently there: σ was a product of two
  DISCRETE signs and never became a comparand.
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
  The sphere's half retired with the sphere's iso flux arm, which
  Gauss–Bonnet replaced: `props_rim_only_extent` is the cone's alone
  (`cone_apex_level`), on the cone's own bare slant comparand.
- **N3** The cone's `du_of_rims` arm is the FIRST rim's radius
  |v|·sinα (bounded below ≳ K·ε by the same axis-class guard). The
  `T::one()` fallback is REACHED — both callers compute the arm before
  they know whether there is a rim — but **never metered against**:
  the one route from it to a margin refuses on the empty rim list
  first, by `du_of_rims`' opening `is_empty` on the flux lane. The
  material-side gate no longer reads rims at all
  (`props_cone_area_side`, the cone's vector area against its axis),
  and refuses an open loop on its closure decision (`ChartLoopOpen`);
  `s58_iso_rectangle::a_rim_free_cone_refuses_at_both_doors` is the
  row for both.
- **N4** `tangent_normal_parallel`'s arm 1/κ_rel is the ratified D4 ¶1
  tangency lever ("normal-parallel within θ ⟺ within ε of the locus");
  unbounded only in the refusing direction, and the second-order gate
  fires first. The fallback lever (the folded arm, where the
  second-order margin refused and `1/κ_rel` does not exist) reads only
  on a path that already refuses.
- **N5** `ps_frame_seam`, `pcurve_sphere_chart_frame` and
  `split_conic_phase_frame` (and `split_chart_azimuth_frame`, retired by
  PCERT B1: its read goes through `geom_brep::Azimuth::principal`) are
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
