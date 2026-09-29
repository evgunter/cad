# CONTACT-10: the containment refusals carry their decision

The rule is D4 ¶1 (i) in `docs/DESIGN.md`:

- a refusal's recourse follows from its decision and its verdict;
- the decision is a closed type at its site;
- a tighter tolerance is offered only on a band-decided arm of a decision that passes on a nonzero sign;
- that offer is phrased conditionally, with the value the margin gives.

The shared ending table is `geom_brep::recourse::SizedDecision`. This PR follows the shape of PRs 3390 and 3398.

Rows closed by this change:
- `work/contact/contact-near-boundary-endings-say-lower-the-tolerance` (its contain.rs half; 3398 did the census half);
- `work/contact/contain-escalation-carries-no-decision`.

## What changed

**`topo::boolean::ContainDecision`** is a new closed type in `boolean/contain.rs`. It names the question a placement escalated on. Its ending is an exhaustive match over the decision, not a lookup by predicate name.

| decision | rows | pass set | ending |
|---|---|---|---|
| `Boundary` | `bool_contact_vertex`, `bool_contact_edge*`, `bool_contact_arc*`, and the walk's `*_boundary`, `*_segment` and `*_conic_{on,end,trim}` rows | `NonZero` (every definite reading places the point) | the lever, plus the valued tighten |
| `Ray` | `point_in_loop_{side,advance,arm}` | none: a fact about the kernel's choice of ray, not a size the user chose | the lever alone |
| `ArcSpan` | the conic span rows in `carrier_loop` | `NonNegative` (a zero gap is a whole ellipse) | the lever, plus the valued tighten on the short side |
| `ArcEnd` | `bool_contact_arc_end_vertex` | none: its margin is not the point's distance, so no tolerance it gives decides the point | the lever alone |
| `OneCircle` | `bool_face_disc_carrier` | `NonNegative` | the lever, plus the valued tighten |
| `Carrier` | `bool_curved_contain_carrier` | `NonZero` | the lever, plus the valued tighten |
| `WindowPeriod` | `bool_curved_contain_period` | `Positive` | the lever, plus the valued tighten |
| `SolidDoor` | `PointInSolidError::Escalated` via `solid_err` | not carried; the chain breaks here | the lever alone |

The lever for `Ray`, `ArcEnd` and `SolidDoor` is "move the geometry clear of the boundary".

Changes along the chain:

- **`PointInLoopError::Escalated`** gains `decision`.
  - `point_in_loop` and `carrier_walk`'s boundary pass raise `Boundary`.
  - `polygon_walk` and `walk_schedule` raise `Ray`.
  - `carrier_loop` raises `ArcSpan`.
  - Its `Display` renders the payload and the decision's ending. Before, it rendered `Indeterminate`'s own `Display`, which composes the coincidence menu.
- **`ContainError::Escalated`** becomes `Escalated { decision, diag }`, and `From<PointInLoopError>` carries the decision.
  - `ContainError::ending(reading)` is the one ending of every arm.
  - `Display` ends `Escalated` and `RayExhausted` through it, at a build.
- **`topo::validate::classify_contain`** renders every arm through `ContainError::ending(Reading::AtRest)` and returns a `Cow`. That is RESTFRONT's ground; the edit is kept to this renderer and `classify_census_cause`'s return type. There is a seam note in `work/restfront/log.md`.
- **`census::Undecided::WitnessTooClose(Option<ContainDecision>)`**.
  - `of_point_in_solid` carries a `Loop(PointInLoopError::Escalated)` decision.
  - The door's own `Escalated`, `RayExhausted` and `Loop(RayExhausted | CorruptLoop)` carry `None`, as before.
  - A carried decision ends in its lever. The lever literals are shared through `contain_lever!`, so there is one lever text per decision.
  - **The chain breaks at `ValidationError::CensusUndecidable { what: &'static str }`.** A static string cannot hold the tolerance a margin gives, so the census ends on the lever alone. Filed as `work/restfront/census-undecidable-what-cannot-carry-a-valued-ending`.
- **`ContainError::RayExhausted`** (the grazed-parity refusal at contain.rs:~107; spec items 1 and 2 are this one site) ends in "Recourse: move the geometry clear of the boundary". It has a `Recourse:` marker and no tightening, because it carries no margin.
- Pattern-only changes:
  - `boolean/reduce.rs`, `boolean/ops.rs` and `census.rs`'s `contain` use `Escalated { diag, .. }`;
  - `splitting/containment.rs` is REACH's ground, with a seam note in `work/reach/log.md`;
  - `pncad/tests/all.rs` names the new payload type.

## Endings, before and after

The margin is in band at 5e-9 m, with band (1e-9, 1e-8), so K = 10.

| site | before | after |
|---|---|---|
| `ContainError::RayExhausted` `Display` | "…at this tolerance; move the point off the boundary or lower the tolerance" | "…at this tolerance. Recourse: move the geometry clear of the boundary" |
| `ContainError::Escalated` `Display` | "contfp: {payload} — a near-coincidence; Recourse: declare the coincidence, move the geometry, or lower the tolerance (D4)" (`Indeterminate`'s menu) | "contfp: {payload}. {decision's ending}" |
| `classify_contain`, `Escalated`, `Boundary` at 5e-9 m (or -5e-9 m) | "Recourse: move the geometry clear of the boundary" | "Recourse: move the geometry so the point lies either exactly on the boundary or clearly off it, or, if this distance is intended, tighten the tolerance below 5e-10 m" |
| same, `Boundary` with a straddling enclosure | as above | "Recourse: move the geometry so the point lies either exactly on the boundary or clearly off it" |
| same, `Boundary` with a poisoned margin | the defect ending | the lever, then "; an unreadable or collapsed margin may indicate a kernel bug worth reporting" |
| same, `ArcSpan` at +5e-9 m | "Recourse: move the geometry clear of the boundary" | "Recourse: move the geometry so this arc stays clearly short of a full turn, or, if this arc is intended, tighten the tolerance below 5e-10 m" |
| same, `ArcSpan` at -5e-9 m | as above | "Recourse: move the geometry so this arc stays clearly short of a full turn" |
| same, `OneCircle` / `Carrier` / `WindowPeriod` | as above | each decision's lever, plus the valued tighten on its passing side ("difference between the circles", "distance", "arc") |
| same, `Ray` / `ArcEnd` / `SolidDoor` | as above | "Recourse: move the geometry clear of the boundary" |
| `classify_contain`, `RayExhausted` | "Recourse: move the geometry clear of the boundary" | unchanged |
| `PointInLoopError::Escalated` `Display` | "…too close to call: {Indeterminate with the coincidence menu}" | "…too close to call: {payload}. {decision's ending}" |
| census `WitnessTooClose`, from a loop walk's `Boundary` | "…Recourse: move the parts until their bounding boxes no longer overlap" | "…Recourse: move the geometry so the point lies either exactly on the boundary or clearly off it" |
| census `WitnessTooClose`, from the door's own escalation or an exhausted schedule | as above | unchanged |

## Rows

Added:
- `boolean::contain::tests::an_escalation_ends_as_its_decision_gives_it`. It pins 14 rows, valued and unvalued, over every decision. Each row checks the ending at `Build` and `AtRest`, the `ContainError` `Display` (exact), the `PointInLoopError` `Display`, and the at-rest `RingNestingUndecided` rendering.
- `boolean::contain::tests::an_exhausted_schedule_ends_in_the_lever_alone`. It pins the `RayExhausted` `Display` exactly, and its at-rest rendering.
- `census::tests::a_witness_too_close_ends_in_its_decisions_lever`. It covers `Boundary`, `Ray` and `ArcSpan` carried from the walk, the door's own escalation, and an exhausted schedule.

Moved or widened:
- `test_support_samples::contain_errors` now samples `Escalated` once per decision, labelled `…/Escalated/<Decision>`.
- The `Undecided` samples add `WitnessTooClose(Some(d))` for every decision.
- `editor-core/tests/refusal_concision_chains.rs`'s `Loop(Escalated)` sample becomes three: `Boundary`, `Ray` and `ArcSpan`.
- `topo/tests/review_m3_pr3_pil.rs::the_verdict_is_blind_to_the_normals_sign` now compares the decision too.
- `sweep/tests/contfp_reads_arcs_on_their_carriers.rs` matches the struct variant.

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
  - **filed:** `PointInSolidError`'s `Display` (`Escalated`, `RayExhausted`, `Loop(RayExhausted)`) ends in the menu, and the door takes no declaration. Filed as `work/contact/point-in-solid-escalation-carries-no-decision`.
  - **filed:** `BooleanError::Escalated` gets the menu for every contfp decision, because reduce.rs and ops.rs drop the decision. A boolean does take a declaration, so the menu is right for a coincidence decision but not for `ArcSpan`, `WindowPeriod` or `Ray`. Filed as `work/contact/boolean-door-drops-the-containment-decision`.
  - **not this unit:** the other `{diag}` sites (`chart_region.rs`, `euler.rs`, `merge_faces.rs`, `pcurves.rs`, `shell.rs`, `replace_face.rs`, `splitting/mod.rs`, `chord_join.rs`) are other programs' escalations, each tracked by that program's coincidence-menu rows, such as `work/band/every-escalation-carries-the-coincidence-recourse-first`.
- A differently worded tightening: a grep for "tighter" and for "lower ε" finds `ChartRegionError::RayExhausted`'s `Display`, "or read the pair at a tighter ε", which is unconditional and unvalued. Filed as `work/chart/chart-region-ray-exhausted-says-read-at-a-tighter-eps`. The other hit, contain.rs's old "lower ε" comment, is rewritten.

## Findings filed

- `work/contact/point-in-solid-escalation-carries-no-decision`: `PointInSolidError::Escalated` names none of its roughly thirty rows, and its `Display` ends in the coincidence menu. It also records that `Loop(CorruptLoop)` is reported as `WitnessTooClose` rather than `CorruptInstance`.
- `work/contact/boolean-door-drops-the-containment-decision`.
- `work/restfront/census-undecidable-what-cannot-carry-a-valued-ending`.
- `work/chart/chart-region-ray-exhausted-says-read-at-a-tighter-eps`.

## Territory

`work.py territory` flags these paths as other programs' ground:

- `validate.rs` (RESTFRONT): the edit is kept to `classify_contain`'s rendering; seam note in `work/restfront/log.md`.
- `splitting/containment.rs` and `boolean/ops.rs` (REACH): seam note in `work/reach/log.md`.
- The `chord_join.rs` and `chart_region.rs` entries are pattern-free reads, not edits. The rows filed on the chart and restfront slates are new files only.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01HkgsMyrV52i5fDxhA2ojxL
