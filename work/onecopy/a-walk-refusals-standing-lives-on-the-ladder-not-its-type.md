---
id: a-walk-refusals-standing-lives-on-the-ladder-not-its-type
kind: issue
title: A spline walk refusal's standing was Refusal::of_walk on the ladder; containment's fold of it remains
status: closed
opened: 2026-11-09
closed: 2026-11-10
---

`geom_brep::ladder::Refusal::of_walk` is the one table of a spline walk refusal's standing: `Exhausted` is unsettled, `Unbounded` is a limit. It lives on the ladder only because `geom` cannot see `ladder::Refusal` (its own doc says so). Once the ladder lives in `geom_core`, the table becomes `PieceWalkRefusal::refusal()` in `geom`, beside its `sentence()` and `recourse()`, and each caller reads `why.refusal()`.

**Wrappers that carry a `geom::PieceWalkRefusal`** (sweep at `onecopy/carrier-standing`, PR 6629):
- Delegating to `of_walk`:
  - `geom_brep::curve_pair::MeetFault::Unwalked`;
  - `geom_brep::curve_face::BoundaryRefusal::Unread`, through its `MeetFault`;
  - `topo` `validate::EdgeTangentUnread::Unwalked`, and through it `link::TouchUnread::EdgeTangent`.
- Carrying it with no standing of their own; their docs now point at the walk's:
  - `geom::CarrierRefusal::Unwalked`;
  - `geom_brep::SiteRefusal::Unwalked` (`pair_section.rs`);
  - `topo` `chord_join::SplitJoinError::SectionUnwalked`, which `boolean/join.rs` also raises.
- Carrying it with no standing at all:
  - `geom_brep::CarryError::Unwalked` (`pair_section.rs`);
  - `topo` `boolean::SectionUnbuilt::Unwalked` (`boolean/mod.rs`).
- **Folding both walk refusals into one variant:** `topo` `splitting/containment.rs` `ConicArc::hit`, `CarrierRefusal::Unwalked(_) => ConicHit::Unsettled`. `boolean/region.rs` then reads `ConicHit::Unsettled` as `RegionRefusal::RayExhausted`. So a carrier with no speed bound would be told as an exhausted ray. A conic's speed bound is its radius, so `Unbounded` does not reach it today. Splitting it needs a `RegionRefusal` arm for a missing bound.

**Sweep and its blind spot.** Pattern: `Unwalked\(|SectionUnwalked|PieceWalkRefusal` across `crates/`. Two kinds of site are invisible to a pattern on the walk's arms:
- a wildcard `Unwalked(_)` fold, which hides that two standings were merged;
- a standing stated only in prose ("a limit, not a fault"), which the type never checks.

Both were found by reading each hit, not by the pattern.

Open this once the `geom_core::ladder` move has landed. The doc-only carriers then point at `PieceWalkRefusal::refusal()`.

## Narrowed (2026-11-09, QML T3, `onecopy/refusal-standing`)

QML T3 moves the ladder to `geom_core::ladder`, so the table is `geom::PieceWalkRefusal::refusal()`, beside its `sentence()` and `recourse()`. `Refusal::of_walk` is gone, and its two callers read `why.refusal()`:
- `geom_brep::curve_pair::MeetUnread::refusal`'s `MeetFault::Unwalked` arm (and through it `curve_face::BoundaryRefusal::Unread`);
- `topo`'s `validate::EdgeTangentUnread::Unwalked`, which `link::TouchUnread::EdgeTangent` reads through `EdgeTangentUnread::refusal`.

The four carriers with no standing of their own point at `PieceWalkRefusal::refusal` in their docs:
- `geom::CarrierRefusal::Unwalked`;
- `geom_brep::SiteRefusal::Unwalked` (`pair_section.rs`);
- `geom_brep::CarryRefusal::Unwalked` (`pair_section.rs`), carried as `CarryError::Refused(..)` and as `topo`'s `boolean::SectionUnbuilt::Carry(..)`;
- `topo`'s `chord_join::SplitJoinError::SectionUnwalked`.

The sweep above named `CarryError::Unwalked` and `SectionUnbuilt::Unwalked`. Neither exists: both are the one `CarryRefusal::Unwalked`, carried.

**Left on this row:** the fold in `splitting/containment.rs` (`ConicArc::hit`, `CarrierRefusal::Unwalked(_) => ConicHit::Unsettled`, read by `boolean/region.rs` as `RegionRefusal::RayExhausted`). Splitting it needs a `RegionRefusal` arm for a missing bound, and the fold is unreached today, since a conic's speed bound is its radius.

## Closed (2026-11-10, `onecopy/walk-standing`)

There is nothing left for this row's code to do. On main at `origin/main` (2026-11-10):
- **The table** is `geom::PieceWalkRefusal::refusal()` (`crates/geom/src/curves/on_carrier.rs`), beside `sentence()` and `recourse()`. `Refusal::of_walk` is gone (`grep -rn of_walk crates/` is empty).
- **Every reader of a walk refusal's standing** reads `why.refusal()`: `curve_pair::MeetUnread::refusal`, `validate::EdgeTangentUnread::refusal`, and through it `link::TouchUnread::EdgeTangent`.
- **The carriers with no standing method** (`geom::CarrierRefusal`, `geom_brep::SiteRefusal`, `geom_brep::CarryRefusal`, `chord_join::SplitJoinError`) carry the walk refusal whole, and their `Unwalked` docs point at `PieceWalkRefusal::refusal`.
  - "`CarryError::Unwalked`" and "`SectionUnbuilt::Unwalked`" are the one `CarryRefusal::Unwalked`, carried as `CarryError::Refused(..)` and `SectionUnbuilt::Carry(..)`.
  - No ladder reads a carry refusal's standing: `section_cert.rs` folds every `CarryError` into `Section::Unmarched`, and PR 6726's trace of `BooleanError::refusal`'s readers finds `CarryRefused` read after the reduction only.
  - So a `CarryRefusal::refusal()` would have no reader. It would also need a ruled standing for each of its seven other arms (`Unminted`, `NoLane`, `March`, `Winds`, `PadShort`, `TubeUnread`, `Seat`), which is a question for whoever first reads one on a ladder, not this row.
  - `SplitJoinError`'s `SectionUnwalked` arm gets its standing by delegation in PR 6726's `SplitJoinError::refusal()`.
- **The fold** (`splitting/containment.rs` `ConicArc::hit`, `CarrierRefusal::Unwalked(_) => ConicHit::Unsettled`) is the ladder lane's, under ruling (b): `ConicHit::Uncertified`, a typed fault, in its any-pins PR. It is not touched here.

No code changed, so no outcome moves.
