---
id: row-2-is-a-typed-standing-in-code
kind: issue
title: Row 2 is ratified as Refusal::Unbuilt behind a shared Standing trait (6702), and no code carries it
status: open
opened: 2026-11-16
priority: P2
cost: H
refs: [6702]
---

Found finishing `onecopy/section-error-standing`, when the orchestrator asked that unit's row-2 arms to move to `Refusal::Unbuilt`.

## What

PR 6702 (72496d084c) ratified `docs/DESIGN.md`'s `[failure.taxonomy]` text (`docs/DESIGN.md:2039`):
- Each refusal type a valid input can reach states every variant's class in one exhaustive method of the shared `Standing` trait.
- Row 2 is `Refusal::Unbuilt`.
- `Limit` is a row-1(b) limit no tolerance moves, ending on its own lever.

6702 changed only the design text and the fork log. In code, `geom_core::ladder::Refusal` is still `{Margin, InBand, Limit, Unsettled}` (`crates/geom-core/src/ladder.rs:154`), no `Standing` trait exists, and every refusal type's row-2 arm stands `Limit`.

## What closing it takes

1. **`Refusal::Unbuilt` in `geom_core::ladder`:** its ranking beside `Limit` in every ladder read, its ending (`Refusal::recourse`), and the frontier it makes listable.
2. **The `Standing` trait,** with each type's `refusal()` as its one exhaustive method.
3. **`Limit` narrowed to row 1(b)** at every site: each arm whose comment or doc says row 2 moves to `Unbuilt`. Among them, `geom_brep::SectionError::refusal`'s three row-2 arms (`onecopy/section-error-standing`):
   - `RoutesToGeneralRung`
   - `NoMarch`
   - `BeyondOperandExtent`

   `Carrier(EllipseInvalid::SemiAxisNotFinite)` stays `Limit`, since it is row 1(b).
4. **Measure** which ladder outcomes move where `Unbuilt` ranks apart from `Limit`.
