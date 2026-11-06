# ROUND — log

Append-only (`work/README.md`, The log merges by union).

## 2026-11-06 — picked up (`active`)

A new sitting holds the track for its P0 and P1 rows (ten open, 27/30).
The program had no `plan.md` or `log.md` since the 2026-09-20 cut; both
are opened here. First step: a premise check of every row against the
current tree (the rows were filed 2026-09-05 to 09-18), then re-pricing
the legacy `cost: D` rows and cutting units.

## 2026-11-06 — premise check and the cut

A read-only survey checked all ten rows against main `230be11d05`.

- **Closed, already fixed elsewhere:** `arc-carrier-has-three-spellings-under-a-comment-saying-one`
  (PATHS 5b left one chord lowering, `Arc2::from_chord`) and
  `fillet-tangency-is-not-the-constructors-node` (PATHS 5a/5b: a segment
  carries its constructor's arc, the row's route 2). DECIDE's
  `declared-tangency-needs-the-registered-identity-door`, parked on the
  latter, is un-parked with a note that the arc-carrier fillets still
  register no tangency.
- **Re-priced** (legacy `D`): `carriers-are-identical-…` M + design,
  `fillet-step-discoverability-…` M (the refusal as the guide, not docs
  alone), `anchor-fit-…` M + design (a payload choice, weighed with the
  fused-step fork).
- **One unit for four rows.** The fillet-recourse rows
  (`validate-rs-hosts-…`, `fillet-inband-recourse-…`, `the-guided-replay-…`)
  and PROFILETAIL's `profile-endings-say-lower-the-tolerance-and-route-by-name`
  are one repair: ratified D4 ¶1 (ii) asks for a closed decision type
  where `PathError::Escalated` now routes by predicate name, and the
  fillet sentences' home, their tolerance arm and the guided door's text
  all follow from that type. Claimed the PROFILETAIL row (P3 → P1, it
  is the unblocking part); its carrier keeps its other three
  passengers. Filed `the-path-doors-escalation-carries-its-gate` (P1 H)
  as the carrier.
- **Found beside it:** the guided-door characterization row
  `a_guided_replay_still_renders_the_coincidence_recourse_for_a_fillet_gate`
  is vacuous since the declare lever retired (the box lever's own words
  contain "move the geometry"); the unit rewrites it.
- **Not a fork, ruled here:** the P0 turn gate keeps the levered margin
  (D4 [tolerance.margins] (i) requires it); the repair is the refusal's
  class and payload, a faithful elaboration of D4 ¶1 (iv) (the definite
  arm tells its in-band sibling's story). Phase 1 of the unit measures
  and proposes the class; if it needs a binding or editor contract
  change with several viable shapes, it comes back as a fork.
- **Forks, designers first:** carrier identity (two designers drawn,
  byte 217) and the refused corner's payload, rows 9 and 10 together
  because both decide what `CornerRefusal`/`CornerReason` carry for an
  author (three designers drawn, byte 12). Draws on
  `analysis/design-fork/round-*`.

## 2026-11-06 — turn gate, phase 1 adjudicated

The lane reproduced both misclassifications: the arc pair (0.57° on a
50ε leg) and the straight pair (`path_corner_turn`, levered by the
ray-origin-to-anchor separation, where even a 64° corner on a 0.6ε
separation reads "parallel"). `CarriersParallel` has four producers;
the two unlevered ones (`path_carrier_meet` and the circle–circle reach
Zero) are true and keep it.

Ruled unilaterally (a dominant argument, so not a fork): the refusal
kind `NoCornerForFillet` stays, because what the gate decides (no
corner to round at tolerance) is true of every input that reaches it;
`PathNoCornerReason` gains an arm carrying the factors (sin φ, align,
the lever and which lever), named for what the gate decided rather than
for a cause it cannot see. Rejected: a new `PathError` variant with its
own Python tag. That would move exactly parallel rays, where "parallel"
is true, to a new tag, to work around a pre-existing gap (a path
refusal's reason has no Python spelling), which is filed on pydoor
instead. The straight pair's definite arm names its own lever (the
anchor separation); the escalation lane is told its in-band
`path_corner_turn` ending must tell the same story.

## 2026-11-06 — escalation, phase 1 adjudicated

The lane's plan: a closed `profile::Gate` (one variant per decided
name, 77, declared by one table), an `Escalation { gate, source }`
minted only by the crate's funnel, so no ending can be looked up by
name; `DECISION_ENDINGS`, `fillet_recourse_for` and their kin deleted;
the fillet sentences in `fillet_recourse.rs`; the guided door composing
the box lever with the gate's lever through `lever_ending`; and the
read-back sentence's name routing replaced by a closed `seg::ArcRead`.
Ruled: one `PathError::Escalated { site, gate, source }` with a closed
site rather than a second variant for the stored-form read; existing
row kinds kept (`segment_straightness` stays `Lever`). On the two
corner turns my first answer (`Lever`, by analogy with
`path_junction_turn`) was wrong and is withdrawn: both are levered by
a length on the linear band, so they are `Sized` and the conditional
tighten is derived where a smaller ε decides; `path_junction_turn`
stays `Lever` and the lane files a row asking whether it is the odd one
out. Review tier raised from single to **dual** (H): the closed type is
what every path-door escalation carries from now on.

## 2026-11-06 — forks reconciling; a P0 found by measuring one

Both forks asked for facts the designers could not settle by reading,
so a probe lane measured them (branch `round/probes`,
`crates/profile/tests/round_probe_*`):

- **M1:** `line(L).cusp().fillet(r)` and `.cusp().fillet_arc(..)` move
  the straight leg's end vertex BACKWARDS (the ray-extension merge reads
  every by-tangent ray as a forward extension): the authored end leaves
  the path, and past the start the declared cusp is silently dropped;
  both validate. Filed as P0
  `a-cusp-before-a-fillet-pulls-the-straight-leg-end-backwards` and
  dispatched (`round/cusp-fillet-keeps-its-leg`, single FULL review):
  the merge extends a leg only along its forward tangent, which holds
  under either answer to the carrier-identity fork's open sub-choice.
- **M2:** the closed-form radius of the fillet that ends at the anchor
  equals the anchor-fit boundary on every straight×straight and
  line×arc entry, and on 2923 of 3185 grid-A arc×arc entries; in every
  other case it is below the loop's boundary because the other derived
  corner builds there; never above. Exact values land on the overrun
  side about a third of the time, so a rendered value must sit strictly
  on the fitting side.

Reconciliation so far (rows on the `[ev]` PRs): the refused-corner
designers converged on the shape after one round and are writing final
sections on the measurements; the carrier-identity designers agree on
retiring the arc identity decision and have crossed twice on the
straight-leg merge, so the last round asks for the question beneath
the split.

## 2026-11-06 — carrier identity goes to Ev (PR 6542)

Converged after three rounds: retire the arc identity decision (both
sure) and the straight-leg merge with it (both, one unsure). The PR
states the question beneath the split that crossed twice: is a
profile's segment list what the author emitted, or the maximal figure
where merging is free (is loft correspondence the profile's job)?
Draw bytes on `analysis/design-fork/round-carrier-identity`.

## 2026-11-06 — the refused corner goes to Ev (PR 6547); the cusp fix in review

The three refused-corner designers converged after two rounds and a
measurement: the `replay_drawn` side door (PR 6306, built on Ev's
2026-11-02 chat request) is the contract, completed; the anchor-fit
refusal carries both legs' setback and extent plus the closed-form
radius at the anchor, rendered strictly below, and its sentence values
both levers; a closed `Recourse`. A sweep of 40 radii below the
closed-form radius over 4073 entries found no re-trip, so "or less"
holds. Draw bytes on `analysis/design-fork/round-refused-corner-payload`.

The P0 cusp fix is PR 6545 (CI green): `ByTangent { No, Forward,
Reversed }`; only a forward ray merges; a cusp-reversed ray emits its
own segment from the leg's end, which validation refuses as an overlap
exactly as `.cusp().line(..)`. Single FULL review dispatched; it reuses
the probe lane's idle target because the box was down to 8 GB.
