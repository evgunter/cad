---
id: the-shell-door-is-a-third-door-value-the-certified-enclosure-census-does-not-know
kind: issue
title: The shell door is a third door value and the CertifiedEnclosure impl census still reads two rosters
status: open
opened: 2026-09-22
priority: P3
cost: D
---


## What

`crates/topo/tests/certified_enclosure_impl_census.rs` reads every
`impl … CertifiedEnclosure for X` in `crates/*/src` and checks each
DOOR's wiring roster against that set, both directions. Its `ROSTERS`
constant holds two entries — `props.rs`'s
`holds_the_certified_quadrature::<…>()` and `chart_region.rs`'s
`holds_the_certified_region_doors::<…>()`. Its header states the claim
those two are supposed to carry: *"a sixth `impl CertifiedEnclosure`
would form both doors and owe a row in each without anything going
red. This census is that red."*

LANE-3 added a THIRD door value of the same class — `topo::ShellDoor`,
one private fn-pointer field, one constructor `ShellDoor::certified()`
bounded `Decide + CertifiedBounds + AtRestPolicy`. A new
`CertifiedEnclosure` impl would form that door too, and the census does
not know it exists: the sentence above is now one door short of the
tree, and it reads green either way.

The shell door IS pinned by pointer identity — in
`props.rs`'s `at_rest_policy_tests::certifying_arms_are_the_doors`,
which `fn_addr_eq`s each certifying arm's door against
`crate::shell::shell_open::<T>`. It is not pinned through a
`wiring_rows`-shaped module with a needle helper, which is the shape
`ROSTERS` reads.

## Why it was not fixed where it was found

Adding a third roster entry does not work as the census stands: the
census requires a roster to name EVERY `CertifiedEnclosure` scalar,
and `certifying_arms_are_the_doors` is instantiated at `f64`, `Probe`
and `Interval` only — `at_rest_policy_tests` has no `Sym` row for any
of its three methods, which predates LANE-3. So closing this means
either giving the shell door its own `wiring_rows` module with the
full roster, or giving `at_rest_policy_tests` a `Sym` arm and
teaching the census this roster's shape. Both are outside LANE-3's
fence (`crates/topo/tests/*` is TCOST's and S-TINT's).

It rides naturally with the wiring-module fold LANE-4 carries: three
copies of the pointer-identity pin now exist (`QuadLane`'s,
`RegionLane`'s and the shell's), and whatever one module they fold
into is the thing the census should read.
