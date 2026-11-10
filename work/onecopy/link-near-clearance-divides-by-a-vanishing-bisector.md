---
id: link-near-clearance-divides-by-a-vanishing-bisector
kind: issue
title: link's Near clearance reads along a bisector that vanishes where the two ends are near-antipodal, a [simple.conditioned] defect to re-pose
status: closed
opened: 2026-11-09
priority: P2
cost: M
closed: 2026-11-10
pr: 6703
---


Found by QML T2 (`onecopy/refusal-standing`), acting on PR 6637's third-delta note.

## What

`link::ArcReach::clears` (`crates/topo/src/link.rs`, the `Near` arm) reads whether the segment `q`–`w` clears the apex `p` as both ends' distances along `half = normalize(q̂ + ŵ)`, the bisector of their directions. Where `q̂` and `ŵ` are near-antipodal, `q̂ + ŵ` is a vanishing quantity, and normalizing it amplifies its rounding into the direction it reads along. The distances along that direction are then noise, and can be decided negative.

DESIGN `[simple.conditioned]` says such a formula is a defect, to be re-posed, never covered by K. It is not a refusal to name (orchestrator, 2026-11-09).

## Witnesses

A `debug_assert!` that no clearance reads negative (QML T2, reverted) fired on six tests:

- `topo::all a_doubled_line_does_not_revert::the_unequal_pinch_grafted_as_a_cavity_refuses_its_wedges_at_tier_3_prime`, at both ε;
- `topo link::cleared_rows::a_keel_off_the_arcs_is_read_at_the_faces_reach_not_the_cleared_radius`, at ε 1e-12;
- `topo::all a_decided_overlap_decides_its_pair::a_crossing_touch_outranks_an_escalation_read_before_it`, at ε 1e-12;
- `topo::all coincident_line_wedges::a_pinwheel_beside_an_escalation_refuses_nothing_undecidable`, at ε 1e-12;
- `topo::all a_keel_on_the_floor_subtracts_as_a_slit::a_fate_corner_inside_an_edge_of_its_faces_division_builds` (slow set), at both ε;
- `topo::all a_keel_on_the_floor_subtracts_as_a_slit::a_partner_flush_beside_the_keel_unions_with_the_slit_at_tier_three` (slow set), at both ε.

To reproduce, put `debug_assert!(sign != Sign::Negative, ..)` in `clears`' `ends` closure and run those rows with `--profile ci` (debug).

## Re-posing

One candidate: read each end's distance along the wedge's own in-plane normal, or read the side directly, rather than along the bisector. That direction does not vanish with the angle between the ends.

## Done when

- The clearance is re-posed so that no reading direction vanishes with the configuration.
- The witnesses above pass with the debug assertion that no clearance reads negative, kept as a pin.
- `clears`' doc (which states the reachable negative today) is rewritten to match.
- `Unread::NoReference`'s "every arc decidedly meets the apex" arm is re-examined.

## Closed (2026-11-10): re-posed as the segment's own clearance

**Measured first.** A temporary print at the near arm, over the six witnesses, read 20882 arcs at default ε and 20731 at 1e-12. `|q̂ + ŵ|` reached `0` exactly, where the bisector is a 0/0 NaN and so was the old reading, and `8e-17` to `7e-16` above it. There the two ends read `±0.04` to `±1.9` m along the bisector against a true clearance of `1e-16` m. They were decided negative 123 times at default ε and 142 at 1e-12. The ends' distances agreed to `3e-15` relative, so the arc's ends do stand at one distance.

**Re-posed.** `ArcReach::clears` reads, for both reaches, `link::metric::Distance::clearance(p, (q, w))`: the distance from `p` to the segment's nearest point (`link::to_segment`), a fifth `Distance` constructor. It is a norm of a difference, so no direction is read off a quantity that vanishes. At one distance it is `ρ·|q̂ + ŵ|/2` to the coordinates' rounding, and a segment through `p` reads `0`, not the NaN the far reach's normalized `m − p` gave there. A clearance is never decided negative, so `clears` never answers "does not clear". The `debug_assert` is kept as the pin.
- `Unread::NoReference` no longer claims an arc that decidedly meets the apex; it is reached only by a cone with no wedge.
- `link::foot`, which only the far reach read, is gone.

**Pinned** by `link::clearance_rows::nearly_opposite_ends_read_the_segments_own_clearance`: `δ` down to `0` at five radii and five turns. It is red under the bisector reading ("an arc decided to meet the apex"). The six witnesses pass at both ε with the pin live.

## Follow-up (2026-11-10, `onecopy/link-norms`, from PR 6703's review)

- The class's two siblings are re-posed through `Distance::apart(p, q)`, a sixth constructor that is `|p − q|` with no direction read off it. They read a norm as `Distance::of(x, f, normalize(x − f))`, which is NaN at the very configuration they decide:
  - `Link::lowest_on`'s `meets`;
  - `Cone::touches`' ray tie.
- A sweep of `topo` for `Distance::of(a, b, (a − b).normalize())` in any spelling finds no other.
- `ArcReach::clears` returns `Result<(), Unsettled>`, so the impossible "does not clear" cannot be written. In release a clearance decided negative refuses as a zero does; it no longer passes silently. The dead `Ok(false) => Rung::Passes` arm and its doc line are gone.
- The measurement narrative lives here (above), no longer in `clears`' doc.

## Review of PR 6734 (2026-11-10)

The follow-up's claim that the remaining normalized directions are a ray's own was false. The review named four more, each now guarded at its site or re-posed:
- **`face_piece`'s bisecting probe**, `normalize(unit(qa) + unit(qb))`, the shape 6703 retired. It rests on a premise, stated at the site and not asserted: `side` decided the pair under half a turn apart, past the band, and two unit directions `θ < π` apart sum to `2·cos(θ/2) ≥ sin θ`. Were it to fail, the probe would be NaN and its readings refuse in band (`PieceInBand`).
- **`Cone::reads_given`'s wedge bisector**, `normalize(a − p) + normalize(b − p)`. It rests on a premise, stated at the site and not asserted: a wedge is under half a turn (`Sector`'s constructors), and each ray's point stands at its reach off `p`. Were it to fail, the reference would be NaN and its clearance refuse in band.
- **`Link::lowest_on`'s `along`**, `normalize(e)`, read only by the `apart` closure and the twin roots. Both are two points of the segment's line apart, so both now read `Distance::apart`, and `along` is gone.
- **`LINK_BALL`**, `Distance::of(y, x + n·d, n)` with `n = normalize(y − x)`. That is `|y − x| − d`, a norm less a length, which is `Round::offset` of a convex sphere. It now reads so, through `Distance::at(&Round::sphere(x, d, true), y)`, with no new constructor. It reads `−d` at the centre, where the old spelling was NaN; where the scalar moves the centre (`AtTheAxis`) it reads nothing, and the refused candidate keeps its refusal.

**`Distance::clearance` of a segment of no length.** `to_segment`'s foot parameter divides by the segment's squared length. Coincident ends, or ends near enough that the square underflows, read `0/0`. At `Interval`, two straddling enclosures of one point give a squared length that holds zero without being it, and the divide reads an invalid enclosure. Either way the arc refused in band, although the clearance is plainly `|q − p|`. A reference along `q`'s own ray at `q`'s distance is `q` itself, or an enclosure of it computed apart, so the parity reading meets this arc whenever `q` stands on a reference's ray.

`clearance` now asks whether the squared length is nonzero for every value it stands for, a structure decision (`geom_core::structure_nonzero`, C6), not one against the band. Where it may be zero, the clearance is `p`'s distance from the hull of the ends (`either` per coordinate; the first end at a point scalar). That is sound for any segment, since the hull box holds it, and exact where the ends are one point. Pinned by `link::clearance_rows::a_segment_of_no_length_clears_by_its_points_distance`, which covers coincident ends and ends `1e-170` apart at `f64`, and coincident and straddling enclosures (half-widths `1e-15`, `2e-15`) at `Interval`. It is red under an exact-zero read, with the review's `InBand { margin: Invalid, "link_arc" }`.

**Pins of what moved** (`link::tie_rows`):
- On a segment of enclosures, `meets` joins a candidate on its foot. The old spelling dropped it.
- `touches`' tie reads `q` on a closed ray Zero, where the old spelling left it unread. Its verdict, near, is the same.
