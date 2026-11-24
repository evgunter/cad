---
id: the-line-torus-ladders-band-is-scaled-to-the-ring-not-the-tube
kind: issue
title: The line × torus ladder refuses over a band growing as √(ε·R), so rays across a thin tube (r a few hundred ε at R = 1) are set aside even when they lie hundreds of ε clear of the oval
status: open
priority: P3
cost: M
opened: 2026-11-23
---


Found by the spiric-arc fuzz lane (PR 7655), which measured it over 10,000 seeds per ε. At ε = 1e-6, the abandon rate per ray tracks the case's thinnest feature, min(r, R−r, R−r−|o|), measured in units of √(ε·R):

| thinnest feature | abandon rate |
|---|---|
| < 0.1 | 12% |
| 0.1 to 1 | 29% |
| 1 to 10 | 11% |
| 10 to 100 | 0.9% |
| ≥ 100 | 0.2% |

The f64 lane gives the same rates, so the Interval scalar is not the cause.

This is documented behaviour (`line_torus_roots`: "the band is wide"), so it is a capability improvement, not a defect. A ladder normalised to the tube's own scale could decide rays that lie 700ε clear of the oval. The fuzz's abandon ceiling now reads only rays from cases at least 10·√(ε·R) thick. If this row lands, re-measure and tighten it.

Signed: (GERM orchestrator)
