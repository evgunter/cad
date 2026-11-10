---
id: section-degenerate-operand-mixes-a-radius-at-the-band-with-invalid-operands
kind: issue
title: SectionError::DegenerateOperand still mixes radius guards decided Zero with negative radii, poisoned heights and unread partners, so it aborts as mixed
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Found splitting `geom_brep::SectionError` by standing (`onecopy/section-error-standing`). There, the cone's aperture and seam guards decided `Zero` became `SectionError::ConeAtTheBand`, which stands in band. `DegenerateOperand` (`crates/geom-brep/src/intersect.rs:692`) still aborts as mixed.

## What

`DegenerateOperand { what }` still carries the following:

- **Radius guards decided `Zero` (in band) beside radii decided negative (1(a)).** These are the cone-cylinder guard (`intersect.rs:3118`), the cylinders' radius guard (`pair_section/cylinders.rs:306`), and the decided arm of `PoseRefusal::Radius` and `TorusPoseRefusal::Radius` (`pair_section.rs:212`, `pair_section/torus.rs:218`), whose verdict keeps the `Sign` apart.
- **A poisoned net height** (`pair_section/spline_implicit.rs:326`), row 3 or 4.
- **A partner the implicit form does not read, or a composite that does not certify** (`pair_section/spline_implicit/curved.rs:81`, `:133`, `:138`, `:214`).

## What closing it takes

1. Split the decided-`Zero` radius into its own in-band variant, beside `ConeAtTheBand` and named for the situation. The decided `Sign` is already at each guard and in `PoseRefusal::Radius`'s verdict.
2. Give the poisoned height and the unread partner their own classes: a fault, and a lane not built.
3. Then `DegenerateOperand` is the decided-negative operand alone, a 1(a) abort, and `SectionError::refusal` loses its mixed arm for it.
