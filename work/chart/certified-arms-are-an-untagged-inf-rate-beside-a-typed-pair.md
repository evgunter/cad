---
id: certified-arms-are-an-untagged-inf-rate-beside-a-typed-pair
kind: issue
title: chart_region::certified_arms is an inf rate per kind with no InfSpeed tag, the rate pair's next member
status: open
opened: 2026-09-15
refs: [three-tables-of-the-chart-arms]
priority: P1
cost: E
---



## Finding

`crates/topo/src/chart_region.rs`, `certified_arms`: the chart's
metring arms per kind — a plane's exact `(1, 1)`, a cylinder's
`(r, 1)`, and a certified INF assembly on every other carrier, gated
through `Margin::of` — are metres per chart unit, which is exactly
what `geom_core::InfSpeed` now names. They are the one shipped inf-side
rate producer the RATE-PAIR unit did NOT type, by the D283 ruling's
own scoping: the pair landed with `metered` and the sup door and the
three blurred sites, and "the remaining hand sites as their programs
touch them" is unit (iii).

Typing it would put the direction where the reader cannot miss it.
`chart_region.rs:105`'s module doc and the note at `:2576` both spend a
paragraph on *why* an inf is owed here and why quoting
`geom_brep::chart_stretch_sup` "would invert exactly that" — the
sup/inf confusion the pair exists to make unrepresentable, argued in
prose one file away from a `SupSpeed` that says it in the signature.
The assembly's own pin (`the two readings are 16x apart on one chart`,
in the same file) records that a full sup-swap of `certified_arms`
passed the entire topo+sweep suite green, which is the sharpest
statement of why prose is not enough here.

## Its neighbour, the same shape

`geom_brep::chart_stretch_inf`'s `ChartStretchInf` is the other half:
`inf_u`/`inf_v` are inf rates and `sup_u`/`sup_v` are the very numbers
`chart_stretch_sup` now answers as `SupSpeed` (the two doors report ONE
sup — `the_two_doors_report_one_sup` pins that equality). Its fields
stayed bare `f64`/`T` for the same scoping reason, and `area_inf` is an
AREA rate, which the linear pair does not name at all. Whoever types
`certified_arms` will pass through this struct on the way.

**And the naming, which is the same finding from the reader's side.**
`chart_stretch_sup`, `ChartStretchInf::sup_u` and `certified_arms` are
three spellings of ONE concept — a chart's per-axis arm — of which the
rate pair typed one and a half (`chart_stretch_sup` answers a
`SupSpeed` pair and now refuses the cone; `chart_stretch_sup_v` answers
the `v` channel alone). A reader meeting the three has no name that
tells them apart, which is why the drift hazard above is a naming
hazard first. Whoever types these should settle the vocabulary in the
same pass rather than adding a fourth.

## The rest of the class, found by two reviews

The sweep that filed this row keyed on identifier substrings
(`speed|rate|stretch|meter|lever`) and on the `Margin::` door
spellings. Four members carry none of those, so the pattern was blind
to every one of them; they are the blind spot instantiated, and they
belong on this row rather than in a PR body.

- `crates/editor-core/src/clearance.rs`, `chart_arms` and
  `metred_rect`: the same per-kind table again (plane `(1, 1)`,
  cylinder `(r, 1)`), multiplied by hand into a `MetredRect` that
  `certifies_outside` decides at the band. Its own doc declares itself
  a COPY of `certified_arms` and says why (the original is private to
  `chart_region`), so the drift hazard the copy names is now also a
  typing hazard: two tables, one of which will be typed first.
- `crates/topo/src/chart_region.rs`, `ScaledFace::build`'s `arm_u` /
  `arm_v`: the site that actually performs `p.x * arm_u` — the
  hand-spelled crossing `certified_arms` feeds. A typed
  `certified_arms` types this signature with it.
- `crates/geom-brep/src/offset_meters.rs`,
  `MeterError::NormalFloor`'s `speed_lever: f64`: a PUBLIC receipt
  carrying a sup rate untagged, beside a `PatchRegularity` whose own
  speeds are `SupSpeed<f64>`. RATE-PAIR left it bare deliberately and
  the reason is a real obstacle, not scope: `MeterError` derives
  `PartialEq`, which `SupSpeed` does not have and will not (no `==` on
  a tagged rate), so typing the field means hand-writing `PartialEq`
  for a public error — a rate comparison outside the classify seam —
  or dropping the derive from a public type. `Display` is not the
  blocker: its text survives a `.get()`.

`crates/topo/src/chart_bound.rs`'s `assembled(u_arm)` was the fifth and
is no longer open: RATE-PAIR's fix pass typed it `ChartArm<T>`, so the
span check picks its door off the arm's kind.

## Who owns it

Contested, and worth saying so in one place rather than rediscovering
it. SCALAR's `keep_out` says `chart_region.rs` is code-quality Track
M's; TOPO's says Track Q's (S-BOOL/CURVED); neither names TRIM. What
puts the row here is precedent, not territory — TRIM already carries
`three-tables-of-the-chart-arms` on the same function from the same
direction, and one file should not have two homes. Whoever takes it
should reconcile the two `keep_out` lines in the same pass.

## Shape

`certified_arms` answers `InfSpeed<T>` per axis; the `Margin::of` gate
keeps its own reading of a collapsed arm (the pair is a tag, not a
positivity witness); the exact plane and cylinder arms are inf bounds
by being exact, as `param_rate`'s closed forms are. Small, and it
wants to land with — or just after — TRIM's
`three-tables-of-the-chart-arms`, which proposes making the same two
exact arms reachable through a `pub` door: whoever writes that door
chooses its return type, and this row says which one.

## Home

Filed by SCALAR's RATE-PAIR lane, which reached `chart_region.rs` only
to read it. `chart_region.rs` sits in no program's `paths:`; TRIM
already carries `three-tables-of-the-chart-arms` on the same function
from the same direction, so this lands beside it rather than opening a
second home for one file.
