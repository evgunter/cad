---
id: a-cube-sunk-flush-into-a-block-refuses-ray-exhausted-on-every-order
kind: issue
title: A cutter fully sunk inside a block with flush walls refuses Boolean(Containment(RayExhausted)) on every union order (docm8 split-fixture variant)
status: closed
opened: 2026-09-28
priority: P3
cost: E
closed: 2026-10-26
---


An incidental observation from CONTACT-8's second fix pass, not
investigated. It is a variant of `docm8_flat_merged`'s split fixture
with `s` sunk fully inside `a` (z 0.5–0.75, walls flush). Every union
order refuses `Boolean(Containment(RayExhausted))`. That is a refusal,
not a wrong answer. Measure where the rays go and whether the flush
walls exhaust the ray schedule, and decide whether a flush containment
is decidable here.

## CONTACT close-out triage (2026-10-08)

Probably fixed by CLEAVE's PR 3655 (the shell-witness ladder), which landed after this row was filed. Run it once, and close it if it builds.

## Closed (2026-10-26, easy-fixes lane)

The witness builds. `split_fixture` with `s` sunk to `z ∈ [0.5, 0.75]` (`block_on(doc, a, (0.2, 0.4), (0.0, 1.0), 0.5, 0.25)`) fuses in all six union orders to the volume `1.5` of `a ∪ c`, with no `RayExhausted`. Pinned as `crates/editor-core/tests/wire_legal_union_refusals.rs` `a_split_fixture_with_its_block_sunk_flush_fuses_in_every_order`. Verified: `cargo nextest run -p editor-core --test all -E 'test(/a_split_fixture_with/)'`, 2 passed.
