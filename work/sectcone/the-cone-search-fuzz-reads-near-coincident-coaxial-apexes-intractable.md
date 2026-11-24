---
id: the-cone-search-fuzz-reads-near-coincident-coaxial-apexes-intractable
kind: issue
title: The cone search fuzz reads two coaxial cones whose apexes stand just past the band apart as Intractable
status: open
opened: 2026-11-24
priority: P1
cost: M
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
