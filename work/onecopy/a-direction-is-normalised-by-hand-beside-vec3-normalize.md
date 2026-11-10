---
id: a-direction-is-normalised-by-hand-beside-vec3-normalize
kind: issue
title: a direction is normalised by hand, v / v.norm(), at about 200 sites beside Vec3::normalize, and three files keep a fn unit of their own
status: closed
opened: 2026-11-06
priority: P3
cost: M
closed: 2026-11-06
pr: 6572
branch: onecopy/normalize
---


Found by `onecopy/curve-pair-tidy`'s sweep, while routing `polish::bound`'s closure and `pose.rs`'s `fn unit` to the one home.

## What

`geom_core` `Vec3::normalize` (`crates/geom-core/src/linalg/vec.rs`) is the home of "this vector's direction". It applies an exact power-of-two prescale where the squares would leave the format, so it is total at every finite magnitude. A hand-spelled `v / v.norm()` is that body without the prescale: it is the same bits wherever the largest component lies in `(2⁻⁵¹¹, 2⁵¹¹)`, and a zero or infinite direction outside it.

**First pass**, the shape `x / x.norm()` (same name) or `* (T::one() / ….norm())`, at the merge base 230be11d: 201 hits. topo 68, geom-brep 56, sweep 37, geom-core 21, geom 10, step-import 5, editor-core 3, demos/tour 1. Some examples:

- `crates/topo/src/boolean/sphere_region.rs` (`:201`, `:203`, `:260`, `:336`, …): eight sites.
- `crates/topo/src/ring_path.rs` (`:208`, `:217`, `:225`, …): seven.
- `crates/geom-brep/src/intersect.rs` (`:1341`, `:1557`, `:2128`), `crates/geom-brep/src/pcurve_cache.rs` (`:6247`, `:10449`, `:11207`).
- `crates/geom-brep/src/props/curved.rs` `:2591` (`far * (T::one() / far.norm())`), `crates/geom/src/curves/on_carrier.rs` `:471`.

**Copies of the function itself**: `crates/topo/src/boolean/section_cert.rs` `fn unit` (`:640`, `v / v.norm()`), and the closures `crates/topo/src/boolean/node.rs` (`:366`, `:468`) and `crates/geom-brep/src/carrier_net.rs` (`:1497`).

**Second pass**, aimed at the first pattern's blind spot: a norm bound to a name first, then divided by (`let n = v.norm(); … v / n`). 67 more: geom-brep 23, topo 23, step-import 13, sweep 3, editor-core 2, geom 2, geom-core 1. Many of these also use the length, which `normalize` does not return. Each such site needs a reading: either it keeps the length and its own division, or `normalize` wants a sibling that returns both.

**Not searched:** a division through a helper or a macro, and vectors in `[f64; 3]` form (the test files' own `fn unit`s, e.g. `crates/topo/tests/sliver_shell_role.rs`, `crates/test-utils/src/cone_cut.rs`). The tests' fixture helpers are out of scope for a kernel home.

## Owed

Route each kernel site to `Vec3::normalize`, delete the three function copies, and add a gate (`scripts/gates/`) that refuses the first-pass shape in `crates/*/src`. Fixtures are exempt. Each site moves in its own crate's run, and a moved test outcome is read as a change in the extreme-magnitude behaviour, not restored.

## Closed

PR 6572 routed every production first-pass site and the copies of `unit` (including `geom-brep` `intersect/section_bounds.rs`'s, spelled through `norm_squared().sqrt()`) to `normalize`. `scripts/gates/normalize-by-hand.sh` refuses the shape; its header states its blind spots. Second-pass sites whose length feeds other arithmetic keep their division, listed in the PR body. The residue `profile` `chord_director`'s witness prescale is `work/onecopy/profile-chord-director-prescales-by-its-witness-beside-normalize.md`.
