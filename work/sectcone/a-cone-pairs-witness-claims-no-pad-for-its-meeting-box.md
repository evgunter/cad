---
id: a-cone-pairs-witness-claims-no-pad-for-its-meeting-box
kind: issue
title: A cone pair's witness on the section value is its meeting box's middle, up to the sweep's rounding charges off the section, and claims pad 0
status: open
opened: 2026-11-24
priority: P2
cost: M
---

Found by U2 (`the-section-certificate-reads-the-cone-pairs-on-the-value`), whose traced search (`topo` `section_cert/cone_pair_search.rs`) measured how far the value's witnesses stand off the carriers.

- **The witness is the middle of a box.** `pair_section/ruling.rs` `Sweeper::event` witnesses a component at `ruled.meeting(&Cell::at(phi), sheet).mid()`. That box is widened by the cone side's rounding charges (`RuledCone::charges`, `point`), so its middle can stand up to those charges off the section. The hosted runs saw about `10³` units of `ε·(|w − apex| + scale)`, where the retired ruling charts stood within `4.1`. For example, `8.7e-13` m at a pose of scale about 2.
- **The component claims no pad for it.** `cones::cone_ruled` builds its components with `SectionComponent::closed_form`, which sets `pad: 0`. topo's W3 places a witness at the band padded by twice the pad (`section_cert::certify_section_pair`). At the 1e-12 row, a witness `1e-12` off the section is placed at a band of the same size.
- **Related:** a pose decided on a `Zero` margin (the apex on the partner) lists witnesses on the exact pose, the margin off the true section. The search allows `64·margin` for them (`DECIDED_SLACK`).

**Fix:** either witness at the root itself (the projective root read once at the ruling, unwidened), or carry the box's half-width as the component's pad.

**Why P2, cost M, and not landed in U2** (PR 7637's review, 2026-11-24): W3's "Out past the band" stays sound only while the witness's offset is below the band, and at the 1e-12 row it is not clearly below, so the priority rises. The fix is not E:
- `SectionComponent::pad` is an `f64`, while the meeting box is an enclosure in `T`, so carrying its half-width needs a `T`-to-bound read the sweep does not have.
- The same witness (`Sweeper::event`'s `meeting(..).mid()`, `pad: 0`) serves the cylinder × cylinder and cylinder × sphere sweeps. The fix belongs to every ruled sweep, with rows for each.
