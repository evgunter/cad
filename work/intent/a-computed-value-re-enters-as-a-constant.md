---
id: a-computed-value-re-enters-as-a-constant
kind: issue
title: Computed values re-enter the symbolic lane as constants (from_f64 laundering); unaudited, and it makes a Sym theorem false
status: open
opened: 2026-10-08
---

The symbolic tier's zero verdicts (ERROR-DESIGN E12) are theorems about
the expression the `Sym` lane saw. A construction that turns a value it
computed back into a constant (`T::from_f64` of a computed f64) hands the
lane a different function, and a Zero over it is a theorem about the
wrong function: two typed `5 mm` slots laundered to `Lit(5)` compare
equal as polynomials. FORK-S4F (fork log row 93) makes the
`unproven-coincidence` door read those verdicts, so a laundered site can
report a false "proven".

Known sites, from FORK-S4F designer B's round 2 (branch
`design/intent-s4-s4f-B`): `crates/topo/src/chart_region.rs` lifts a chart
rect's f64 bounds into `T` before a Zero decision
(`T::from_f64(uh) - T::from_f64(ul)`), and `joint.rs` lifts a spline knot
domain. There are about 481 `from_f64(` sites in `sweep`, `topo`, `geom`
and `geom-brep`, unaudited.

Owed: an audit of each site (keep the value in `T`, or enter a numerical
routine's output as one opaque symbol per call), and the re-valuation
test FORK-S4F names, which detects a laundered quantity whenever the
re-valued variables move it.
