---
id: a-node-id-above-i64-max-cannot-become-metadata
kind: issue
title: meta::to_value refuses IntOutOfRange for any name whose node id is above i64::MAX, about half of minted ids, at any depth
status: review
opened: 2026-10-03
priority: P3
cost: M
branch: recipe/meta-u64-ids
pr: 4071
---


## The finding

Measured by PR 3909's last fix pass (a temporary probe on the 1 MiB stack; see DOCTAIL's `a-name-through-a-non-json-serializer-recurses-once-per-level`, "Partly closed by PR 3909"): a `StableName` through `meta::to_value` refuses `MetaError::IntOutOfRange` at every depth, one level included, when its node id is above `i64::MAX`, because `RecipeNodeId` serializes as a `u64` and `MetaValue` holds `i64` integers. A minted id is a 64-bit digest (`mint.rs`, `Mint::insert`), so about half of all names cannot become metadata, by accident of their id.

## What would close it

Either `MetaValue` represents every `u64` a kernel value serializes (an unsigned arm, or a 64-bit-exact integer), or ids cross into metadata in a spelling that is not an integer. Decide which by what readers of a metadata value need; a row: a name minted with an id above `i64::MAX` round-trips through `to_value`/`from_value`.

Filed by the RECIPE orchestrator from PR 3909's fix-pass report.

## Built (2026-10-05, PR 4071)

`MetaValue::Int` holds a `MetaInt`: every `i64` and every `u64`, no wider, persisted as the bare integer (unchanged bytes for every value that could be written before). A name minted above `i64::MAX` round-trips through `to_value`/`from_value` at one level and as deep as any name (`tests/meta_minted_ids.rs`), and a document carrying it in metadata saves, loads and pins the same. The same arm carries `StepId` and `VarId`. Nothing remains on this row.

Fix pass (2026-10-05, PR 4071): `MetaInt` hands a non-negative value to a visitor as a `u64`, as `serde_json` does, so a reader that takes only `u64` (`persist::wire::plane_ref`, a `ProfileProgram`'s `plane`) comes back through `from_value` at every id, not only above `i64::MAX` (`tests/meta_minted_ids.rs`, plane ids 5, `i64::MAX`, `i64::MAX + 1`, `u64::MAX`). An integer past both ends of the range in a saved file is refused for its range (`MetaError::IntOutOfRange`'s text) rather than as a float. No stored byte or pin moved.
