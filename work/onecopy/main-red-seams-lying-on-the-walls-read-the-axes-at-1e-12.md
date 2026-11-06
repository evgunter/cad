---
id: main-red-seams-lying-on-the-walls-read-the-axes-at-1e-12
kind: issue
title: main red at 1e-12: sweep parallel_cylinder_join::seams_lying_on_the_walls_read_the_axes_across_the_region (U ab: OK BAD … v=8.858311527)
status: dispatched
opened: 2026-11-06
priority: P1
cost: E
branch: onecopy/seams-1e-12
---


Found by the PR 6491 takeover lane (`onecopy/meet-pose`). It is red at `CAD_TOLERANCE_EPS=1e-12` on main `264f1407`, with a panic byte-identical to the head's (`U ab: OK BAD … v=8.858311527`). No work row named it. It is in the slow set, so the per-PR gate never runs it.

## What closing it takes

1. Reproduce it on current main at 1e-12.
2. Bisect to the commit that moved it, or find that it was never green at 1e-12.
3. Read the panic: which pair, and which reading.
4. Fix the cause at its home, or re-pin it with the reason.
