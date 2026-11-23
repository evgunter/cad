---
id: blend-split-rows-and-must-carry-prose-residue
kind: issue
title: blend/sweep: residue of PRs 3667 and 3670 (split-row source spellings; prose copies of the must-carry mapping)
status: open
opened: 2026-10-01
priority: P4
cost: E
---


From the delta reviews of PRs #3670 and #3667 (2026-10-01).

- **Two spellings of a split row's source half.** `rim_phase` (ladder, `crates/sweep/src/blend/surgery.rs`) names `meridian_splits` after `m`, the key it split; `rim_phase_annulus` now names `frag.source`, pinned by `blend_tworims::every_band_crossing_names_the_seam_its_foot_split`. The ladder's comment calls naming the original "an unpinned choice", a reason the annulus no longer shares. Pick one and pin it for both.
- **Prose copies of the must-carry mapping.** `MustCarryVerdict::description` (`crates/geom-brep/src/dihedral.rs`) is the code home; `extrude.rs` module steps 4 and 6 and the `description` doc restate the verdict list, and the rim rule's K crossover is stated in both `lib.rs` and `upgrade_rim`'s arm comment, with `Extruded::body` → crate docs → step 6 → the arm pointing round. One home, pointers elsewhere.
- **`description(s1, s2, witness)` takes the pair separately from `must_carry_over_edge(surf1, surf2, …)`**; "in its order" is held by a doc sentence only.

## Re-read (2026-10-26, tracker sweep)

Two of the three bullets are done on main; the prose-copies bullet
stands.
- **Split-row source spelling: done.** Both phases now record the
  fragment's source: `rim_phase` pushes `frag.source` from
  `split_fragment` (`crates/sweep/src/blend/surgery.rs:4069-4071`, its
  comment naming the original source "the way the annulus phase names
  its seam splits"), and `rim_phase_annulus` pushes `f.source`
  (`surgery.rs:5220-5222`). The "unpinned choice" comment is gone.
- **`description`'s pair: done.** `MustCarryVerdict::description` takes
  `pair: SurfacePair` (`crates/geom-brep/src/dihedral.rs:1174`), so no
  doc sentence holds the order.
- **Prose copies: stands.** `crates/sweep/src/extrude.rs:37-48` (step 4)
  and `:52-66` (step 6) and `crates/sweep/src/lib.rs:78-108` still
  restate the verdict list beside `MustCarryVerdict::description`'s doc.
