---
id: survivor-folds-a-corrupt-fusion-list-onto-a-dead-key-outside-the-contact-remap
kind: issue
title: zip::survivor guards a corrupt fusion list with debug_assert! only, so fused_into, fused_through and Welds::kept fold onto a dead key where it compiles out
status: open
opened: 2026-10-02
---


## Finding

Found by FUSE's fix pass on PR 3874 (the descendant-chase unit). That
PR made the contact remap's vertex chase refuse a corrupt fusion list
in every build: `Descendants::live_vertex` (`crates/topo/src/boolean/ops.rs`)
now folds through `zip::survivor_checked`, which answers
`BooleanError::JoinDesync { what: "a fusion row names a key an earlier
row killed" }`. `zip::survivor` itself (`crates/topo/src/boolean/zip.rs`)
still guards the same condition (`fusions_well_ordered`) with
`debug_assert!` only, and three readers call it unchecked:

- `BooleanNaming::fused_into` (`ops.rs`, public) — the dead → survivor
  map the names lane and a discard's `bordered` ends read;
- `fused_through` (`ops.rs`) — the seam correspondence after each zip;
- `Welds::kept` (`crates/topo/src/boolean/finish.rs`) — a pinch weld's
  surviving vertex.

Rows `(a, b), (c, a)` fold `c` onto the dead `a`. With debug assertions
compiled out that is a dead key handed on as a survivor — not a drop,
a wrong key. Today `[profile.release]` sets `debug-assertions = true`
(`Cargo.toml`), and its comment says the stanza comes back out before
publishing, at which point these three read corrupt lists silently.
PR 3874 measured the contact-remap arm with
`CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false`: with the check reverted
to debug-only, a declared v-v contact dropped as `Ok`.

## What a taker owes

Route the three through `survivor_checked` (or validate each fusion
list once where it is built) and propagate the refusal. `fused_into` is
public and returns a map, so its signature is the decision. Not done in
PR 3874 because the unit's fence is the contact remap.
