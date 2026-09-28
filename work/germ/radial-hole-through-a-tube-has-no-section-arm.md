---
id: radial-hole-through-a-tube-has-no-section-arm
kind: issue
title: A cylinder perpendicular to a torus axis (a radial hole through the tube) refuses on reach at the section certificate
status: open
opened: 2026-09-28
priority: P2
cost: M
refs: [torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere]
---

## What

A cylinder whose axis is perpendicular to a torus's — a radial hole
through the tube, a common part feature — is an oblique pose to the
section certificate (PR 3372) and refuses on reach. The pose is
tractable: `sin(u − φ)` solves a quadratic in `v`, and the arc ends
are roots of a degree-8 polynomial (the spec's §2.4 and Q8). The
certificate's other torus arms show the shape a new arm takes:
components with essential flags and one closed-form witness each.
