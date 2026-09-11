---
id: naming-a-boolean-chain-is-theta-names-per-step
kind: issue
title: Naming a boolean chain is Theta(names) per step - the depth term is gone, the constant is 0.7 us per named entity
status: open
opened: 2026-09-11
---


## What PERF-2 left

PERF-2 removed the depth term from `wire.boolean.name_emitter`: on
`die` the emitter fell 18.5 → 4.7 ms of a 17.6 ms boolean stage, and
the cost PER NAMED ENTITY went flat — 0.92 µs at step 1 (52 rows) to
0.73 µs at step 21 (552 rows), where it had been 1.21 → 4.59 µs. The
per-step trace still grows, and that growth is now honest: the die's
name table grows 52, 77, 102 … 552 rows (+25 per pip) because each
step's result body has one more pip's entities to name, so even a
depth-free emitter pays Θ(names emitted).

PERF-2's spec asked for the emitter under **10 %** of boolean time.
It is at **27 %**. The gap is not depth, it is the constant: closing
it means taking ~0.7 µs per named entity down to ~0.2 µs, which is a
different kind of work from the one PERF-2 did and is what this item
holds.

## The named leads, measured or cited

- **The double search at `crates/editor-core/src/names/defer.rs:74-77`.**
  `upstream_name` finds the name in `reverse` and then looks the SAME
  name up in `forward` only to learn whether the entry is `Tied` — a
  whole `BTreeMap` search per named entity, ~9 % of the emitter.
  Folding the tie flag into the reverse map's value removes it. What
  stopped PERF-2: that forward lookup is also the site of the
  "an operand name table's forward and reverse directions disagree"
  refusal, whose presence is a documented decision in that file
  ("the arm stays because the invariant is the emitter's to assert,
  not to assume"). Trading it for 9 % is a decision, not a speed-up.

- **`merged::constituents_through_wrappers`
  (`crates/editor-core/src/names/merged.rs:34`) is the last O(depth)
  walk in the boolean naming path.** It is called for every face at
  every boolean step and peels `FromA`/`FromB` wrappers to the foot,
  returning `None` when the foot is not a merged face — which on a
  chain is the common case, so the whole descent is walked to answer
  "no". It did not surface above the noise in the post-change
  callgrind, which is why PERF-2 left it; a deeper chain than the
  die's 21 would change that.

- **The set-valued role arguments are still `Vec<StableName>`, and
  they are this unit's sweep blind spot.** `RoleSeg::Merged`
  (`crates/editor-core/src/names/role.rs:664`) and `RoleSeg::BandFace`
  (`:777`) hold names BY VALUE where every other name-carrying variant
  now holds a `NameRef`, so building one deep-copies each constituent
  instead of sharing it — `crates/editor-core/src/names/emit_blend.rs:172`
  is the live site (`names.push((*e.name).clone())` per incident
  edge). The copy is O(path) rather than O(depth) now, so it is a
  constant and not a term; converting them is mechanical but touches
  the `Merged` sort, which is a PERSISTED canonical order, so it owes
  a differential of its own
  (`crates/editor-core/tests/perf2_name_keying_differential.rs` is the
  row to run).

## Where to measure

The kernel lane's stage spans on `die` (harness on `perf/explore-kernel`,
release with debug assertions off, under the build slot). The numbers
above are medians of 5; `die` full rebuild is 19.2 ms and
`wire.boolean.run_pair` 12.7 ms of it, so the emitter is no longer the
document's largest term and the next reading should say what is.
