# CONTACT-10: the containment refusals carry their decision

The rule is D4 ¶1 (i) in `docs/DESIGN.md`:

- a refusal's recourse follows from its decision and its verdict;
- the decision is a closed type at its site;
- a tighter tolerance is offered only on a band-decided arm of a decision that passes on a nonzero sign;
- that offer is phrased conditionally, with the value the margin gives.

The shared ending table is `geom_brep::recourse`. This PR follows the shape of PRs 3390 and 3398.

Rows closed by this change:
- `work/contact/contact-near-boundary-endings-say-lower-the-tolerance` (its contain.rs half; 3398 did the census half);
- `work/contact/contain-escalation-carries-no-decision`.

## What changed

### The shared table (`geom_brep::recourse`)

Three additions, which `work.py territory` places under no program. There is a seam note in `work/encl/log.md`, since ENCL authored the table.

- **`SizedPass::Definite`**: a decision whose every definite reading answers (positive, zero or negative). It refuses only an undecided margin, and it tightens on either side. The point's place against a boundary is this, not `NonZero`, because the point may lie exactly on the boundary.
- **`RefusedArm::Straddle`**: two sound bounds on one margin straddle the band, so no margin states the reading. It is not a poisoned margin.
  - A sized decision names the lever alone on it.
  - `Unsized::LastResort` reads it as undecided.
- **`LeverOnly`**: the shared shape of a lever-only decision. Every arm ends in the lever, and a poisoned margin adds the unreadable-margin note. It replaces the hand-written lever-only branch.

### The loop walk (`splitting/containment.rs`, REACH's ground)

**`LoopDecision { Boundary, Ray, ArcSpan }`** is exactly the set the walk raises.

| decision | sites | shared-table shape |
|---|---|---|
| `Boundary` | `point_in_loop`'s pre-pass (`*_segment`, `*_boundary`) and `carrier_walk`'s boundary pass (`LoopEdge::contact`: segment, conic `on`/`end`/`trim`, and the boundary disagreement) | `SizedDecision`, size "length", `Definite`. Its margins are the point's distance, or a straight edge's own length where that edge reads as a point. |
| `Ray` | `polygon_walk`'s side and advance rows, and `walk_schedule`'s arm row | `LeverOnly`: the point is already clear of the boundary, and the margin is a fact about the kernel's choice of ray |
| `ArcSpan` | `carrier_loop` via `ConicArc::of` | `SizedDecision`, size "arc", `NonNegative`. The site escalates only on a negative in-band margin (over-wound) or a straddle, so the valued arm is unreachable. It ends in the lever alone. |

**`Escalation { Margin, Straddle, Decided }`** records how an escalation's reading stands, as the site knows it. It fills the one gap `Indeterminate` leaves: `invalid_margin` sites raise `Invalid` for three different reasons.

- `ConicArc::hit`, `LoopEdge::contact` and `ConicArcError::Escalated` now return `(Escalation, Indeterminate)`.
- The two ellipse straddles (`*_conic_straddle`) are tagged `Straddle`. They no longer take the unreadable-margin note.

Both types live beside their raiser, so `boolean::contain` depends on `splitting` and not the reverse. Both are re-exported at `topo`'s root.

### Placement (`boolean/contain.rs`)

**`ContainDecision { Loop(LoopDecision), ArcEnd, OneCircle, Carrier, WindowPeriod }`**.

| decision | site | shape |
|---|---|---|
| `Loop(Boundary)` | contfp's pre-pass: vertex, and `LoopEdge::contact` | as above |
| `ArcEnd` | `bool_contact_arc_end_vertex` | `LeverOnly`. In band it is `Margin`. Every end decided against every vertex is `Decided`, not a poisoned `Invalid`. |
| `OneCircle` | `bool_face_disc_carrier` in `loop_shape` | `SizedDecision`, "gap between circles", `Definite` |
| `Carrier` | `bool_curved_contain_carrier` (cylinder, sphere, torus, cone) | `SizedDecision`, "distance", `Definite` |
| `WindowPeriod` | `bool_curved_contain_period` | `SizedDecision`, "sweep", `Positive`: zero and negative are the door's remainder, not an answer |

Changes to `ContainError` and its renderers:

- **"No decision named" has one spelling, `None`:**
  - `ContainError::Escalated { decision: Option<ContainDecision>, escalation, diag }`, where `solid_err` gives `None`;
  - `census::Undecided::WitnessTooClose(None)`.

  It has one lever, "move the geometry clear of the boundary". `SolidDoor` is gone.
- **One source for every lever: `boolean::placement_lever(Option<ContainDecision>)`**, which delegates to `LoopDecision::lever` for the walk's decisions.
  - Every `SizedDecision` and `LeverOnly` here reads it.
  - The census builds its `&'static str` sentences from it once, in a `LazyLock`.
  - The `contain_lever!` macros are gone.
- **`ContainError::ending(reading)`** covers every arm. `RayExhausted` is every ray grazing, a zero on the ray's own decision, so it ends as `LeverOnly(Ray)` gives it.
- **`ContainError`'s `Display`** is one match that writes the payload, then `. {ending(Build)}`. The ArcLoopUnsupported lever text and the Corrupt repair now exist once, in `ending`.

### Renderers

- **`topo::validate::classify_contain`** (RESTFRONT's ground) returns `(&str, String)` through `ContainError::ending(Reading::AtRest)`. `classify_census_cause` wraps it. There is a seam note in `work/restfront/log.md`.
- **`census::Undecided::WitnessTooClose(Option<LoopDecision>)`**. It carries only decisions the walk can raise.
  - The walk's `Escalated` carries its decision, and the walk's `RayExhausted` carries `Some(Ray)`.
  - The door's own `Escalated` and `RayExhausted` carry `None`.
  - `Loop(CorruptLoop)` now reads as `CorruptInstance`. Before, it was reported as a corner too close to a boundary.
  - The lead names the corner as the point the lever speaks of.
  - **The chain breaks at `ValidationError::CensusUndecidable { what: &'static str }`**, so the census ends on the lever alone. Filed as `work/restfront/census-undecidable-what-cannot-carry-a-valued-ending`.
- **Spec items 1 and 2 are one site:** `ContainError::RayExhausted`'s `Display`, the grazed-parity refusal at contain.rs ~:107.

### Other items from the review

- **`solid_contain.rs`'s comment** "the split's wrapper states its own recourse": checked, and it holds. `chord_join::SplitJoinError::RingHoming` renders `diag.payload()` with its own `Recourse:`, and never `PointInLoopError`'s `Display`, so nothing doubles.
- Pattern-only changes:
  - `boolean/reduce.rs`, `boolean/ops.rs` and `census.rs`'s `contain` use `Escalated { diag, .. }`;
  - `pncad/tests/all.rs` names the new payload types (`LoopDecision`, `Escalation`, `Option<ContainDecision>`).

## Endings, before and after

"Before" is `main` at `f4e9aa68b`. The margin is in band at ±5e-9 m, with band (1e-9, 1e-8), so K = 10. "Poisoned" is `MarginDiag::Invalid`.

| site / arm | before | after |
|---|---|---|
| `ContainError::RayExhausted` `Display` | "contfp: every direction … — the point sits within ε of the boundary at this tolerance; move the point off the boundary or lower the tolerance" | "contfp: every direction of the parity schedule grazed the face's boundary, so no ray read a definite crossing count at this tolerance. Recourse: nudge the point so no boundary corner lines up with it" |
| `ContainError::Escalated` `Display` | "contfp: {payload} — a near-coincidence; Recourse: declare the coincidence, move the geometry, or lower the tolerance (D4)" | "contfp: {payload}. {the decision's ending, below}" |
| `ContainError::Corrupt` `Display` | "…does not resolve; repair the body's topology before asking it a containment question" | "…does not resolve. {kernel defect ending}" |
| `ContainError::ArcLoopUnsupported` `Display` | "…expresses the region there — refused rather than answered; model the boundary with lines, circles or ellipses" | "…expresses the region there. Recourse: model the boundary with lines, circles or ellipses" |
| `PointInLoopError::Escalated` `Display` | "whether a point lies in a loop is too close to call: {payload} — a near-coincidence; Recourse: declare the coincidence, move the geometry, or lower the tolerance (D4)" | "cannot place a point in a loop: {payload}. {the decision's ending}" |
| at rest, `Loop(Boundary)`, ±5e-9 | "Recourse: move the geometry clear of the boundary" | "Recourse: move the point exactly onto the boundary or clearly off it, or, if this length is intended, tighten the tolerance below 5e-10 m" |
| at rest, `Loop(Boundary)`, straddle (ellipse bounds) | the defect ending (it read as `Invalid`) | "Recourse: move the point exactly onto the boundary or clearly off it" |
| at rest, `Loop(Boundary)`, poisoned | the defect ending | the Boundary lever, then "; an unreadable or collapsed margin may indicate a kernel bug worth reporting" |
| at rest, `Loop(Ray)`, in band | "Recourse: move the geometry clear of the boundary" | "Recourse: nudge the point so no boundary corner lines up with it" |
| at rest, `Loop(Ray)`, poisoned | the defect ending | the Ray lever, then "; an unreadable or collapsed margin …" |
| at rest, `Loop(ArcSpan)`, −5e-9 (the only in-band arm the site raises) | "Recourse: move the geometry clear of the boundary" | "Recourse: move the geometry so this arc stays clearly short of a full turn" |
| at rest, `Loop(ArcSpan)`, straddle | the defect ending | "Recourse: move the geometry so this arc stays clearly short of a full turn" |
| at rest, `ArcEnd`, in band | "Recourse: move the geometry clear of the boundary" | "Recourse: move the point clear of the arc's end" |
| at rest, `ArcEnd`, every end decided (`Decided`) | the defect ending | "Recourse: move the point clear of the arc's end" |
| at rest, `ArcEnd`, poisoned | the defect ending | the ArcEnd lever, then "; an unreadable or collapsed margin …" |
| at rest, `OneCircle`, +5e-9 | "Recourse: move the geometry clear of the boundary" | "Recourse: put the loop's arcs on one circle or on clearly different ones, or, if this gap between circles is intended, tighten the tolerance below 5e-10 m" |
| at rest, `Carrier`, ±5e-9 | "Recourse: move the geometry clear of the boundary" | "Recourse: move the point exactly onto the face's surface or clearly off it, or, if this distance is intended, tighten the tolerance below 5e-10 m" |
| at rest, `WindowPeriod`, +5e-9 | "Recourse: move the geometry clear of the boundary" | "Recourse: move the geometry so the wall sweeps clearly less than a full turn, or, if this sweep is intended, tighten the tolerance below 5e-10 m" |
| at rest, `WindowPeriod`, −5e-9 | "Recourse: move the geometry clear of the boundary" | "Recourse: move the geometry so the wall sweeps clearly less than a full turn" |
| at rest, no decision named (`solid_err`), in band | "Recourse: move the geometry clear of the boundary" | unchanged |
| at rest, no decision named, poisoned | the defect ending | "Recourse: move the geometry clear of the boundary; an unreadable or collapsed margin …" |
| at rest, `RayExhausted` | "Recourse: move the geometry clear of the boundary" | "Recourse: nudge the point so no boundary corner lines up with it" |
| at rest, Corrupt and ArcLoopUnsupported | defect / remodel | unchanged |
| census `WitnessTooClose`, the walk escalated on d | "a corner of one lies too close to the other's boundary to place at this tolerance. Recourse: move the parts until their bounding boxes no longer overlap" | "a corner of one, the point the check tests, lies too close to the other's boundary to place at this tolerance. Recourse: {d's lever}" |
| census `WitnessTooClose`, the walk's rays all grazed | as above | "… Recourse: nudge the point so no boundary corner lines up with it" |
| census `WitnessTooClose`, the door's own escalation or rays | as above | "… Recourse: move the geometry clear of the boundary" |
| census, the walk's `CorruptLoop` | the `WitnessTooClose` sentence | `CorruptInstance`: "one part's topology could not be walked. {kernel-or-file defect ending}" |

## Rows

### Driving the sites with real geometry

- `topo/tests/review_m3_pr3_pil.rs::each_walk_site_tags_its_decision`. It uses `point_in_loop` on prism top faces.
  - A point at the band's midpoint off a square's edge escalates in the pre-pass on `Boundary`, with a value, and ends in the valued tighten.
  - A dart whose tip sits at the band's midpoint off the first schedule ray's line escalates in `polygon_walk` on `Ray`, and ends in the lever alone.
- `boolean::contain::tests::contfp_tags_a_point_in_band_of_an_edge_as_the_boundary_question`. contfp on the holed box's top face, in band of its outer edge, gives `Some(Loop(Boundary))` with `Margin` and a valued ending at rest.
- `boolean::contain::tests::the_sphere_arm_tags_a_point_in_band_of_its_carrier`. `curved_face_placement` on a prism side relabelled as a sphere, in band of the sphere, gives `Some(Carrier)` with `Margin`.
- `splitting::containment::tests::the_span_rule_tags_an_over_wound_margin_and_a_straddle`. `ConicArc::of` on an a/b = 100 ellipse:
  - an over-wind at the band's midpoint gives `(Margin, negative value)`, ending in the lever alone;
  - an over-wind of half the zero band gives `Straddle`, ending in the lever alone with no note.

  `carrier_loop` maps both to `LoopDecision::ArcSpan` unchanged. A body with such an edge is not cheap to build, so the row stops at `ConicArc::of`.
- `splitting::containment::tests::an_ellipse_tighter_than_the_band_straddles_it` (existing) now asserts that the `hit` straddle is tagged `Straddle`.
- **Not driven**, because none is cheap to reach with a real body:
  - `ArcEnd`: a conic end within the band while the point is clear of both vertices;
  - `OneCircle`: two arcs of circles within the band of each other;
  - `WindowPeriod`: a cylinder window within the band of a full turn, past the chart trim.

  They are pinned through the renderer only.

### The renderer

- `boolean::contain::tests::an_escalation_ends_as_its_decision_gives_it`:
  - 17 rows over every decision and every arm a site can raise, plus the unnamed decision, pinned by literal text;
  - each row checks the ending at `Build` and `AtRest`, that it opens with `placement_lever`'s lever, the `ContainError` `Display` (exact), the `PointInLoopError` `Display` for the walk's decisions, and the at-rest `RingNestingUndecided` rendering;
  - it asserts that `ContainDecision::ALL` is covered.
- `boolean::contain::tests::the_marginless_refusals_end_in_one_sentence_each`: the `RayExhausted`, `Corrupt` and `ArcLoopUnsupported` `Display`s, exactly.
- `census::tests::a_witness_too_close_ends_in_its_decisions_lever`:
  - for every `LoopDecision`, the census tail equals the lever that `ContainError::ending` names;
  - it covers the walk's exhausted schedule, the door's own escalation and exhausted schedule, and `CorruptLoop` reading as `CorruptInstance`.
- `geom_brep::recourse::tests::a_definite_decision_tightens_either_side_and_a_straddle_names_the_lever` and `…::a_lever_only_decision_ends_in_its_lever`. The pass-set rows now include `Definite`.

### Word-budget samples, widened

- `test_support_samples::contain_errors` gives:
  - for the unnamed decision and each of the 7 decisions: `Margin` with an in-band value, and `Margin` with a poisoned margin;
  - `Straddle` for `Loop(Boundary)` and `Loop(ArcSpan)`;
  - `Decided` for `ArcEnd`.

  They are labelled `…/Escalated/<decision>/<escalation>/<kind>`. The `Undecided` samples add `WitnessTooClose(Some(d))` for each `LoopDecision`.
- `editor-core/tests/refusal_concision_chains.rs`: `Loop(Escalated)` samples every `LoopDecision`, with a value and poisoned.

**What moved in the battery.** The Python suite and the `editor-core` rows that assert refusal texts did not move. The 75-word budget rows did, twice:

- On the first cut of the levers, `every_at_rest_finding_renders_to_the_standard` and `every_check_finding_renders_within_the_budget` ran 76–81 words.
- On the widened samples, the carried renderings of `OneCircle` (value and poisoned) and of poisoned `Ray` ran 76–80 words at rest. The separation check's chain of a poisoned `Boundary` or `Ray` ran 79–82 words.

Each time the levers were shortened to fit, rather than raising the budget:

- "move the geometry so the loop's arcs lie on one circle or on clearly different circles" became "put the loop's arcs on one circle or on clearly different ones", with size "gap between circles";
- the Ray lever became "nudge the point so no boundary corner lines up with it";
- `PointInLoopError`'s lead became "cannot place a point in a loop:". The budget row reads it inside the separation check's chain, so this let the poisoned forms fit.

The word-budget samples also dropped a positive `ArcSpan` margin, which the site cannot raise. They sample its over-wound margin and its straddle instead.

## Sweep: "lower the tolerance" in `crates/topo/src`

| hit | disposition |
|---|---|
| `contact.rs:107`, `:242`, `:332` | doc comments about `COINCIDENCE_RECOURSE`'s third arm; not an ending |
| `validate.rs` `too_close` (`:2195`, `:2197`) | the coincidence menu, spelled for `CensusEscalated` and the chart-region `Escalated`/`RayExhausted` arms. D4 ¶1 (i) permits it ("the three-arm sentence … is the recourse of a decision whose refused side is a declarable coincidence"), composed per (ii). 3398's `no_validate_ending_says_lower_the_tolerance` holds exactly those arms to it. |
| `validate.rs:9575`, `:12627`, `:12633` | tests asserting the phrase's absence |
| `boolean/contain.rs:107` | **fixed** |

**What the literal grep cannot match**, and the second pass aimed at that gap:

- The phrase lives in `geom_core::COINCIDENCE_RECOURSE`, so a site that composes the constant, or renders an `Indeterminate` through its own `Display` (`{diag}`), says it without the literal. A grep for `COINCIDENCE_RECOURSE` and for `{diag}`/`{cause}`/`{source}` over `crates/topo/src` gives this list:
  - **fixed here:** `ContainError::Escalated`'s `Display` and `PointInLoopError::Escalated`'s `Display`.
  - **filed:** `PointInSolidError`'s `Display` (`Escalated`, `RayExhausted`, `Loop(RayExhausted)`). Filed as `work/contact/point-in-solid-escalation-carries-no-decision`.
  - **filed:** `BooleanError::Escalated` gets the menu for every contfp decision. Filed as `work/contact/boolean-door-drops-the-containment-decision`.
  - **not this unit:** the other `{diag}` sites (`chart_region.rs`, `euler.rs`, `merge_faces.rs`, `pcurves.rs`, `shell.rs`, `replace_face.rs`, `splitting/mod.rs`, `chord_join.rs`), each tracked by its program's coincidence-menu rows, such as `work/band/every-escalation-carries-the-coincidence-recourse-first`.
- A differently worded tightening: a grep for "tighter" and for "lower ε" finds two arms of `ChartRegionError`'s `Display`, both "…or read the pair at a tighter ε", which is unconditional and unvalued:
  - `RayExhausted`;
  - `TouchingBoundary` (`chart_region.rs` ~:399).

  Both are filed as `work/chart/chart-region-ray-exhausted-says-read-at-a-tighter-eps`.
- A poisoned-margin note on a sound escalation: `invalid_margin`'s three meanings. The loop walk's and contfp's straddles and decided fallbacks are now tagged. The point-in-solid door's are not, and that is recorded in its row.

## Findings filed

- `work/contact/point-in-solid-escalation-carries-no-decision`: the door names neither its decision nor how its reading stands, and its `Display` ends in the coincidence menu. Its repair shape now asks for `SizedPass::Definite`.
- `work/contact/boolean-door-drops-the-containment-decision`.
- `work/contact/full-turn-question-has-three-spellings`: ArcSpan and certify's `ParamWinding` are one decision whose lever text is copied, because certify's table is private. `WindowPeriod` is a different pass set on a different object. Unifying them reaches into certify, so it is not contained here.
- `work/restfront/census-undecidable-what-cannot-carry-a-valued-ending`.
- `work/chart/chart-region-ray-exhausted-says-read-at-a-tighter-eps` (RayExhausted and TouchingBoundary).

## Territory

- `validate.rs` (RESTFRONT): the edit is kept to `classify_contain`'s rendering and `classify_census_cause`'s wrap; seam note in `work/restfront/log.md`.
- `splitting/containment.rs`, `splitting/mod.rs` and `boolean/ops.rs` (REACH): seam note in `work/reach/log.md`.
- `geom-brep/src/recourse.rs` (no program): seam note in `work/encl/log.md`.
- The rows filed on the chart and restfront slates are new files or edits to files this branch filed.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01HkgsMyrV52i5fDxhA2ojxL
