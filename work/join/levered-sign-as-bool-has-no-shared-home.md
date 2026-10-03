---
id: levered-sign-as-bool-has-no-shared-home
kind: issue
title: A levered sign read as a bool has no shared home across boolean/
status: open
opened: 2026-10-03
---


## What

Eight `crate::validate::decide_nonzero(` calls in
`crates/topo/src/boolean/` read a levered or plain margin's sign. Four
of them read it as a bool and spell out the same match, each with its
own refusal:

- `recl.rs` `wedge_is_reflex` (`bool_wedge_reflex`, ~:840): Positive →
  true, refusal `Coincide::Sectors` / `DeclarationRead::Moot`.
- `recl.rs` `resolve_edge_edge`'s membership closure (`bool_dir_same`,
  ~:959): Positive → true, refusal `FlankSense` or `CurvedFlankSense`
  after reading the declarations for the flank pair.
- `sectors.rs` `direction_sense` (`bool_dir_same`, ~:952): Positive →
  true, refusal `Escalated { DirectionSense }`.
- `insert.rs` `strut_order` (`bool_strut_order`, ~:1230): Positive →
  false, refusal `Coincide::Sectors` / `Moot`.

The other four keep the sign or discard it: `ops.rs` `ball_against_plane`
(`bool_sphere_extent_gap`, ~:1378), `ops.rs` `sphere_extent_scan`
(`bool_sphere_sphere_gap`, ~:3004), `ops.rs` `recut_lean`
(`bool_sphere_recut_align`, ~:3230), and `vtxfac.rs`'s `surface`
(`bool_sector_coplanar`, ~:325), which refuses on either outcome.

A helper of "a levered sign read as a bool or a `Coincide::Sectors`
refusal", as the PR 3962 review (S2) suggested, would serve only
`wedge_is_reflex` and `strut_order`. `bool_dir_same`'s refusal depends
on the face kinds and the declarations, so the helper would change its
payload. The part all four share is the bool read alone, returning
the diagnostic for the caller to map. Whether a helper that small earns
its place is the open question.

Found by the PR 3962 review (S2). Its fix pass left the three sites as
they are.
