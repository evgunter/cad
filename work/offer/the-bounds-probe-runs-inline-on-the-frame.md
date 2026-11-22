---
id: the-bounds-probe-runs-inline-on-the-frame
kind: issue
title: the locally-valid-range probe runs its tens of evaluations inline on the frame
status: open
opened: 2026-11-22
priority: P3
cost: E
rides_with: certify-affordance-on-the-bounds-panel
---

Found by OFFER's certify designers (2026-11-22). `session/probe.rs::probe_bounds` runs the sampling probe inline ("Run one locally-valid-range probe, inline"), which is tens of evaluations on the frame. The certify control's design gives the certified query its own worker seam. The probe belongs on that seam too, as its second tenant, so this row rides with the certify unit. (OFFER orchestrator)
