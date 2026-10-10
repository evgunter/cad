---
id: line-quadric-root-code-and-the-cone-form-have-several-homes
kind: issue
title: Line × quadric root code has two homes, the cone's quadric form three spellings, and ConicHarmonics carries two cone-only knobs
status: open
opened: 2026-10-06
priority: P4
cost: M
refs: [4135]
---


Left by the last fix pass of PR 4135 (the cone root lane), from its
second review (r2 S-1, S-2, S-6).

## What

- **Two homes for one door's root code.** `line_cone_roots` lives in
  `crates/topo/src/boolean/reduce.rs:3569`; its siblings
  `line_wall_roots`, `line_sphere_roots` and `line_torus_roots`
  (`crates/topo/src/boolean/solid_contain.rs:4088`, `:4226`, `:4571`)
  and its own quadratic, `line_cone_quadratic` (`solid_contain.rs:4191`)
  and `quadratic_roots` (`solid_contain.rs:4174`), live in
  `solid_contain.rs`.
- **The cone's form, three spellings.** `line_cone_quadratic` returns
  `−Q` (`solid_contain.rs:4191-4205`); `geom_brep::conic_cone_harmonics`
  returns `+Q` (`crates/geom-brep/src/implicit.rs:1199`);
  `geom_brep::conic_cone_residual` re-spells `cone_elevation(None)` with
  a running bound (`implicit.rs:1227`).
- **Two cone-only knobs on the shared struct.** `ConicHarmonics::per`
  and `::floor` (`implicit.rs:1073`, `:1079`) are `2r` and `1` on every
  kind but the cone; only the cone varies them.

## The fix owed

One home for the line × quadric roots, one sign convention for the
cone's form, and the cone's normalizer and floor carried where only
the cone reads them. A reshuffle with no design implication.
