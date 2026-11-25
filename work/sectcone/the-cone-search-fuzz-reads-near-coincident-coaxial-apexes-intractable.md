---
id: the-cone-search-fuzz-reads-near-coincident-coaxial-apexes-intractable
kind: issue
title: The cone search fuzz reads two coaxial cones whose apexes stand just past the band apart as Intractable
<<<<<<< HEAD
status: dispatched
opened: 2026-11-24
priority: P1
cost: M
branch: sectcone/cone-search-coaxial-apexes
=======
status: closed
opened: 2026-11-24
priority: P1
cost: M
closed: 2026-11-24
pr: 7725
>>>>>>> origin/main
---


Filed by the cone × sphere / cone × cone door lane (PR 7457), from its CI
run on `46fc7b84d7`.

`topo` `boolean::section_cert::cone_search::the_cone_arms_agree_with_the_traced_section`
(slow set, a randomized search) reported one mismatch, reproducible with
`CAD_FUZZ_SEED=0x520d16ca1fc9237e CAD_FUZZ_EFFORT=1`:

> coaxial cone: cone apex (42.50, −65.96, −10.29), axis (−0.909, −0.139,
> −0.393), α 0.2677; coaxial cone d 1.56e-9, α₂ 1.1632: margin 1.56e-9:
> not classified: Intractable

The partner is coaxial, its apex 1.56e-9 from the cone's at a scale near
80 m. The pair takes `section_cert::classify_known`'s `cone_ruled` arm
(PR 7637), which reads it on the section value. `Intractable` there comes
from `of_pair_section` mapping a value refusal (`Unaccounted`,
`DegenerateOperand` and the rest of its list) before any carry, since a
carry refusal reads `Unmarched`. So the refusal is the classification's
(`pair_section/cones.rs` `ConeRuledPose::against_cone` or the ruling
sweep), not the march's. The search's must-answer rule expects a
classification at this margin. Decide which side is wrong: the apex
margin's lever (`kc_apexes_apart` reads `Margin::of`, with no lever at an
80 m scale), or the search's must-answer distance for a coaxial pair.

A second seed reproduces the class, on PR 7457's CI run of `5c97fce38f`:
`CAD_FUZZ_SEED=0x7e0e64c77639b551 CAD_FUZZ_EFFORT=1`, a coaxial partner
whose apex stands 4.4e-10 from the cone's at a scale near 75 m (α 0.3325,
α₂ 0.6417), `Intractable`.

A third, on `55815e7ae8`: `CAD_FUZZ_SEED=0x5bb0e2144c8691f3`, apex gap 3.3e-10
at a scale near 160 m. On PR 7457 the row failed on three of its last five
runs. Main's last three nightly jobs that ran it passed (job ids 138289046100,
138289046159, 138289046193). Reading the code path, this is not PR 7457's: the
pair reaches `section_cert::classify_known`'s `cone_ruled` arm (PR 7637),
the value's two-cone pose (`against_cone`, unchanged by 7457), and topo's
`cone_pair` → `cone_cone` on `Elsewhere`. But the frequency is not
established either way; replaying the seeds on main settles it.

## Closed (2026-11-24, PR 7725)
The refusal was the narrow cone's aperture guard (`ConeAtTheBand`) decided `Zero` over the search's hair-wide region. Topo's `of_pair_section` mapped that to `Intractable`. It now maps it to R-tan (`section_cone_at_the_band`), the same reading the value's ladder already gives it (`Refusal::InBand`). The fixed row `coaxial_cones_a_hair_apart_over_a_hair_wide_region_refuse_on_the_aperture` reproduces seed 0x520d16ca1fc9237e.
