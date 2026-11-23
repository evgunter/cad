---
id: the-section-certificate-reads-a-plane-and-a-torus-on-its-own-rows
kind: issue
title: The section certificate reads a plane against a torus on its own rows, beside the section value's pose
status: open
opened: 2026-11-23
priority: P3
cost: M
refs: [a-torus-near-a-tilted-cut-stops-at-the-extent-scan]
---


A plane against a ring torus is now served by the section value
(`geom_brep::PlaneTorusPose`, `crates/geom-brep/src/pair_section/plane_torus.rs`):
its tilted poses decide their tangency and count their loops on the
rows `pt_near_tube`, `pt_far_tube`, `pt_between_tubes`. Two readers of
the same pair still read it on rows of their own, so one pose is
decided more than once (C5: "decided once, and every reader reads it").

- **The section certificate** (`topo::boolean::section_cert`,
  `classify_known`'s plane × torus arm, `torus_pair` → `torus_plane` →
  `plane_cut`) counts the same loops on
  `section_torus_plane_near_tube`, `section_torus_plane_far_tube` and
  `section_torus_plane_between`, unlevered and with no rounding slack,
  and reads every pose that way, axis-aligned ones included. The pose's
  rows read the same margins across `rounding_charge`, so near a
  tangency the two can disagree in band.
- **The join's germ frame** (`topo::boolean::join::pt_germ_frame`)
  reads `plane_torus_section` levered at the torus face's reach, while
  the pose reads it levered over the torus's ball. Where the pose reads
  a closed-form pose and the frame a tilted one, the operation's
  section table holds nothing for the pair and the germ refuses
  `BooleanError::GermSectionSpiric`.

Settles when the certificate's plane × torus arm reads the value's pose
for its tilted poses (as its cone × torus arm reads `ConeTorusPose`),
and the frame's lever and the pose's agree, or the refusal is pinned by
a row that straddles them.
