---
id: arm-one-function-level-hit-list
kind: issue
title: "[topo.homes] arm 1: Body-free functions inside the engines, by signature, beyond the curve × face doors"
status: open
opened: 2026-11-03
---


Found by `crossing-doors-leave-topo-for-geom-brep`'s second sweep pass (2026-11-03): crate-visible `fn`s under `crates/topo/src/{boolean,splitting}/` (test modules and `*_rows`/`*_probes`/`*_fuzz`/`*_sweep` files aside) whose signatures name no `Body`, no `*Key` and no boolean record or region type. Counts are Body-free / all such fns in the file:

- `boolean/sectors.rs` 29/38 (`departure_arm_lever`, `bisector_zero_refusal`, `datum_code`, `side_code`, `pierce_side`, …);
- `boolean/boxes.rs` 22/30 (`sweep_pad`, `slab_extent`, `cone_frustum_extent`, `ball_extent`, `torus_extent`, `ring_window_extent`, …);
- `boolean/join.rs` 12/20, `boolean/region.rs` 12/15, `boolean/solid_contain.rs` 10/24 (its line roots are `the-line-roots-door-is-still-built-in-topo`), `boolean/fan.rs` 9/10, `boolean/reduce.rs` 6/18, `boolean/contain.rs` 5/15, `boolean/insert.rs` 5/8;
- whole files: `section_cert.rs` (`section-cert-classification-takes-no-body`), `carrier_eq.rs`/`plane_eq.rs` (`carrier-eq-and-plane-eq-take-no-body`), `trace.rs` and `conic_quadric/` (`the-conic-quadric-door-is-still-built-in-topo`), `splitting/{order, pieced_arc, spiric_arc, spline_arc}.rs` (`splitting-arc-readers-take-no-body`).

What the pattern cannot see: a signature that takes an engine-declared type standing for topology (it counts as Body-free here when the type's name is not in the list), a private helper (not counted), and a Body read through a closure argument (`impl FnMut(…) -> …`, as `section_cert::certify`'s `place`). Each hit is checked by `[topo.homes]` rule 1 before it moves; many are one engine's own helpers whose only caller is that engine, where the move buys a smaller engine and nothing else.
