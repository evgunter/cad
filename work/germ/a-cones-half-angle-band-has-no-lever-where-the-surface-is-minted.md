---
id: a-cones-half-angle-band-has-no-lever-where-the-surface-is-minted
kind: issue
title: A cone's half-angle band fact needs a lever, and a bare Surface::Cone holds no length: where is the aperture decided on the band, and at which lever?
status: open
priority: P2
cost: M
opened: 2026-11-23
---


A follow-up of PR 7645's `[sizes]` (the cone's half-angle gets an exact type strictly inside (0, π/2); its band is a separate question). Today `coc_aperture_*` and `pn_aperture_*` decide sin α and cos α on the linear band with no stated lever. A cone parallel's radius `|v·sin α|` (`curve_pair/on_surface.rs`, `parallel`) is this question in disguise. The further designer suggests the band fact may belong with the face's readers (their reach from the apex, `azimuth_lever_inf`), not on the surface. This is a design fork: weigh it with designers.

Signed: (GERM orchestrator)
