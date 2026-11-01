---
id: radial-hole-through-a-tube-has-no-section-arm
kind: issue
title: A cylinder perpendicular to a torus axis (a radial hole through the tube) refuses on reach at the section certificate
status: closed
opened: 2026-09-28
closed: 2026-10-10
priority: P2
cost: M
refs: [torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere]
branch: germ/radial-hole-through-a-tube
pr: 4530
---

## What

A cylinder whose axis is perpendicular to a torus's — a radial hole
through the tube, a common part feature — is an oblique pose to the
section certificate (PR 3372) and refuses on reach. The pose is
tractable: `sin(u − φ)` solves a quadratic in `v`, and the arc ends
are roots of a degree-8 polynomial (the spec's §2.4 and Q8). The
certificate's other torus arms show the shape a new arm takes:
components with essential flags and one closed-form witness each.

**2026-10-10, closed (PR 4530, DR-147, HOLDOUT pair).** Torus × a cylinder whose axis is square to the torus axis answers by the meridian-plane double cover: every fold and strip crossing is a circle meeting a circle or a line, so nothing isolates a root. A fold too shallow to place within the band refuses R-tan `_fold_drift`. A fold or strip crossing at a tube's top refuses R-order (`Section::Unordered`), and the argument for answering it is filed as `square-wall-crossing-at-a-tubes-top-refuses-r-order` (P3).
