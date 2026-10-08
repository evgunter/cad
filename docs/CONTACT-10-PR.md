# CONTACT-10: the containment refusals carry their decision

The rule is D4 ¶1 (i) in `docs/DESIGN.md`: a refusal's recourse follows from its decision and its verdict, and the decision is a closed type at its site.

The shared ending table is `geom_brep::recourse`.

Rows this addresses:
- `work/contact/contain-escalation-carries-no-decision`, with a "Carried" section added;
- the contain.rs half of `work/contact/contact-near-boundary-endings-say-lower-the-tolerance`.

## Reconciliation with `main` (and PR 3493)

The branch was cut before main reworked containment:
- `ContainError` split its folded arms;
- the ray walk got one driver, which keeps a ray's in-band margin as evidence;
- `SizedPass` moved to `geom_core` with `AnySign`;
- PR 3493 added `BooleanDecision::Containment`.

Every crate file the first implementation touched conflicted or was superseded. The merge (`da0220ab2`) therefore takes main's code whole, and the change is re-applied on top. The pre-merge implementation is `b4d1bcab6`. What moved:

- **Spec items 1–2 were already done on main.** `ContainError::RayExhausted` ends through `ray_walk::NoRaySettled` in "Recourse: move the geometry", and `classify_contain` ends it in the same lever. This branch leaves both alone.
- **`SizedPass::Definite` is dropped.** Main's `SizedPass::AnySign` is the same pass set: every definite reading passes, and only an undecided margin refuses.
- **Carrier takes 3493's finding.** `bool_curved_contain_carrier`'s pass set is the caller's:
  - a definite `Out` at `curved_face_containment`;
  - a residual at `reduce::wall_crossing`, where the point is a certified root.

  So `ContainDecision::Carrier` is lever-only (`recourse::LeverOnly`) at every reading. It is no longer the two-sided valued tighten the first cut gave it. The other decisions' pass sets do not depend on the caller:
  - the boundary place answers on every definite reading wherever contfp is asked;
  - the period rung passes only positive at both callers, as 3493 found.
- **One type survives on the Boolean path: the walk's.** `BooleanDecision::Containment` now carries it: `Containment { decision: Option<ContainDecision>, escalation: Escalation }`.
  - A carried decision ends through `refusal_routes::Ending::Placement` in the decision's own lever alone, `ContainDecision::lever_ending`, from the one lever source contfp's `Display`, `classify_contain` and the census read.
  - The Boolean offers no tolerance for it. It asks the walk of many points, and a smaller tolerance moves the other points' readings into the band as it decides this one, so no margin the refusal carries binds the operation. 3493 reached the same conclusion for the rungs it could not tell apart. The executed-offer census (`offer_rows::every_arm_that_offers_a_value_has_an_executed_case`) also holds the Boolean to an executed case for any valued offer, and none exists for these arms.
  - `BooleanDecision::Containment` remains only as the Boolean's route. Every Boolean escalation is routed by a `BooleanDecision`, and that is where its subject and offer keys live. It holds no second table.
  - `decision: None` (`BooleanDecision::CONTAINMENT_UNNAMED`) is a rung the escalation does not name: the solid door's, the sphere region's, a cone face's nappe read. It ends in a lever alone (`LeverPass::ByRung`), now the one unnamed lever every door shares: `placement_lever(None)`, "move the point clearly inside or outside the face". 3493's "move the parts so they meet clearly inside or clearly outside that face's boundary" was the same advice in the Boolean's words. The subject has already introduced the point, so one text serves every door.
  - The wrap sites are `reduce::esc`, the span walk's curved placement and `carrier_touch`. 3493's row `a_containment_escalation_on_a_residual_rung_names_its_lever_alone` now asserts the carrier rung names itself and ends in its own lever.
- **Filed rows, re-checked against 3493:**
  - `boolean-door-drops-the-containment-decision` is **closed**: the decision is carried.
  - `point-in-solid-escalation-carries-no-decision` is **narrowed** to what is left: the point-in-solid door's own escalation names no decision, and its `Display` ends in the coincidence menu. `topo::test_support::LATER_STORIES_OWNED`'s `Containment` stories (PR 3513's executed offers) now name this row. They were logged under `contain-escalation-carries-no-decision`, which this unit addresses.

## What changed

### `geom_brep::recourse` (no program; seam note in `work/encl/log.md`)

- **`RefusedArm::Straddle`**: two sound bounds on one margin straddle the band, so no single margin is carried. It is not a poisoned margin.
  - A sized decision names its lever alone on it.
  - `Unsized::LastResort` reads it as undecided.
  - `not_yet` adds no note.
- **`LeverOnly`**: the shared shape of a lever-only decision. Every arm ends in the lever, and a poisoned margin adds the unreadable-margin note.

### The loop walk (`splitting/containment.rs`)

- **`LoopDecision`** is the closed set the walk raises, each with its own subject and lever:

  | decision | sites | shape |
  |---|---|---|
  | `Boundary` | `point_in_vertex_polygon`'s pre-pass, and `carrier_walk`'s boundary pass (`LoopEdge::contact`; the boundary disagreement, tagged `Decided`) | sized "distance", `AnySign` |
  | `Ray` | `polygon_walk`'s rows, `walk_schedule`'s arm row, and `carrier_walk`'s reach, side and advance rows | `LeverOnly` |
  | `ArcSpan` | `carrier_loop`'s span (ellipse, and a scaffold conic's span) | sized "arc", `NonNegative` |
  | `Plane` | `certify_plane`'s rows (`point_in_loop`'s preconditions) | `LeverOnly`, a residual |

- **`Escalation { Margin, Straddle, Decided }`** records how the reading stood. `ConicArc::hit`, `LoopEdge::contact` and `ConicArcError::Escalated` now carry a `ReadEscalation`, so the conic straddles (`*_conic_straddle`) are tagged `Straddle`.
- **`PointInLoopError::Escalated { r#loop, decision, escalation, diag }`**. Its `Display` reads "{subject} is undecided: {payload}. {ending}", in place of `Indeterminate`'s menu.
- **Dependency direction:** `boolean::contain` depends on `splitting`, and never the reverse.

### Placement (`boolean/contain.rs`)

- **`ContainDecision { Loop(LoopDecision), ArcEnd, OneCircle, Carrier, WindowPeriod }`**:

  | decision | site | shape |
  |---|---|---|
  | `Loop(Boundary)` | contfp's pre-pass (`one_vertex`, `LoopEdge::contact`) | as above |
  | `ArcEnd` | `bool_contact_arc_end_vertex`: `Margin` in band; `Decided` where every end decided against every vertex | `LeverOnly` |
  | `OneCircle` | `bool_face_disc_carrier` | sized "gap between circles", `AnySign` |
  | `Carrier` | `bool_curved_contain_carrier` (cylinder, sphere, torus, cone) | `LeverOnly` (3493) |
  | `WindowPeriod` | `bool_curved_contain_period` | sized "gap", `Positive` |
  | `None` | `solid_err`; the sphere region's `RegionRefusal::Escalated` | the unnamed lever alone |

- **One source per decision:** `placement_subject`, `placement_lever` and `placement_ending`.
  - contfp's `Display` reads them: "contfp: {subject} is undecided: {payload}. {ending at a build}".
  - So do `validate::close_to_boundary` (at rest), the Boolean, and the census.

### Renderers

- **`validate.rs`** (RESTFRONT; seam note in `work/restfront/log.md`):
  - `classify_contain`, `classify_point_in_solid` and `classify_census_cause` return their recourse as a `Cow`.
  - `close_to_boundary` renders through `placement_ending(…, Reading::AtRest)`.
  - `OFF_BOUNDARY` is gone.
- **`census.rs`**: `Undecided::WitnessTooClose(Option<LoopDecision>)`.
  - `of_point_in_solid` carries the walk's escalation, and maps the door's own escalation to `None`.
  - Its sentence is the lead plus `placement_lever`, built once in a `LazyLock`.
  - **The chain breaks at `CensusUndecidable { what: &'static str }`**, so the census ends on the lever alone (`work/restfront/census-undecidable-what-cannot-carry-a-valued-ending`).
- `chord_join::ring_side` carries the walk's escalation whole. Before, it rebuilt it without its decision, and `first_decided` is generic over the escalation.

## Endings, before (main `d00100e82`) and after

The margin is in band at ±5e-9 m, with band (1e-9, 1e-8), K = 10. "Note" is "; an unreadable or collapsed margin may indicate a kernel bug worth reporting". `{subject}` is `placement_subject` of the carried decision (or of none: "whether a point lies inside a face, on its boundary, or outside it").

| site / arm | before | after |
|---|---|---|
| contfp `Escalated` `Display` | "contfp: {payload}" then `Indeterminate`'s coincidence menu | "contfp: {subject} is undecided: {payload}. {ending}" |
| walk `Escalated` `Display` | "whether a point lies in a loop is too close to call: {payload}" then the menu | "{subject} is undecided: {payload}. {ending}" |
| at-rest lead (`RingNestingUndecided`, `CensusUnsupported`) | "a point of it lies too close to a boundary to place at this tolerance" | "{subject} is undecided at this tolerance" (each subject names "a point", which the levers then call "the point") |
| at rest, `Loop(Boundary)`, ±5e-9 | "Recourse: move the geometry clear of the boundary" | "Recourse: move the point exactly onto the boundary or clearly off it, or, if this distance is intended, tighten the tolerance below 5e-10 m" |
| at rest, `Loop(Boundary)`, straddle or `Decided` (the walk's boundary disagreement) | the defect ending (read as `Invalid`) | the Boundary lever alone |
| at rest, any decision, poisoned | the defect ending | its lever + note |
| at rest, `Loop(Ray)` | "Recourse: move the geometry clear of the boundary" | "Recourse: nudge the point so no boundary corner lines up with it" |
| at rest, `Loop(ArcSpan)`, −5e-9 or straddle (the only arms its site raises) | as above | "Recourse: move the geometry so this arc stays clearly short of a full turn" |
| at rest, `Loop(Plane)` | as above | "Recourse: move the geometry so the face is flat and the point lies on it" |
| at rest, `ArcEnd`, in band or `Decided` | as above, or the defect ending | "Recourse: move the point clear of the arc's end" |
| at rest, `OneCircle`, +5e-9 (no reader yet: below) | as above | "Recourse: put the loop's arcs on one circle or on clearly different ones, or, if this gap between circles is intended, tighten the tolerance below 5e-10 m" |
| at rest, `Carrier`, ±5e-9 | as above | "Recourse: move the point exactly onto the face's surface or clearly off it" |
| at rest, `WindowPeriod`, +5e-9 | as above | "Recourse: keep the wall clearly short of a full turn, or, if this gap is intended, tighten the tolerance below 5e-10 m" |
| at rest, `WindowPeriod`, −5e-9 | as above | that lever alone |
| at rest, no decision named | "Recourse: move the geometry clear of the boundary" (the defect ending if poisoned) | "Recourse: move the point clearly inside or outside the face" (+ note if poisoned) |
| Boolean `Escalated`, carried decision | "…: whether a point lies inside a face, on its boundary, or outside it is undecided: {payload}. Recourse: move the parts so they meet clearly inside or clearly outside that face's boundary" | the same subject and payload, then the decision's own lever alone (+ note if poisoned), never a tolerance |
| Boolean `Escalated`, no decision named | as above | "…Recourse: move the point clearly inside or outside the face" (one unnamed lever at every door) |
| census `WitnessTooClose`, the walk escalated on d | "a corner of one lies too close to the other's boundary to place at this tolerance. Recourse: move the parts until their bounding boxes no longer overlap" | "testing a corner of one against the other, {d's subject} is undecided at this tolerance. Recourse: {d's lever}" |
| census `WitnessTooClose`, the door's own escalation | as above | the same sentence for no decision named, ending in the unnamed lever |
| `RayExhausted` (all doors) | main's ending | unchanged |

**Boundary's valued tighten is offered at contfp and at rest, never at the Boolean.** At rest and at contfp the refusal is about one point's place against one face. A smaller tolerance below |m|/K decides that point's distance, and the point's place is a size the user may intend.

The Boolean asks the walk at many points of the operation, and a smaller tolerance that decides this point moves other points' readings into the band. So no carried margin binds the operation there. The executed-offer census also holds the Boolean to a run case for every valued offer, and none exists. The Boolean therefore ends in the lever alone.

## Rows

**Sites driven with real geometry:**

- `topo/tests/review_m3_pr3_pil.rs::each_walk_site_tags_its_decision`:
  - a point at the band's midpoint off a square's edge gives `Boundary`, with a value and the valued tighten;
  - the 15-gon of `ray_exhausted_is_reachable`, each vertex moved off its ray line by the band's midpoint, leaves every ray in band, so the walk refuses on `Ray` with its lever alone.
- `boolean::contain::tests::contfp_tags_a_point_in_band_of_an_edge_as_the_boundary_question`: the holed box's top face gives `Some(Loop(Boundary))` with `Margin`, and the valued tighten at rest.
- `boolean::refusal_routes::tests::a_containment_escalation_on_a_residual_rung_names_its_lever_alone` (3493's, widened): a point in band of a cylinder wall gives `Some(Carrier)`, and the Boolean ends in the Carrier lever alone.
- `splitting::containment::tests::the_span_rule_tags_an_over_wound_margin_and_a_straddle`: `ConicArc::of` on an a/b = 100 ellipse gives a negative in-band margin and a `Straddle`, both ending in the lever alone.
- The existing `an_ellipse_*_straddles_it` rows now assert the `Straddle` tag. The scaffold-span row asserts `ArcSpan`.
- `each_walk_site_tags_its_decision` also drives `Plane`: a query at `(1, 1, 1 + m)` on the prism square, off its plane by the band's midpoint, escalates `Plane, Margin` at `certify_plane` and ends in its lever alone.
- **Not driven** (no cheap body):
  - `ArcEnd`;
  - `OneCircle`. Its ending has **no reader yet**: check 9's arm 4 discards `loop_circle`'s escalation and falls through to arm 5. Surfacing it moves check 9's verdicts, so it was not contained here. Filed as `work/restfront/check-9-drops-the-one-circle-escalation`.
  - `WindowPeriod`;

  These are pinned through the renderer.

**Renderer rows:**

- `boolean::contain::tests::an_escalation_ends_as_its_decision_gives_it`:
  - 19 rows over every decision and every arm a site raises, plus the unnamed decision, pinned by literal text;
  - each checks `Build` and `AtRest`, that the ending opens with `placement_lever`, the exact contfp `Display`, the walk's `Display`, and the at-rest `RingNestingUndecided` rendering;
  - it asserts that `ContainDecision::ALL` is covered.
- `census::tests` (main's `of_point_in_solid` row): the walk's escalation reads as `WitnessTooClose(Some(Boundary))`.
- `refusal_routes::tests` (the every-decision ending table): `every_decision()` now yields exactly the pairs sites raise, `boolean::CONTAINMENT_RAISED` (13 pairs), which the samples read too. A carried one is checked to route to `Ending::Placement`, to render `ContainDecision::lever_ending`, and to offer no tolerance.
- `offer_rows::every_site_names_the_decision_it_raises`: the source census of wrap sites is re-baselined to `CONTAINMENT_UNNAMED` for the unnamed sites and `Containment` for the carrying ones.
- `geom_brep::recourse::tests::a_straddle_and_a_lever_only_decision_name_the_lever_alone`.

**Word-budget samples, widened:**

- `test_support_samples::contain_errors`:
  - per decision and for none: an in-band value and a poisoned margin (`ArcSpan`: over-wound);
  - plus the straddles and `ArcEnd`'s `Decided`;
  - labelled `…/Escalated/<decision>/<escalation>/<kind>`.
- The `Undecided` samples add `WitnessTooClose(Some(d))` for each `LoopDecision`.
- The editor-core chains sample every `LoopDecision` in the forms its sites raise.

**What moved:**
- The Python suite and the text-asserting editor-core rows did not move.
- `every_check_finding_renders_within_the_budget` went red once, on main's newer subject rule: an escalation must say what was decided. The walk's `Display` now leads with its decision's subject.

**One red row, not this change's.** At `CAD_TOLERANCE_EPS=1e-12` (local battery on `9414addd6`), `sweep::parallel_cylinder_join::a_tipped_rod_whose_origin_is_stored_far_joins_along_its_rulings` refuses as `CrossingInsertion { Certification { Escalated { check: MappedSource, cause: carrier_matches_mapped_source, margin 1.25e-12 } } }`.
- That is edge certification inside a crossing insertion, a path this diff does not touch. The diff changes no verdict anywhere; it changes refusal words and carried types.
- The row came in on main with TANG's `7641c3271`.
- Main's nightly full suite is red. I could not read which tests, because the job log is served from a host the session's `gh` does not reach.
- It is not re-run on main here: the disk has 4.7 GB free, too little for a second target directory.

## Sweep: "lower the tolerance" in `crates/topo/src`

| hit | disposition |
|---|---|
| `contact.rs:237`, `:380`, `:470` | doc comments on `COINCIDENCE_RECOURSE`'s history; not an ending |
| `validate.rs:10810`, `:14781`, `:14787` | tests asserting its absence |
| `boolean/offer_rows.rs:2934`, `:2993` | a synthetic offer child and its judge, test-only |
| `chart_region.rs:405` (`TouchingBoundary`, and `RayExhausted` beside it) | "or read the pair at a tighter ε": unconditional and unvalued. Filed as `work/chart/chart-region-ray-exhausted-says-read-at-a-tighter-eps`. |

**What the literal grep cannot match:** main's `COINCIDENCE_RECOURSE` is now "declare the coincidence, or move the geometry", so the menu no longer says it. The class that is left is an unvalued or undeclarable recourse composed through `Indeterminate`'s `Display`. A grep for `{diag}` in containment's ground gives:
- contfp's and the walk's `Display`: fixed here;
- `PointInSolidError`'s `Display`: filed (narrowed row);
- `chart_region`'s `Escalated`: the chart program's.

## Findings filed

- `work/contact/point-in-solid-escalation-carries-no-decision` (narrowed).
- `work/contact/boolean-door-drops-the-containment-decision` (**closed** here).
- `work/contact/full-turn-question-has-three-spellings`: now four spellings, adding `BooleanDecision::ArcSpan`'s `Unsized::Defect` beside `LoopDecision::ArcSpan`'s user lever.
- `work/contact/census-containment-escalation-drops-its-decision`: the doors that drop or ignore the carried decision.
  - `census::read_containment` drops it into `CensusEscalated`'s coincidence menu. Fixing it needs a field on RESTFRONT's `CensusEscalated` or a re-split of `editor_core::attribute`'s `Escalated`/`Unsupported` halves, which is not contained here.
  - `SplitJoinError::RingHoming` ends in `JOIN_RECOURSE`.
  - `rim_wedge`'s `lift` drops it.
  - `sphere_region` drops the `Escalation` tag.
- `work/restfront/check-9-drops-the-one-circle-escalation`.
- `work/restfront/census-undecidable-what-cannot-carry-a-valued-ending`.
- `work/chart/chart-region-ray-exhausted-says-read-at-a-tighter-eps`.

## Left as is

- `BooleanError::PointInFaceRefused`'s `Display` still has an `Escalated` arm. Its doc says a sliver margin is `BooleanError::Escalated`, never this, and its raisers (`reduce::esc`, `extent_scan_refusal`) route `Escalated` elsewhere. Making it unrepresentable needs a `ContainError` without that arm, a new type at three raise sites, so it is not trivial. An `unreachable!` in a `Display` would panic a renderer, so I did not add one.

## Territory

- `validate.rs` (RESTFRONT): seam note.
- `geom-brep/src/recourse.rs`: seam note in ENCL's log.
- `splitting/containment.rs`'s previous owner, REACH, has closed; its log went with it.
- `boolean/*` is shared ground.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01HkgsMyrV52i5fDxhA2ojxL
