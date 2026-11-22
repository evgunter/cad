---
id: the-cylinder-data-doors-leave-a-test-only-arm-two-paths-and-a-wide-surface
kind: issue
title: After 7030: the cylinder data doors leave a test-only arm door, two paths to cylinder_datum, a wide pub surface, two radius sources, and an unmeasured tail at 1e-6
status: open
opened: 2026-11-16
priority: P3
cost: M
---


Left by PR 7030's review (`work/onecopy/the-replays-cylinder-offset-and-radii-are-a-second-spelling-of-the-kernels-data.md`). 7030 made the replay read two cylinders' tilt, offset and radius from the kernel's one computation (`geom_brep::carrier_eq::cylinder_datum`, through `topo::boolean::cylinder_pair_data`). What it leaves:

1. **A test-only arm door (F7).** `topo::boolean::carrier_pair_arm` (`crates/topo/src/boolean/carrier_pair.rs`) and `geom_brep::carrier_eq::consumed_arm` (`crates/geom-brep/src/carrier_eq.rs:775`) have no production caller now; the replay's tilt comes levered in `CylinderPairData`. Their readers are two test rows: topo `lever_rows::the_pair_arm_is_the_arm_the_cylinder_tilt_is_levered_at` and `the_cylinder_pair_data_are_the_ladders`, and editor-core `replay::tests::the_axis_tilt_is_levered_at_the_pairs_reach_not_a_metre`. Either fold them into the data door (the tilt's arm as a field, or the test reading `cylinder_pair_data`'s tilt), or say why a public arm reading earns its place.
2. **Two paths to `cylinder_datum` (F9).** The ladder reaches it through `ladder_reading` → `at_consumed_extent` → `cylinder_data` (`carrier_eq.rs:506`). The replay reaches it through `cylinder_data_at` → `at_consumed_extent` → `cylinder_datum` (`carrier_eq.rs:710`). They agree today because both call `at_consumed_extent` (a row pins it). One path that both read would hold that by construction.
3. **The pub surface (F11).** 7030 made public, in `geom_brep::carrier_eq`: `cylinder_data_at`, `CylinderPairData` (also re-exported at the crate root), and `CARRIER_CYL_AXIS_OFFSET`, the last for a test. `consumed_arm` was made public by 6773's move. Decide which belong in the API, and narrow the rest (`#[doc(hidden)]`, or a test-support door).
4. **Two radius sources (F15).** On the stated path the replay reads `Margin::of(ra − rb)` itself (`crates/editor-core/src/coincide/replay.rs` `one_cylinder`), because the radius needs no extent there. The kernel reads the same expression through `cylinder_radius`. Give the replay a door that reads the radius without the extent, or make the extent-free reading of the radius the data door's own.
5. **The 1e-6 tail (F14).** 7030's probe of `one_cylinder`'s outcomes over the 799 editor-core door and corpus slow tests compared the ported code with the old at default ε and 1e-12 in full, and at 1e-6 for 51 of 65 calls before a pause stopped it. Re-run the probe (63cecf86, reverted in 8d9a7790) at 1e-6 over the same filter, and diff it.
6. **`implicit.rs`'s exhaustive arms (F1's remainder).** `crates/geom-brep/src/implicit.rs` lists `Surface::Nurbs(_) | Surface::Approx(_)` as a match arm in `implicit_gradient` (~310), the lever-arm reader (~381), `implicit_hessian` (~529) and the rounding-charge reader (~771). These are arms of exhaustive matches, each with a poison or `None` of its own, not predicates, so `geom_brep::is_analytic` cannot replace them mechanically. A shared early return (`if !is_analytic(s) { return poison }`) before each match would make that set one list too.
