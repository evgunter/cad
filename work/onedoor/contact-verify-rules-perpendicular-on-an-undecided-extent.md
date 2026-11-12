---
id: contact-verify-rules-perpendicular-on-an-undecided-extent
kind: issue
title: contact_verify's tangent pass rules a pair Independent on a zero sign with no gate deciding its extent positive
status: open
opened: 2026-11-12
priority: P1
---

Filed by the ONEDOOR orchestrator from the face-pair designers (2026-11-12). Both found it by reading; neither executed it.

`crates/topo/src/boolean/contact_verify.rs`'s tangent pass (row 1) takes the sign of the two normals' dot product levered by `extent`, and rules `TangentRow::Independent` (perpendicular) on `Sign::Zero`. `extent` comes from `edge_extent_sup`, and nothing decides it positive before the sign is read. A collapsed locus would therefore read every pair definitely perpendicular. Certification's twin (`tangent_gap_orientation` in `geom-brep`'s `certify`) uses `decide_nonzero` and cannot fail this way.

Wanted: a probe on a collapsed locus. The designed fix is the face-pair unit's shared `normal_alignment` door, which returns `Perpendicular` only past a lever decided positive (`face-pair-contact-pairing-has-four-spellings`, "Designed").
