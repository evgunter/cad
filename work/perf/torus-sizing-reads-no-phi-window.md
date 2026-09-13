---
id: torus-sizing-reads-no-phi-window
kind: issue
title: torus sizing reads no per-face phi window: the sharp per-window bound is left on the table (<= sqrt((R+r)/R) columns, plus ~6% from the joint max)
status: open
opened: 2026-09-10
---


## The residue

`mesh::sizing::torus_grid_steps` sizes every torus face and every
circle edge adjacent to one from the WHOLE-TUBE sups of the chord
bound: `A = R + r` (`sup ‖P_θθ‖` over all φ), `B = r`
(`sup ‖P_θφ‖`), `C = r`. A face is an iso-rectangle in (θ, φ)
(`geom_brep::props::require_iso_rectangle`), so its φ window is
known, and the bound is sharper on it in two ways the sizing does
not take:

1. **The window sups.** On a window W, `A_W = R + r·max_W cos φ` and
   `B_W = r·max_W |sin φ|`. A face avoiding the outer equator admits a
   coarser azimuth step by up to `√((R + r)/R)` in columns (1.11× on
   `hollowring`'s inner half-tubes; `√2` in the limit `R → r`).
2. **The joint maximum.** The two φ-dependent terms
   `(R + r cos φ)·h_u² + 2r|sin φ|·h_u·h_v` are never maximal at one
   φ; the sizing charges both sups at once. The sharp maximum over φ
   is `R·h_u² + r·h_u·√(h_u² + 4h_v²)`, worth ~6 % in cells on
   `hollowring`; it has no closed-form inversion for `(h_u, h_v)`.

Together this is one lever: size against the per-window sharp bound
rather than the whole-tube one. The certificate (`cert::cert_torus`)
already reads each triangle's own window, so the slack is visible in
the sweep row's readings (`tests/perf1_torus_sizing.rs`: worst
certificate 0.39–0.50 of δ against the δ/2 target).

## Why it is deferred

Boundary/grid coincidence through the chord pass. `chords.rs` sizes a
circle edge's chords BEFORE any face's walk has established a window,
from the adjacent surface alone (`sizing::torus_boundary_step`), and
the interior grid uses the same steps, so a rim edge's chord points
land exactly on the grid's columns. A window read on the grid side
alone would put the boundary rows off the grid's; a window read on
the edge side needs the face's φ extent at chord time, which is a walk
product.

## What a fix needs

- The face's φ window available at chord time: either the walk's
  window computed per face before the chord pass (the edge's two
  adjacent faces may have different windows — the edge takes the
  finer step, which is the face on the outer side), or the chord pass
  reading the face's rim circles' φ from their radii
  (`ρ = R + r cos φ`) plus the meridian arcs' parameter ranges.
- A sizing that is per face (both faces of a shared rim edge agree on
  `h_u` only when their windows give the same `A_W`; when they do not,
  the edge is chorded at the finer step and the coarser face's grid
  gets one row denser than its own window asked — still coincident,
  since the grid takes the boundary count as a floor per direction).
- For the joint max: a one-dimensional solve for the scale at the
  fixed aspect `h_u√A_W = h_v√C` (monotone in the scale; a few
  Newton steps), with the closed form as the starting point.
- The inverse-pair row (`sizing::tests::torus_grid_steps_and_cert_torus_are_an_inverse_pair`)
  and the sweep row's exact-count claim need re-stating per window.
