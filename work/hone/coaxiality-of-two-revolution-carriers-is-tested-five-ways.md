---
id: coaxiality-of-two-revolution-carriers-is-tested-five-ways
kind: issue
title: Coaxiality of two revolution carriers is tested five ways, and glue's coaxial rows skip every pair but cylinder-sphere silently
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`, #5078). The rule it tests is D3's "One reading per question" (`docs/DESIGN.md`, D3): every decision about carriers is a reading (of one carrier against a point, of a carrier pair's pose, or of a pair outcome that takes the pose); each reading is one exhaustive match with one method per arm and no fallback, and every consumer reads it. C5 (`crates/geom-brep/README.md`) is that rule's instance for surface and curve pairs.

## What

Coaxiality of two revolution carriers is decided separately in:
- `carrier_eq`'s `cylinder_data`/`torus_data` (`crates/topo/src/boolean/carrier_eq.rs`);
- intersect's `cc_coaxial` and the cone×cylinder coaxial test (`crates/geom-brep/src/intersect.rs`);
- `glue::coaxial_rows` (`crates/topo/src/boolean/glue.rs:205`);
- `blend/battery::support_coaxiality` (`crates/sweep/src/blend/battery.rs:1146`);
- `offset_axial`'s frame (`crates/topo/src/offset_axial.rs`).

`glue::coaxial_rows` matches only cylinder×sphere and has `_ => continue` (`glue.rs`, ~l.230-238). It also drops an `Err` from `cylinder_sphere_section` with no row and no refusal (`if let Ok(..)`, ~l.240).

## Owed

Re-aimed to D3 "One reading per question". Coaxial is one class of the carrier pair's *pose*, decided once by the pose reading. The "one coaxiality reading" is that pose reading's coaxial class, with one method per revolution-kind pair, and the five tests become consumers of it.

- `glue::coaxial_rows`' `_ => continue` is a consumer matching on kinds. It matches on the pose reading's answer instead, and refuses typed the pairs it does not serve.
- Its dropped `Err` from `cylinder_sphere_section` (no row, no refusal) is a typed refusal read as a verdict, which D4's line "A typed refusal is never read as a verdict" (`docs/DESIGN.md`, D4 item 3) forbids. It propagates, or takes a path that decides the question itself.
- `blend/battery::support_coaxiality` and `offset_axial`'s frame read the pose reading rather than testing coaxiality themselves.

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
