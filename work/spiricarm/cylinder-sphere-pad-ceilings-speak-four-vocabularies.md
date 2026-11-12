---
id: cylinder-sphere-pad-ceilings-speak-four-vocabularies
kind: issue
title: The cylinder x sphere pad ceilings above 1e-12 speak four vocabularies, and the pose rows' sharpness guard reads only at eps exactly 1e-9
status: open
opened: 2026-11-11
priority: P3
cost: M
---


Found in the review of PR 6840. This is the class only; no row is red.

The sweep suite states a curved body's volume-pad ceiling, at ε above 1e-12, in four separate ways:

- a per-ε `match` on `tol().eps()` returning measured widths: `crates/sweep/tests/a_closed_conic_in_the_partners_face.rs:1223` (`ellipse_pad_ceiling`);
- a fixed constant at every ε: `crates/sweep/tests/continuation_pose.rs:52` (`PAD_CEILING = 1e-4`) and `crates/sweep/tests/crossing_equal_rods.rs:218` (`PAD_CEILING = 1e-3`);
- the judging door's `Budget` (`Fixed` / `Resolution`), with `judged_at_resolution`: `crates/sweep/tests/common/differential.rs:276`, `:298`;
- the cylinder × sphere frame's resolution ceiling plus `(ε, width)` guard tables, after PR 6840: `crates/sweep/tests/cylinder_sphere_frame.rs:465` (`pad_ceiling`), `:407` (guard lookup), `crates/sweep/tests/verbs_cylsph_opening.rs:235` and `crates/sweep/tests/a_ringed_wall_divided_by_a_carried_chord.rs:33`.

Each makes a different claim (a derived bound, a measured guard, or a bare number). None names which kind of claim it is in a shared type, so a reader cannot tell a soundness bound from a regression guard without reading the derivation.

**The nine pose rows' sharpness guard has a single ε key.** It is `&[(1e-9, 4.0 * MEASURED_PAD[index])]` (`crates/sweep/tests/cylinder_sphere_frame.rs:505`, read by an exact float compare at `:407`). Before PR 6840 it was the same test spelled `eps != 1e-9`. At any other ε, nightly rows included, those rows are held only to the resolution ceiling, which at 1e-6 is thousands of times their pad.

Done when the suite has one vocabulary for "derived ceiling" against "measured guard at this ε", and the pose rows carry guards at the nightly's ε rows (or say per row why one is absent).
