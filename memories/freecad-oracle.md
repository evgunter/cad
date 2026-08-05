---
name: freecad-oracle
description: FreeCAD 1.1.2 headless is installed at ~/.local/share/cad-work/freecad/squashfs-root/usr/bin/freecadcmd — the acceptance oracle for STEP export AND (since M7) the foreign-corpus source + volume oracle for STEP import
metadata:
  type: reference
---

FreeCAD 1.1.2 (extracted AppImage, checksum-verified; apt only had
0.18 from 2019) is installed at
`~/.local/share/cad-work/freecad/squashfs-root/usr/bin/freecadcmd`
on this machine — Evan approved the install 2026-07-23. It is the
external-import acceptance oracle for STEP export: headless import
via `Part.Shape().read(path)`, asserting validity,
solid/shell/face/edge/vertex counts, and volume. Integration is
DONE and canonical in-repo (admesh pattern, STEP-shaped):
`scripts/check_step.sh` + `scripts/step_import_check.py` — locate
via env var with the above path as default, skip loudly when
absent so cargo stays hermetic. Hosted CI's step-import job
installs its own checksum-verified FreeCAD 1.1.2 AppImage (#94),
version-matched to this local oracle — keep the versions in sync
if either side upgrades.

Since M7 it is ALSO the STEP-import oracle: the M7-2 foreign corpus
is 13 FreeCAD-authored fixtures (mm units, base cones, vertex-loop
sphere, structure roots) that `crates/step-import` must adopt
first-class, with FreeCAD's own volumes as the acceptance
comparison (#189).
