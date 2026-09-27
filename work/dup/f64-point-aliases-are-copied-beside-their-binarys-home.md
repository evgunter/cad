---
id: f64-point-aliases-are-copied-beside-their-binarys-home
kind: issue
title: An f64 point constructor that lifts nothing (fn p2(x, y) -> Point2<f64> { Point2::new(x, y) } and its p3/v3/pt kin) is spelled at 179 fn definitions, most in test binaries that already hold one
status: open
opened: 2026-09-26
priority: P4
cost: D
---



## Finding

Measured by the S-DUP lane that folded the lifting class
(`coordinates-lifted-into-a-point-are-spelled-per-component`), at
`360eb7320`. That row's class LIFTS `f64` coordinates into a scalar
`T`; this one lifts nothing: a function or closure whose whole body is
`Point2::new(x, y)` (or `Point3`/`Vec3`/`Vec2` with its own arguments
passed straight through). It is the one-line rename of a `const fn`
constructor, spelled per suite.

**179 `fn` definitions and 14 closures.** Instrument: every `fn` taking
two or three `f64` and returning a `Point`/`Vec` at `f64`, over
`git grep` with no path argument, with the body on the next line
compared against `<Type>::new(<its own arguments>)`; 185 matched the
signature, 179 have that body (the other six compute something). The
closures: the same body behind `|x: f64, y: f64| ...`.

By test binary, with the binary's existing home where it has one (the
counts include the homes themselves):

- `sweep` (`tests/all.rs`): 84 `Point2` + 4 `Point3` + 4 `Vec3`. The
  binary holds **three** `f64` `p2`s already — `revolve_common::p2`,
  `mate2_common::p2`, and a private one in `common/cone_nappe.rs` —
  and `common/mod.rs`'s routing list calls `revolve_common` "the place
  `p2` and `eps` presently live despite belonging to no verb". So the
  fold here has a routing step first: which home.
- `profile` (`tests/all.rs`): 34 `Point2`, of which 33 copy
  `common::p2` (the binary's one home). None of the 33 copying suites
  glob-imports `common`, so each fold is a delete plus an import.
- `topo`: 11 in `src` test modules (eleven files, four of them
  `review_m1_*`), and 9 `Point3` + 7 `Vec3` in `tests/`.
- `mesh`: 14 in `tests/`, beside `tests/common/mod.rs::p2` and
  `common/witness_bodies.rs`'s private `p3`/`v3`;
  `r1_probe_bool_route.rs` and `r1_probe_hash.rs` copy the home's `p2`.
- `stl`: `review_m3_pr55_e2e.rs` copies `tests/common/mod.rs::p2`.
- `editor-core` 3, `viewer` 2, `profile/src` 2, `mesh/src` 1,
  `geom-brep` 2.

## Why filed rather than folded

Size alone. The fold is mechanical per site, the way
`common::interval` was, but it is 185 matching signatures (179 of them
pass-through) in 169 files across nine crates — well past one PR
beside the class it was measured from. Method item 6 still applies
site by site when it is taken: `p2(1.0, 2.0)` against
`Point2::new(1.0, 2.0)` is a readability call, and a suite that
prefers the constructor may inline rather than import.

## Why this row is on this slate

Its member files sit on twelve programs' ground (`work.py territory
--files -` over the files carrying a matching signature, 2026-09-26):
`tint` and `tcost` on 154 of the 169, `paths` on 36, `tess` on 11, then
`shell`, `vdoc`, `exch`, `chrome`, `reach`, `curved`, `chart` and
`atrest` on one to three each. No one of those owns the class, and one
thing spelled many times across all of them is this program's charter
(method item 14); any of them may claim a crate's share by `git mv`
of a split row.

## Blind spots

The instrument reads the body on the line after the signature, so a
body that rustfmt splits across lines is counted as "computes
something" rather than as a pass-through. It sees only parameters
typed `f64` in the signature; a generic `T` helper used only at `f64`
is outside it. Closures are counted only when their parameters are
annotated `f64`. A binding of the constructor itself —
`let p2 = Point2::<f64>::new;` (`sweep/tests/bitdump.rs`,
`must_carry_rule.rs`), `let pt = Point3::new;` — is not a definition
and is not counted; those name the constructor and hold no body.
