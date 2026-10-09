# CARVE — log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/carve/plan.md`. A/B band 5600–5699
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-17) — opened in BLEND's cut

Opened by BLEND's orchestrator as BLEND's one successor on BLEND's
ground, per `work/README.md`: *residue is re-homed before the sweep …
to a new program opened for it when the residue coheres into a track
of its own (a dozen items on one territory are a successor's opening
slate, and the closing program opens it)* (Ev, 2026-09-06), applied as
VIEW applied it to itself on 2026-09-17: the parent stays open, the
rows move by `git mv`, each track's charter is the sentence true of its
rows and false of the others'.

**BLEND is not closed by that act and is not closed by this one.** Its
unit order is landed to unit 12, unit 14 and unit 15 are at their gates
and unit 13 is walked as blocked; its thirty-two live residue rows were
review accretion on one crate and its neighbours, and twenty-one of
them cohere into this track. BLEND stays open with its three unit rows,
its exit walk is a separate `[ev]` step, and `docs/DOC-LEDGER.md`
records the sweep when it happens.

21 rows arrived from `work/blend/`, each by `git mv` with its body,
its id and its history unchanged — no row's prose was edited on the way
past, and the item schema carries no `program:` field, so a re-home is
the move and nothing else. The same cut placed eight rows on PATHS (the
profile fillet door), one on SYM, one on TOPO and one on META;
`work/blend/log.md`'s cut entry carries the table and the charters.

Nothing dispatches until the opening sitting writes the unit order.

## BLEND closed (2026-09-17)

BLEND's exit walk (PR #2826, "merged!" from Ev in chat) closed the
parent the day this program opened; the sweep (`docs/DOC-LEDGER.md`
sweep 17) deleted `work/blend/`. Four rows arrived with the walk —
`S90-impl` (BLEND's unit 13, blocked on PROPS' `H5`),
`contact-edge-arm-is-picked-from-the-carrier-kind-not-the-dihedral`,
`ruled-band-keys-a-d-hole-rim-on-the-caps-outer-cycle` and
`corner-config-recourse-and-policy-assert-a-default-for-any-tag` — so
the slate is twenty-five rows. Every path in `paths` is this program's
alone now; the "BLEND stays open" clause left `keep_out`.

## Announced seam from TOPO (2026-09-24)

TOPO's `kevs-fan-merge-needs-a-re-describing-kill-door` (branch
`topo/kev-describing-door`; PR title "TOPO: kev refuses a merge that
would strand a carrier; kev_describing takes the re-descriptions")
lands Ev's ruling (c) on PR 2527. `Body::kev` stays keys-only and now
refuses, before mutating, a fan merge that would re-base a certified
edge onto the surviving vertex (`EulerOpError::MergeRebasesCarriers`,
naming every such member) or move one end of a null edge
(`RebasedNullEdge`). It carries a merge that moves nothing: an empty
fan, or a killed null edge. `Body::kev_describing(he, &[(EdgeKey,
EdgeCurveSpec<T>)], tol)` is the kill that takes a band and the merged
members' re-descriptions. A listed member is certified at the merged
endpoints. An unlisted one passes the re-basing gate `mev`'s fan site
passes. `kev_describing(he, &[], tol)` is the kill with a band and
nothing re-described.

**Your files, and what changed in them.**

- `crates/sweep/src/blend/surgery.rs` (BAND's and CARVE's): the two
  closure kills take `kev_describing`. `rim_phase`'s is
  `"rim closure kev"` and `rim_phase_annulus`'s crossing kill is
  `"annulus closure kev"`; the refusal site names are unchanged. Each
  hands its one merged member the chord between the endpoints the
  merge gives it (`EdgeCurveSpec::line_between`, `merged_chord_spec`),
  the scaffolding the surgery's struts and trims carry. That member is
  the upper meridian remnant, or the mate seam's rim-side piece. The
  final pass still states the slit as the band's seam, the meridian
  arc, through `attach_contact`. (The first push handed the arc under
  arc scaffolding; at the certified scalar that scaffold's residual
  enclosed wider than eps = 1e-12's band and the kill escalated, so the
  interval 1e-12 rows went red. The chord encloses at ulps.) Without
  this change these were 118 of the 129 sweep refusals S93 measured.
  At the head, `cargo test -p sweep --lib --test all` shows 1746
  passed and 7 ignored, at default eps and at 1e-12.
- `crates/sweep/src/revolve/full.rs` (CARVE's): the full-revolve
  zip's two kills take `kev_describing(he, &[], tol)`. Each one merges a
  copied vertex into its coincident original across a certified
  closing circle, and the merged fan keeps its carriers, which are
  re-certified under the run's band. The keys-only kill takes no band,
  so it would refuse these merges.
## 2026-09-24 — seam: TOPO re-worded one comment in `blend/surgery.rs`

TOPO's `half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete`
(branch `topo/mint-rows-at-the-mint-site`) re-worded the comment above
the surgery's closing `mint_pcurves` in `crates/sweep/src/blend/surgery.rs`
("the input's caches are stale the moment the first strut lands"):
`mev`, `mef` and `mekr` now leave a face with complete rows complete
or rowless, never half-minted, so the pass is described as minting the
faces the surgery builds and re-deriving the rest. No code in the file
moved. The same unit's sweep run measured the fillet surgery's
"annulus mate trim mef" meeting states the closed-form lane cannot
mint mid-surgery; the operators leave those faces rowless for this
pass rather than refusing.
## 2026-09-27 — a note from ATREST: a P0 filed on CARVE's slate

Posted by the ATREST orchestrator so it is seen at CARVE's next sitting.
ATREST-4 (PR #3190, merged) widened tier 3's check 6 to planar loops
carrying arcs, and its review turned up a producer defect on CARVE's
ground: `work/carve/sweep-cap-plane-winds-against-a-convex-arc-region.md`.
A profile whose outer boundary carries a large CONVEX arc (the row's
C-shape, a 350° arc) passes `Profile::validate`, and `extrude`,
`loft_body` and a partial `revolve` all mint BOTH caps inside out —
`cap_points`' Newell sum over the vertices plus one apex per arc winds
against the region. Tier 3 certified these bodies until ATREST-4; it now
refuses exactly the two caps, pinned in `m5_s10_face_sense.rs` by rows
that go red when the verbs are fixed. The row carries the repro and the
shape of the fix (orient the cap by `profile`'s arc-exact winding).

Signed: (ATREST orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)

## Seam from TOPO, fix pass on PR 3161 (2026-09-29)

`Body::kev_merged_members(he)` is new and public: the merged members
of `kev(he)`, in the dying vertex's orbit order, each with the two
endpoints the merge gives it (`topo::MergedMember { edge, start, end }`,
`he_plus` forward order). It is read from the same plan and endpoint
reading the two kill doors certify against, so a caller no longer
re-derives what the merge will do. `kev`'s plan phase now also refuses
`OrbitBroken` where the dying vertex's orbit reaches a half-edge that
does not start there (two `next` tears could walk it through the killed
half, and the describing door then panicked); on a valid body nothing
changes.

**Your files, and what changed in them.**

- `crates/sweep/src/blend/surgery.rs`: `merged_chord_spec` now reads
  the member's merged endpoints from `kev_merged_members` instead of
  re-deriving them (its old rule, `start == dead`, agreed with the
  door only on valid bodies). It takes the refusal site name, so a
  read-door refusal is reported as `"rim closure kev"` or
  `"annulus closure kev"`. The chord it hands the kill is unchanged.
- `crates/sweep/src/blend/surgery.rs` `"rim kev"`,
  `crates/sweep/src/blend/open/planar.rs` `"corner kev"` and
  `crates/sweep/src/blend/open/ruled.rs` `"cap vertex kev"`: each is a
  spur kill, so the keys-only kill merges no fan. Each now says so and
  `debug_assert!`s it through `kev_merged_members`, where before the
  claim was only measured.
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `sweep/src/blend/surgery.rs`, `sweep/src/extrude.rs`, `sweep/src/lib.rs`, `sweep/src/loft.rs`, `sweep/src/revolve/full.rs`, `sweep/src/revolve/partial.rs`. The sweep walls state `wall_sense` in their spec (the post-mint `set_face_sense` calls are gone), caps state `true`, and the transient hole discs state `false` (their loop becomes the cap's ring). (TOPO implementer)
- 2026-09-30 — Seam note from TOPO: In PR 3513 (branch `topo/every-escalation-names-its-decision`), `geom_brep::enters_material`, `enters_material_order2` and `classify_dihedral` return `LeverEscalation { rung: LeverRung, diag }` (the arm gate or the reading) instead of a bare `Indeterminate`, and a decided-zero arm carries its decided margin (`geom_core::k_stats::decide_positive_reported`) where it carried `INVALID`; `sweep::extrude` and `sweep::revolve::upgrade` read `.diag` and behave as before, a zero arm's payload now quoting its margin; the dropped rung is filed at `work/issues/section-arm-guards-escalate-untyped-and-certify-reads-the-dihedral-arm-as-transversality.md`. (TOPO implementer)

- 2026-10-01 — Seam note from AUTHOR: Ev ruled on #3551 that an extrude's distance is a positive depth with a structural `side`, and that a negative depth refuses with a recourse naming `side`. The work is filed on EDIT as `extrude-distance-is-a-depth-and-a-side`. Its kernel half is `sweep::Extrusion`/`ExtrudeError` (CARVE/STRUT) and its eval wiring `wire_extrude` (WIRE). The rule's follow-ons are `carve/revolve-angle-is-a-signed-size-beside-a-directed-axis` and `edit/pattern-spacing-is-a-signed-size-beside-a-direction`. (AUTHOR orchestrator)

- 2026-10-01 — Seam note from PROPS (`props/recourse-grammar`, the last unit of that program): the D4 ¶1 (i) recourse GRAMMAR moved in `geom-core`, so refusal text changed across the tree. `COINCIDENCE_RECOURSE`, `NO_DECLARATION_RECOURSE` and `SPLIT_PLANE_RECOURSE` lost their unvalued `", or lower the tolerance"` tail and are now the LEVERS alone; `DEFINITE_COINCIDENCE_RECOURSE` retired into `COINCIDENCE_RECOURSE` (with the tail gone the two were one string). The valued conditional arm has one home, `geom_core::Indeterminate::ending(levers)`, composed through `MarginDiag::sized_recourse`: a site that holds an escalation gets "Recourse: {levers}, or, if this size is intended, tighten the tolerance below {m/K} m", and loses the offer exactly where the margin gives no value. `Indeterminate`'s own `Display` (and `under`) therefore renders a LABELLED recourse now, with each margin kind's first lever folded inside it, so `test_utils::refusal::recourse_markers` counts 1 where it counted 0. `MarginDiag`'s invalid rendering says "NaN or a refused enclosure", not "poisoned". Assertions written as `contains(COINCIDENCE_RECOURSE)` followed the constants; literal pins of "lower the tolerance" did not and were re-baselined. (PROPS implementer)
- 2026-10-01 — Claim from BAND: `full-revolve-emits-split-planar-walls` moved to `work/band/` (id kept, `parent: swept-continuation-walls-reach-the-boolean-unmerged`). Ev ruled the two rows the same way ("construct"), so they land as one builder rule on branch `band/sweeps-build-one-wall-per-run`: extrude and revolve build one wall per run, and a full revolve's planar walls are one face. Paths touched on CARVE/STRUT ground: `sweep/src/{swept,extrude}.rs`, `sweep/src/revolve/{full,partial,mod}.rs`. (BAND implementer)
- 2026-10-01 — Seam note from PCERT: `extrude-mints-no-pcurve-rows` moved to `work/pcert/` (claimed by `git mv`, id unchanged). Ev ruled on PR 3617 that a curved face's pcurve rows are mandatory at rest (C4 in `crates/geom-brep/README.md`), so a producer that does not mint now reds tier 3; PCERT's at-rest unit (branch `pcert/at-rest-rows-mandatory`) makes `sweep::extrude` mint and will touch `crates/sweep/src/extrude.rs`. (PCERT orchestrator)

## 2026-10-06 — sitting opened; the D10 hold read; CARVE cut on its priority seam

An orchestrator holds CARVE again (`status: active`). Before planning,
it read the D10 hold (BAND's log, 2026-10-03, carries the text; CARVE's
own log never received it) and DESIGN.md D10, and checked every row
against the hold's list.

**Three rows are on the hold's ground** and are parked on
`d10-one-way-to-say-intent-is-unbuilt`, each with its reason in its
body: `half-revolve-caps-are-never-an-operand` (the boolean's
undeclared-coincidence refusal), `loft-walls-keyed-per-segment-on-a-declared-carrier`
(declared continuations),
`loft-between-opposite-turning-joints-reverses-a-seam-between-stations`
(the declared-cusp exemption and the undeclared-tangency refusal).
A fourth, `revolve-angle-is-a-signed-size-beside-a-directed-axis`,
looked held (a node slot's shape) until the pattern-step row showed Ev
had already answered it on PR 3941: the angle stays signed, and what
is left is refusal text. It went to CARVETAIL open.

**The cut.** CARVE measured 62.5 budget points against 30. It keeps
its seven P0 rows, 30 points once the two legacy `D` rows are priced
`M`. Six unpriced rows were priced. Two went to STRUT, whose charter
they fit: `extrude-arc-walls-are-ruled-in-n-not-w` (one wall rule
spelled one way for line legs and another for arc legs) and
`sweep-body-makes-every-caller-derive-its-start-frame` (five spellings
of one derivation). The other P1–P4 rows and the three parked ones went
to the new CARVETAIL (`work/carvetail/`, band 11000–11099), which
opens `ready`.

Dispatch order and review posture: `work/carve/plan.md`.

Signed: (CARVE orchestrator)

## 2026-10-06 — first dispatches

**Built now**, each as its own cloud session (this box has four cores),
each with a single FULL review to follow (`plan.md`, Review posture):

- `sweep-cap-plane-winds-against-a-convex-arc-region` →
  `carve/cap-winds-with-the-region`. One home for the cap's
  orientation, read from the profile's arc-exact winding. Seam:
  PATHS' open PR 4169 touches the same verbs.
- `skin-coincident-section-check-is-an-unbanded-f64-compare` →
  `carve/one-door-for-coincident-sections`. One banded decision about
  section distinctness, and one refusal that is true in every regime.
- `self-closed-link-sharing-its-vertex-records-two-junctions` →
  `carve/self-closed-link-counts-its-vertex-twice`. Reproduce first.
  Seam: `blend/` is shared with BAND (active) and STRUT.

**Weighed first** by an Opus and a Fable designer each, concurrently,
on the same problem statement and no candidate solutions. The labels
were blinded at dispatch on `analysis/design-fork/carve-2026-10-06`:

- the placement of a loft's sections
  (`self-overlapping-spines-build-and-validate` with
  `two-section-loft-with-an-inverted-top-normal-builds`);
- `loft-v-parameterization-is-the-first-strips-so-a-rolled-section-changes-the-body`;
- `intersection-pair-order-is-unpinned-and-extrude-disagrees-with-itself`.

Signed: (CARVE orchestrator)

## 2026-10-06 — the three design rows weighed; everything weighed is built, none went to Ev

All three designer pairs reached one recommendation, and none of them
changes text Ev ratified, so none became an `[ev]` PR or a fork-log row.

- **`Intersection`'s pair is unordered.** The two designers agreed in
  round 0. They split on one detail, whether the constructor refuses
  equal keys; the orchestrator kept the refusal at the certification
  door (D4). Built: `carve/surface-pair-is-unordered`, a cloud session.
- **The loft's v is a function of the whole section set** (Eq. 10.8
  over every control row, summed in sorted order), and **a sweep's v
  is its path parameter.** Converged in round 0. Built:
  `carve/loft-v-is-the-whole-sets`.
- **A loft promises an embedded boundary, certified at its door.**
  Round 0 split. Round 1 crossed on the reversed list, so a second
  round followed (the 2026-09-30 crossover rule), and the pair
  converged: the contract is embedding, the instrument is the clearance
  engine in `topo` at a certifying scalar, the f64 lane certifies by an
  exact lift once its price is measured, the stacking fold retires, and
  a reversed list builds. The interim the pair agreed on (the fold also
  reads the far normal) is built as `carve/fold-reads-the-far-normal`
  and closes `two-section-loft-with-an-inverted-top-normal-builds`.
  `self-overlapping-spines-build-and-validate` becomes the
  door-certificate unit, parked on SHELL-3 and on two rows filed on
  CLEAR today. CARVETAIL's opposite-turning-joint row is re-parked from
  the D10 hold onto it. Retiring the fold changes S-BOOL's Q2, an agent
  recommendation (#1373) with no wording of Ev's found. The f64 lift
  sits beside Ev's #1737 ruling for `shell`, so Ev is told now and again
  when the certificate unit is specified.

The coincident-sections lane was steered by message: the sliver
decision lives in `skin.rs`, not in the fold, which retires.

Signed: (CARVE orchestrator)

## 2026-10-06 — `self-closed-link-sharing-its-vertex-records-two-junctions` closed (PR 4185)

`walk_chains` counts every link at both its ends, so a self-closed link
holds two ends at its one vertex. Such a link walks alone: closed with
no junction when nothing else is there, and a corner beside anything
else. Reproduced on real links (the dome's equator rim with another
rim's start moved onto its vertex); no body the tree builds reaches it.

Review: single FULL (Opus). It ran eight extra incidence shapes in every
request order and 485 blend-family tests. It traced the lane's two open
points and found that the fix turns a possibly silent outcome into a
typed one (`resolve_annulus` refuses `UnsupportedChain` before
carving). It found no MAJOR. Its fix pass corrected three comments the
fix had made false (the walk's rule is now stated in link ends;
`cap_incidence`'s manifold premise), and its every-order row joined the
suite. Two rows were filed, both P3/M:
`corner-valence-reads-a-self-closed-edge-once` (five readers count a
self-closed edge once, against the walk's ends) and
`a-walked-chain-closes-at-a-corner-without-a-junction` (pre-existing;
the cube's top loop plus one vertical).

Signed: (CARVE orchestrator)

## 2026-10-06 — `skin-coincident-section-check-is-an-unbanded-f64-compare` closed (PR 4186)

Whether two adjacent loft sections are apart is one banded decision at
the loft door. `loft_body` and `sweep_body` share a private `build`
(validate → stacking fold → skin → assemble), and every pair not apart
refuses `DegenerateStacking { slab }` at every scale, the ~1e-16
underflow included. The skin's residual compare is an exact structure
check, `SkinError::NoParameterStep { section }`, whose text never claims
the sections coincide and whose one lever is to move section i away
from section i−1. `loft_parameters` now goes through `validate_loft`.

Review: single FULL (Opus). It found no MAJOR. It probed closed
non-planar sweeps with bit-equal stations, rotated coincidences,
denormal steps and the Interval scalar. Its fix pass:
- dropped the range lever, which was not apt for a pinned hinge;
- added the census row;
- scoped `loft_parameters`' doc;
- made the `DegenerateStacking` text true of the crossing case;
- added two rows: a sweep with bit-equal stations, and a hinge pinned
  only up to rounding.

The last pins a silent defect, pre-existing at the merge base:
`loft_geometry` returns `Ok` with control coordinates around 3.5e15 for
a 2-unit section. Filed on CARVE:
- `a-wall-pinned-between-two-loft-sections-refuses-at-the-wrong-door`
  (P1, H, design);
- `sweep-places-vanishing-tangent-is-a-bare-f64-compare` (P3, M,
  design);
- `loft-doors-take-a-non-finite-placement` (P3, M).

The fold-retirement note is on the certificate unit's row. Steered
mid-unit: the orchestrator first asked for the decision in `skin.rs`,
then withdrew that on reading the PR, whose two-layer structure states
two true facts.

**On the gate's 1e-6 row.** The fix-pass head was red only on
`rest_zip_admission::the_tangent_lever_keeps_building_pure_contacts`.
That row failed identically on a clean `origin/main` (`a9c038c37`): ZIP's
admission fix pass `290d95a31` added it, and the per-PR gate runs the
1e-6 row only for a diff touching `sweep`. A later main reportedly fixes
it; a note is on ZIP's log. Main is now red at 1e-6 on PATHS'
`one_segment_loop::a_split_through_the_seam_builds_as_the_two_arc_form_does`
(from PR 4169); a note is on PATHS' log. This PR merges once its own
rows are green, with any red confined to rows that are red on main.

Signed: (CARVE orchestrator)

## 2026-10-06 — `sweep-cap-plane-winds-against-a-convex-arc-region` closed (PR 4187)

`sweep::swept::cap_plane` is the one home for all six cap sites
(extrude's, loft's and the partial revolve's two each). It keeps
Newell's plane over `cap_points` and flips it exactly when one decided
comparison, the K predicate `cap_plane_orientation`, says Newell's
normal opposes the region's (± the sketch normal, by the verb's
`reverse` and the cap's end). The C-shape's caps now point out of the
material in extrude, loft and partial revolve, at f64 and Interval.

**Ruled mid-unit: B2.**
- A (the placed sketch normal) failed CI with 13 rows in `editor-core`
  and `step-export`. The placed `c2` is a far looser enclosure at
  Interval than Newell over the actual points, and that is a geometric
  reason, not output stability.
- C (an arc-exact vector area) would have been a third copy of the
  circular-segment formula.
- B2 is bit-identical wherever Newell already agrees. The reviewer
  dumped every face surface and vertex on base and branch to confirm.

Review: single FULL (Opus). It found no MAJOR. Its fix pass:
- The orientation refusal now carries the escalation's payload and the
  shared `KERNEL_DEFECT_ENDING`. It had forwarded a recourse menu that
  offered "declare the coincidence".
- The new arms are in the concision census, plus two rows.
- The right-handed-frame claim is a stated precondition that cites
  PATHS' `sketch-plane-holds-the-affine-and-the-witness-dies-at-the-read-boundary`.
- The lane's overstated TESS row became the true
  `a-thin-arc-bounded-face-refuses-as-corrupt-geometry-at-a-coarse-delta`.
## 2026-10-06 — `two-section-loft-with-an-inverted-top-normal-builds` closed (PR 4188)

The stacking fold also decides section k+1's normal against slab k's
displacement, under the same `loft_stacking` band. A far section that
does not face along the stack refuses `FarSectionNotForward { slab }`,
and an in-band far reading escalates as its own
`FarStackingEscalated`. The downward-facing top section and an interior
section facing back (z = 0, 1, 0.5 with normals +z, −z, −z) now refuse
at the door; before, one built and validated and the other refused
opaquely at an Euler certification.

Review: single FULL (Opus). It found one MAJOR, which matters beyond
this PR: **the designers' argument for the interim was false.** The
check refuses embedded, correctly oriented bodies (a hood whose top
turns 100°; an oblique arc sweep), because 3-D rings can turn edge-on
to the stack and stay simple. The orchestrator ruled to keep it as a
DISCLOSED CONSERVATIVE interim. Today the inverted-top loft builds
silently, and a false refusal is the cheaper failure in a charter whose
subject is bodies that should refuse. The old near check already
over-refuses the mirror case. Every sentence that claimed "every
refusal is a fold" was corrected. The over-refused bodies are pinned as
rows that should build once the certificate lands, and that cost is
recorded on the certificate unit's row. Filed on CARVE:
`a-reflected-loft-placement-evades-both-normal-checks` (P1, M).
**A class finding:** an argument a designer pair agrees on is still a
claim to falsify. This one survived two reconciliation rounds and fell
to a reviewer's first probe.

Signed: (CARVE orchestrator)

## 2026-10-07 — `loft-v-parameterization-is-the-first-strips-so-a-rolled-section-changes-the-body` closed (PR 4193)

A loft's v is now a function of its whole section set: Eq. 10.8 chord
length over the outer loop's control rows, with per-section shares
sorted. Rolling a section about its own normal, or relabelling its
vertices, no longer changes the body. `loft_body`, `loft_parameters`,
`sweep_body` and `sweep_geometry` share the one helper.

Three rulings during the unit:

- **The tube: holes read the outer loop's parameters (option B).** The
  outer loop is structurally unique, so the rule stays label-free. A
  hole cannot move the outer walls. The tour's scaled-hole tube holds
  its identity to 1.9e-16; averaging the holes in left it 5.1e-5 off.
- **The coil: sweeps take the loft's rule for now.** The designers'
  path parameter was built, and it refused the square coil at
  assembly. The root cause is the frame law: the sweep frame is a
  minimal rotation from the START tangent, not a rotation-minimizing
  frame, so it spins the section where the path runs back
  anti-parallel. The path parameter is deferred to the frame-law row
  `sweep-frame-is-a-minimal-rotation-from-the-start-tangent` (P1, H,
  design). It is deferred, not dropped.
- **`lily_leaf_b`: a tour wall pinned to QUAD (lily wall 17, default ε
  only).** Under the new v, its volume escalates on QUAD's in-band
  convergence arm (margin −2.7e-9). That is
  `quadrature-convergence-test-escalates-instead-of-refining`, and the
  evidence is added there. QUAD has no orchestrator, and the fix needs
  the C3 factoring, so the wall is the honest interim. `finding_13`
  skips only its Pappus containment, and says so.

Wall 15 was retired and then restored across the merges; it ends as on
main. The cap-plane order row
`a-loft-caps-plane-is-summed-in-the-authored-vertex-order` was measured
at P2. Frames: the nonuniform loft bulges as derived (1.646 at 32.6% →
1.853 at 38.5%); the tube's bend shifts slightly; everything else moves
at pixel level. The gate is ok on the head that carries main through #4215.

Signed: (CARVE orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4366, merged): `geom_brep::enters::LeverEscalation`'s `rung` and `diag` are private; read them with `rung()`/`diag()`, re-quote only through `with_diag`, which keeps the gate's verdict. `BooleanError::of_lever_rung(gate, read, rung, diag)` is the one boolean spelling. The dihedral lever-arm decision is told in one shape ("long enough, for how its faces curve, to measure their angle"), with the lever "clearly longer and no face curves tightly there"; pin `validate::tests::the_dihedral_arm_is_told_in_one_shape` (it reads source literals: a natural "long enough … angle … face" wording elsewhere trips it). (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4401, merged): `LeverEscalation::with_diag` is gone. The one re-quote door is `quoting_reading`; ask `re_quotes()` first. A decided arm keeps its verdict only onto a reading of the same rejected sign, and otherwise is returned unchanged. The dihedral arm clause is `geom_brep::DIHEDRAL_ARM_CLAUSE` / `dihedral_arm_clause!()`; compose it, never re-spell it. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4411, merged): `geom_brep::must_carry_over_edge` now reads the second-order sagitta on every pair (tier 3's walk), so an in-band sagitta escalates `InBand(SecondOrder)` out of lane too; `UnderDetermined` out of lane means every station read Positive (or a station read Zero/Negative). An out-of-lane all-Positive pair now costs `CERT_SAMPLES−2` `tangent_second_order` samples. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4416, merged): `geom_core::lever_recourse(lever, note)` is the one spelling of a lever-alone ending, and `Indeterminate::undecided(subject, ending)` the one "{subject} is undecided: {payload}. {ending}"; compose them, do not re-spell. In `topo::boolean::refusal_routes`, `Ending::Lever` is now `Lever(&str)` (`LeverPass` is gone). Rendered texts are unchanged. (ENCL orchestrator)
