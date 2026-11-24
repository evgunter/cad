---
id: the-section-certificate-reads-the-cone-pairs-on-the-value
kind: unit
title: Topo's section certificate and table read cone × oblique cylinder and cone × cone on the section value; the duplicate ruling sweep in section_cert/ruling.rs retires
status: closed
opened: 2026-11-22
priority: P1
cost: H
branch: sectcone/section-cert-cone-pairs
parent: cone-germ-pairs-against-a-curved-face-have-no-section-frame
closed: 2026-11-24
pr: 7637
---

**Review tier: single FULL** (Opus). A move of an existing classifier onto the value, with the oracle rows re-pointed; the certified logic is U1's, already dual-reviewed.


U2 of `cone-germ-pairs-against-a-curved-face-have-no-section-frame`, after U1 (`the-cone-ruled-side-on-the-section-value`, PR 7357).

The work:

- **Route the general-pose pairs:** `topo::boolean::section_cert` routes its oblique `cone_cylinder` / `cone_cone` through `geom_brep::pair_section`, as `cylinder_cylinder` was routed.
- **Retire the duplicate sweep:** `section_cert/ruling.rs`'s general-pose use retires, and so do the functions PR 7357's body lists as duplicated.
- **Re-point the tests:** `section_cert/cone_pair_search.rs` and `section_cert_cone_pair_rows.rs` take the value as their oracle subject, re-pointed and not deleted.
- **Serve the cone pairs:** `serves` / `region_for` / the glue door take the cone pairs.
- **The geom-brep cone side's f64 rounding sites:** PR 7357 raised `bare-rounding-sites.tsv`'s `ruling.rs` re-pose count from 5 to 12. These sites are the same debt as topo's copy, and this unit retires them or schedules them.

## Seam facts (from the lane's first pass, 2026-11-22)

1. **`Elsewhere` keeps its closed forms.** The value answers `Elsewhere` for cones through one apex and for parallel cones of one aperture, and `classify_known` maps `Elsewhere` to `Intractable`. Topo answers these today (`TouchAtVertex`, the common rulings through the apex, `cone_plane_pair`), and U2 keeps reaching those closed forms.
2. **The apex touch.** The value lists it as a `Point` component, `Unminted(NoCarrier::ApexTouch)`, beside `Coincidence(ConeApex)`. Map it explicitly: a `Touch` when it stands alone, R-tan otherwise.
3. **`serves` stays false.** Flipping `serves(ConeRuled)` reaches the join: it would panic `glue::coincidence_site`'s `ConeApex` `unreachable!` and send `rides_carried` to `JoinDesync`. U2 calls `pair_section` for `Arm::ConeRuled` directly in `classify_known`, and keeps `serves` false until the march exists (the doors unit, and U3). The "serve the cone pairs" item moves out of U2.
4. **The region.** A sweep-only door's default table has no region, so the value reads `NoRegion`. Give the call the face pair's overlap ball, as topo's `cone_pair` did. `cylinder_cylinder` already accepted the same loss.

**Added by the cone × sphere / cone × cone door lane (PR 7457).**
`serves` now takes cone × cylinder, cone × cone and cone × sphere, so the
section table holds them and the join carries them on the value.
`topo::boolean::section_cert` `classify_known` reads cone × cylinder and
cone × cone on the value (this unit's `cone_ruled` arm, PR 7637), and
cone × sphere still on its own `cone_pair`, ahead of its `serves` arm. So
for cone × sphere the certificate's classification and the value's are
still two. Its copy is `section_cert.rs` `cone_sphere` (a closed-form
nappe count); `section_cert_cone_rows.rs`
`sphere_and_cone_every_nappe_class` and the R-tan row
(`section_cone_sphere_apex`, `section_cone_sphere_generator`) re-point onto
the value, whose apex on the sphere is a node or a touch
(`ConeRuledPose`), never R-tan, and whose tangency beside a crossing
refuses `Pinched { kc_sphere_tangency }`.
