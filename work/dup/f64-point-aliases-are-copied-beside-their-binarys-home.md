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

## The non-sweep share (lane B, 2026-09-28)

**Ruling applied** (Ev, 2026-09-28): inline by default, keep a helper
only where it concretely helps. Every crate but `sweep` is done here;
the `sweep` share is lane A's and appends its own section.

**Census, re-taken at `c1b202b2e`.** Instrument: every tracked `.rs`
file (`git ls-files`, no path claim), matched for (a) a `fn` of any
visibility and arity, generic or not, whose brace body (multi-line
included) is `<Point2|Point3|Vec2|Vec3>[::<T>]::new(<its own parameters
in order>)`; (b) the same body behind a `let`-bound closure, annotated
or not; (c) a `let` binding of the constructor itself (`let pt =
Point3::new;`). Outside `crates/sweep`: **142** hits — 87 `fn`, 14
closures, 41 bindings (profile 40, topo 70, mesh 18, editor-core 5,
geom-brep 3, stl 2, viewer 3, step-export 1). The `fn` count agrees
with this row's 179 less `sweep`'s 92.

**Disposition.**

- **134 inlined**: the definition deleted and `<Type>::new(` written at
  each call, scoped to the definition's own block, and every per-binary
  home with it — `profile`, `mesh` and `stl`'s `tests/common::p2`,
  `mesh`'s `common/witness_bodies` private `p3`/`v3`, and `topo`'s
  `chart_region::tests::pt` with its two importing modules.
- **2 kept**, each a `let` binding local to one six-row face table,
  where inlining makes rustfmt break every row into a six-line record:
  `geom-brep/src/props/mod.rs`'s
  `hand_built_cube_volume_sign_tracks_orientation` (`p`) and
  `step-export/tests/common/mod.rs`'s `die_pips` (`v`). Neither binary
  holds another copy, so there is nothing to fold onto them.
- **5 left**: rustdoc examples, a user's chain rather than a test
  helper — `profile/src/path/family.rs` (four) and `topo/src/lib.rs`.
- **1 not a member**: `topo/tests/cube_doors_agree.rs`'s `ident`, the
  identity map passed as a value beside `sheared`, never called.

**Second pass, at the instrument's gaps**: struct-literal bodies
(`Point2 { x, y }`), `[x, y].into()` / `from` bodies, a `macro_rules!`
point helper, a renamed type (`Point2 as …`, `type P = Point2<f64>`),
and a residual `fn(f64, f64[, f64]) -> Point/Vec<f64>` signature with
any body — no member outside `sweep` (the residual signatures compute
something). Re-run over the working tree after the fold: only the
eight above remain.

**Status**: open until the `sweep` share lands.
