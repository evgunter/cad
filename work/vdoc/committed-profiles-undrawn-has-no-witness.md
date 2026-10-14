---
id: committed-profiles-undrawn-has-no-witness
kind: issue
title: viewer: CommittedProfiles::undrawn has no row that can fail
status: open
opened: 2026-09-30
priority: P3
cost: M
---

Found by PATHS 5a (`retire-the-stored-bulge`, PR 3527). The committed
pass counts profiles it could not draw (`viewer::sketch::CommittedProfiles`'s
`undrawn`, surfaced as `ViewerApp::profiles_undrawn` and
`frame::profiles_badge`). Its one witness was `profile_draw`'s subnormal
bulge row (now `a_validated_sub_tolerance_arc_draws_as_its_line`), whose arc the pass used to count undrawn. Since 5a the
committed pass draws the VALIDATED kind, and validation classifies that
sub-tolerance arc a line, so it draws straight and the row now asserts
`undrawn` is empty. No row makes `undrawn` non-empty, so a pass that
never counted anything would stay green.

**What would close it.** A row whose validated profile carries an arc
the flattener refuses (a carrier that is not drawable at a validated
arc's vertices), asserting it is counted in `undrawn` and that the badge
renders.
