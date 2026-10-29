---
id: validity-refuses-an-interior-chart-singularity
kind: issue
title: Validity refuses a revolution-chart face whose loop is rims only (a pole or apex interior to the face)
status: open
opened: 2026-09-18
refs: [rim-only-sphere-cap-panics-at-census, import-normalizes-the-rim-only-cap]
priority: P0
cost: H
---


Filed by the TESS orchestrator from Ev's ruling on `[ev]` PR 2850
(2026-09-18, in chat): **a chart singularity inside a face is a vertex
of it** — a loop of rims only, with the sphere pole (or cone apex) in
the face's interior, is not a face of this kernel. Deliberately not in
DESIGN.md (Ev: provisional, "too much weight" there); this unit states
the rule in validity's own docs, present tense.

Tiers 1–3 accept a sphere face bounded by one latitude circle with the
pole interior (measured: `validate`, `validate_closed`,
`validate_geometric` all `Ok`). The ruling says that statement is not a
face, so validity refuses it — by the structural fact (the face's loop
on a revolution chart classifies with no meridian), in whichever tier
owns rim/meridian classification; the unit says which and why. The
Euler door is the one native way to state it
(`crates/topo/tests/mesh12_rim_row_reach.rs::two_level_rim_cap`).

Lands AFTER `work/foreign/import-normalizes-the-rim-only-cap.md`. With it,
props' rim-only arm (PR 2741: `sphere_rim_only_pole_level`,
`require_rim_only_closed`, and the rim-only half of
`rim_interior_side`'s callers) is unreachable through a valid body and
retires in the same unit or a PROPS one; the arm is whole at
`6a1f6d60d`, which `work/tess/consider-emitting-the-rim-only-cap-instead-of-normalizing-it.md`
records for the tabled alternative — repoint that row at the retiring
commit's parent.

Signed: (TESS orchestrator)

## Evidence from TESS-1 (fix pass, 2026-09-20): one fact, three spellings, two classifiers

TESS-1 landed `mesh`'s refusal of this face
(`TessellateError::MeridianFreeCurvedFace`, raised by
`walk::require_a_meridian`), and its two reviews independently named
the same CLASS. It is filed here because this unit will need exactly
the predicate in question.

**The structural fact is one — "the face's loop has no meridian" — and
the kernel spells its consequence three ways today:**

- `geom_brep::props::curved`, the flux lane: `DegenerateFace` for the
  cone apex cap and the one-rim cylinder face — a banded VALUE decision
  (`require_extent` on levels that carry no extent), not a statement
  about the loop (`work/props/cone-apex-cap-refuses-degenerateface.md`);
  the sphere member is admitted and measured by folding the pole in
  (`sphere_rim_only_pole_level`).
- `props`' shape door, torus arm: `NotIsoRectangle { what: "torus face
  without a meridian" }` (`torus_parse`).
- `mesh`: `MeridianFreeCurvedFace { face, surface }`, every kind.

**And two classifiers decide it.** `props` reads it off its own
boundary parse, twice — `!b.rims.is_empty() && meridian_axes.is_empty()`
in `sphere` and again (as `meridian_axes.is_empty()` under the
rim-only extent test) in `boundary_material_sign`, and
`meridians.is_empty()` in `torus_parse`. `mesh` reads it off
`topo::chart_iso::classify_kind`'s `TravKind`s (`walk::LoopKinds`).
The two classifications agree on certified carriers and are not the
same code.

**What both reviewers proposed**: a kinds-only predicate — does this
loop's classified traversal list contain a meridian — homed in
`topo::chart_iso`, beside `iso_side_starts` (which `topo::coherence`
already walks with the same kinds). This unit's validity rule is that
predicate asked at tier 2 or 3; when it exists, `mesh`'s
`walk::require_a_meridian` should cite it rather than keep
`walk::LoopKinds` as a second home, and `props`' three reads are
candidates for the same citation (PROPS' call). Two things the
predicate's author should know from TESS-1's measurements:

1. It is an EXISTENCE test, and existence is not enough to make the
   face meshable: a rim-only cap wearing a meridian spur that stops
   short of the pole has a meridian and still has the pole in its
   interior, and the one-seam sphere has only meridians and zero width
   (`crates/mesh/tests/loops_the_meridian_guard_admits.rs`;
   `work/tess/rim-free-loop-on-a-poleless-chart-meshes-as-a-hole.md`).
   "A chart singularity inside a face is a vertex of it" is the rule;
   "the loop has a meridian" is one necessary condition of it.
2. `classify_kind` splits circle carriers on a float
   (`|axis · chart.axis| > 0.5`); the kinds are structural only after
   the carriers are certified (props' `require_iso_rectangle`). A
   validity rule asked BEFORE that certification inherits the caveat.

When this unit lands, `MeridianFreeCurvedFace`'s doc sentence "bodies
carrying the face can pass `topo`'s validation as it stands" becomes
false and is this unit's to correct, with the rows in
`crates/mesh/tests/meridian_free_face.rs` that assert tier 3 `Ok` on
the cap bodies.

## A second consumer, and a second predicate (TESS-5, 2026-09-22)

TESS-5 added the mirror premise on the other axis:
`mesh::TessellateError::SingleColumnCurvedFace`, raised by
`walk::require_two_columns` when a loop has no rim and every iso side it
opens is carried by ONE edge — so every meridian of it stands on one
chart column and it bounds no domain. Members closed: the torus face
bounded by one meridian circle, the one-seam sphere, the one-generator
cylinder and the one-generator cone
(`crates/mesh/tests/loops_with_no_rim.rs`).

**The same siting argument applies, and more sharply.** This predicate
reads `topo::chart_iso::iso_side_starts`' own answer plus the EDGE each
opening belongs to, so it is `iso_side_starts`' immediate consumer — the
function both reviewers named as the neighbour the meridian predicate
should live beside. If the kinds-only predicate moves to
`topo::chart_iso`, this one belongs in the same move, and
`walk::require_two_columns` should cite it. `topo::coherence` already
walks the same kinds with the same rule, so it is a candidate consumer
too.

What the predicate's author should know, on top of the two notes above:

3. This one is not an existence test but an INCIDENCE test, and the
   quantity it reads is the openings' edge identity — available at tier
   2/3 from the loop cycle without any geometry beyond the
   classification. It refuses exactly the loops for which a zero extent
   is FORCED; it does not certify that an admitted loop has width — two
   coincident edges open two columns that are one, which is
   `work/tess/two-coincident-edges-open-two-columns-that-are-one.md` and
   a rung above (carrier identity, not edge identity).
4. Its ε is `iso_side_starts`' separation band and no other. A validity
   rule asked at a different band will draw the line elsewhere on a
   junction within ε of the chart axis — measured unreachable from
   every minting door probed (`walk::iso_side_starts`' docs carry the
   sweep), but a tier-2/3 spelling owes the same statement.

`SingleColumnCurvedFace`'s doc says "bodies carrying such a face are
refused by tier 3 today — for a reason of tier 3's own, never for this
one". If this unit gives tier 3 the reason, that sentence is this unit's
to correct, and `crates/mesh/tests/loops_with_no_rim.rs` carries the
tier-3 assertions that would move.

## Evidence from the sphere arm (FLUX `flux/sphere-arm`, 2026-10-10)

- **The ruling is ratified.** Ev answered `[ev]` PR 4549 ("this
  works"): *a chart singularity inside a face is a vertex of it* now
  bears a refusal, and `crates/geom-brep/README.md` C12 (7) states it.
- **Its scope is wider than rim-only loops.** Props' sphere arm now
  reads every sphere face by Gauss–Bonnet, and `sphere_loop_side`
  reads a one-loop face as the side of its loop holding no chart pole.
  Where a pole lies off the loop on EACH side, the face holds a pole
  under either sense, so the reader answers `Unencoded` and the face
  measures under its bit. That covers the rim-only cap, a tilted cap
  around a pole. This row's rule must refuse any sphere face with an
  interior pole, not only a meridian-free one.
- **The kernel-minted witness is fixed.** The fillet's oblique corner
  patch (`crates/sweep/tests/m5_pr12_fix_pass.rs`,
  `f4_an_oblique_trihedron_builds_and_passes_tier_3`) held its chart's
  south pole inside the corner triangle; the blend now poles every
  corner patch on one of its feet (`sweep::blend::arms::corner_chart`,
  `work/band/a-blend-corner-patch-holds-a-chart-pole.md`), and
  `crates/sweep/tests/corner_patch_chart_poles.rs` holds every corner
  face of a corpus to `Encoded`.

The sphere's rim-only arm (`sphere_rim_only_pole_level`,
`rim_interior_side`) is gone from props with the iso flux branch, so
the retirement this row describes has happened on the props side.

## Rows that move when this lands (FLUX, PR 4746)

`crates/step-import/tests/fixtures/seam-region/side_cut_equator_split.step`
is a side-cut ball whose sphere is split at the equator, so each face
holds a chart pole off its loop. `poles_inside_a_seam_region.rs` pins two
things about it: it imports and measures, and with both sphere faces
flipped check 6 reads no side for their seam region, leaving the flip to
check 7. Check 6 reads a pole that lies inside a face as nothing, because
this row is open. `side_cut_meridian_split.step` holds a pole inside a
face too. When validity refuses such a face, all three rows in that file
go red at import, and so does `tier_gate`'s corpus row. Re-point them
then: `sphere_region_material_sign` may read a pole on no member's loop
as outside the region once this rule is enforced.

## Re-aimed by FOREIGN's designer pair (2026-10-29)

Both designers on `import-normalizes-the-rim-only-cap` found the kinds-only "the loop has no meridian" predicate is not the rule (this row's own FLUX and TESS sections say as much): a tilted cap holds a pole with no meridian in its loop, and a meridian spur stopping short of the pole has one and still holds it. The rule both reached, form-neutral: **every chart singularity in a face's closure is a vertex of it**, read off the face's region. Import's door (that row's unit) uses the same reading, so validity refuses exactly what import re-mints. When that row closes this one's trigger fires; re-open it then on this rule. (FOREIGN orchestrator)

## Import's door has landed (FOREIGN implementer, 2026-10-29)

`import-normalizes-the-rim-only-cap` closed: STEP import makes every sphere pole and cone apex in a face's closure a pole-form vertex (`NormalizationKind::ChartSingularityVertex`; the reading is `crates/step-import/src/normalize/singularity.rs`, `scan` and `holds`, on the pre-body `SolidSpec`, at the coarser of the file's band and the run's). So no committed import fixture ships the state this row refuses, and the seam-region rows named above now read normalized bodies (`poles_inside_a_seam_region.rs` asserts every flipped sphere face is named); they should not go red when this rule lands. The Euler doors still state the face (`crates/topo/tests/mesh12_rim_row_reach.rs::two_level_rim_cap`, `mesh`'s `witness_bodies::one_circle_cut`).

## Grown by `[ev]` PR 6014 (2026-10-29, fork row 215: the wrap tip)

The rule is now DESIGN `[topology]`'s, both directions at tier 3: every chart singularity in a face's closure is a vertex of it, AND every wrap edge's tip lies on its face's chart singularity (refusing a meridian that stops short of the pole: `spur.step`, which passes tiers 2–3′ with the tier-2 exemption alone). Tier 2's "no valence-1 vertices" becomes "no valence-1 vertices but a wrap edge's tip", read off the stored wrap flag (structural, no ε). The two-half form import mints today (PR 6007) satisfies both until revolve and import flip. Read the tip's incidence as a certified reading of its pcurve end on the chart's singular set, not a point-to-pole distance at a band. (FOREIGN orchestrator)

## Scope at dispatch (RESTCURVE orchestrator, 2026-10-29)

This row builds the first rule only: every chart singularity in a face's closure is a vertex of it. The rule reads the face's region, form-neutral as the FOREIGN re-aim says. It holds today: import normalizes to the two-half pole form (PR 6007), and that form satisfies the rule.

The wrap-tip half from PR 6014 is now `tier-3-admits-a-wrap-tip-only-on-its-faces-chart-singularity`. That covers tier 2's valence-1 exemption and tier 3's tip-on-the-singularity. It is parked on the shared loop walk, which Ev asked to land first. This row adds no exemption and admits no tip.
