---
id: the-rotational-lane-reads-the-sphere-chart
kind: unit
title: The rotational-chart Green lane, stage 2: the sphere chart (projected chords, poles from the winding, rings on a trimmed sphere)
status: closed
opened: 2026-11-04
priority: P0
cost: H
branch: spiricarm/rotational-lane-sphere
refs: [a-projected-boundary-on-an-analytic-chart-has-no-volume-lane, a-rotational-chart-green-lane-reads-projected-chords, the-rotational-lane-omits-the-image-chains-junction-gaps-from-its-region-term]
closed: 2026-11-04
pr: 6509
---


Stage 2 of 3 of the chord-volume decide. Stage 1, `a-rotational-chart-green-lane-reads-projected-chords`, landed as PR 6478. Read its PR body and its dual-review row `docs/dual-review-log/rows/PR-6478.md`: the classes its reviewers found are likely to recur here.

This stage closes `a-projected-boundary-on-an-analytic-chart-has-no-volume-lane`, the P0 row: every cylinder × sphere boolean that builds refuses at the result gate, and teapot wall 2 stops there.

## What it builds

- **The sphere as the lane's second `RotationalChartQ` entry:**
  - `F = r³ sin v`, `G = r² sin v`;
  - on a projected chord, `sin v = z/|ξ|`, one `√`, inside C9;
  - on harmonic edges, the existing arithmetic.
- **Pole terms read off the winding.** `Φ = −Σ∮F du + 2πr³(1_N + 1_S)`, where `W` is the face's total azimuth winding and `1_N − 1_S = W`. For `W = 0`, the area range `(0, 4πr²]` decides whether the face holds both poles or neither, cross-checked against tier 3's pole side. This is premise (g): check it, don't assume it.
- **The routing.**
  - A trimmed sphere face goes to the lane rather than refusing "a trimmed face with no volume lane on its chart".
  - The ring gate (`reads_rings` in `topo::props::face_flux_about`) opens for a trimmed sphere.
  - Circle-only sphere faces keep Gauss–Bonnet (`props/curved.rs` `sphere_face`), unless folding them onto the lane is natural and costs nothing. If you fold them, say so.
- **The junction gap.** Fix `the-rotational-lane-omits-the-image-chains-junction-gaps-from-its-region-term` (P2) here. The region term omits a junction connector's `−F(v_j)·Δu`. That gap is pre-existing and shared with the harmonic lane, and the sphere's `F` makes it larger. Cover it for every chart the lane holds, with a row that goes red without the fix.

## Oracles (containment, never narrowness)

- **Carrier-swap fixtures on the sphere**: a meridian or tilted circle rewritten as an exact rational quadratic. Cases:
  - a face holding one pole, and a face holding both;
  - rings;
  - a chord loop that winds;
  - the body translated by 1e3.
- **The expected values are computed independently** of the function under test. Stage 1's swap rows were circular until the review caught it.
- **`cylinder_sphere_frame`'s 54 runs go SOUND against its `oracle`.** Its door pins retire, or move to whatever stops next.
- **Teapot wall 2 moves on to the handle's torus face** (`demos/tour/src/teapot.rs` `wall_probes`; `work/splice/curved-demo-joins-build.md` changes).
- **Inclusion–exclusion is NOT admissible**, because the chord bounds complementary faces with opposite traversal.

## Rules carried from the stage 1 reviews

- Every new pad or guard ships with a row that goes red when it is removed.
- A pin that names a chart kind reads the refused face's kind (`topo::boolean::refused_face_kind`).
- A "disclosed, unproven" gap becomes an issue file, not a PR-body line.

Review tier: **dual**. The lane's sphere chart and its pole rule are a soundness surface the torus stage builds on.

## Outcome

PR 6509. The sphere is the lane's second chart: its poles' terms come off the face's winding with its sense bit, which premise (g) found the rule needs, and every junction's connector is in the region term, which closes the junction-gap P2 on both charts. Cylinder × sphere results build at their volumes, but none is a legal operand and five differences refuse at the piece sort: point_in_solid cannot read a chorded sphere face (`work/inside/point-in-solid-refuses-a-sphere-face-bounded-by-a-spline-chord.md`, P0), which is where the frame's rows wait to go SOUND. The torus stage is stage 3's.
