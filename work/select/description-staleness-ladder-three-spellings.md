---
id: description-staleness-ladder-three-spellings
kind: issue
title: The description staleness/adjacency-coherence ladder has three hand-kept spellings, already drifted - it wants one home
status: open
opened: 2026-08-31
refs: [1378, 1382, 1390, G9]
priority: P1
cost: H
---

## From GitHub issue 1391

Opened 2026-08-31; 0 comments.

(S-BOOL orchestrator) Filed from BOOL-1's dual review ([#1378](https://github.com/evgunter/cad/pull/1378)); both reviewers converged on the class independently. At #1378's frozen head `3f14f3c4`, three sites hand-spell "is this edge description coherent with its adjacent faces / stale":

- `crates/topo/src/splitting/finish.rs:477–489` (the #1378 smooth arm's keep-vs-restate ladder),
- `crates/topo/src/boolean/ops.rs:966–996` (`describe_minted_edges`' smooth arm),
- `crates/topo/src/validate.rs:2199–2218` (tier 3's `DescriptionNotAdjacent` reader).

Measured drift, not hypothetical: the split arm has no `Chart(c) if c.seam` clause where both siblings demand the chart be BOTH adjacent surfaces; the boolean keeps an `Intersection` naming the correct adjacent pair on a smooth edge where the split restates it ("wrong whatever it names") — one of those is wrong per D2; the boolean leaves `Scaffold` alone where the split restates; and the boolean **rebuilds** line carriers (`line_between`) where the split **restates** (#1382 carries that instance). Nothing states that the sites differ or why.

The ask: one home for the ladder (likely beside `EdgeDescription`), with the per-site policy differences either erased or stated as parameters — the S4 one-vocabulary-N-copies shape. Candidate additional copy sites a reviewer named for whoever takes it: `sweep/src/revolve/upgrade.rs:177`, `geom-brep/src/certify.rs:1643` (the other non-empty smooth arms), and #1390's extrude cap-rim arm.

S-BOOL ground (`boolean/`, `splitting/`; the validate.rs reader is Q's G9-adjacent territory) — scheduled behind the BOOL-Q track lanes or as its own unit; #1378's fix pass may close the seam-clause gap locally without waiting for this.

## Home

S-BOOL: two of the three spellings are in `crates/topo/src/splitting/*` and `crates/topo/src/boolean/*`, S-BOOL's territory, and the issue names S-BOOL ground with the `validate.rs` reader as Track Q's `G9`-adjacent seam.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to CURVED (its charter names S-BOOL's ceded ground and inherits at S-BOOL's exit) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.

## Evidence from PR 3580 (TOPO, 2026-09-30)

`crates/topo/src/attach.rs` now reads adjacency coherence through one
private `Named::adjacent_to`, over both the spec
(`EdgeDescriptionSpec`) and the stored description (`EdgeDescription`),
for `set_edge_curve`'s check and the re-chart doors'
(`set_face_surface`'s stranding refusal and
`set_face_surfaces_describing`). That folds the attach layer's own
copy into one, and it is a candidate home for the ladder: it already
takes the faces' surfaces as parameters, so a door asking about a state
it has not written yet can ask it. `validate.rs`'s tier-3 reader and
`splitting/finish.rs`'s keep-vs-restate arm still spell it by hand.

## Evidence from CLEAVE (branch cleave/smooth-arms)

`splitting/finish.rs`'s keep-vs-restate arm now chooses the kind
through `geom_brep::must_carry_over_edge`, the same rule the boolean
arm asks. It keeps an existing description only when that description
is of the demanded kind and names the current pair. So a coherent
chart on a jet-determinate seam is now re-minted intrinsic, and a
`TangentIntersection` on an under-determined one is restated. The
adjacency half of the ladder is still spelled by hand at both sites,
and the sites still differ:

- On a jet-determinate edge, the boolean re-mints even a coherent
  `TangentIntersection`, while the split keeps it.
- On an under-determined edge, the boolean keeps a coherent
  `TangentIntersection`, while the split restates it.

## Evidence from onedoor/at-rest-adjacency (2026-11-03)

Tier 3's reader is no longer hand-spelled: `validate.rs`'s
`DescriptionNotAdjacent` pass and `Body::stored_description_adjacent`
both call `Named::adjacent_at_rest` (`crates/topo/src/attach.rs`),
the surface-only `Named::adjacent_by_surfaces` plus face identity
(wrap iff the two halves bound one face). The gate and the re-chart
planner keep the surface-only reading under that name.

`crate::boolean::describe_edges`' smooth arm (`crates/topo/src/boolean/ops.rs`,
the `let stale = match ...` ladder) still spells the at-rest reading by
hand and leaves out face identity in two arms: a pair form and a
non-wrap `Chart` on an edge whose halves bound ONE face read not
stale, where `adjacent_at_rest` refuses both. Its wrap arm already
matches. Folding it onto `adjacent_at_rest` (scaffold arm aside) is not
a drop-in: on such an edge the restate path writes
`EdgeCurve::restated_at(rest_locus(..))`, which for a one-surface rest
is `restated_in_chart`, a non-wrap `Chart` (`EdgeDescriptionSpec::chart`),
so the edge would be re-described and still refused. The ladder's one
home has to decide what a one-face edge rests as (a wrap chart) before
that arm can take the shared reading.
Until then `merge_coplanar_faces`' debug postcondition
(`stored_description_adjacent` over every kept-boundary edge
`describe_edges` described) is the tripwire for that one-face hole: it
panics in a debug build, by intent, rather than return a body tier 3
refuses.

## Weighed (ONEDOOR, 2026-11-03)

Two designers converged after two reconciliation rounds; the recommendation is `[ev]` PR 6427. Ratified `[edges]` (PR 5216) leaves no stored kind choice at a two-surface edge, so this ladder needs no home. The question it answers goes away instead:
- the form is read off the sides by key;
- one restatement over the target form;
- one door re-describes an existing edge through the gate;
- the act that decides a coincidence leaves one surface record.

Once ratified, the row re-homes to SELECT, after SELECT's selector and in-band units. The one-record-per-coincidence part is its own unit, ahead of SELECT.

**Evidence for SELECT's in-band unit, found by the designers:**
- `describe_edges` (`boolean/ops.rs`, the `stale` match) keeps a witness `Intersection` on an edge it now reads smooth and under-determined, and tier 3 refuses it `NotTransverse`. Example: two trimmed planes at an angle below ε/reach.
- The split's `describe_section_boundary` reads a spline neighbour with `classify_dihedral` (always in band), where the boolean reads `classify_seam`.

**Correction to this row's table:** the "unconditional writes" in `crates/sweep` (revolve upgrade, extrude strut and cap rim, blend contact) are scaffold conversions. They are further spellings of the same ladder.

## Moved to SELECT (ONEDOOR, 2026-11-03)

Ev ratified the final state on PR 6427 (`[one-way.places]` and `[coincidence.door]` in `docs/DESIGN.md`, and `crates/geom-brep/README.md`'s edge-description paragraph):
- an edge's form is read off its sides by key (wrap, chart or pair);
- one restatement over the target form carries only the old description's data;
- one door re-describes an existing edge through the write gate;
- no door states a description for an existing edge; only a mint does.

The build is SELECT's. It lands after SELECT's selector and in-band units (the `onpair-has-no-incidence-lane-on-a-spline-adjacent-pair` units, and the in-band refusal), because the door's core matches the three ratified forms, not today's five arms.

When it lands, these retire:
- `describe_section_boundary`'s and `describe_edges`' smooth arms;
- `rest_parted_wrap_edges`;
- `reask_pair`/`reask_chart` and the per-variant re-ask matches;
- the sweeps' scaffold upgrade ladders.

It also closes `a-smooth-chart-describe-edges-keeps-reads-under-determined-only-at-its-extent` and `the-onpair-re-ask-doors-have-no-under-determined-fixture`.

Prerequisite outside SELECT: the act deciding a coincidence leaves one surface record (`onedoor/coincidence-leaves-one-surface-record`, ONEDOOR). The form reading off keys is exact only once that holds.
