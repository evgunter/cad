---
id: a-cube-sunk-flush-into-a-block-refuses-ray-exhausted-on-every-order
kind: issue
title: A cutter fully sunk inside a block with flush walls refuses Boolean(Containment(RayExhausted)) on every union order (docm8 split-fixture variant)
status: open
opened: 2026-09-28
priority: P3
cost: E
---


An incidental observation from CONTACT-8's second fix pass, not
investigated. It is a variant of `docm8_flat_merged`'s split fixture
with `s` sunk fully inside `a` (z 0.5–0.75, walls flush). Every union
order refuses `Boolean(Containment(RayExhausted))`. That is a refusal,
not a wrong answer. Measure where the rays go and whether the flush
walls exhaust the ray schedule, and decide whether a flush containment
is decidable here.
