---
id: the-section-certificate-reads-the-cone-pairs-on-the-value
kind: unit
title: Topo's section certificate and table read cone × oblique cylinder and cone × cone on the section value; the duplicate ruling sweep in section_cert/ruling.rs retires
status: open
opened: 2026-11-22
priority: P1
cost: H
parent: cone-germ-pairs-against-a-curved-face-have-no-section-frame
---


U2 of `cone-germ-pairs-against-a-curved-face-have-no-section-frame`, after U1 (`the-cone-ruled-side-on-the-section-value`, PR 7357).

The work:

- **Route the general-pose pairs:** `topo::boolean::section_cert` routes its oblique `cone_cylinder` / `cone_cone` through `geom_brep::pair_section`, as `cylinder_cylinder` was routed.
- **Retire the duplicate sweep:** `section_cert/ruling.rs`'s general-pose use retires, and so do the functions PR 7357's body lists as duplicated.
- **Re-point the tests:** `section_cert/cone_pair_search.rs` and `section_cert_cone_pair_rows.rs` take the value as their oracle subject, re-pointed and not deleted.
- **Serve the cone pairs:** `serves` / `region_for` / the glue door take the cone pairs.
- **The geom-brep cone side's f64 rounding sites:** PR 7357 raised `bare-rounding-sites.tsv`'s `ruling.rs` re-pose count from 5 to 12. These sites are the same debt as topo's copy, and this unit retires them or schedules them.
