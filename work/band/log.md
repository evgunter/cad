# BAND — the log

## 2026-09-20 — opened

Cut out of CARVE, which was carrying 73 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed CARVE's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

9 rows arrived by `git mv` with their ids, bodies and history
unchanged. CARVE keeps its band 5600-5699; band 7800-7899 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

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
## 2026-09-25 — first sitting: taken, wave 1 dispatched

Taken by the BAND orchestrator; `status: active`. The review-posture
question the plan left open is gone: the A/B it inherited is
suspended, so review is per unit by the orchestration tiers.
`in-band-corner-verdicts-…` arrived unpriced and is P1/E (one routed
sentence), which puts the slate at 30/30.

Wave 1, three lanes on disjoint ground, each a FULL single review
(each changes what a refusal or a name says, where believing it takes
more than reading it):

- `ruled-band-keys-a-d-hole-rim-on-the-caps-outer-cycle` — branch
  `band/ruled-d-hole-ring-crease`.
- `blend-slit-name-collides-when-two-rims-share-a-meridian` — branch
  `band/band-slit-discriminator`. Announced seam: EDIT's and EMIT's
  `names/role.rs` and EDIT's `names/emit_blend.rs`.
- `subdivided-profile-side-coplanar-walls-gate` — branch
  `band/subdivided-side-walls`. Announced seam: CARVE/STRUT's
  `extrude.rs`, TOPO's boolean gate if the answer is a refusal.

Decided without Ev (sequencing, with a recommendation):
`sweep-emits-no-contact-record-for-declared-cusps` closes by
`Extruded` carrying the declarations, not by ratifying caller
ownership (reason in the plan), and runs after the subdivided-side
row since both are `extrude.rs`'s lowering. `S90-impl` waits on
SCALAR's LANE-4 (#3194).

Ev, in chat the same day: the review posture is the per-unit tiers
("it is indeed replaced by the per-unit review tiers"), and
`Extruded` carrying the declared cusps is the close ("carrying the
declared cusps sounds great").

## 2026-09-25 — `subdivided-profile-side-coplanar-walls-gate` closed (PR #3244)

Measured, no kernel change; reviewed by the orchestrator's read (a
test suite and three filed rows, nothing a reader has to take on
trust). The item's own premise was backwards: extrude and revolve DO
give a declared continuation's walls one surface key; what refuses is
that nothing merges them before the boolean, and the recourse the
refusal names (`merge_coplanar_faces`) has no door above
`topo::Body`. Filed: `swept-continuation-walls-reach-the-boolean-unmerged`
(P0, the fork — goes to Ev, `needs_ev`), `subdivided-rim-fillet-refuses-at-the-collinear-joint`
(P1), and CARVE's `loft-walls-keyed-per-segment-on-a-declared-carrier`
(P2). Blind spot the lane named: `revolve/tube.rs` unmeasured.

The fork goes to Ev as an `[ev]` PR editing F7. The declared-cusp row
(`sweep-emits-no-contact-record-for-declared-cusps`) was waiting on
this lane's `extrude.rs` ground and is now free to dispatch.

## 2026-09-25 — the continuation-merge fork ruled

Ev ruled option 1 on the `[ev]` PR: sweeps over a declared straight
continuation run the structural merge as a documented final stage
(F7 amended in the same PR). `swept-continuation-walls-reach-the-boolean-unmerged`
becomes the implementation row; it dispatches after the declared-cusp
lane lands, because both change what `Extruded`'s face handles mean.

Ev also pointed at the new orchestrator protocol (PR #3247: a designer
pair weighs every design fork before its `[ev]` PR, and orchestrator
branches keep the program prefix even when the harness pins another).
Applied from here on, not retroactively; this sitting's orchestrator
branch moves to `band/orchestrator`.

## 2026-09-25 — note from GERM: one merge seat for the sweep

GERM found that a FULL revolve of an axis-touching profile emits every planar
wall as two same-key halves (`work/carve/full-revolve-emits-split-planar-walls.md`),
and has put it to Ev. Both designers recommend the same fix: the revolve runs
the structural merge as a final stage. When `swept-continuation-walls-reach-the-boolean-unmerged`
is implemented, please make it ONE final-stage merge call at the end of the
sweep, not a continuation-runs-only targeted merge. Otherwise the full-revolve
row has to widen it again. — (GERM orchestrator)

## 2026-09-25 — `ruled-band-keys-a-d-hole-rim-on-the-caps-outer-cycle` closed (PR #3243)

Built rather than re-refused: `chord_site` cuts in whichever cap cycle
(outer or ring) carries the keyed half-edge, so a ruled crease ending in
a cap's ring carves. Full single review. Its one MAJOR: the lane's claim
that ring creases are concave-only was false — a keyhole gives a convex
one, which this PR turns from a refusal into a carve, and a bore inside
the removed sliver then carves silently wrong. Taken as: land the build
(the outer-cycle route to the same silent-wrong body is already on main)
and close the defect as the NEXT unit; the keyhole is pinned as an
ordinary row. Filed by the lane and its fix pass:
`ruled-cut-off-leaves-a-cap-ring-inside-the-removed-sliver` (P0, now
with both routes), `body-not-intact-renders-every-detail-as-a-reference-that-did-not-resolve`
(P2), ZIP's `blind-d-pocket-subtract-refuses-with-join-internal-words`,
ATREST's `check-9-does-not-check-a-ring-nested-inside-another-ring`, and
evidence on CONTACT's `axis-coincident-lap-trips-the-planar-join-invariant`.

GERM's note above (one final-stage merge call at the end of the sweep)
goes into the continuation-merge unit's brief verbatim.

## 2026-09-26 — `blend-slit-name-collides-when-two-rims-share-a-meridian` closed (PR #3245)

The teapot lid's rims roll in ONE `Node::Fillet`. The kernel record
(`BlendNaming::slits`, `meridian_splits`) carries the slitting band's
source edges, and `RoleSeg::BandSlit`/`BandCross` carry that set as a
discriminator — option (a), because N4 names from birth data alone. The
item undercounted: `BandCross` collided one row later the same way.
Full single review, no MAJOR; its fix pass took the reviewer's rows
(the one that matters: an unrewritten `band` silently retargets a held
slit name, and nothing else in the suite saw it), made `band` a
discriminator in `walk_names` rather than a derivation, and gave the
band identity one home. `annulus-rim-phase-…` gained the note that its
hand-kept retain is now load-bearing for `BandCut` uniqueness. Filed:
INSTR's `tess-budget-baseline-names-predate-piece-naming`.
## 2026-09-26 — `sweep-emits-no-contact-record-for-declared-cusps` closed (PR #3257)

`Extruded`, `Revolved` and `Lofted` carry `declared_contacts`, one
`Tangent` pair per declared cusp joint. Full single review, then a fix
pass that went past the review's recommendation on the orchestrator's
call: the review said land and file the upstream move; instead the
validated profile now keeps the cusp/smooth kind the author declared
(`ValidatedLoop::cusp_joints`, decided once in `judge_joints` through
the path door's own predicate, recorded in the guided replay), so the
sweep reads it and its duplicate predicate and three public error
variants are gone; `blend_arcs` stopped listing an arc between cusps.
Announced seam: PATHS (`crates/profile`). Delta review: mergeable, two
MINORs filed as `declared-joint-kind-zero-margin-reads-smooth`. GATHER's
`product-gate-refuses-a-declared-cusp-sweep-the-verb-now-declares` now
has its measured red-first row.

## 2026-09-26 — `ruled-cut-off-leaves-a-cap-ring-inside-the-removed-sliver` closed (PR #3271)

The silent-wrong path the D-hole unit widened is shut: before any `mef`,
a convex ruled cut-off meters every cap edge but the two rims it
shortens — other rings and the cut cycle's own other edges — against a
region enclosing the removed sliver (annulus about the section centre,
cut by a half-plane), and refuses `RingClearance` on the cap. Full
review found two MAJORs: the first meter tested straight edges by their
infinite line (a square drive hole on a D-shaft's axis refused), and the
cut cycle's own edges were unmetered (an L-channel into the sliver carved
silently wrong — the lane had filed it P1 "unmeasured"). The fix pass
gave edge metering one windowed home (the ladder rim's outer walk now
uses it too) and closed both; a delta review found them closed by rows
that go red on revert, plus one MINOR (the sentence said "removes" on
concave bands), fixed by `RingClearance` carrying the chain's convexity.
Remaining false refusals are disclosed on
`ruled-cut-off-builds-a-bore-wholly-inside-the-removed-sliver` (P2).
Filed: `cap-sliver-floor-arc-term-and-whole-circle-arm-are-unpinned` (P2),
`arc-window-membership-has-three-spellings` (P1).
- 2026-09-28 — Seam note from ENCL: PR 3382 (merged `9bf495c768`) adds `geom_brep::recourse`, the one table for sized decisions. `Reading`/`RefusedArm` moved there from `certify`, alongside `SizedPass`, `SizedDecision` and `Classified`. certify and the offset meters both route through it. The shared unreadable-margin note now reads "an unreadable or collapsed margin may indicate a kernel bug worth reporting". Filed on your slate: `work/band/blend-endings-say-lower-the-tolerance-and-route-by-name.md`. `sweep::blend::ClassifiedMargin` is a third spelling of a decided margin beside `recourse::Classified`; convergence is noted on the encl certify-span row. (ENCL orchestrator)
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
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `sweep/src/blend/surgery.rs`. `blend/surgery.rs`'s three re-charts moved from `set_face_surface_and_sense` to `set_face_surface`, bit unchanged. (TOPO implementer)

## 2026-10-01 — `cap-rim-smooth-arm-decides-by-argument-not-by-the-rule` closed (PR #3667)

The row's diagnosis was wrong: the arm is reachable at 1 < K < √φ (Ev's
PR 2119 ruling admits any K > 1), so it is routed through the must-carry
rule rather than made unreachable; stored descriptions are unchanged
(plane pairs read UnderDetermined). The verdict → description mapping
gained one home, `MustCarryVerdict::description`, used by all four smooth
arms. Single review (style + claims), then a fix pass and a delta review;
both clean. Filed: CARVE's `extrude-arc-walls-are-ruled-in-n-not-w`,
CLEAVE's `topo-smooth-arms-decide-descriptions-outside-the-must-carry-rule`,
and the residue row `blend-split-rows-and-must-carry-prose-residue` (P4).

## 2026-10-01 — `annulus-rim-phase-keeps-a-second-spelling-of-the-split-provenance` closed (PR #3670)

The annulus rim phase splits its seams through `split_fragment`, which
now takes the expected source and refuses inside the one home if its
lookup disagrees; every annulus row names `frag.source`, as the ladder's
`slits` row does. `blend_surgery`'s debug postcondition checks the SOURCE
half of every birth row (an exhaustive destructure of `BlendNaming`). No
name moved (sweep, editor-core naming, tour teapot rows). Full review,
fix pass, delta review; the review's row
`every_band_crossing_names_the_seam_its_foot_split` is what tells a
right source from a merely valid one. Residue (the ladder still names
`meridian_splits` by the split key) is on
`blend-split-rows-and-must-carry-prose-residue`.

## 2026-10-01 — the three recourse rows and ENCL's ending row closed (PR #3690)

`corner-config-recourse-and-policy-assert-a-default-for-any-tag`,
`every-escalation-carries-the-coincidence-recourse-first`,
`in-band-corner-verdicts-route-to-the-corner-configuration-recourse` and
`blend-endings-say-lower-the-tolerance-and-route-by-name` closed as one
unit. `BlendError::Escalated` carries a closed `BlendDecision` (11
decisions at 13 construction points); subject, lever and ending are
exhaustive matches; no blend refusal renders "declare the coincidence";
sized endings go through `geom_brep::recourse`. Full review, fix pass,
delta review, a last small pass, and a python-census fix (`BlendDecision`
listed interior beside `BlendSite`). Known cost: `ContactSecondOrder`
offers no tolerance until ENCL's must-carry row says which reading
escalated. Filed: ENCL's `recourse-table-has-no-lever-only-ending` and
`must-carry-in-band-verdict-does-not-say-which-decision-escalated`;
BAND's `dependent-normals-refusal-carries-no-margin-for-its-ending`,
`dependent-normals-names-a-run-out-policy-the-tag-says-cannot-help`.
`blend-surgery-invariant-ends-in-a-tag` priced P4/E.

## 2026-10-01 — continuation walls ruled: construct (PR 3647)

Ev approved the designer pair's converged answer, superseding the
2026-09-25 merge-stage ruling: sweeps build one wall per cosurface run;
F7 is back to boolean outputs as the one op with a merge stage. The row
`swept-continuation-walls-reach-the-boolean-unmerged` is now the
implementation (fork-log row 36; A = Fable, B = Opus).

## 2026-10-01 — `arc-window-membership-has-three-spellings` closed (PR #3700)

The row's "three spellings" were two questions: a selection ("may this
angle bracket lie in this periodic window" — sweep's `CircleFrame::misses`,
geom's curve boxes, and a fourth copy found in `mesh/src/cert.rs`), now one
home `geom::periodic_window_may_hold(phi, window, period)`; and topo's
decided in/out/in-band membership (`arc_trim`, `chart_azimuth_margin`),
a different question that keeps its own code, argued once at `arc_trim`.
`misses` asks in its relative frame and is exactly the old predicate on
its domain (pinned against an oracle with a census row). `CircleMargins`
stays: an unsigned per-piece distance reads an enclosing ring as clear.
Single review, fix pass, delta review, last pass.

## 2026-10-01 — `subdivided-rim-fillet-refuses-at-the-collinear-joint` closed (PR #3701)

A rim of collinear plane–plane links between the same two faces, meeting
at valence-2 joints, fillets and chamfers as ONE band face
(`OpenBand::admit`'s `Joint`, one `joint_verdict` shared with predicate 2;
per joint one kef of the lower strut and one kev of the spur), named
`BandFace` of the chain's sorted edge names. Predicate 2's clearance screen
groups a joined run as one feature and checks each joint's foot exactly
against the trimlines at the run's ends; nothing it cannot read is skipped.
Full review, fix pass, delta review, last pass. Merged with two red rows
that are main's own (filed there: REACH's 1e-6 `SectionLoopMixed`, PROPS'
thread-count golden); every other row green. Filed:
`ruled-band-refuses-a-joint-on-one-support-pair`,
`unmerged-subdivided-wall-rim-fillet-names-no-merge-recourse`.

## 2026-10-01 — `annulus-rim-host-outer-boundary-is-metered-only-by-the-sampled-screen` closed (PR #3715)

Every closed rim's host AND mate outer boundary is now metered in closed
form (`support_boundary_clearance`), not only by predicate 2's sampled
screen: four silent-wrong carves (a notch or a tilted cut whose closest
approach falls between sample stations) now refuse `RingClearance`.
Non-line/circle edges are bounded by geom's certified boxes with the
ellipse's exact height range, and a refusal decided by a box says so
(`bounded`). Full review (MAJOR: an ellipse ball assumed `major ≥ minor`),
two fix passes, delta review. Merged with two red rows that are main's own
(as #3701). Filed: `face-clearance-screen-skips-boundary-edges-it-cannot-read`,
`support-boundary-meter-reads-a-co-requested-edge-at-its-stored-place`,
`support-boundary-meter-bounds-other-carriers-by-the-whole-carrier`.

## 2026-10-01 — S90 ruled: the blend stays differentiable (PR 3724)

Ev approved the designers' converged answer: the blend doors stay generic
under DL5; H-R3's fillet third is retired. `S90-impl` becomes the pin
(dual tangent rows), P1/M. Fork-log row 40 (A = Opus, B = Fable).

## 2026-10-02 — `S90-impl` closed (PR #3764)

The blend's dual tangents are pinned: a sweep row checks a filleted and a
chamfered cube's `Dual64` volume tangent against central differences
(blend size and an upstream cube size seeded) and the fillet against two
closed forms; an editor-core row differentiates a stack-up measure
through a 3D fillet. Each row reds under a tangent-freezing mutant
(radius read through `.lo()`; chamfer distance; edge point). Reviewed by
the orchestrator's read (tests only, mutants demonstrated). Filed TINT's
`dual-tangent-is-unpinned-for-seven-recipe-verbs`.

## 2026-10-02 — continuation walls closed: sweeps build one wall per run (PR #3736)

Closes `swept-continuation-walls-reach-the-boolean-unmerged` and the claimed
`full-revolve-emits-split-planar-walls` (Ev's "construct" ruling, fork-log
row 36). Extrude and partial revolve build one wall per run of collinear
profile pieces (`swept::wall_runs`), a station being an `mev` chain with no
strut; a full revolve collapses each run (`kef`/`kev`/`kemr`), so a planar
wall in a solid is one face. Walls are stored per run; names go through
`PieceRun` (a one-piece run is spelled as its bare locator, so bodies with
no multi-piece run keep their names) and one constituents view
(`names/merged.rs`). No `merge_coplanar_faces` is left on the sweep path.
Dual review, class H (DR-38): both NOT-MERGEABLE-AS-IS on the
same two CI-demonstrated MAJORs, tally 0; the union was taken in one fix
pass, plus a delta read. Filed: `swept-cocircular-arc-runs-build-one-wall`
(P1), `lamina-plane-annulus-keeps-its-slit`,
`swept-operand-merge-repairs-left-in-tests`, `swept-run-walls-style-residue`,
SHELL's `cap-rim-refusal-order-follows-the-arena`. Merged over main's two
filed reds (thread-count digest; the 1e-6 open-sign row).
## 2026-10-02 — `face-clearance-screen-skips-boundary-edges-it-cannot-read` closed (PR #3786)

Predicate 2's boundary-pair screen reads each support loop whole
(`screened_loop`) and refuses what it cannot read instead of skipping it.
Two arms are reachable on a tier-1-valid body through `fillet_edges`: a
lone-vertex ring and an uncertified boundary carrier, both now
`UnsupportedGeometry` naming the entity (each pinned by a row that reds on
main). The other three (unresolved face, loop/cycle, half-edge) are torn-body
states, refused `BodyNotIntact`. `touches_any` refuses an unresolved edge
rather than reading it as apart, and the surgery's three
`filter_map(get_half_edge)` member reads became its typed no-cycle refusal.
Reviewed by the orchestrator's read (cost E).

## 2026-10-02 — a P0 landed from SHOW: `a-plane-plane-blend-cannot-end-at-an-unrequested-corner`

Filed by SHOW's `split-node-chords-by-name-has-no-demo` (PR 3842) and
raised to P0 / cost H / `design: true` at that PR's review: chamfering
or filleting one edge of a box refuses `UnsupportedRunOut`, a face's
whole rim `ChainNotG1`, and only all twelve edges build. Both doors are
in the row's scope. The planar band's end geometry is a design fork in
the shape of FILLET-H7's (Ev's ruling on PR 1736): two designers first.
The bracket scene pins it live (walls 1–3). Its sibling
`a-blend-refuses-a-solid-of-several-shells` landed at P2.

## 2026-10-03 — HOLD: a refactor of dependency, placement and intent is underway (Ev, `[ev]` PR #3990)

Ev has opened a redesign of how a document says that one thing depends
on another and that things are meant to coincide. The question and Ev's
direction are `work/recipe/one-way-to-say-dependency-and-intent.md`;
the design lands through `[ev]` PR #3990. The direction, in short: no
node consumes another; no raw numbers (every slot holds a variable);
nodes are operations on typed variables; no absolute coordinates
(spaces are what is related to what, placements are relations); tangency
and coaxiality by construction; checked assertions replace declared
contacts; contact and tangency complaints become lints where the
answer is already known.

**Do not start a new unit that meaningfully uses** any of: the node
vocabulary's edges and consumption (`Node::inputs`, product roots),
`Expr`/document parameters and literals, placement (`Datum`
coordinates, `Transform`, `Pattern`/`PlacedUnion` frames, gauges,
offsets, mates and their solve), declared pairs and declared contact
(`Boolean`/`Union` `declare`, `ContactClass`, continuations, seams),
the undeclared-coincidence and undeclared-contact refusals, axis
declarations, `ParamSource`, the parameter-coincidence lint, or
`Measure`/`Assertion`.

**A unit already started may be finished**, even where it collides with
the above — land it as planned. Park each row the hold covers
(`status: parked`, `blocked_on: [one-way-to-say-dependency-and-intent]`,
so the row fires when the ruling closes). If that leaves your program
with nothing it may start, set its `status` to `blocked` and stop.

## 2026-10-03 — the intent refactor's hold now waits on the build, not the ruling (Ev ratified #3990)

Ev ratified DESIGN.md D10 on PR #3990, and the ruling
`one-way-to-say-dependency-and-intent` is closed. The hold announced in
the entry before this one CONTINUES until D10 is built: it now waits on
`work/recipe/d10-one-way-to-say-intent-is-unbuilt.md`. Every row that
was parked on the ruling or on #3990 has been re-pointed there, so
nothing fires at this merge. Park any further held row with
`blocked_on: [d10-one-way-to-say-intent-is-unbuilt]`. Units already
started may still finish. Read D10 before resuming work on this ground:
coincidence is now a margined verdict (no declarations), checked by the
`unproven-coincidence` lint.

## 2026-10-06 — `swept-cocircular-arc-runs-build-one-wall` closed (PR #3826)

Extrude and the full revolve build one wall per run of adjacent cocircular
same-turn arcs (`swept::joins` → `Join::{Corner, Run, Cut}`; the full
revolve's `collapse_runs` sums the arc sweeps); a circle cut into arcs keeps
its C12.5 cut, one wall per arc on one key (the orchestrator accepted that
reading: a curved run closing its full period refuses to merge). The partial
revolve still builds an arc run one wall per arc — the props meridian fold
groups pieces by split lineage — parked as
`partial-revolve-arc-runs-wait-on-the-meridian-fold` and stated at the README
claim site. `offset_axial::reauthor` reads a re-authored arc's sweep nearest
the old one (an atan2-cut fix a collapsed meridian exposed). Full review
(APPROVE-WITH-FIXES, no MAJOR): the fix pass made the arc boolean rows answer,
corrected stale station prose, guarded `wall_runs`' one-corner loop, adopted
the probes (Vector, hole loop, k = 5, Interval, π-summing runs, the hollow
sweep across the atan2 cut, f64/Interval names) and folded duplicate helpers.
Merged up to main over ~3000 commits; main's split-ball row
(`pi_seam_and_kiss_through_the_boolean`) re-pointed to a hand-cut equator. No
re-baseline moved. Merged over main's own red at 1e-6
(`pocket_ring_steep_ellipse`, filed on JOIN).
## 2026-10-06 — the plane–plane run-out fork goes to Ev (`[ev]` PR #4085)

`a-plane-plane-blend-cannot-end-at-an-unrequested-corner` (P0) put
through the design-fork protocol: fork-log row 72. Designers A and B
both diagnosed one missing idea (a band's end as its section by the face
it runs into) and a face-whole planar carve, recommended the cut-off /
mitre / patch by request count, and converged after three rounds; the
only crossover was the mitre's name key, settled once both measured that
a wider role variant is additive on the wire. PR 4085 asks Ev to rule
the rule itself, the C8 and H7 text, whether an extruded box's isosceles
mitre should lint as an unproven coincidence under D10, and the
vocabulary. The designers flagged three rows to file once Ev rules:
`CornerConfig::policy` maps `MixedConvexity` to `RunOutFeather`;
`RuledPlan::plan` refuses a requested cap rim (the ruled band's own
2-of-3 gap); the inner corner of an L-shaped rim (a torus pivot about L,
or the end-face rule; the torus is degenerate there).
## 2026-10-06 — `a-hole-filleted-at-both-rims-in-one-fillet-panics-in-blend-surgery` closed (PR #4088)

A through-bore's two rims filleted in one call panicked at the surgery's
naming postcondition (in the kernel, not only through the document): both
are ladder rims sharing the bore wall as mate, the wall's seams run rim to
rim, and the second carve's `meridian_splits` row named the fragment the
first carve left rather than the source seam. The row now names
`frag.source`, as the annulus phase does, the band telling the two splits
of one seam apart; `shared_support_gate`'s doc no longer says two ladder
rims never meet. Rows: the extruded bore at N = 2…4 and box ∖ cylinder
build at the two-tori closed form and equal the two single-rim calls in
either order (red on main), and one document `Fillet` over both rims mints
distinct names. Reviewed by the orchestrator's read (one-key fix, the
convention pinned by the new rows).

`declared-joint-kind-zero-margin-reads-smooth` parked on
`d10-one-way-to-say-intent-is-unbuilt`: profile declared-joint kinds are
the hold's ground (D10 makes tangency constructed).

## 2026-10-06 — `support-boundary-meter-reads-a-co-requested-edge-at-its-stored-place` closed (PR #3822)

The ladder case the row named was already exact (arm (a) meters each open
link's trimline against the widened ring); what carved wrong on main was
a ruled link beside an annulus rim — a rod on a squared washer's bottom
face with the bore rim, a tier-3-valid body whose bottom boundary crossed
itself. `support_boundary_clearance` now reads a co-requested outer-cycle
edge at its own trim (`co_requested_trim`): an open link as its trimline,
another rim's arc as that rim's whole trim circle (exact by closed form
and containment). Full review (APPROVE-WITH-FIXES, no MAJOR; every probe
refused on its closed-form margin and carved bodies matched it); the fix
pass took all six points: the arm (c) sibling row closed (predicate 3's
`s > r` keeps a plane-cap trim disc out of the sliver) with the remaining
open-link candidate filed as `ruled-cap-meter-reads-a-co-requested-open-link-at-its-stored-place`
(P3); the rim-arc branch kept with its unreachability argued at the site;
"this link's trim on this face" given one home (`EdgeBlend::trims`,
`Link::trim_on`, `open_trimline`); the reviewer's boundary sweep adopted
as rows.

## 2026-10-06 — the plane–plane run-out ruled (Ev, PR #4085)

Ev approved all four decisions: the request-count rule (cut-off, mitre,
patch), the C8/H7 text, and the vocabulary; decision 3 (whether a box's
isosceles mitre lints) went to INTENT at Ev's request (#4094), where Ev
answered: land it under (b), recorded as a value-decided coincidence and
proven at INTENT's stage 4 (#4097). Fork-log row 72 completed (byte 26:
Opus = A, Fable = B; both matched after round 3). The row's build order is
in its `## Ruled` section; step 1 dispatches next. Filed
`chamfer-ends-in-a-curved-end-face` (P3) from Ev's curved-end-face
question.

## 2026-10-06 — `a-blend-refuses-a-solid-of-several-shells` closed (PR #4113)

A blend carves each chain inside the shell it lies in and leaves every other
shell untouched, byte for byte down to the pcurve rows (the inventory gate
that refused any body but one solid of one shell is gone). The row's premise
was wrong: the bracket split's leg-tip half is two solids of one shell each;
the door covers that and a solid with a void shell. A chain or corner whose
supports lie in two shells is tier-1-invalid and refuses `BodyNotIntact`
(the `AcrossShells` variant the lane first minted was folded in). Full
review (APPROVE-WITH-FIXES, no MAJOR) — its N6 showed a tier-3′ backstop at
the door would refuse a valid island round and miss one-shell overlap, so
none was added; the overlap hole the lane found is
`blend-material-is-never-checked-against-faces-that-are-not-its-supports`
(P0, in flight). The fix pass also caught that an empty request on an
empty body had begun to answer `SurgeryInvariant`. Filed JOIN's
`pinch-tessellate-row-escalates-coincidence-at-eps-1e-6` (main red at 1e-6;
merged over it). First BAND units run as cloud sessions.

## 2026-10-06 — `fillet-support-ring-must-be-a-circle` closed (PR #4119)

The ring carry-through meter reads a ring piece by piece: a polygonal ring
(a prism unioned onto a face) is metered edge by edge against the region the
carve sweeps, exactly, and a ring counts as one circle only when it is one
closed circle edge. That second change also fixes three configurations
where main builds a tier-3-INVALID body (a lens ring of two bores in one
subtraction order, a three-arc ring, a hostless-annulus lens), each now a
row. A multi-edge ring refuses at its least margin, independent of
subtraction order. The duplicate whole-circle readings (`CircleMargins`,
the `effective` widening) are retired: one home for ring reading. The heat
sink rounds its plate after the union, gated on tier 3. Full review
(APPROVE-WITH-FIXES, no MAJOR) taken in full. Filed:
`a-ring-of-ellipse-edges-refuses-the-ring-meter` (P3). The lane's duplicate
filing of main's 1e-6 pinch red was dropped for JOIN's (PR 4113).

## 2026-10-06 — run-out steps 1–2 landed (PR #4121): the plane–plane band cuts off at its end face

Under Ev's PR 4085 ruling: chain G1 classifies plane–plane junctions (a
definite turn breaks the chain into two ends; `Turn` refuses until the
mitre, step 4); predicate 6 reads the request count; the planar band carves
locally (`blend/open/end_face.rs`, shared with the ruled band), so one edge,
any subset ending at unrequested corners, and three edges of one corner
(patch + cut-offs) chamfer and fillet at their closed forms; an oblique
chamfer builds, an oblique fillet refuses until step 3. The whole-face
planar path was deleted early (step 5), shown bit-identical by both
reviewers' and the fix pass's base/head differentials. The bracket's chord
walls retired. Dual review, class H (DR-89): R1 NOT-MERGEABLE-AS-IS on a
concave cut-off swallowing end-face features unmetered (tally 1); R2
APPROVE-WITH-FIXES. The union was taken: the concave end face LOSES the
sliver too (the README sentence from PR 4085 corrected to say so — the
decision unchanged, Ev told), arm (d) pinned by a witness, crossing cut-off
feet refused at plan. Filed `a-requested-ring-edge-refuses-at-the-ring-meter`.
The row stays open for steps 3–5 (oblique fillet, mitre, overrun).
## 2026-10-06 — `lamina-plane-annulus-keeps-its-slit` closed (PR #4136)

A lamina full revolve's plane annulus is one face with its bore as a ring:
no seam slit and no `Meridian(Seam, s)` name, as a solid's planar wall
already was (fork-log row 36's "construct"). The blend surgery and rim
routing follow it: a plane annulus's one-link bore routes to the annulus
carve, `lone_host_trim` carves a lone crossing, and the ruled band's support
gate admits a plane support with rings, metered by the ring pass. STEP's
washer exports 6 edges (census goldens re-blessed). Full review
(APPROVE-WITH-FIXES, no correctness defect — every carve at its closed form)
taken in full: a sentence-level prose sweep across blend, topo, mesh and the
README; the check-9 outer-edge witness rebuilt through a public `mekr_chord`;
ΔV pins; the deepest-gap reading; and 14 probe rows. Reconciled with 4119's
piece-by-piece ring readers. Filed: TESS
`mesh-slit-annulus-rows-no-longer-build-a-slit`, CLEAVE
`revolve-seam-split-volumes-miss-their-closed-form-at-eps-1e-6` (main's red,
merged over); SHELL `offset-door-declared-transport-has-no-built-witness`.

## 2026-10-06 — run-out step 3 landed (PR #4173): an oblique plane end face cuts a round band off in an ellipse

A fillet whose plane end face is oblique to its edge ends in the end plane's
elliptic section of its cylinder, for the planar and the ruled band on both
convexities: built through `Curve3::ellipse`'s deciding door, the sliver
enclosed exactly (the disc to the sliver's reach less the ellipse, cut to
the half-plane and the section's box). `cap_transverse` picks the kind; the
near-perpendicular window where the ellipse is not distinct from a circle
escalates as its own `BlendDecision::CapEllipse` with a followable recourse.
The bracket's fillet walls retired; the tour rocker's wall 2 (the same
over-refusal on a circle end) retired into builds. Full review
(APPROVE-WITH-FIXES; every arc on both faces to 1e-16) taken in full. Filed:
`second-order-cap-window-escalates-small-drafts` (P3); witnesses on CONTACT's
`at-infinity-probe-measures-in-closed-form-only` and QUAD's
`quadrature-convergence-test-escalates-instead-of-refining`. The run-out row
stays open for step 4 (the mitre) and the overrun.

## 2026-10-07 — the P0 landed (PR #4143): every band is metered against every face it could reach

Predicate 2's reach (`blend/reach.rs`) meters each band against every face
of the body that is not a support of its chain, in any shell, and refuses
before any mutation (`BlendError::FaceClearance { at, chain, margin,
bounded }`). Each reach is an intersection of 1-Lipschitz bounds (straight
links closed at their end faces, circular links in their meridian sheet,
corner patches of their own). Faces are pruned on certified boxes
(`topo::FaceBoxes`). Cells halve until they clear, reach `escalate`, or a
per-face budget of 4096 runs out; a spent budget refuses and never passes.
Two bands of one request are read against what each leaves. The reach arm
runs in the surgery after predicate 6 and before any mutation; C8 and
`run_battery` are re-worded to say so, since the arm needs the plan's feet.
The class-H dual review (DR row in this PR; both APPROVE-WITH-FIXES, tally
0) was taken in full:
- every sign is now decided (a bound scaled by an undecided sign is
  vacuous, never wrong);
- an end face tangent to the spine refuses typed;
- the corner skip-union is gone;
- `replaces` is restricted to the edge's extent plus a slab;
- the corner bounds are pinned against analytic oracles.

The cost of the 42-link request fell from about 4 s to 0.3 s at
`Interval`. Filed: `blend-reach-takes-an-open-arc-link-over-the-whole-turn`
(P3), plus a second seed on VACUITY's pick-face guard. This unholds PR 4092.

## 2026-10-07 — sided radius headroom landed (PR #4092): r is limited only where a support bends toward the ball

Predicate 1 (`fillet3_radius_headroom`) reads each support's curvature on the
side the ball rolls (`geom_brep::SurfaceSide`, `min_radius_of_curvature_toward`).
A ball outside a bore, boss or dimple is no longer limited by that radius.
Bodies built at the Pappus closed forms to 1e-12.

The full review's MAJOR (a convex band past a thin wall's far face, which
the old two-sided limit happened to refuse) was held on the P0 and is now
closed by PR 4143's reach meter, without touching the headroom. The
reviewer's `thin_flare` is a row: `r = 1.5` and `1.6` refuse `FaceClearance`
on the far face at a definite negative margin at all three eps rows, and
`r = 1.4` builds. The Klein inner corner gives the same refusals, held in
its scene.

The minors were taken in full:
- the cone-foot row closed as unreachable (spine regularity refuses first);
- the torus arms stated unreachable through `fillet_edges`;
- `SurfaceSide` replaces the bool;
- the margin tightened to `r·(1 − r/arm)`.

The tour rocker's wall 1 retired. Filed:
`rocker-crease-radius-stays-at-the-eye-after-wall-1-retired` (P3).

## 2026-10-07 — run-out step 4 landed (PR #4209): two requested edges at an isosceles corner meet in a mitre

Under Ev's PR 4085 ruling and INTENT's option (b), two requested edges at a
trivalent vertex whose third edge L is unrequested now meet in a mitre, chamfer
or fillet, on both convexities, where `fillet3_turn_isosceles` decides the
trihedron isosceles.
- **The verdict.** The margin is the larger of the levered face-angle cosine
  difference and the gap between the two feet on L. The feet term came from
  the dual review: it is stricter, never looser, so Zero puts the midpoint
  foot within band of both trimlines at acute angles too. A definite verdict
  refuses `UnsupportedRunOut { TURN_NOT_ISOSCELES }` (renamed from
  `TURN_OVERRUN`, whose text was false at supplementary angles); in band it
  escalates.
- **The geometry.** The mitre is a chord, or `cylinder_cylinder_section`'s
  ellipse; L splits at the turn foot.
- **Names and coincidences.** The new roles are `Mitre` and `TurnFoot`, with
  content tags 49/50, because main's `Crossing`/`EdgeCrossing` took 47/48
  while the PR was open. The Zero verdict is recorded as
  `DecidedCoincidence::IsoscelesTurn`, a typed seam until D10 has a door
  (`work/intent/value-decided-coincidences-have-no-recording-door.md`).
- **Clearance.** The mitre needs no new reach code: each link's window
  closes at the other band's support plane. Void and island rows refuse with
  the meter and go red without it.

Class-H dual review (DR row in this PR; both APPROVE-WITH-FIXES, tally 0),
taken in full:
- the oblique-turn cap pad pinned;
- the turn-feet arm of `shared_rims_clear` pinned where the screen passes;
- `core`'s window padded at a planar end;
- in-band `SectionError` escalates `MitreSection`;
- the frustum checked against exact and point-membership oracles;
- one turn reading per vertex.

The bracket's wall 3 stays (its section-face corners are not isosceles). The
run-out row stays open for step 5: the overrun at a non-isosceles turn,
including the supplementary chamfer whose feet coincide, then deleting the
residue.

## 2026-10-07 — the run-out row closes at step 4; step 5 split

Per the ruling's build order, `a-plane-plane-blend-cannot-end-at-an-unrequested-corner`
closes with PR 4209. Step 5 (the non-isosceles overrun, a numeric probe
before its spec) is now its own row,
`a-non-isosceles-turn-overruns-past-the-mitre` (P2, H, design). The
whole-face planar path step 5 was to delete went at step 2.

## 2026-10-07 — step 5's design to Ev (the non-isosceles overrun)

PR 4239's probe measured the overrun on 2250 corners and the three witnesses
(the bracket, the sheared box, the leaning prism). Two designers (A/B,
fork-log row 121, renumbered from 85 at merge) agreed on the final state from their first reports. After
two reconciliation rounds they also agreed on the verdict's two readings,
the three-leg recourse, and what a later blend of L meets (a chamfer of L
builds at 0 of 980 corners at an equal setback). Design reports, closed
forms and measurements: the `[ev]` PR and the item's probe findings.
## 2026-10-07 — the reach meters an end face away from its vertex (PR #4254)

A link's reach now skips only its chain's supports and the faces its own
window is capped by (recorded by `straight_reach` as it builds the caps,
plus a corner patch's supports at a patch end), not every face at any
vertex of its chain. In production a multi-link chain spans valence-2
joints only, so the hole lived at a jointed chain's far end face; that
witness is pinned
(`a_jointed_chains_far_end_face_is_metered_against_the_near_link`; red
with the chain-wide skip put back). A curved face where a straight window
ends fails loud (`SurgeryInvariant`), so the skip's premise is structural.

`band_reach_for_tests` now breaks chains at turns as production does
(`battery::broken_at_turns`); no reach row moved under it (256 tests
diffed). One row moved for the structural skip: a circular reach has no
caps, so an open arc's flat is now metered (latent; open arcs refuse
upstream). Single full review (APPROVE-WITH-FIXES, no MAJOR), taken in
full. Filed: `tint/tipped-rod-join-escalates-at-1e-12` (main's red). Not
chased: `point_in_solid` answering `VolumeUncertified` on 533 sample
points of a tier-3-valid jointed-chain body (the review's probe K control).

## 2026-10-07 — the blend doors take a finished operand (PR #4252)

`fillet_edges` and `chamfer_edges` take `&AtRestBody<T>`. At a dual they read
`gate_unverdicted` first (`build.rs::operand_gate`) and refuse
`InsideOutOperand` / `ScaffoldingOperand`. At f64 and `Interval` an inside-out
or scaffolded body cannot be finished, so it never reaches either door. The
clockwise wedge and the slit dome are pinned, both verbs, red without the
door read. The change is threaded through editor-core, verbs, pncad-py, the
tour, and ~500 test call sites (`test_support::at_rest`, `#[track_caller]`).
`topo::Unfinished` is public; its refusal texts are one home (consts), true
for a `ShellWinding` finding.

**Behaviour change:** editor-core's blend node now pays tier 3 on its
target, which roughly doubles a chained blend node's cost (tier 3 ≈ the
blend). No corpus, tour or guide document regressed. `UnfinishedOperand`'s
text names the input's operation, and every arm fits the 75-word budget.

**Review.** Cost M: a full review raised a MAJOR, so a second full review
followed the fix pass, and it raised another of the same class. Both MAJORs
were the PR having deleted the only row of an arm a FINISHED body still
reaches: the half-band gate's curved single host, and the hostless host
gate (a pinched planar host). Both are now witnessed rows. The filed
`blend-scaffolding-arms-behind-the-operand-gate` closed: its two remaining
screen arms are provably unreachable through the doors (tier 2 checks 1
and 4), and are kept as stated defence for direct `run_battery` callers.
Declined: `#[non_exhaustive]` on `Unfinished`, because the one foreign match
should break the build on a new case. Filed:
`restfront/dual-operand-read-passes-an-orientation-it-cannot-measure`.
Not changed: `common::cavity::cut`'s re-gate (66 callers).

## 2026-10-07 — a straight cap edge clear of the cut-off's sliver carves (PR #4271)

The cut-off sliver's clearance meter read a cap edge only as wholly inside
the ball's section, wholly beyond the reach, or wholly short of the floor,
so an edge that left the enclosure Ω by different faces refused where it
was clear. `line_clearance` now reads a straight edge exactly: the least of
Ω's defining function over its ends, the radial foot and the terms'
pairwise crossings, at every scalar (a quotient whose divisor's bracket
meets zero is skipped; soundness against the sliver rests on the ends).
Three rectangular-hole witnesses carve, at `Interval` too, and the
keyhole at `r = BR` carves at its closed form. The half-plane wedge near
the feet had no witness on main (pinned as a row).

Class-H dual review (DR row in this PR; both APPROVE-WITH-FIXES, tally 0),
taken in full:
- the `Interval` collapse at a floor-parallel edge;
- a cancellation far from the centre;
- a candidate-set fuzzer that can go red, at both scalars;
- an independent Ω oracle;
- the filed rows corrected.

Filed: `blend-reach-refuses-a-bore-clear-of-a-ruled-cut-offs-sliver`,
`cap-sliver-meter-reads-a-curved-edge-term-by-term`.

Claim 1 (a bore wholly inside the sliver dying with it) is NOT built. The
unit row stays open, re-titled to that question: may a blend delete an
authored feature wholly inside the material it removes? It goes to Ev
after the designer protocol (both designers recommend building it).

## 2026-10-08 — the rocker's crease carves at the blend radius (PR #4287)

`demos/tour/src/rocker.rs`: `R_CREASE` rises from `R_EYE` (1/4) to
`R_BLEND` (1/2), as sided headroom (PR 4092) allows. Tier 3 holds, ΔV matches
`−2·crease_cut(r)·depth` to 1e-12, and the census stays 38/57/21 at genus 2.
The sweep still asserts the old radius at its closed form. The kernel and
FreeCAD frames are re-rendered: only the keyhole's two creases are rounder.
`TestRocker`'s mirror and the README row follow. Filed on CIW (P1):
`ciw/render-gui-sorts-ordinal-digest-node-ids-as-ints`; the GUI render
lane crashes on PR 4244's `ordinal:digest` ids, so the viewer cells are
unrendered until it lands.
## 2026-10-07 — the bore-in-the-sliver question to Ev

PR 4271 left the unit's claim 1 open as a design fork: may a blend delete an
authored feature wholly inside the material it removes? Two designers (A/B,
fork-log row 86) both recommend building it, silently, as a Boolean does.
They split on whether the blend's naming records the consumed faces, and
on one C8 sentence. The implementing lane had recommended keeping the
refusal; that is the stated alternative. The `[ev]` PR carries the question.
- 2026-10-09 — Seam note from ENCL (PR 4411, merged): `geom_brep::must_carry_over_edge` now reads the second-order sagitta on every pair (tier 3's walk), so an in-band sagitta escalates `InBand(SecondOrder)` out of lane too; `UnderDetermined` out of lane means every station read Positive (or a station read Zero/Negative). An out-of-lane all-Positive pair now costs `CERT_SAMPLES−2` `tangent_second_order` samples. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4416, merged): `geom_core::lever_recourse(lever, note)` is the one spelling of a lever-alone ending, and `Indeterminate::undecided(subject, ending)` the one "{subject} is undecided: {payload}. {ending}"; compose them, do not re-spell. In `topo::boolean::refusal_routes`, `Ending::Lever` is now `Lever(&str)` (`LeverPass` is gone). Rendered texts are unchanged. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4422, merged): `geom_brep::recourse::RefusedArm::SignCertain` now takes `Option<MarginDiag>`; construct with `SignCertain(None)` unless the decision is a residual miss, and match with `SignCertain(_)`. `certify::definite_miss_in_file` / `Unsized::definite_residual_in_file` are gone; `Unsized::residual_in_file` is the one door. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL: dispatching `encl/sweep-must-carry-escalation`. `sweep::blend::surgery`'s contact-edge, `sweep::extrude` (strut, cap rim) and `sweep::revolve::upgrade` will map `MustCarryEscalation` exhaustively instead of `.diag()`. The blend's second-order reading gets its `AnySign` tolerance ending back, and a first-order station stops reading as `ContactSecondOrder`. Texts may move on in-band must-carry refusals. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4450, merged at `59cdb05871`). `topo::DihedralReading::of_lever` / `of_must_carry` are the one map from a dihedral escalation to a reading; use them, not `(Lever(e.rung()), e.diag())` by hand. `sweep::blend` has two new closed decisions, `ContactArm` and `ContactWedge`, and `ContactSecondOrder` now offers the tolerance (`AnySign`). `ExtrudeError`/`RevolveError` `SliverJoin`/`SliverRim` now carry `reading: DihedralReading`, and their second-order text reads through `Indeterminate::undecided`. The editor-core fault line for `Escalated(contact)` is exactly at the 75-word budget. (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4474, merged): the material pairing is a decision. `geom_brep::MATERIAL_PAIRING` and `MATERIAL_PAIRING_CLAUSE` end it at tier 3 (`WedgeCheck::MaterialPairing`) and split finish (`SplitFinishError::DescribeSideEscalated`), with a true tolerance offer. Where the wedge would re-decide below m/K, `pairing_at_wedge` quotes the wedge and offers w/K, as `at_wedge` does. `MaterialStations`' `Break` is `MaterialStop { check, cause }`. The cusp-side Zero reads via `decide_nonzero`. New k_stats predicates appear on the refusal path only: "material_pairing_offer_wedge" and "material_pairing_wedge". (ENCL orchestrator)

## 2026-10-12 — Ev ruled on the bore in the sliver (PR 4285)

Build it: a feature wholly inside the material a band removes or fills goes
with it, as under a Boolean; a crossing feature still refuses. No `Consumed`
record until something needs one. C8 now says face clearance reads three
ways. A consumption lint over the blend and the Boolean as one rule, which
needs lints to store their own resolution first, is filed on INTENT
(`a-feature-an-operation-consumes-has-a-lint`). Fork-log row 119 (renumbered
from 86 at merge). The build goes in the ruling's four steps.

## 2026-10-12 — a requested ring edge meters as a strip (PR 4286)

A planar support's ring that carries a plane–plane link's requested edge
is metered at the strip (arm (d)), not refused at the ring meter.
`a-requested-ring-edge-refuses-at-the-ring-meter` closes. Three full
reviews (tier M, sequential: each of the first two raised a MAJOR):
- review 1 (`fcd3d374a6`): `reach::screened` read a two-sided region where
  arm (d) is one-sided, so clear far-side geometry reached
  `SurgeryInvariant` (also on main for the outer cycle). Fix pass 1 made
  the region one-sided, and `screened` refuses a vertex inside it typed.
- review 2 (`7781e428ee`): the one-sided bound's `slab` setback disabled
  the cross-chain excuse at oblique poses (the sealed void + island
  refused at three of five poses that built on the base). Fix pass 2
  split the readers: `screened` keeps the one-sided bound, the excuse
  reads the two-sided region where `screened` finds it vacant of other
  vertices; a row builds the island at every pose.
- review 3 (`6754d3515c`): APPROVE-WITH-FIXES, no MAJOR; it reached the
  vertex-only premise with a curved far-side edge and found no reader.
  A finisher took its minors (the spike row now pins arm (d) with an arc
  no vertex screen sees; the docs state the vacancy as a vertex reading).

Filed: `the-cross-chain-excuse-misses-an-oblique-cell-past-a-windows-end`
(the 1.1 rad pose, red on the base too),
`the-strip-screen-reads-a-supports-vertices-not-its-edges` (with its
witness row), and evidence added to
`the-support-screen-refuses-a-feature-across-the-edges-line`. Main's
1e-6 red of `r1_lane0_e2e` is noted on its ssiedge item.

## 2026-10-12 — the bore ruling's step 1: predicate 2 reads three ways (PR 4797)

Predicate 2's reach and the cap meter (arm (c)) read each face and cycle
clear, wholly inside, or crossing, and feed one verdict for the request
(`reach::Unclear`): a crossing anywhere refuses as a crossing, then a
bound, and `BlendError::WhollyInside` only when every non-clear reading is
inside. "Wholly inside" still refuses; the consuming carve is step 3.
Because a wall that meets a window's end plane reads as a bound, every
through-bore inside a sliver refuses as a bound at its wall today; the
step-3 prerequisite (a non-strict cap-plane reading, a prism reading for
the ruled crease) is in the unit's Findings. The README states today's
refusal beside the ruling's end state (Ev on PR 4285: rephrase where
misleading).

Cost H, a concurrent dual (DR row `PR-4797`). Both reviews found the same
MAJOR: the first version's verdict was whichever face or cycle was read
first, so a crossing feature could be refused `WhollyInside` by order.
Tally 0. A union fix pass fixed it, and a confirming full review on
`0f5d13362d` approved, with notes for steps 2–4 recorded on the unit.
`blend-support-ring-meters-read-two-ways` triaged P3/M.

## 2026-10-12 — Ev ratified the non-isosceles turn's overrun (PR 4245)

Ev's conditions (2026-10-08) were checked numerically on the PR, with
figures in `crates/sweep/doc/turn-overrun/`. PR 4844's dual review then
showed the box-corner limit was stated wrong: the chamfer's chord is the
gap times a corner-fixed ratio, so both readings are quadratic there. The
README sentence and a comment were corrected before Ev answered "this
design sounds good!". Fork-log row 121 (renumbered from 85 at merge) gets
its second half. The build goes in three steps; step 1 is PR 4844.

## 2026-10-12 — the turn reads its feet and its overrun chord (PR 4844, overrun step 1)

A two-requested-edge turn at a trivalent plane–plane vertex reads two
closed-form lengths per verb: the gap between the bands' feet on the third
edge, and the overrun chord. Both Zero builds the mitre and records a
`CommonFoot` coincidence (unproven at the document's door); both definite
refuses `TURN_OVERRUN` naming the far band; anything else escalates. The
sheared box's supplementary chamfer, refused before, now builds (convex and
concave, at f64 and Interval, at the closed form). Cost H: a concurrent dual
(DR row `PR-4844`), both APPROVE-WITH-FIXES, tally 0. Both reviews derived
that the box-corner chamfer chord is the gap times a corner-fixed ratio —
the `[ev]` analysis had said linear — and the README and PR 4245's comment
were corrected before Ev ratified. The turn recourse drops its "clearly
apart" leg until step 2 builds the overrun; step 2 restores it.

## 2026-10-13 — the bore ruling's step 3: a ruled cut-off's sliver consumes a through-feature wholly inside it (PR 4896)

A through-feature between the two caps of a convex ruled cut-off, wholly
inside the sliver the band removes, now goes with it: a prism reading
(`reach::prism_inside`) reads the feature's faces through the cap cycles
read inside — the cap's sliver is the prism's slice, so the two readings
agree — and the carve (`consume`) retires its faces, edges and rings.
`consume::plan` admits only an annulus of one-cycle faces and refuses
`WhollyInside` for anything else; a blind bore refuses as a bound. The two
witness bores build at `ΔV = −2·A·L + π·a²·L`. Step 2's remainder landed:
a consumed name resolves `Vanished { PredicateFlip }`, and `WhollyInside
{ at }` is canonical by `EntityId`. Cost H, a concurrent dual (DR row
`PR-4896`), both APPROVE-WITH-FIXES, tally 0. Both showed by mutant that
nothing pinned the prism's slack and tilt terms; two rows now do. The
reviews contradicted each other on a one-edge bore: the fix pass found the
seamed one-edge bore builds (one face, one cycle) while a seamless wall
(two cycles) would have been admitted and then refused untyped, so `plan`
now refuses faces with rings. Step 4 (planar band, concave fill, chamfer)
has its owed list on the unit.

## 2026-10-13 — the bounds roster line PR 4896 owed (`reach::displaces`)

PR 4896 added `reach::displaces`, a sole-bracket bound door, with no
`bounds_census` roster line; geom-core's census turned main red (filed
three times: P0 by FLUX and TANG lanes, P1 by PCERT). The PR's own CI and
the orchestrator's local gate (sweep + editor-core) never ran the census,
which carries no `gated_to!` marker and so runs only when a diff seeds
geom-core. The line's disposition is Selection, inheriting `Unclear`'s.
Before merging a blend PR that adds a `Bounds` site, run
`cargo nextest run -p geom-core -E 'test(bounds_census)'`.

## 2026-10-13 — a chamfer's non-isosceles turn builds its overrun (PR 4926, overrun step 2)

Where both readings are definite, a chamfer turn builds the mitre plus the
far band's cut-off at the near band's face of L, clipped at the mitre's end
q down to the far foot p_k; every vertex trivalent, the far band's
`EndArc`/`FootVertex` named. The "move the feet clearly apart" recourse is
back for the chamfer. Tour wall 3 builds (`bracket.png` re-baselined). The
fillet still refuses `TURN_OVERRUN` (step 3). Cost H: a concurrent dual (DR
row `PR-4926`), both APPROVE-WITH-FIXES, tally 0; both built the overrun
right against oracles of their own (a clipped cube; Boolean slabs), now
rows. Both showed by mutant that arm (c)'s whole-sliver meter at a turn
makes no refusal the reach does not, and one that it refuses a boss clear
of the overrun: filed as `a-turns-overrun-meters-the-whole-cut-off-sliver`
(the fix revises the README's ratified clause, so it is Ev's). The
short-near-edge spurious `FOOT_INSIDE_A_FACE` is now executed on its item.

## 2026-10-13 — the bore ruling's step 4: the planar band and the chamfer consume a through-feature wholly inside their sliver (PR 4957)

The planar band hands its crease to the cap meter, so step 3's prism
reading and carve take a through-feature wholly inside the material a
convex plane–plane cut-off removes. A chord end's sliver is read as its
exact triangle, so the chamfer's convex section takes no slack (keyed on the
sliver, not the verb). The concave fill split to its own fork,
`a-concave-fill-and-a-whole-shell-inside-a-bands-region` (`[ev]` PR 4963);
a void inside a convex band's material is owed and still refuses
`WhollyInside`. Cost H: a concurrent dual (DR row `PR-4957`), both
APPROVE-WITH-FIXES, tally 0. Each found one of the triangle's terms unpinned
(the outside witness; the two rim sides), killed only by its own probe; both
are rows now. Filed `a-chord-ends-triangle-could-read-a-cycle-outside` (a
missed loosening). The overrun's sliver, built through the same chord end,
keeps crease `None`, pinned by a slot row. Merged with main brought in after
PR 4992 fixed PR 4197's `transform_rigid` break on main.

## 2026-10-13 — a fillet's non-isosceles turn builds its overrun (PR 4962, overrun step 3; the item closes)

A fillet whose feet on the unrequested edge L stand definitely apart
builds the overrun, at convex and concave corners: the far band's ordinary
cut-off at the near band's face, in the section any cut-off picks, its arc
windowed from p_k to the mitre's end q (G1 at both). `TURN_OVERRUN` is
gone. Near a box corner, on the side whose far face is oblique, the end
ellipse escalates past the turn's band (to ≈0.0256° at r = d); the turn
recourse now hands on to that recourse, walked to a build. Cost H: a
concurrent dual (DR row `PR-4962`), both APPROVE-WITH-FIXES, tally 0; both
built the right bodies against oracles of their own (a planes-solved
quadrature with membership grids; a Monte Carlo opening), now rows. The two
residues (`an-overruns-sliver-refuses-a-short-near-edge`,
`a-turns-overrun-meters-the-whole-cut-off-sliver`) are verb-neutral and
carry the fillet's evidence; the round sliver over-refused nothing probed.

## 2026-10-13 — the line meter's Interval low end far from the centre (PR 5025, P1)

The Interval line meter read `G` through the point `q0 + e·[s]`, so `s`
sat in every coordinate and a near-0/0 crossing candidate's spread
lowered even a term flat along the segment. `g` now reads each term with
`s` once (`√((s + b)² + d2)`, `a + k·s`); each still encloses its term at
every point of `[s]`. Main reddened the fuzz row on 8/1000 seeds; the
branch on 0/1000 and on ~400k effort-100 cases. Both seeds' cases are
fixed rows. Cost E, verified by the orchestrator (the seeds green on the
head, clippy clean).

## 2026-10-14 — BodyNotIntact says how the body broke (PR 5021)

`BodyNotIntact` carries a typed `Breach { Unresolved, Unwalked,
Disagrees }` and renders that cause, not "did not resolve" for every
detail; all 213 raise sites classified, nine mixed ones split. Each
breach's text is pinned, with a real refusal per kind where one is
reachable (`Disagrees` through the public door; the other two only
crate-internally, since the doors validate tiers 1 and 2). The lane
filed seven sibling refusals that render several causes through one
sentence, across six programs. Cost E, verified by the orchestrator.

## 2026-10-14 — Ev ruled the band's feature semantics (PR 4963)

Re-forked on Ev's comments into one question and answered by two
designers in agreement (fork-log rows 139–140): Boolean semantics is a
blend's definition (convex `S − R`, concave `S ∪ F`); a feature crossing
the band surface refuses as an unbuilt case, with the intention to build
it once the Boolean serves the band × feature pairs; a mouth a fill
covers is sealed; an advisory lint for every operation reports a feature
an op makes vanish or change type unasked; `docs/DESIGN.md`'s
void-insertion convention gains the blend's seal as a fourth producer.
The reorder recourse (`subtract(fillet(body), hole)`) needs the blend's
edges re-picked upstream, because a name on a feature's output is that
node's `From { read, of }`. Build order and status:
`work/band/a-concave-fill-and-a-whole-shell-inside-a-bands-region.md`.

## 2026-10-14 — a fillet corner's chart poles on its third foot (PR 5048)

`arms::corner_chart` poles every fillet corner patch on one of its feet,
so one pole is a vertex and the antipode lies outside (the feet are
independent, predicate 6); the patch stays in an open hemisphere and its
`u` window in `[0, θ]`, `θ < π`. Ev's C12 (7) rule held for blend corners
at last: the f4 clip read `Unencoded` on main and reads `Encoded` here,
over 160 corners. Cost M, full review APPROVE-WITH-FIXES (text only; the
geometry held over 2000+ charts and 124 built corners, and a mutant);
the fix pass took the reviewer's oblique corpus into the row. Oblique
fillet corners still do not tessellate (`NotIsoRectangle`, the P0 tess
item, which now carries them as a witness); box corners move only by
signed zeros at axis poses and by ulps elsewhere.

## 2026-10-14 — the cap sliver reads reach and floors over its rim pieces alone (PR 5110)

`CapSliver::of` no longer takes the end curve's own terms into `reach`
or the floors: over the compact sliver the distance and each floor's
linear function reach their extremes on its boundary, and inside an arc
of the strictly convex end curve an extreme would need the outward
normal to point the way the function grows, where the sliver runs on
past it; so the rim pieces hold every extreme (proof at the site). The
review re-derived it and ran the whole sweep suite instrumented (10,383
sliver constructions; the dropped terms never beat the rims by more
than 6.5e-14, foot-tie rounding). One row pins the arc extremes the cap
meter reads (the far and along-high arms). Cost M, full review
APPROVE-WITH-FIXES (a stale doc). Filed
`a-cap-slivers-rim-arcs-whole-circle-arms-have-no-cap-row`: the rim
arcs' whole-circle arms, now the only source of those extremes, have no
cap row.
## 2026-10-14 — sheet_clip clips a cylinder or plane at any pose (PR 5052)

`sheet_clip` admitted a cylinder by its stored origin's offset and an
unlevered tilt gate. A wrong admission was only ever a lost clip (the
clip is the face boundary's span along the trace, and `z` is affine
along every ruling, so the boundary's span holds the face at every
pose); the fix reads no origin and, after review, clips a cylinder or
a plane at any pose, ⊥ included. `sheet_fn` reads its slack at the foot
of the reach box's centre. `tilt`, the circle arm's `lean` and
`circular_reach` take their chords without cancellation; main's
`√(2−2|b̂·a|)` under-read the slack in f64 (a zero-band lean levered
over a long box read 0). Cost M, full review APPROVE-WITH-FIXES (no
MAJOR: 9412 clips at 14 leans, 0 missed a face point; mutants red); the
fix pass ported the reviewer's corpus as rows.

## 2026-10-14 — the band feature rule written down; the crossing recourse names the reorder (PR 5077, step 1)

C8 and `crates/sweep/README.md` predicate 2 state Ev's rule (PR 4963):
`S − R` / `S ∪ F`, what is built today, and the refusals sorted by the
meter that reads them (the screen and the ring meters; the reach;
`WhollyInside`). The clearance and ring recourses offer the reorder
("if that face came after the edge, blend before it, re-picking the
edges there"); rows walk it to a build for a chamfer against a skew
bore and for either verb against a parallel bore, and pin the fillet
against a skew bore refusing at the Boolean (`GermFrameUnsupported
{ Cylinder, Cylinder }`) until that pair is served. Filed
`work/doctail/an-editor-action-moves-a-node-after-another.md`; the
lint item carries Ev's op-agnostic ruling. Cost M, full review
APPROVE-WITH-FIXES (two README sentences named the wrong arm or row).

## 2026-10-14 — blend refusals say what their site met (PR 5116)

`Breach` gains `Scaffolding` (with topo's own recourse, "finish that
edit first", shared through one literal with `Unfinished`'s door
sentence) and `Degenerate`; null-edge and lone-vertex sites leave
`UnsupportedGeometry`'s shape recourse; folded `None`s refuse per cause
through one `surgery::certified_curve` read (`stored_piece` and
`battery::carrier_of` its `Result` forms). `Scaffolding` is reachable
through the public `run_battery_for` on a raw body (a row pins it);
`Degenerate` only crate-internally. Cost M, full review
APPROVE-WITH-FIXES. Filed `blend-two-geometry-refusals-meet-no-shape`
and `blend-reach-may-refuse-a-valid-thin-cylinder-as-degenerate`.

## 2026-10-14 — a thin cylinder boxes by its boundary (closes blend-reach-may-refuse-a-valid-thin-cylinder-as-degenerate)

Tier 3 admits any exactly positive radius; the face box's levered axis
refused one the band does not decide positive. No door was found to
mint one (profile, STEP, fillet each refuse it at the band), but the
margin chain does not exclude it, so `face_box_rule` now falls back to
the boundary hull (sound for any ruled face) where the axis is a
direction and the lever does not decide. The review's fix pass splices
the lever's escalation superseded on that fallback, clips the slab to the
hull on every coordinate (one home for the ruled containment), and
filed `cleave/separating-axis-reads-swallow-their-unit-escalations`.

## 2026-10-14 — a bar inside a concave band's fill is absorbed (PR 5099, step 2)

A bar between two end walls wholly inside the material a concave band's
fill adds goes at `ΔV = +(A − A_bar)·L`: a cut-off's cap ring read
inside its sliver carries its crease on either convexity
(`SliverOrigin::CutOff`), and nothing else reads the side, so step 1's
carve runs for the fill unchanged. Cost H: a concurrent dual (DR row
`PR-5099`), both APPROVE, tally 0. The fix pass took in both reviewers'
probes; the ruled table's island-voids cell carries an erratum (the
voids lie in `F`, which fills them), and the step-2 refusals steps 3 and
4 inherit are pinned (a bump on the bar, a sealed tube, blind holes in
the walls). Filed `a-feature-inside-a-turns-overrun-is-not-carved`.

## 2026-10-14 — a cup on one end wall goes with the band (PR 5161, step 3)

The end-wall half of step 3: a blind bore under a convex plane–plane
band and a pin under a concave one, wholly inside the region, go at
`ΔV = ∓(A·L − V_cup)`. A cup's faces off the cap are certified through
their edges read in the band's exact core, a plane face not parallel to
the spine by its boundary, and `consume` takes a disk on one cycle,
bridging a face's rings first, so a bump on a bar now builds too. The
strip-mouth half (dimple, boss) and the ruled band's cups are filed.


The fix pass of its dual review: the core reading of an edge now
tightens its depth (it reported the first certifying piece's, which,
not the lean term, refused a slightly leaning bore under the fillet,
now built); rows witness the disk count, the piece radius and the
plane reading's slack; C8 says an absorbed island's voids are filled.
Filed `a-feature-inside-a-region-that-is-no-disk-or-annulus-is-not-carved`
and `a-cup-leaning-off-the-spine-under-a-fillet-is-charged-its-boxs-diagonal`.

- 2026-10-15 — **No live row is at or above this track's own band (P0).** The 51 rows left are 5 at P2, 36 at P3, 3 at P4, 7 with no band. Consider wrapping up: land the work already in flight and close the program by its exit walk. Failing that, at least re-home the low-priority rows to a program whose band matches them (by `git mv`, ids unchanged, per the closing rules), and do not start any more low-priority work here.
  Band the 7 unbanded rows first: if one of them turns out to be at or above P0, this note does not apply.
  (tracker sweep)

## 2026-10-19 — a cup on a support's strip goes with the band (PR 5338, step 3)

The strip half of step 3: a dimple under a convex plane–plane band and
a boss under a concave one, their mouths rings on a support's strip,
wholly inside the region, go at `ΔV = ∓(A·L − π·a²·h)`, where the band
is one link cut off at both ends. The ring pass reads a support's ring
against the strip across the edge; the clearance screen stands down on
a ring that reading decides; the reach reads the cup in the band's core
less that support's side, its other edges in the whole core. A ruling
square to the support pays nothing where the section is convex (the
chamfer's) or the face lies on the support's side of the fillet ball's
centre; a cup reaching past that centre pays its run across the spine,
since at an acute dihedral a line along the support's normal can leave
the section through the arc and come back in the cusp by the other
support (both reviewers' probes, now rows in `band_dihedral_cups`, with
the cap half's at 30°, 60°, 120° and 135°, which main builds right).
The screen and arm (a) read a ring through one piece set
(`surgery::strip_ring`), so a leaning round dimple's elliptic mouth now
goes too. `consume` needed nothing new. The followability row's blind
hole from the support face now builds. Filed
`a-cup-on-a-strip-beside-a-band-not-cut-off-at-both-ends-is-not-carved`.
## 2026-10-19 — a cut-off arc at an oblique end reads off atan2's cut

`an-interval-fillet-cut-off-at-an-oblique-end-reads-its-arc-backward`
closed: a foot half a turn from its end ellipse's `u_ref` read the hull
`[−π, π]` at `Interval`, so the sheared box's fillet refused where
`f64` built. `surgery::arc_window` re-anchors at the half turn and is
the one window read for `CapSliver::of` and `attach_contact`. A shear
ladder of both signs, convex and concave, both verbs, at both scalars.
Filed `a-rim-split-at-its-carriers-anti-seam-refuses-at-interval` and
one issue for the seam-anchored reads in `topo`.

## 2026-10-19 — the blend arms place their centres at their inputs' scale (`band/arms-centre-placement`)

`sheet_center`'s round × round arm reads `geom_core::Circles` for the
offset circles' radical foot and half-chord, and `corner_ball` solves
for the centre's offset from its first vertex. Rows in
`blend_arms_centre_placement.rs` hold both at a few ulps of scale at
`f64` and `Interval` (main: about `ratio` ulps / NaN past 1e8, and
`u·|p|/det`). Closed `sheet-center-circle-pair-cancels-at-unequal-offsets`
and `blend-corner-ball-cramer-anchors-at-the-world-origin`. Filed
`plane-sphere-blend-spine-radius-by-differences-of-squares` here and
`work/shell/topo-planar-offset-cramer-anchors-at-the-world-origin.md`.

## 2026-10-19 — the slate re-banded for the hand-back

Ev (in chat): finish what matters, then hand the tail back rather than
working through it. The eight unbanded items now carry a band and a
cost (the Interval oblique cut-off's `BodyNotIntact`, a normal fillet
refusing on the certified scalar, P2; the precision and funnel items
P2; the overrun and dependent-normals refusals P3).
`ruled-cut-off-builds-a-bore-wholly-inside-the-removed-sliver` closes:
its four steps are built and its void residue is the concave item's
step 4. `blend-reach-never-meters-the-bands-of-one-chain-against-each-other`
drops to P3 (no door reaches it while every chain is straight). Being
finished before the hand-back: step 3b (strip-mouth cups), the two
`arms.rs` centre placements, and the Interval oblique cut-off. Step 4
(whole shells and the seal) and the P3 tail stay on the slate.

## 2026-10-19 — the track handed back (`ready`, P2)

Ev (in chat): finish what matters, then hand the tail back. Finished
before the hand-back: step 3's strip half (PR 5338, a dimple or boss on
a support's strip; its dual review found a wrong body at acute
dihedrals, fixed by giving the ν-stray only inside the fillet ball's
centre half-space, and the merged cap half was probed at 30°–135° and
is unaffected), the two `arms.rs` centre placements (PR 5348), and the
Interval oblique cut-off (PR 5346). The program is `ready` at P2: step 4
(whole shells and the seal, H), the anti-seam Interval refusal, three
error-propagation rows, and the P3 tail. The 1e-12 needle main red
filed here is closed as a duplicate of CPAIR's bisected item.
- 2026-10-20 — **Seam (NURBS):** PR 5477 merged (`6a61ea97`, CPAIR ground). `band_turn_overrun_fillet::the_needle_builds_then_refuses_at_the_turn_as_it_flattens` and `leaning_prisms_build_at_the_oracle_over_leans_radii_and_scales` now pass at `CAD_TOLERANCE_EPS=1e-12`. They were red in the 10-17, 10-18 and 10-19 nightlies, bisected to PR 5227. `band_ruled_cap_ring::a_bore_inside_the_d_rods_removed_sliver_goes_with_it_at_interval` stays red at 1e-12, on the Interval polish meter, filed as `work/cpair/curve-pair-polish-at-interval-reads-the-iterates-width`. (NURBS orchestrator)
- 2026-10-26 — (tracker sweep) Closed three rows already fixed on main: `fillet3-chain-arm-mints-indeterminates-after-a-definite-sign` (both sites go through `classify_positive`, `4c59258a00`), `blend-reach-reads-two-extents-off-a-root-of-one-minus-a-square` (both reads respelt without cancellation, `f84f8ef95d`), and `end-face-vertex-window-picks-its-first-half-turn-by-floor` (the floor pick was deleted with the end curve's terms, `12ee8c73f0`, PR 5110). Re-read notes added to `blend-split-rows-and-must-carry-prose-residue` (two of its three bullets are done), `unmerged-subdivided-wall-rim-fillet-names-no-merge-recourse` (its witness row is gone; the refusal stands), `swept-run-walls-style-residue` (the gate's count), and the line numbers of `unclaimed-half-edge-read-as-a-minus-half-in-band` and `blend-rim-and-seam-walks-read-a-ring-by-an-unproven-cycle-walk`. Priced the two unpriced rows: `a-blend-of-no-edges-returns-its-input` P3 E with `design`, and `a-knife-edge-plane-blend-divides-by-one-plus-the-normals-dot` cost M. Dropped the plan's stale SCALAR note (`S90-impl` closed 2026-08-21). The track is still far over budget (about 100/30): it is owed a priority-seam split.
- 2026-10-26 — (split) BAND measured about 100 points against 30 (twelve P2, four P4, about 85 of P3), so it split along its priority seam. BAND keeps the P2 slate and the P4 residue, and four cheap P3 rows that sit beside step 4: the surgery's ring walk and half-edge reads, the empty request's design question, and the swept-operand test repairs. That is 20 points. The P3 tail of conservative refusals and wide meters split by which reader refuses, into three new programs, each `ready`, its rows moved by `git mv` with ids kept. **BANDRULE** (26.5) takes 13 rows: the shapes the band-feature rule or the chain door would build (features in a region that is no disk or annulus, inside a turn's overrun, cups on a strip, leaning cups, a ruled band's cup, a touch in the strip, the core depth on a spiric or spline crease, a chamfer into a curved end face, a ruled joint, an unmerged subdivided rim) and the endings that name the wrong recourse (the two `DependentNormals` rows, the two shape-recourse sites). **BANDREACH** (18.5) takes 8 rows on predicate 2, the reach meter in `reach.rs` and the support screen in `battery.rs`. **BANDMETER** (28.5) takes 12 rows on the surgery's meters: `ring_clearance_pass`, the rim's support-boundary meter, the cap sliver and the turn's overrun sliver. **CARVEREST** (to 21.5) takes three `battery.rs` rows that read the chain walk and the corner: `corner-config-indeterminate-has-no-real-body-witness` and `open-rim-end-corner-of-a-partial-revolve-does-not-classify` (`corner_at`, beside CARVEREST's corner-valence row), and `blend-an-edge-in-another-shell-moves-predicate-4-at-a-cavity-corner` (`walk_chains`' pairing, beside its corner-junction row). **PROFILETAIL** (to 34.5/35) takes `declared-joint-kind-zero-margin-reads-smooth`, which is `profile::validate` ground. Code comments citing the old `work/band/` paths were re-pointed.
- 2026-10-26 — (easy-fixes lane) Closed `plane-sphere-blend-spine-radius-by-differences-of-squares`: `plane_sphere_blend` factors both differences of squares, pinned near the pole in `m5_pr12_blends`.
- 2026-10-26 — (easy-fixes lane) Closed `a-bands-core-depth-reads-no-speed-for-a-spiric-or-spline-crease`: `band_reach`'s core depth reads `speed_range(c).1`.
- 2026-10-26 — (ATREST orchestrator) Filed `blend-reads-its-chain-convexity-off-the-link` (P1 M) here. It is parked on ATREST's `material-and-convexity-are-read-three-ways`, which makes `topo::link` the one reader of a cell's local material. Blend's `fillet3_convexity_sign` is the same predicate that build deletes from `Gap`.
