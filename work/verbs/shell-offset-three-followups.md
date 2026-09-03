---
id: shell-offset-three-followups
kind: issue
title: shell/offset follow-ups - curved-rim narrowing, the three-owner winding predicate, the per-call pcurve mint
status: open
opened: 2026-08-27
github: 1058
refs: [1048, 1019]
---

## From GitHub issue 1058

opened 2026-08-27, 0 comments.

**Three small items banked by OFF-D PR-2 (#1048), filed together because each is a line of work rather than a unit.**

**1. `OpenFaceRingUnsupported`: narrow the curved-rim refusal.** `topo::shell_open` refuses a designated face that is not planar, because its rim would be a CURVED face carrying a ring loop and the closed-form property inventory has no reading for that shape (the same kernel-wide limitation the fillet band's ring-free annulus works around). The refusal is honest but wider than the defect: a curved rim is only unreadable for the props/quadrature lane, and a caller who never asks for volume or area is refused anyway. Narrowing means either giving the inventory a curved-face-with-ring reading, or moving the refusal from construction time to the props call that actually cannot answer.

**2. `bool_ring_run_winding` has three owners, not one.** The predicate name is decided by `topo::validate`'s planar-boundary check, `topo::boolean::join`'s ring-run test, and `topo::merge_faces`' role normalization. Any verdict-log assertion that filters on the `bool_` prefix therefore has to except it by name — `verbs_shell::shell_runs_no_intersection_machinery` does, with the reason on the page. Giving the validator's and merge_faces' uses their own names would make the prefix filter exact again and remove the exception.

**3. The whole-body pcurve mint is still per call.** `topo::shell` calls `replace_faces_offset` per chart, and each of those re-mints the whole body's pcurve map, so an `n`-chart body pays `n + k` whole-body mints (`k` = designated faces). Measured on #1019: at 3–6 charts the whole verb is 16–23 ms in release and the quadratic term is invisible, so this is not urgent. The lever when it becomes urgent is a composite door that defers the mint to one call at the end; `pcurves::staleness_posture::DECLARED` already has the vocabulary for a door that declares it does not re-mint.

## Home

All three sites are `crates/topo/src/shell.rs`, `replace_face.rs` and the offset lane, in VERBS' `paths:` territory.
