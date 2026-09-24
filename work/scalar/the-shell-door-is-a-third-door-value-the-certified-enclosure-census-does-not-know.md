---
id: the-shell-door-is-a-third-door-value-the-certified-enclosure-census-does-not-know
kind: issue
title: The shell door is a fourth door value and the CertifiedEnclosure impl census still reads two rosters
status: closed
opened: 2026-09-22
closed: 2026-09-24
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

LANE-3 added another door value of the same class — `topo::ShellDoor`,
one private fn-pointer field, one constructor `ShellDoor::certified()`
bounded `Decide + CertifiedBounds + AtRestPolicy`. A new
`CertifiedEnclosure` impl would form that door too, and the census does
not know it exists, and it reads green either way.

Counting the doors: there are FOUR door values in the tree —
`geom_brep::OffsetFitLane`, `topo::QuadLane`, `topo::RegionLane` and
`topo::ShellDoor` — and four pointer-identity pin sites, three of them
`wiring_rows` modules (`props.rs`, `chart_region.rs` and
`geom-brep/src/offset_fit_lane.rs`, whose three `fn_addr_eq` rows the
`ROSTERS` list does not name either) plus the shell door's inline rows
in `props.rs`'s `at_rest_policy_tests`. The census header was
corrected at LANE-3's fix pass to say which two it reads; the
`ROSTERS` list itself is untouched.

The shell door IS pinned by pointer identity — in
`props.rs`'s `at_rest_policy_tests::certifying_arms_are_the_doors`,
which `fn_addr_eq`s each certifying arm's door against
`crate::shell::shell_open::<T>`. It is not pinned through a
`wiring_rows`-shaped module with a needle helper, which is the shape
`ROSTERS` reads.

## What is left, and what was wrong about the first answer

The blocker this row was filed with does NOT hold, and the correction
belongs here: it said a roster entry cannot be added because the
census requires a roster to name EVERY `CertifiedEnclosure` scalar
while `certifying_arms_are_the_doors` runs at `f64`, `Probe` and
`Interval` only. The missing `Sym` arm was three lines in `props.rs`,
inside LANE-3's fence, and it passes —
`at_rest_policy_tests::sym_over_f64_gates_run_the_doors` now covers
all four policy methods at `Sym<f64>`, added at LANE-3's fix pass on
both reviewers' finding. So the roster's SCALARS are no longer the
obstacle.

What is genuinely left is the census-side work, which stays outside
LANE-3's fence (`crates/topo/tests/*` is TCOST's and S-TINT's): the
`ROSTERS` edit, and the shape it reads. `ROSTERS` names a file and a
needle instantiated as `helper::<…>()`, so it can only read a pin
written as a helper called per scalar; the shell door's pin is inline
in the gate rows, and the offset fit's `wiring_rows` — the fourth pin
site, unlisted too — would want its own entry.

It rides naturally with the wiring-module fold LANE-4 carries: FOUR
pin sites now exist (`QuadLane`'s, `RegionLane`'s, the offset fit's
and the shell's), and whatever one module they fold into is the thing
the census should read.

## Closed by LANE-4P (PR 3165)

The four door values are pinned in one shape: one `#[cfg(test)]`
helper per door, in the file that defines it, returning
`Result<(), &'static str>` naming the field that moved, called once
per scalar — `geom-brep`'s `holds_the_offset_fit()` (at `f64`, the
constructor being concrete), and `topo`'s
`holds_the_certified_quadrature`, `holds_the_certified_region_doors`
and `holds_the_certified_shell_door` (new, in `props.rs`'s
`wiring_rows`; the at-rest policy rows keep testing what the arm
answers). The census's `ROSTERS` names all four with where each is
formed (`CertifyingScalars`, or `F64Only` with its reason), and a new
row enumerates the door VALUES in `crates/*/src` — every inherent
zero-parameter constructor returning `Self` (bare, or in an `Option`
or a `Result`) bounded on the right, concrete at `f64`, or named
`certified`/`fit` — and reds on one with no roster entry and no
`NOT_A_DOOR` exemption, and on an `impl` head it cannot read. A third
row requires each helper to hand every fn-pointer field of its door
to `fn_addr_eq`. Its blind spot is stated in the census header.

**What LANE-4 still owes:** the fifth door (`PcurveFittedLane` →
`FittedLane<T>`) lands in this shape — its helper comparing all three
of its fields and one row per scalar it is formed at, beside its
fields, and its `ROSTERS` entry.
The census reds until it does, provided its constructor is one the
reader sees.
