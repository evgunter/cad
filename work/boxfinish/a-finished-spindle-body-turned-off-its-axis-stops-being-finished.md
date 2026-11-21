---
id: a-finished-spindle-body-turned-off-its-axis-stops-being-finished
kind: issue
title: A finished spindle body turned rigidly stops being finished: tier 3 refuses the image with VolumeUncomputable or SheetApexReached
status: open
opened: 2026-11-21
priority: P1
cost: M
---

Found by the r2 review of PR 7225 (`oneround/box-region`, NOTE 11) and reproduced by that PR's lane at its merge base and its head. Not that PR's: no session-box verdict is involved.

**What.** A spindle body that is finished where it was built is refused by tier 3 once a rigid motion turns it, for most turns (three of the four tried below). A finished body moved by an isometry must stay finished: the move door keeps every family but the session box up to isometry (`finish::FindingFamily::under_isometry`), so the moved value is held finished while a fresh gate on the same image refuses it.

**Reproduction** (`sweep::test_support::try_revolved_about_y_at`, `Revolution::Full`, ε = 1e-9, at the origin, `topo::transform_rigid` then `topo::validate_geometric`):
- the apple, profile `[(0, −200) bulge 1.6, (0, 200) bulge 0]` (a spindle's outer sheet): finished at home; turned 0.7 rad about `(1, 2, 3)`, 0.5 rad about `x`, or 2 rad about `(0.3, −0.8, 0.5)`, refused `VolumeUncomputable { solid, source: Face { face: 1v1, source: Escalated { cause: Indeterminate { margin: Enclosure { lo: −1005.31, hi: 1005.31 } } } } }`; turned 1 rad about `z` it is still finished (a turn the quadrature happens to read). The lemon `[(0, −300) bulge 0.5, (0, 300) bulge 0]` the same, with `±471.24`.
- the frustum with a spindle cap `[(0, −100), (300, −100), (200, 200) bulge 0.8·tan(π/8), (0, 320)]`: the same turns refuse `SheetApexReached { face, point: SheetApexDown }`.

The volume enclosure is symmetric about zero and as wide as the face's whole volume term, so the spindle face's quadrature is not reading the turned frame. The sheet-apex check reads the apex reached where the unturned body's check does not.

r2's differential (756 placements of 14 revolved profiles under random turns): 32 such refusals at PR 7225's merge base, 17 at its head (where the box's refusal now comes first on the rest).
