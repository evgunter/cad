---
id: stablename-key-is-quadratic-on-a-boolean-chain
kind: issue
title: StableName is a recursively boxed BTreeMap key, so naming a boolean chain is quadratic - 40 percent of die's rebuild
status: open
opened: 2026-09-10
parent: PERF-2
---

## The finding

Measured by the PERF kernel lane (`perf/explore-kernel`; release with
debug assertions off so the number is the algorithm's, 4 vCPU under the
build-slot mutex, medians of 5). On `die` (a 21-long subtract chain, 84
nodes) the boolean nodes are 35.2 of a 36 ms full rebuild, and
`wire.boolean.name_emitter` is **21.4 ms of that — 55–60 % of boolean
time, ~40 % of the whole rebuild**. The per-step trace grows 0.061 →
2.451 ms along the chain: quadratic in chain depth, measured rather than
argued (`work/perf/plan.md`'s finding 15).

The cause is concrete. `StableName` is a recursively boxed value
(`crates/editor-core/src/names/role.rs:396,398` `FromA(Box<StableName>)`
/ `FromB`, plus `FromMember` and ~20 further `Box<StableName>` fields
at `:454-599`) and it is the KEY of the naming table
(`crates/editor-core/src/names/table.rs:69,96-107`, a `BTreeMap`), so
every insert during emission (`names/emit_topo.rs:518-524`) is O(depth)
comparisons plus a deep clone of the name.

## What a fix is

"Do this faster" with the semantics untouched: `RoleSeg`'s nesting is
the readable statement of descent and stays the definition. Intern
names (a name is an id into an arena of segments; comparison and
cloning become O(1)), or key the table by a content hash of the name
beside the structural value, or flatten the descent chain into a
segment list with O(length) compare and no allocation per level. The
spec chooses; the pin is that every emitted name and every table
lookup answer is unchanged (D5 names are persisted, so the persisted
form must round-trip byte-for-byte). DOCM territory (`names/*`), edited
2026-09-09; DOCM has no unit dispatched. Announced in the log.
