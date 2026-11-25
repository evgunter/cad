---
id: a-crossings-bracket-is-as-wide-as-the-composite-form-is-ill-conditioned
kind: issue
title: A crossing's bracket is read on the composed form, so its reach is the form's conditioning and not the crossing's
status: open
opened: 2026-11-24
---


Found by the crossing-parameter unit, 2026-11-25. DEMONSTRATED by `geom-brep`'s `composite::crossing_parameter_rows::on_a_thin_torus_the_reach_is_bands_long_and_the_end_reading_carries_it`.

`composite::crossings` brackets a crossing between the last parameter where the composed form's enclosure reads one strict sign and the first where it reads the other (`last_on_side`, `first_on_side`), and the carrier's length over the bracket is the crossing's reach. On a torus of 100 m by 0.1 m at a sine of `10⁻³` the canonical quartic's enclosure reads no sign over more than a hundred zero bands, so the crossing reaches that far and refuses against an end, a crease, another crossing or a trim edge inside it, though the metred residual `implicit_residual` changes sign at one parameter there.

What would narrow it: a second bracket read on the surface's metred residual in certification arithmetic at a point (the carrier's net and the surface crossed to `Interval`, evaluated at the parameter), which has the composite's zero set on the piece and rounds at the surface's size and not its fourth power. It was not built: the unit keeps one certified reading, and the thin-torus rows are its test.
