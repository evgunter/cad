---
id: a-carried-sites-foot-is-certified-at-the-reading
kind: unit
title: A carried site's foot is decided at the reading: CarriedComponent::reads certifies the along-residual Zero, so the foot is the site's orthogonal foot, not a witness
status: closed
opened: 2026-11-22
priority: P1
cost: M
branch: sectcone/certified-foot
closed: 2026-11-23
pr: 7408
---

## Closed (2026-11-23, PR 7408)

`CarriedComponent::reads` certifies a site's foot. `foot_along` decides `|a|/(1 − κr)` Zero, where `a` is the along-residual, `r = |p − C(t)| + Kε`, and κ comes from `curvature_hi_over` over the arc's run. Before that, the bend `1/κ − r` is decided positive. A site past an open end reads at that end. Padded components take a short Newton polish first; closed forms keep their exact witness. Membership (`witness`, read by `holds`, `ended_by` and `distinct`) is split from place (`reads`), and `site` decides membership on every component before certifying the one foot. The measured pads are 1.5e7–1.3e8·ε, with κ·pad ≤ 0.62. Single FULL Opus review: APPROVE-WITH-FIXES, no MAJOR. Follow-up: `a-carried-sites-foot-escalates-where-its-polish-stalls-or-clamps-at-a-run-end` (P3).


Weighed 2026-11-22 by two designers, from PR 7354's fork (log, 2026-11-22). Both reached the same recommendation independently.

## What

The join orders a germ's partners by their feet on the carried section (`bool_join_foot_travel`). It assumes each `SiteFoot.t` is the site's place along the component, and `CarriedComponent::reads`' doc says exactly that ("the foot's parameter is the site's place along it"). But `reads` decides only `On`: the carrier passes within `ε + pad` of the site at `t`. `within_span`'s doc says the opposite: "`t` is the reading's witness … not its foot". Two descriptions of one value disagree.

- **Closed forms** (pad 0): a witness can stand up to ε along from the true foot, which the default K = 10 absorbs.
- **Fitted components:** the pad is at least the tube radius, which nothing bounds by the band. So a definite order can rest on witnesses alone once the pad is about 4ε or more. This exposes cylinder × sphere, the torus pairs and the spline pairs.

## Done when

- **The foot is decided:** `reads` decides the along-residual `t̂·(p − C(t))` Zero across the gap (curvature by `curvature_hi_over`), or the reading escalates. Every `SiteFoot` is then the site's orthogonal foot within the band's zero.
- **C5 says so:** one sentence in `crates/geom-brep/README.md` beside "a site's component is its foot on the carried section, its one reading". That clause was agent-written in `09c75d27f0` (`sect/components-by-foot`), and no ruling of Ev's turned up behind it, so this lands with the unit.
- **The pads are measured first:** measure `CarriedComponent::pad` on the cylinder × sphere and torus fixtures, and report it.

Rejected by both designers: an along-reach carried on `SiteFoot` and charged with `band.padded` everywhere a foot is read (one fact spelled at every consumer), and any `cot ψ`-style slack (the uncertainty does not exist on a foot).

It waited on the conic-pairs unit because both edit `along_sign` / `Along`; that merged in PR 7354.
