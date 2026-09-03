---
id: concave-closed-rim-has-no-band
kind: issue
title: fillet: a CONCAVE closed rim has no band — the closed-rim carve is convex-only, and the seam-vertex tag fires on both sides
status: open
opened: 2026-08-29
github: 1244
refs: [1222]
---

## From GitHub issue 1244

opened 2026-08-29, 0 comments.

Surfaced by the BLEND-1 review (PR #1222), by execution on two independent bodies.

**The shape.** Every closed-rim carve in `crates/sweep/src/fillet/surgery.rs` — the quad LADDER and the ANNULUS alike, and now the annulus's multi-link seam-split arm — gates on convexity in `resolve_rim`:

```
"a concave chain adds material, which the surgery does not build — not implemented"
```

A concave rim's blend ADDS material, which is a different surgery from the material-removing carve the module builds: the band is still a torus about the same spine, but the strips it replaces run the other way and the excise/merge sequence is not the same walk.

**Why it is filed now rather than left implicit.** The `CornerConfig::SeamVertex` tag's firing condition (`battery.rs`'s `is_seam_vertex`) is purely INCIDENCE — two rim arcs carrying one support pair, plus two co-surface seam meridians — and never reads convexity. So a chain stopping at a **concave** seam-split rim's vertex is tagged `SeamVertex` exactly as a convex one is, and is handed `FILLET3_SEAM_VERTEX_RECOURSE`. The recourse therefore has to be true on both material sides, which is why it names the request unconditionally and conditions the CARVE on the convex side (BLEND-1's fix pass). Until this issue closes, the honest sentence has a hedge in it, and that hedge is what this issue would remove.

**Measured.** A waisted pole-touching revolve (two cones meeting at radius 0.5) and the tour's lily lantern both show it: the lantern's fourth transverse rim (carrier radius `0.253`) refuses `"a concave chain adds material"` while its other three carve. Pinned live at:

- `crates/sweep/tests/blend_seam_split_rim.rs::a_concave_seam_split_rim_still_refuses` (the gate),
- `…::the_waisted_bodys_convex_rims_carve_so_its_concave_row_is_not_vacuous` (the fixture reaches the door),
- `crates/sweep/tests/review_blend1_r2_probes.rs::the_seam_vertex_recourse_is_true_at_every_site_the_tag_fires` (the composed honesty pin: the recourse's promise and the whole-rim answer, on BOTH material sides),
- `demos/tour/tests/blend1_r1_wall6_probes.rs` (the lantern's fourth rim).

**What would close it**: the material-adding closed-rim band — the convexity gate in `resolve_rim` relaxed, with the excise/merge walk's orientation-agnosticism established rather than assumed, and a concave fixture with its own closed form. Note the adjacent convexity work already scheduled: the concave plane-plane chamfer and the convexity-parametric fillet corner are separate units and neither of them opens this door.

**Consumers**: the tour's lily lantern (the waist of any flower-like revolve), and any solid of revolution with a waist or a fillet-into-a-pocket rim.

## Home

`work/issues/` — `crates/sweep/src/fillet/*` was S-BLEND's ground and that program is closed; VERBS cedes fillet band/surgery explicitly in its keep_out.
