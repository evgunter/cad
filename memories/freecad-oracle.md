---
name: freecad-oracle
description: FreeCAD 1.1.2 headless is installed at ~/.local/share/cad-work/freecad/squashfs-root/usr/bin/freecadcmd — the STEP external-import acceptance oracle
metadata:
  type: reference
---

FreeCAD 1.1.2 (extracted AppImage, checksum-verified; apt only had
0.18 from 2019) is installed at
`~/.local/share/cad-work/freecad/squashfs-root/usr/bin/freecadcmd`
on this machine — Evan approved the install 2026-07-23. It is the
external-import acceptance oracle for STEP export (M4 PR 7's
previously-open caveat, now dischargeable): headless import via a
python script calling `Part.Shape().read(path)`, asserting
validity, solid/shell/face/edge/vertex counts, and volume. Working
example: `~/.local/share/cad-work/freecad/import_check.py`
(smoke-tested on the F6 spike's in-house AP214 cube → VALID, 6/12/8,
volume 1.0). Integration follows the `scripts/check_admesh.sh`
pattern ("admesh pattern, STEP-shaped") — locate via env var with
the above path as default, skip loudly when absent so cargo/gate
stay hermetic.
