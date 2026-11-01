---
id: the-mate-tool-offers-two-of-the-five-mate-kinds
kind: issue
title: the mate tool offers two of the kernel's five mate kinds and picks faces only
status: open
opened: 2026-10-31
priority: P0
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
holding several mates is `work/chrome/mate-tool-commits-one-mate.md`.
