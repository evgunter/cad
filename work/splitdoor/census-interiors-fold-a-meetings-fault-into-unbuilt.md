---
id: census-interiors-fold-a-meetings-fault-into-unbuilt
kind: issue
title: census: InteriorsUnread::of files MeetFault::Unsettled as Unbuilt, a limit, where a coarser band may settle it (its fault fold is gone)
status: open
opened: 2026-10-29
priority: P3
cost: E
---

Found by the review of PR 5993 (`splitdoor/roots-unsettled-split`).

`crates/topo/src/census.rs:10944` (`arc_interiors_meet`, the refused
`meet` of two spans) reads `MeetFault::Escalated` and `MeetFault::Touch`
as `InteriorsUnread::Undecided` and every other fault as
`InteriorsUnread::Unbuilt`, the census's frontier ("the reading is not
built"). `Unsettled` and `Unsupported` are limits and belong there.
`Desync` (a carrier the meeting table misread) and `Contradiction` (two
readings of the pose that contradict, now including a root finder's
`RootFault`) are the kernel's defect, told as a reading not yet built.

The repair is a kernel-defect ending for those two (`InteriorsUnread` has
none today) and a row that drives a `Contradiction` through it; the
sibling `census-meet-faults-read-as-census-unsupported` is the same fold
at the census's cross pass.

## 2026-11-05: the ladder now ranks this fold (onecopy/ladder-climb-3, PR 6536)

Census's four interior-meeting searches (`shared_run`,
`planar_interiors_meet`, `shared_interior_witness`, `arc_interiors_meet`)
now read `geom_brep::ladder::climb`. They rank by `InteriorsUnread::refusal`:
`Undecided` in band, `Unbuilt` a limit, and a limit is ranked ahead of
in band. So this fold now chooses the refusal:
- every fault `InteriorsUnread::of` files as `Unbuilt` outranks an
  escalation kept beside it;
- `Desync` and `Contradiction`, the kernel defects above, are among them.

**A dispute of this row's premise that `Unsettled` is a limit.**
`MeetFault::Unsettled` (`crates/geom-brep/src/curve_pair.rs`, the
variant "a root finder, a meter or the walk's budget reached its limit
before it could certify") is a limit only if no tolerance moves it. If
a coarser band can settle the walk, it is the ladder's `Rung::Unsettled`,
ranked after in band. That is the misfiling PR 6495 found for
`chart_singularity`'s `Unread::Carrier`, which had been ranked at the
consumer.

So the repair has two parts:
- the kernel-defect ending this row names;
- `MeetFault` stating each variant's standing beside its definition (a
  `refusal()` returning `ladder::Refusal`), read by census instead of
  re-ranked in `InteriorsUnread::of`.

This section leaves the row's verdict as it stands. No row in the
topo + geom-brep `ci` set reaches a search that keeps both kinds.

## 2026-11-06: `MeetFault` states its standing (onecopy/refusal-standing)

`MeetFault::refusal` (`crates/geom-brep/src/curve_pair.rs`) now states
each variant's standing beside its definition, for `boolean::overlap`'s
witness search, which reads it through `BoundaryRefusal::refusal`:
- `Escalated` and `Touch` are ranked by their margin;
- `Unsettled` is unsettled, ranked after in band, as this row's dispute
  above argues;
- `Unsupported` is a limit;
- `Desync` and `Contradiction` abort, as kernel faults.

**Census does not read it yet.** Routing `InteriorsUnread::of` through
`MeetFault::refusal` would move census's outcomes:
- `Unsettled` would rank after an in-band escalation instead of ahead
  of it (as `Unbuilt`);
- `Desync` and `Contradiction` would stop the search instead of being
  kept as `Unbuilt`.

That is this row's decision to take, not a port's, so census is left as
it is. Closing this row means `InteriorsUnread` reading
`MeetFault::refusal` (with an abort kind, `Result<Refusal, Aborts>`)
rather than re-ranking each fault in `InteriorsUnread::of`.
## 2026-11-06: the fault half is done (onecopy/engine-leaves-pins)

`InteriorsUnread::of` now panics on `Desync` and `Contradiction` (`census.rs` `meet_fault`), by `[failure.taxonomy]` row 4. That replaces the kernel-defect ending this row asked for, which `[failure]` forbids for a state only a bug reaches. What is left is the second part above: `Unsettled`'s standing.

## 2026-11-09: a walk that could not run joins the fold (onecopy/carrier-standing, PR 6629)

`MeetFault` now carries a spline walk that could not run as its own arm, `Unwalked(geom::PieceWalkRefusal)`. That arm's standing is `geom::PieceWalkRefusal::refusal` (it was `ladder::Refusal::of_walk` before the ladder moved to `geom_core`): `Exhausted` is unsettled and `Unbounded` is a limit. Before, the walk was folded into `Unsettled(Exhausted)` or `Unsupported`.

`InteriorsUnread::of` files `Unwalked` as `Unbuilt`, beside `Unsettled` and `Unsupported`. Both of the old variants were already there, so no outcome moved. The new arm does sharpen this row's dispute. `MeetFault::refusal` reads an exhausted walk as unsettled, and this fold still ranks it as a limit. The repair, delegating to `MeetFault::refusal`, is the semantics change this row argues, so it was left here.
