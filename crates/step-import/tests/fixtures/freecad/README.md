# The FreeCAD-authored foreign corpus

STEP files written by **FreeCAD 1.1.2** (AppImage, `freecadcmd`), which
emits through **Open CASCADE 7.8** — the first geometry this kernel
adopts that it did not write. `gen.py` (the `Part`-shape `.exportStep`
path) and `gen_import_export.py` (the `Import.export` path GUI users
hit) are committed beside them as **provenance**: they document which
primitive/boolean/fillet each file is and with what dimensions, so every
expected census and closed-form volume in the suites is derivable from
source rather than from a readback.

Regeneration is **manual and never a test dependency**. The suites read
the committed files, so `cargo test` stays hermetic — FreeCAD is not
needed to run them. (The one optional row that does use FreeCAD, the
re-export oracle, locates `freecadcmd` from the environment and skips
loudly when it is absent.)

To regenerate after a FreeCAD upgrade — and only ever deliberately,
since these files ARE the measured dialect:

```
cd crates/step-import/tests/fixtures/freecad
freecadcmd gen.py
freecadcmd gen_import_export.py
```
