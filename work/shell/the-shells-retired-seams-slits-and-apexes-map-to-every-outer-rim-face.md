---
id: the-shells-retired-seams-slits-and-apexes-map-to-every-outer-rim-face
kind: issue
title: The shell's lineage maps a retired seam, slit or apex to every outer rim face
status: open
opened: 2026-10-12
priority: P3
cost: E
---

## What

`Shelled::lineage` (`crates/topo/src/shell.rs`) divides each seam,
slit and apex the chart reduction retired (`ShellNaming::dead`) into
every outer rim face of the body, its rest removed, because
`ShellNaming` does not say which chart a retired edge or vertex
belonged to. The census confirm still chooses among those faces, so a
record lands only where the touch is, but the lineage overstates the
images: a record on a dead seam of one chart is put to every other
chart's rim as well, and the division names no minted cell.

## Fix

Record the chart each retired edge and vertex belonged to in
`ShellNaming` (a row per retired cell naming its rim), and divide it
into that rim's faces alone. Row: a body with two designated charts, a
record on one chart's retired seam, asserting the lineage names that
chart's rim only.

Moved from `work/contacthold/` to `work/shell/` on 2026-11-25, when CONTACTHOLD closed (`docs/doc-ledger/contacthold-leaves-the-tracker.md`); id unchanged. The cleanup re-read the row against main at `6003877ad` and found it live.
