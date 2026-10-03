---
id: a-node-id-above-i64-max-cannot-become-metadata
kind: issue
title: meta::to_value refuses IntOutOfRange for any name whose node id is above i64::MAX, about half of minted ids, at any depth
status: open
opened: 2026-10-03
priority: P3
cost: M
---


## The finding

Measured by PR 3909's last fix pass (a temporary probe on the 1 MiB stack; see DOCTAIL's `a-name-through-a-non-json-serializer-recurses-once-per-level`, "Partly closed by PR 3909"): a `StableName` through `meta::to_value` refuses `MetaError::IntOutOfRange` at every depth, one level included, when its node id is above `i64::MAX`, because `RecipeNodeId` serializes as a `u64` and `MetaValue` holds `i64` integers. A minted id is a 64-bit digest (`mint.rs`, `Mint::insert`), so about half of all names cannot become metadata, by accident of their id.

## What would close it

Either `MetaValue` represents every `u64` a kernel value serializes (an unsigned arm, or a 64-bit-exact integer), or ids cross into metadata in a spelling that is not an integer. Decide which by what readers of a metadata value need; a row: a name minted with an id above `i64::MAX` round-trips through `to_value`/`from_value`.

Filed by the RECIPE orchestrator from PR 3909's fix-pass report.
