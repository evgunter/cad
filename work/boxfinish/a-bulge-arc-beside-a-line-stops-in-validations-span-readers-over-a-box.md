---
id: a-bulge-arc-beside-a-line-stops-in-validations-span-readers-over-a-box
kind: issue
title: A bulge arc beside a line stops in validation's span readers over a box, where its crossing with the line is found through the half-chord's root
status: open
opened: 2026-11-22
---


Found by the signed-radius unit
(`a-verb-stores-a-radius-as-a-magnitude-and-the-symbolic-tier-cannot-use-the-sign-its-gate-decided`),
measuring each arc verb over a box against a merge-base tree.

**Measured** (`Sym<Interval>`, the shipped rules, one whole-box replay,
`m10_8_arc_family_interval::replay`): a 4 × 2 mm blank whose right side
is one `arc_to(Bulge)` leg from `(4, 0)` to `(4, 2)` mm, the bulge a
variable over `0.5 ± 0.02` (and `−0.3 ± 0.02`), extruded 1 mm.

- **Before the radius was stored signed:** the profile validates, and
  the extrude refuses `carrier_matches_mapped_source` `[0, 2.3e-4]`
  (`[0, 5.6e-4]` clockwise). The document certified no box.
- **With the radius stored signed and the rim registration retired:**
  the replay stops one stage earlier, in guided validation, between the
  line and the arc that share the vertex `(4, 0)`: `arc_span`
  `[-1.7e-4, 1.5e-4]` (2 of 3 undecided) and `line_span` (1 of 3). The
  document still certifies no box.

**Where it comes from.** `seg::line_arc` finds a secant line's two
crossings as the centre's height along the line plus and minus the
half-chord `√(r² − h²)` (`Line2::half_chord`), and reads each against
both segments' spans. One crossing is the shared vertex. Here
`r² − h²` is the squared apothem, a perfect square of a rational
function of the bulge, and the vertex is `height − apothem`: the root
has to come back as the apothem with its sign, which is the open
question `work/decide/the-apothems-sign-is-a-value-read.md` holds.
Before, the retired registration `rim(a) ≡ radius` gave the tier
`radius² = ‖a − c‖²` through `sqrt(X)² = X`, and the span readings
decided through it; registering the squared form in its place
(`‖a − c‖² ≡ radius²`, true at every parameter value) was tried and
does not restore them.

The same pair on a tangent arc decides (the line is tangent, one
candidate, no root); on a via arc and a fillet the replay stops in
validation on both trees (`carrier_line_circle`, then `arc_span`).

**What would close it.** A pair reading that takes a shared vertex from
the loop's structure, not from the carriers' crossing; or the
half-chord's root read through whatever DECIDE rules on the apothem's
sign.

**The tests that say it is closed.** The document above validates over
its box, and what the extrude then refuses is reported by payload.
