---
id: a-padded-band-reaches-a-band-width-and-the-split-twinned-three-spellings
kind: issue
title: A padded band can reach a BandWidth; the split twinned the width accessors, the schedule fraction and the sub-arc joint
status: open
opened: 2026-11-24
priority: P2
cost: E
parent: from-f64-is-split-by-where-the-number-comes-from
---


Found by the two reviews of PR 7721 (`from-f64-is-split-by-where-the-number-comes-from`); taken, and not done in its fix pass.

- **`Band::padded(computed)` reaching a `BandWidth`.** `geom_core::BandWidth` is built from a `Band`, and a `Band` padded by geometry has a computed width. Four production sites pad a band by a computed `f64`: `geom-brep/src/pair_section.rs` (a box's `pad`, and a tube's pad), `geom-brep/src/pair_section/spline_implicit.rs`, `topo/src/boolean/section_cert.rs`. One review traced the two in `pair_section.rs`: neither padded band reaches a `zero_width()` / `escalate_width()` site (one goes to `decide_reported`, the other to `Foot::on_carrier`). The other two are untraced. The gate's header states the route as a gap; nothing holds it.
- **Twin accessors.** `Band::zero` / `zero_width`, `escalate` / `escalate_width`, `topo::boxes::sweep_pad` / `sweep_pad_width`, `FaceBoxes::pad` / `pad_width`: an `f64` reader beside a width reader for one number, and `boolean/glue.rs` calls both in one function.
- **Two spellings of one fraction.** `geom-brep/src/certify.rs` matches on `schedule_fraction(i, samples)` and enters `T::count_ratio(i, samples - 1)`, with the `- 1` written again at two more call sites; `sweep/src/skin.rs` enters `T::count_ratio(k, m)` where the `f64` breaks come from `geom_brep::sub_arc_joint(k, m)`. Each pair is equal today because both are the same two casts and one division, and a comment is the only record that they must agree. One home each.

Cost: E for the spellings; the padded bands want the two untraced sites read first.
