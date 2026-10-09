---
id: CONTACT-11
kind: unit
title: the torus chart-box check compares areas, so an L-shaped torus face refuses rather than trim by its box
status: closed
opened: 2026-09-29
priority: P1
cost: M
branch: contact/11-torus-chart-l
closed: 2026-10-08
---


Carries `torus-chart-box-check-passes-an-l-shaped-face`.

Spec: `docs/CONTACT-11-SPEC.md`.

Review tier: **single full review.** A chart window that over-covers a
notch trims by the box, which is a wrong answer. The fix swaps a check
that decides whether a trim is licensed.

## Closed

The torus and cone chart trims decide "the face is its chart box" with
one linear, metric test, `solid_contain::chart_polygon_box`. Every side
of the chart polygon must lie on a box side. Each side's distance is
the larger of its two ends' distances, converted to metres through
`SupSpeed` arms that bound the true rate from above:
- torus: `R+r` per radian of the major angle, `r` per radian of the
  minor angle;
- cone: `max|slant|·sin α` per radian of azimuth, and slant exactly.

The torus walk also checks continuity between consecutive images
(`torus_chart_meets`), so a gap anywhere refuses.

At base, the torus check (total variation) passed an L-shaped face, and
a point in the notch read `In`. The first fix pass copied the cone's
shoelace-area check. Its margin is quadratic in a notch's size, so
small notches read Zero. The cone check on main had the same flaw. The
linear test replaces both, and `chord_join::chart_box_defect` is
deleted.

Review: a single full review asked for changes, with three MAJORs
(the quadratic margin, small U notches regressing, and the cone on
main). A delta review then approved with fixes after an under-bound
hunt that found no wrong `In` at any ring, frustum or ε. The short fix
pass pinned the arms' direction with a smallest-notch row, which the
both-arms-halved mutant reddens. The orchestrator read every pass.

Filed:
- `work/exch/step-import-l-shaped-curved-face-has-no-containment-row`
  (the STEP door);
- `torus-walk-steps-over-null-edges-where-the-cone-refuses`;
- `notched-half-donut-owes-its-notch-and-volume-when-torus-plane-lands`.

The sphere row now names the linear side test as its closing shape.
