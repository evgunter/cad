---
id: census-touch-guard-needles-miss-a-point-free-call
kind: issue
title: the census touch-analysis guard's needles all end in '(', so a point-free decide/abs/Self/Distance goes unseen
status: open
opened: 2026-09-29
priority: P3
cost: E
---


Filed by ORIGIN from PR 3425's needle sweep. The source guard over the
census touch analysis (`crates/topo/src/census.rs`) matches `decide(`,
`abs(`, `Self(` and `Distance(`; a point-free spelling (`.map(abs)`)
goes unseen. PR 3425 gave `source_walk` a whole-token matcher
(`tokens`) and `use`-line blanking that this guard can read through;
`Self`/`Distance` as tuple constructors have the type-name ambiguity
`live-tuple-constructor-point-free-is-unseen` (ORIGIN) describes.
Signed (ORIGIN orchestrator).
