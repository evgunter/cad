---
id: boolean-declarations-has-no-geometric-producer
kind: issue
title: API gap — BooleanDeclarations has no geometric producer, so every direct kernel caller hand-writes one
status: open
opened: 2026-08-20
github: 757
refs: [S79]
---

## From GitHub issue 757

Opened 2026-08-20; 0 comments.

Found by a style-lane scan of `demos/` (never covered by the original SMELL-SCAN; `docs/SMELL-SCAN-2026-08.md` §B lists `demos/` as out of scope). Filed per **Protocol v5 / A1**: a disclosed gap owes a concretely scheduled followup rather than a comment.

## The gap

`topo::BooleanDeclarations` (`crates/topo/src/boolean/mod.rs:320`) is exported through the public prelude (`crates/pncad/src/prelude.rs:112`), and `boolean_op_with` takes one. There is a producer — but it is the wrong shape for a direct kernel caller:

- **`editor_core::eval::wire::resolve_declarations`** (`crates/editor-core/src/eval/wire.rs:1100`) maps **authored name pairs + two name tables** → `BooleanDeclarations`. That is the recipe layer resolving a *user's declared intent*.
- **Nothing maps two bodies' geometry → `BooleanDeclarations`.** The only other constructor is `BooleanDeclarations::none()`.

So a caller holding two bodies that were *built* flush, and wanting the coincident faces treated as REST, has no door. `BooleanDeclarations` is reachable from the prelude with no way to fill it.

## What that costs, twice, in the tree

The same ~55-line algorithm exists in two places, each declaring the twinning in prose rather than sharing code:

- `demos/tour/src/booleans.rs:67-122`
- `crates/topo/tests/common/mod.rs:446-498` — the demo's copy names this as *"the topo test-common declarer's twin"*

Both iterate `body.faces()`, match `Surface::Plane`, cross and dot the normals, compute plane offsets, and run three `k_stats::decide_flagged` / `decide` calls against a raw `Band::linear()`. They differ only in the flag strings and `T::one()` vs `S::from_f64(1.0)`.

**The part worth weighing:** authoring a shape currently requires reaching into `geom_core::k_stats`, a telemetry-flagged decision door. A demo — which per `memories/demo-purpose.md` demonstrates *real natural usage* — should not have to make K-telemetry decisions to call a boolean.

## Why this is a library finding and not a demo finding

`memories/demo-purpose.md` (ratified): *demos demonstrate REAL natural usage, and awkwardness is a library finding to record, never to hide.* The demo fighting the API here is evidence about the API.

## Not asserted

Whether the right shape is a kernel-side `declare_flush(a, b, band)`, a `BooleanDeclarations::from_coincident_planes(..)` constructor, or something else is a design question, not settled here. The finding is that the type has a public consumer and no public producer for the geometric case.

Related: `docs/SMELL-SCAN-2026-08.md` (the scan this follows up), and its S18/C11 duplication class — both copies are self-declared in prose, which is the pattern C11 says nothing ever reads.

## Home

S-BOOL: `BooleanDeclarations` lives at `crates/topo/src/boolean/mod.rs`, inside the program's `crates/topo/src/boolean/*` territory, and the missing door is a declared-contact operand gate — its charter. The code-quality register carries the same finding as `S79`, parked on this issue, and explicitly says no track row should be minted for it because it is kernel API work.
