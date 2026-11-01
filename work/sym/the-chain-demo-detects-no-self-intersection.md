---
id: the-chain-demo-detects-no-self-intersection
kind: issue
title: the chain demo detects no self-intersection: adjacent links interpenetrate at every non-zero draw and nothing refuses — the study needs the clearance engine or a boolean before it can say so
status: open
opened: 2026-09-22
priority: P1
cost: H
refs: [SYM-14, symbolic-tier-and-clearance-engine]
---


## What

**Specifically requested by Ev, 2026-09-22 (P1), on #3073:** "the ultimate
goal of this feature is among other things to detect self-intersections,
so it should get filed as a follow-up (potentially a blocked one, if the
features to do this don't exist yet)".

SYM-14's chain (`demos/tour/src/chain.rs`) is four bars and five pins,
every one its own body, no boolean anywhere (the plate's convention: the
study is about where the pins ARE). Adjacent bars butt end to end at a
joint, so at any non-zero joint angle the two rectangles overlap in a
thin wedge on the inside of the bend (about half the bar height times
`tan θ`, ~40 µm at 3σ = 1.7°) and gap on the outside. Two separate
solids in the same place is just two solids: **nothing in the study
detects it, refuses on it, or counts it.** The sheet's 68/512 violations
are the tip pin's 1 mm true-position tolerance; the certifiable box's
wall (0.111 of the study) is the pin cylinder's transversality margin
going poisoned — neither has anything to do with the bars touching.

## What would say it

Two doors exist in the tree, neither reachable by this document today:

1. **The E7 clearance engine** (`crates/editor-core/src/clearance.rs`,
   CLEAR's): a `min_clearance` measure between adjacent links, asserted
   non-negative (or above a clearance), would make interpenetration a
   VIOLATION the advisory lane counts and the certified lane refuses or
   certifies. On the certified lane it is BLOCKED:
   `work/clear/symbolic-tier-and-clearance-engine` — the engine is
   written at `geom_core::Interval` concretely, `MinClearanceLane for
   Sym<T>` answers `None`, and `drive` refuses a document carrying a
   `min_clearance` measure up front (`SymbolicClearanceUnsupported`)
   when the tier is on, so the chain would lose the very lane that
   draws its certified enclosure. On the advisory lane (`f64` replay)
   whether the measure answers at all is unmeasured — it is the first
   thing this row's unit measures.
2. **A boolean union of the links**: interpenetration becomes the
   boolean's business — a union that succeeds says nothing, one that
   refuses says something, and neither is a clearance. The chain is
   planar extrudes, so the prismatic boolean reaches it; but the study
   would then be about the union's refusal class rather than about a
   measured clearance, and the certified lane's reach over a union of
   four bodies is unmeasured.

The alternative that needs no new door — build the links with clearance
so they cannot interfere at ±3σ by construction (rounded ends around
the pin, as a real chain link has) — answers Ev's question for THIS
chain and detects nothing; it is the document's fix, not the feature's,
and is not what this row asks for.

## Home

SYM, with SYM-14 (the demo is here); the certified half waits on CLEAR
(`symbolic-tier-and-clearance-engine`, P1 on CLEAR's slate) and this
row is parked there the day its advisory phase reports blocked too.
A unit from this row: Phase 1 measures the advisory lane's
`min_clearance` over adjacent links per replay (does the `f64` replay
answer it; what the 512 draws say about the interference wedge) and the
union door's refusal class; Phase 2 is the certified detection, which
is CLEAR's composition row first.
