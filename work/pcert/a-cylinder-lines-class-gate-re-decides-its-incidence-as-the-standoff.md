---
id: a-cylinder-lines-class-gate-re-decides-its-incidence-as-the-standoff
kind: issue
title: On a cylinder, a line's class gate reads incidence and then decides the same wall gap again as the norm_gap standoff
status: open
priority: P1
cost: M
opened: 2026-11-10
---


Filed by the PCERT main-reds lane (PR 6528, review fix EE10).

## What

`derive_class`'s cylinder arm (`crates/geom-brep/src/pcurve_cache.rs`)
reads the carrier's incidence first
(`incidence::incident(carrier, surface, place, band)`), which decides
how far the line stands off the wall. For a line it then decides the
class gate's margin as `ruling_miss(...) + standoff`, where
`standoff = norm_gap(radius, radius², w_r)` is the origin's radial
distance from the wall: the same wall gap, decided a second time. PR
6504 added that second read (its fix AB1), so that a Zero bounds the
meridian form's whole miss.

It is two readings of one fact (P1, the definition). On a rigidly
copied body the fact is a value identity the Sym rules do not reduce,
so it costs the D-tab 4 extra numeric decisions
(`pcurve_chart_radial_moving [22, 20] → [18, 24]`,
`work/sym/a-rigidly-copied-cylinder-walls-incidence-identity-has-no-sym-rule.md`).

## Fix

The class gate's margin reads the standoff from the incidence read's
own margin (`shown.margin()`), not from a re-decided `norm_gap`. A Zero
then still bounds the whole miss, because the incidence read already
decided the standoff. Keep the mutant `meridian-standoff-unread` red.
Re-pin the D-tab row (expected back at `[22, 20]`).
