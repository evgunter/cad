---
id: coaxiality-of-two-revolution-carriers-is-tested-five-ways
kind: issue
title: Coaxiality of two revolution carriers is tested five ways, and glue's coaxial rows skip every pair but cylinder-sphere silently
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`), which tests every geometric pair question against C5's rule: one total kind-pair table, one method per pair, no runtime fallback (`crates/geom-brep/README.md`, C5).

## What

Coaxiality of two revolution carriers is decided separately in:
- `carrier_eq`'s `cylinder_data`/`torus_data` (`crates/topo/src/boolean/carrier_eq.rs`);
- intersect's `cc_coaxial` and the cone×cylinder coaxial test (`crates/geom-brep/src/intersect.rs`);
- `glue::coaxial_rows` (`crates/topo/src/boolean/glue.rs:205`);
- `blend/battery::support_coaxiality` (`crates/sweep/src/blend/battery.rs:1146`);
- `offset_axial`'s frame (`crates/topo/src/offset_axial.rs`).

`glue::coaxial_rows` matches only cylinder×sphere and has `_ => continue` (`glue.rs`, ~l.230-238). It also drops an `Err` from `cylinder_sphere_section` with no row and no refusal (`if let Ok(..)`, ~l.240).

## Owed

One coaxiality reading, one arm per revolution-kind pair, with each consumer reading it. `coaxial_rows`' skip and dropped `Err` should each be either a table arm or a typed refusal.

## The dropped `Err`, probed (2026-10-14, `fuse/pair-gaps-fail-loud`)

`coaxial_rows`' `if let Ok(..)` drops two kinds of refusal from `cylinder_sphere_section`:
- `RoutesToGeneralRung`: the centre is definitely off the axis, so having no row is the right answer;
- every other refusal: `Escalated` (`cs_coaxial` or `cs_wall_reach` in band), `RadiusEscalated`, `DegenerateOperand`.

The second kind was probed with a three-arc peg (r = 0.5, z ∈ [0, 2]) and a revolved ball centred 4ε off the peg's axis at z = 1. The same scenes were also run at offset 0. Union and subtract both ran:

| Scene | 4ε off axis | On axis |
|---|---|---|
| Ball crossing the wall (R = 0.75) | refused downstream: a corner's sector side in band | refused: the face pairing is unsupported |
| Ball inscribed in the wall (R = 0.5) | refused downstream: edge-on-curved-face in band | refused: an edge crossing a curved face |
| Ball inside the peg (R = 0.3) | `Ok`, correct: the faces never meet, so coaxiality is moot | `Ok` |

So no scene found reads a wrong answer through the dropped refusal. Where the faces meet, a later decision refuses typed. Where they do not, the question does not bear on the result.

Making the drop fail loud would turn the inside-ball `Ok` into a refusal. That is an over-refusal of a correct answer, so the code was left as it is.

What remains of this half of the row is evidence, not proof: no scene showed the "caught downstream" path failing. A scene that reached a wrong result through the drop would reopen it as a defect.
