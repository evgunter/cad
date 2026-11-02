---
id: the-mate-tool-offers-two-of-the-five-mate-kinds
kind: issue
title: the mate tool offers two of the kernel's five mate kinds and picks faces only
status: closed
opened: 2026-10-31
priority: P0
cost: H
branch: placing/mate-kinds
pr: 6284
closed: 2026-11-02
---


## Finding

The kernel's mate is one of A11's five primitives
(`crates/editor-core/ASSEMBLY.md`, "A `Mate { on, to }` equates two poses
of one kind"): `Frame`–`Frame` coincides, `Axis`–`Axis` is coaxial,
`Plane`–`Plane` rests, `Point`–`Point` is a ball, `Direction`–`Direction`
is parallel. Its poses read more than faces (`PoseDef`,
`crates/editor-core/src/pose.rs`): an axis off a face's carrier **or a
straight or circular edge**, a point off a carrier's centre **or a
vertex**, a frame `Through` an axis and a point, and a frame at a body's
corner (`corner_frame`, `crates/editor-core/src/mate/corner.rs`).

The viewer's mate tool offers two kinds, `plane` and `axis`
(`matetool::MATE_KINDS`, `crates/viewer/src/matetool.rs`), and reads
every pick as a face (`matetool`'s pose reading: `PoseDef::Plane { face }`,
`PoseDef::Axis { of: face }`). So through the GUI a person cannot:

- make two corners coincide (`Frame`–`Frame` over `corner_frame`), which
  is the one-mate answer to Ev's "align the corners of two stacked
  things" (2026-10-31);
- make two edges collinear (`Axis`–`Axis` on edge lines);
- put a vertex on a vertex (`Point`–`Point`), or keep two directions
  parallel (`Direction`–`Direction`).

Ev assumed the GUI already offered every kind the kernel has; it does
not, and no row said so.

## Scope

Every mate kind the kernel has, with the picks its poses read: edge and
vertex picks beside face picks, and the corner frame. How several mates
compose in one placement is the kernel's
`work/msolve/a-consistent-second-mate-after-a-rest-is-refused.md`; the mate tool's draft
holding several mates is `work/placing/mate-tool-commits-one-mate.md`.

## Closed (2026-11-02, PR 6284)

**What landed.** The mate tool offers plane, axis, point and direction, read off faces, edges and vertices:
- **Picking:** vertex picking is new (`VertexSelection`, `PickKinds::WithVertices`). The cursor's pick rule follows the chosen kind, derived from what its seats read.
- **Readings:** which entity a pose reads is the kernel's (`SlotKind::selection_kind`), and what a kind leaves is the kernel's (`SubgroupFamily::leaves`).
- **Sense:** each kind opens at its own sense, with no plane rest carried onto an edge mate.
- **Review:** single FULL. Its nine probe rows all passed, covering a slid and turned copy, vertex occlusion and a held vertex across a fillet. The fix pass took MINOR-1 (the sense default) and S1/S3/S4/S5 plus the stale prose.

**Not in this row's close, and scheduled elsewhere:**
- **`Frame`–`Frame` (corners):** a frame side is two picks. It enters with the placement editor (`mate-tool-commits-one-mate`, Decided: a frame side is an edge seat and a vertex seat).
- **The P2 ride-along** `instance-value-rows-do-not-say-their-axis`: the axis lives in the bundle, not the slot id. It stays open on its own.
- **A point pick that falls back to an edge or face** commits a mate that cannot evaluate (MINOR-2). It is noted on the editor row.

**Filed:**
- `a-direction-mate-on-a-curved-face-reads-its-plane` (P3);
- `a-drawn-vertex-with-no-name-is-left-out-of-the-mark-table` (P3);
- CHROME's `a-checks-cell-for-a-vertex-selects-its-node` (P3);
- GREEN's `the-slow-set-name-check-cannot-see-app-feature-tests` (P4);
- evidence on VSEAM's `face-selection-carries-a-bare-stable-name` (a third identical bare-name selection type).
