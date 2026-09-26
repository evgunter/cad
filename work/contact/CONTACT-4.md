---
id: CONTACT-4
kind: unit
title: contfp reads every loop on its carriers: point_in_carrier_loop replaces the vertex-polygon walk and point_on_arc's pre-pass
status: dispatched
opened: 2026-09-26
priority: P0
cost: D
branch: contact/4-contfp-carriers
---


Carries `contfp-walks-the-vertex-polygon-of-an-arc-bearing-loop` and `point-on-arc-endpoint-zone-compresses-by-sin-half-width`. Spec: `docs/CONTACT-4-SPEC.md`.

Review tier: **single, full**. The carrier walk is well measured (ATREST-9). The risk is the swap under five live callers.
