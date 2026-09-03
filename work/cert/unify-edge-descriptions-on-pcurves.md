---
id: unify-edge-descriptions-on-pcurves
kind: ruling
title: Design conversation — unify edge descriptions on curves-in-chart (pcurves), IsoCurve as the axis-parallel special case
status: closed
opened: 2026-08-12
closed: 2026-08-15
github: 427
refs: [388, 390, 391, 514]
---

## From GitHub issue 427

Opened 2026-08-12; 1 comment.

(M8 orchestrator) Filed from the #388/#391 thread's question (Ev: can we collapse our multiple representations — is MappedCurve a special case of IsoCurve?). **Ev's recorded lean: toward unification.**

## The two representations, and why neither nests in the other as shipped

- `EdgeGeometry::IsoCurve { surface, u, v_start… }` stores NO curve data — a surface key, a fixed `u`, a range. Geometry fully derived: the image of the axis-parallel UV line under the surface's chart. Can only express the u-fixed iso family of an adjacent surface.
- `EdgeGeometry::MappedCurve` stores its own defining data, no surface reference: sketch source + motion (`PlacedSegment` / `ExtrudedPoint` / `RevolvedPoint`), for loci the adjacent surfaces under-determine. Concrete non-iso example: a circular cap rim is a CIRCLE in the planar cap's UV (not an iso-line), and the v-const boundary on the wall's chart (the family IsoCurve cannot express).

Rewritten as "source curve ∘ map", the containment runs the OTHER way: IsoCurve = (axis-parallel UV line) ∘ chart; MappedCurve = (sketch entity) ∘ rigid motion. The common generalization is the classic **pcurve description: (surface, curve-in-UV)** — IsoCurve is its axis-parallel special case; every MappedCurve on a swept wall has an exact UV image in that wall's chart.

## The kernel is already converging on this without naming it

`Pcurve::IsoLine` existed; M8-3 added `Pcurve::IsoArc` (exact chart map, derived and review-verified); #391 built the certification tool a GENERAL pcurve needs — ring-composite certificates give whole-domain sup bounds on chart∘pcurve claims, no sampling, D9-clean. The pieces of "curve-on-surface, exactly certified" exist; they are distributed across special cases that each arrived when a unit hit its wall.

## What the collapse costs (the deliberate part of the current split)

The taxonomy encodes WHERE AUTHORITY LIVES, not just geometry. Built models mint MappedCurve because the sketch is the truth and surfaces derive from it — zero-redundancy bulge payload, "one authoritative source, never two peer representations", and program-anchored naming leans on that recorded sketch→3D connection. Imports mint IsoCurve because the chart is the truth. A pcurve-only representation makes sketch provenance derived rather than recorded.

## Proposed shape (for the conversation, not ratified)

Grow `Pcurve` to general curves-in-UV; `EdgeGeometry` references (surface, pcurve) for chart-anchored descriptions; **keep MappedCurve as the built-model AUTHORITY RECORD (provenance) rather than a geometry class**. Certification via the #391 composite machinery. This subsumes IsoCurve/IsoArc/Seam-as-meridian and retires the per-kind pcurve growth pattern.

## Interactions

- **#388 (Line promotion)**: option (a) there — promotion keeps the IsoCurve description — is the move CONSISTENT with this unification (keeps imported edges chart-anchored; no fake construction provenance). #388 need not block on this issue.
- **#390 (rational patch flux)**: its `compose::tensor` route is the surface twin of the same machinery — if both land, the compose family becomes the kernel's uniform certification substrate.
- **M9 scoping**: this is M9-shaped work (touches topo, geom-brep, step-import, sweep mints); propose it enters the M9 plan as its own design item with a ratification pass on DESIGN.md's edge-description text before any code.

Design-conversation class — awaiting Ev's ruling; not self-merging anything from this issue.

## Comments

**2026-08-15** — comment:

(M9 orchestrator) The M9-D ratification pass is open as PR #514 (docs/PCURVE-UNIFY-DESIGN.md) — proposed ruling U2: unify the DESCRIPTION to (surface, pcurve) while KEEPING the special certification lanes as exactness certificates (full subsumption argued rejected — it would weaken the sup arguments in kind); MappedCurve → authority record; OQ4 stays closed. Four questions for Ev on the PR.

## Home

S-CERT: the conversation's `compose::tensor` twin is the program's rational-flux ground (`crates/geom-brep/src/props/*`), and PCURVE — the program that executed the ruling — is closed. Closed on migration: the ruling this conversation asked for was answered as U2 and ratified into `docs/PCURVE-UNIFY-DESIGN.md`, whose execution is the closed PCURVE program's done-state of record (`docs/PCURVE-EXIT-WALK.md`).
