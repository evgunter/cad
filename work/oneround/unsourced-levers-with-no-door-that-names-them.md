---
id: unsourced-levers-with-no-door-that-names-them
kind: issue
title: Ten unsourced references are no lever and have no door that names them: a sine against the band's ratio, a count, a scaled stand-off, a rounding against a share of the band
status: open
opened: 2026-11-22
design: true
priority: P1
cost: H
---


Found by `oneround/unsourced-stand-ins` (PR 7268, a cut of `unsourced-levers-await-a-source`; Ev on PR 7141: no fourth source, each site re-posed by the role its length plays). Ten `// Unsourced lever:` references are not a lever of anything, and no existing door names what each is. Each stays pinned in `scripts/gates/lever-sources.counts`, and its comment names this row.

## The four kinds, and the ten sites

1. **A dimensionless sine against the band's own ratio.** `geom_brep::dihedral::wedge_reads_zero_at_the_arm` decides `sin θ · Kε` in `(ε, Kε)`: Zero exactly where `sin θ ≤ 1/K`, Positive where `sin θ ≥ 1`. `wedge_reads_zero_at_the_offer` is the same with `tan θ`. Read with a sourced lever instead, each is `levered(sin θ, folded arm)` in the band at the tolerance that decides the arm (`arm/K`, resp. `|cos θ|·arm/K`), and the arm is a `T` (an `Interval` on that lane). (2 references.)
2. **A count of half-turns.** `topo::boolean::fate::tip_wraps` reads `half_turns ± halves` through `fate::sign` at a unit arm. The value is an integer exactly (a straight-edged loop turns `±2π`, each half-turn corner `±π`) plus the rounding of a sum of `atan2`s, and it is decided Zero within `ε`. `sign`'s other caller, `bisector`, was re-posed on PR 7268. (1 reference.)
3. **A stand-off scaled by a dimensionless factor.** `topo::boolean::join::off_conic_slack` is `2ε·cot ψ`: two sites' stand-off from the conic (each up to the band's `ε`) carried through `cot ψ`. It is read as a tie (`bool_join_arc_slack`, slack within the band ⟺ `cot ψ ≤ 1/2`) and as a subtrahend (`bool_join_arc_clear`, `|m| − slack` past the escalation). `geom_brep::intersect`'s `pn_apex_tangent_reach`, `pn_apex_point_reach` and `pair_bound`'s far, near and offset are `size / sine` with `size = |apex_gap| + gap_err`: the plane's stand-off from the cone's apex plus a hand rounding term, over a dimensionless sine (`steep`, `|discr|`, `d`, `m`, the pair's own divisor). (6 references.)
4. **A rounding against the `ε/K` share of the band.** `topo::validate::resolved_as` decides `K·rounding` Zero, i.e. the rounding (a carrier's `carrier_resolution::resolution`, a `Rounded::error`) within `ε/K`. Its refusal carries the margin `K·ρ` into `JoinError::SectionUnresolved`'s `resolution` and into `vertex_section`'s refusal. (1 reference.)

## The facts any answer works within

- `Margin` is a length by signature (its doors' docs); there is no door for a dimensionless reading or a count.
- `Band::padded` and `Margin::less` take an `f64`, so a `T`-valued stand-off (3) cannot pad a band or be charged as an allowance at `Interval`.
- `Band::linear_at`, the band at a caller-named ε, is a `#[doc(hidden)]` instrument for suites, not a production door.
- The one production precedent for a band built from a band's own thresholds is `topo::chart_region`'s `Band::new(band.zero() / 2.0, band.escalate())`.
- PR 7227's review rejected `Margin::levered_inv(size, sine)` for intersect's five: a bare division under a door's name.
- PR 7268's differential (both lanes, ε = 1e-9 and 1e-12) measured the seven conversions it made; these ten were not converted, so nothing about them is measured yet.

## Options

Open; none chosen. The question is whether a door family is added to `geom-core` for some of these kinds (and which), or whether each is restated through the doors that exist; the answer binds the next sites of each kind.
