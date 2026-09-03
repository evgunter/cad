---
id: validate-tier3-curved-boundary-containment
kind: issue
title: validate tier 3 — face-boundary containment on curved surfaces (the last unmarked deferral in the not-yet-checked list)
status: open
opened: 2026-08-19
github: 638
refs: [226, 635]
---

## From GitHub issue 638

Opened 2026-08-19; 0 comments.

Opened by the **S39/H2 stale-claims lane** (#635) so a genuinely unimplemented item keeps a findable marker. Three prose sites carried "M3, with pcurves" as this item's schedule; M3 shipped long ago, so those labels were stale claims and had to go — but deleting them without replacement would have left a real gap with no marker at all, which is the failure mode S39 exists to hunt. This issue is the replacement marker.

## What is not checked

`topo::validate`'s tier-3 header, under **"What tier 3 does NOT yet check (deferred, named)"**:

> **Face-boundary containment on curved surfaces** — a face's loops actually bounding a region of its surface.

What IS covered today, so the gap is stated precisely:

- **Planar, between vertices** — check 5 (sample containment against adjacent planar faces).
- **Planar, orientation half** — check 6 (loop-role winding against the outward normal, line-bounded loops).
- **Curved analytic, orientation half** — check 6's curved arm (M6-6: boundary material side vs the sense bit).

What remains uncovered:

- containment against **curved** surfaces;
- the **region-bounding** statement for arc-bounded planar faces and for curved faces;
- the curved arm's documented residuals — the rimless sphere band; NURBS faces; the quadrature-owned conic-trimmed walls (whose boundary parse refuses typed and is therefore exempt: such a body's flips, single-face and whole-body, certify green today, executed on the tilted-section cylinder and pinned as residual).

## Why it is a marker and not a plan

No design work is proposed here and none is implied. The item is real, it is unscheduled, and the point of the issue is that it stays findable from the three code sites that cite it rather than dissolving into an undated "deferred".

## Cited from

- `crates/topo/src/validate.rs` — `validate_geometric`'s not-yet-checked list (the bullet above)
- `crates/topo/src/validate.rs` — `ValidationError::PlanarBoundaryResidual`'s doc
- `crates/topo/src/validate.rs` — the tier-3 check-5 rustdoc and its in-body check-list comment

Related but distinct: the **material wedge side** (lamina/zero-volume detection) is a separate bullet in the same list with its own state — since M5 S10 the per-face half is no longer missing; what is absent is the edge-local pairing. Not covered by this issue.

## Home

`work/issues/`: `crates/topo/src/validate.rs` is in no open program's `paths` — S-BOOL's territory is `topo/src/boolean` and `topo/src/splitting`, and S-CERT's stops at `geom-brep/src/props`.
