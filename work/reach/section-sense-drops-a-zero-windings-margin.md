---
id: section-sense-drops-a-zero-windings-margin
kind: issue
title: topo: section_sense refuses a zero winding with no margin, and conflates it with a carrier that has no winding
status: open
opened: 2026-09-30
---

(TOPO, the D262 unit's sweep, PR 3532: the same shape as the merge's
outline refusal that unit repaired, on reach's ground.)

## What

`splitting::finish::section_sense` (`crates/topo/src/splitting/finish.rs`)
reads a section face's loop winding through `Body::planar_loop_winding`
and refuses on

```rust
Some(Ok(geom_core::Sign::Zero)) | None => {
    Err(SplitFinishError::SectionWindingUndecided { face, diag: None })
}
```

- A ZERO winding is band-decided: its margin lies within the zero band
  and a smaller tolerance may decide it. The refusal carries
  `diag: None`, so it cannot quote the margin or offer that tolerance
  (D4 ¶1 (i)/(iv)).
- It shares one spelling with `None`, a loop with no winding the kernel
  reads (a NURBS or spiric carrier, a null-edge scaffold), which no
  tolerance decides.

`Body::planar_loop_winding_decided` (`crates/topo/src/loop_winding.rs`,
added by PR 3532) keeps the margin a sign was decided on
(`decide_reported`), which is what a zero verdict's refusal needs.

## Repair shape

Read the decided winding, carry a zero verdict's margin into the
refusal's diagnostics (the way `merge_faces::merged_outline_ring` now
ends a zero winding through `LOOP_WINDING`), and give the unread
carrier its own arm.
