---
id: the-fused-verbs-take-end-and-radius-at-every-directed-tip
kind: unit
title: arc_fillet and arc_fillet_arc take End and Radius at every Directed tip, authored or inherited
status: parked
opened: 2026-11-22
priority: P1
cost: M
parent: tangent-arc-to-needs-an-explicit-tangent-where-line-takes-the-directed-point
blocked_on: [arc-to-gains-the-end-mode]
---


Ruled on PR 7251 (Ev, 2026-11-21). The text this implements is
`docs/PATHS-DESIGN.md` as PR 7251 leaves it (read at `7c8acfd225`):

- §2 Fillet, "Incoming": `arc_fillet(End { p }, r)` "is the fused form
  of the arc that leaves along the bound angle toward `p`: `p` is the
  side's on-path anchor, and the arc runs on past it, on the carrier
  `End` derives, to the fillet that trims it". `Radius{r, side}` is
  admitted "from any DIRECTED tip, heading-taking like the rest of that
  set"; under `.tangent()` it is arc extension.
- §2c: the heading-taking modes (`Sweep`, `ArcLen`, `End`, `Radius` at
  the fused incoming) are "admissible at every Directed tip alike,
  authored or inherited, and at no Point".
- §3's `arc_fillet(spec, r)` row.
- §2a item 6: an `arc_onto` arc filleted at its own end has no
  spelling, while its mirror `End` has `arc_fillet(End { p }, r)`.

## What is true when it is done

- **`FUSED_DIRECTED` admits `End @ Point` and `Radius`**
  (`crates/profile/src/path/program.rs`), for `ArcFillet` and
  `ArcFilletArc` alike. `End @ Start` and `End @ StartArriving` refuse
  there, as every endpoint-full mode's closing target does at
  `FUSED_POINT`. The typed `arc_fillet` / `arc_fillet_arc` on a Directed
  tip take `End` and `Radius` (`TangentIncoming` in `path/family.rs`,
  or a sibling trait if `End`'s anchor does not fit it).
- **Fused `End`.**
  - The carrier is `End`'s tangent-chord arc from the tip along the
    bound angle through `p`.
  - The incoming side is anchored at `p`, an
    `FusedIncoming::Anchored`-shaped side, and the fillet trims past
    `p`.
  - A trim that eats `p` refuses
    `CornerReason::AnchorOutsideTrimmedExtent`.
  - The departure is the bound direction, already junction-checked
    when the director bound, so no second junction check runs at the
    tip.
  - Rows: a rounded corner after `.angle(θ)` and one after
    `.tangent()`. The fused side's carrier is bit-identical to
    `arc_to(End{p})`'s from the same tip, and a test asserts it.
- **`Radius` at a Directed tip.**
  - The centre is derived from the tip's position and BOUND direction
    (`verbs::radius_carrier`, today fed a leg end's `DirectedPoint`).
  - After `.tangent()` the result is bit-identical to today's bare
    leg-end `arc_fillet(Radius{..}, r)` (`FUSED_LEG_END`), and a
    differential test asserts it.
  - After `.angle(θ)` it is the circle tangent to the θ ray at the tip,
    run forward to the trim, with a row of its own.
  - The bare leg-end `Radius` row stays; the retiring unit deletes it.
- **Python.** `PathDirected.arc_fillet` / `arc_fillet_arc` accept `End`
  and `Radius` (`pncad.pyi`'s `_Tangent` alias, `py/path.rs`), and the
  census binds them.
- **editor-core** needs no change for `Radius`: `ProgramArcData` already
  has it. The lattice admits both through `Verb::states`.

## Left for later units

- Deleting the bare leg-end `Radius` rows of `FUSED_LEG_END`, and
  Python's `_LegEndIncoming` `Radius`:
  `continue-to-and-the-bare-leg-end-rows-retire`.

## Check first

- Whether fused `End`'s anchor fits `FusedIncoming::Anchored(PointCarrier)`
  (`family.rs`, today built by `PointIncoming::carrier` from a Point
  tip). That form carries "carrier + derived start tangent + authored
  anchor (junction-checked at the tip)", and here the start tangent is
  BOUND, not derived. If the fused pipeline junction-checks an
  `Anchored` side's start tangent unconditionally, a fused `End` would
  check the director twice. Find out which, and say in the PR how the
  second check is avoided.
- If the fit runs one of validation's span reads on the extended
  carrier past `p` that no other fused mode reaches, stop and report:
  that is new validation ground (PROFILE's pair-pass rows).

## Rows

- **Closes `no-fused-verb-takes-a-departure-tangent-arc-toward-a-point`**
  (moved here with this cut). Close it in this unit's PR.
- Carries part of the parent.

## Ground crossed

- `crates/profile/src/fillet_select.rs` and `validate.rs`, if touched,
  are shared with PROFILE.
- Python is PYDOOR's and BIND's: announce there.
- No schema change.

## Review

Tier **single (FULL)**: two new fused acts with an anchor and trim
semantics. Geometry, but closed-form and on existing machinery.
Difficulty **M**.
