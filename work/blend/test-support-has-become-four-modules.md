---
id: test-support-has-become-four-modules
kind: issue
title: sweep's test_support is four modules in one file, with six f64/_at twin pairs
status: open
opened: 2026-09-08
---


## The shape

`crates/sweep/src/test_support.rs` is 1185 lines and holds four
different kinds of thing behind one module gate. Its header states the
gate and the S52 rule that fixtures live in one place; it says nothing
about what else has moved in.

1. **Fixtures** — `cube:70`, `dome:141`, `waisted:281`,
   `domed_cavity:316`, `lantern`, `spool`, `sphere_zone`, `bowl:919`,
   `hemisphere_on_flat_base:986`, `ball_poled_z:345`, the rod family
   `:1097`. The module's stated subject.
2. **Selectors** — `rim_arcs_at:193`, `arcs_at:252`,
   `one_edge_rim_at:221`, `rod_creases:1115`. A different question from
   "what body does this suite build on": these answer "which entity of
   it does this row mean".
3. **Assertion walkers** — `assert_naming_totality:609` (a body-wide
   record walk that panics with its own report) and
   `assert_promises_either_side:791` (a claim about the TEXT of a
   recourse sentence, which touches no body at all).
4. **Closed-form volume algebra** — `mod pappus:819` with
   `pappus_volume:859`, plus `waist_fill:900` and `wedge_fill:959`.
   These are derivations a row checks the kernel against; they are
   oracles, not fixtures, and an oracle living beside the fixture it
   grades is how an oracle quietly starts reading the kernel.

## The twin pairs

Six fixtures exist twice, once at `f64` and once generic, because the
interval lane needs the second and `f64` callers did not want the turbo
fish: `revolved_about_y`/`_at:109/121`, `waisted`/`_at:281/288`,
`ball_poled_z`/`_at:345/351`, `bowl`/`_at:919/926`,
`hemisphere_on_flat_base`/`_at:986/992`, and the rod pair at `:1097`
and `:1104`. One generic function whose
`f64` use sites read `foo::<f64>(…)` would do; whether the ergonomic
cost is worth removing twelve declarations is the decision, not made
here.

## Rim tables for some fixtures and not others

`waisted:279` documents its three rims as `(radius, station)` pairs in
its doc comment, for `rim_arcs_at` to be called with. `dome:141` does
not, so its callers state `(1.0, 0.0)` at each site and its BORE rims —
two rims of one radius `0.5r` at two stations, the hazard
`rim-seed-finders-disagree-on-at-this-radius` names — are computed in a
review probe (`crates/sweep/tests/review_blend4_r4_probes.rs:226`)
rather than stated where the fixture is built. Whichever way it goes,
a fixture that mints latitude rims should say where they are, or none
should.

## Not fixed here

Splitting the module is a diff across every suite that names it, and
this unit's fence is the seed finders. Raised by the style review of PR
2129 (Q8).
