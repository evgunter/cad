---
id: the-seam-zips-kef-leaves-the-wall-it-closes-half-minted
kind: issue
title: The boolean's seam zip kills each kept section face into its wall with kef, leaving the wall half-minted until the merge door's mint
status: open
opened: 2026-09-30
priority: P3
cost: M
---

Found by `topo`'s `a-null-edge-that-is-killed-leaves-its-face-half-minted`
(branch `topo/null-kill-remint`), whose measurement it is.

That unit closed the half-minted state the boolean's JOIN left on a
curved wall the section crosses: the join's chord `mef`s now mint the
wall's pieces whole at the mint site (`topo::pcurves::site_rows`), so
the operand clones leave `bool_connect` with every minted face
complete. The next door that half-mints them is the seam zip
(`crates/topo/src/boolean/zip.rs`, `zip_seam`): it closes each seam
with `mekr`/`mef` self-loop chords and `kev_describing`, then
`Body::kef`s the kept section face's run into the wall
(`body.kef(rs[(j + 1) % n])`, and `body.kef(rs[1 % n])` after the
loop). The section face inherits the wall's surface (`finish`'s
`mfkrh`) and stores no row, so its half-edges arrive on the wall
without rows — the `kef` case `PcurveMintError::MissingCache`'s docs
list ("merging a complete face with an unminted one on the same
chart").

**Measured** (probes on every Euler door and on `mint_pcurves`, not
committed; sweep's `ci` profile, at that unit's head): 657 faces reach
a whole-body mint half-minted in 62 tests, every one at
`Body::merge_coplanar_faces_declared`'s re-mint
(`crates/topo/src/merge_faces.rs`, before commit), inside the boolean's
own surgery; the boolean's closing `mint_pcurves`
(`crates/topo/src/boolean/ops.rs`, `boolean_op_recut`) then finds none.
The last door to change each: the zip's two `kef`s, 329 faces, and the
merge's own `kef`s (`merge_faces.rs`), 328, merging faces the zip left
half-minted. The merge door's mint re-derives every one of them, so
nothing leaves the boolean half-minted; the state lasts from the zip to
that mint.

Shape, for whoever takes it: the zip is the door that knows the wall
and the section face are on one chart and that the section face's run
is the seam, so it can re-mint the wall's merged loop at the kill (the
site mint over that loop, as `mef` runs it), or `kef` can take the
site-mint rule for a remnant arriving rowless on a minted face of the
same chart (which is `kef`'s contract, TOPO's ground, and needs a
band). Either way the merge's mint stays the boolean's closing pass
for the faces the zip BUILDS.

## Measured after `kef` took the site-mint rule (2026-09-30, PR 3531)

`kef` now re-mints the surviving face when its remnant arrives without
standing rows — across a chart change, or rowless on the same chart —
and the surviving face was complete
(`topo/loop-reparenting-doors-drop-rows-they-could-now-re-mint-under-decide`,
`Body::plan_moved_rows`): the second shape above. Measured on sweep's
`ci` profile with a probe at the whole-body mint (not committed): on
that PR's merge base the whole-body mints met 659 half-minted faces in
64 tests, 657 of them at the merge door's re-mint
(`merge_faces.rs`); at its head they meet 2, the two doors the null
unit named (`shell9_r2_probes`' launder row and
`m8_4_intersection_iso`'s spline), and none at the merge door. The zip
itself is unchanged; whether this row closes is ZIP's call.
