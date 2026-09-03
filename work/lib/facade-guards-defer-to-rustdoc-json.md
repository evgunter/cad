---
id: facade-guards-defer-to-rustdoc-json
kind: issue
title: Three facade guards defer to a rustdoc-JSON check that is not scheduled
status: open
opened: 2026-08-20
github: 696
refs: [689]
---

## From GitHub issue 696

Opened 2026-08-20; 0 comments.

Three guards in `crates/pncad/tests/all.rs` are each a source-text fallback for the same unbuilt mechanism, and each says so in its own docs without anyone owning the follow-up:

- `no_arena_key_is_nameable_through_the_facade_document_surface` — "The intended enforcement was a rustdoc-JSON scan of `pncad`'s public API. This toolchain is stable-only and `--output-format json` is nightly-gated… So this is the FALLBACK."
- `no_raw_loop_minting_door_is_nameable_through_the_facade` — same posture, one file wider.
- `every_document_layer_root_export_is_carried_or_listed` — "A rustdoc-JSON check would close [the root-only blind spot] and is nightly-gated."

Each is honest on its own; three deep with no issue number is a deferral nobody is scheduled to discharge, which is the shape the SMELL-SCAN postmortems keep finding.

**What a rustdoc-JSON pass would buy, concretely:**

1. **Aliases and re-spellings.** The LB13 guard's own stated weakness: a key type re-exported under an alias, or reachable as an associated type or a public field of an allowed type, is invisible to a `pub use` text scan.
2. **Below-root reachability.** The completeness guard reads `editor-core`'s *root*. A public name reachable only by module path and never lifted to the root is invisible to it — the exact structural hole the original closure audit's second pass found (`topo::boolean::ContainError`, `geom_curves::EllipseInvalid`).
3. **Direct `pub` items.** The completeness guard reads `pub use` statements only; a `pub struct` written directly in `editor-core/src/lib.rs` would not be seen. That file currently has zero direct `pub` items, so the blind spot is held shut by a coincidence rather than by a rule.

**What it costs:** `--output-format json` is nightly-gated, so this is a CI change — a second toolchain in the workflow, and a decision about whether the JSON format's instability is acceptable for a gate. That is why three units in a row declined it, and it is a fair decline; what is missing is a place for the decision to live.

**Disposition wanted:** either schedule the CI work, or rule that the three text scans are the permanent answer so their docs can stop pointing at a mechanism nobody intends to build.

Filed from the S20/S21 fix pass (PR #689).

## Home

LIB: all three guards live in `crates/pncad/tests/all.rs` and police the `pncad` facade's public surface — the program's territory and the subject of its curation charter.
