---
id: CONTACT-11
kind: unit
title: the torus chart-box check compares areas, so an L-shaped torus face refuses rather than trim by its box
status: dispatched
opened: 2026-09-29
priority: P1
cost: M
branch: contact/11-torus-chart-l
---


Carries `torus-chart-box-check-passes-an-l-shaped-face`.

Spec: `docs/CONTACT-11-SPEC.md`.

Review tier: **single full review.** A chart window that over-covers a
notch trims by the box, which is a wrong answer. The fix swaps a check
that decides whether a trim is licensed.
