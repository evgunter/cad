# M7-2 acceptance row 7 (the optional oracle row): FreeCAD/OCC reads
# back what this kernel re-exported after adopting a FreeCAD file, and
# says whether it is a valid solid of the expected volume.
#
# Run under FreeCAD's headless interpreter by the `freecad_oracle` test
# in crates/step-import/tests/freecad.rs:
#
#   STEP_FILE=<file.step> EXPECT_SOLIDS=… EXPECT_VOLUME_MM3=… \
#       freecadcmd crates/step-import/tests/oracle/reexport_check.py
#
# The path arrives via $STEP_FILE, never as a positional argument:
# freecadcmd auto-opens positional files it recognizes, which would
# import the file a second time as a document. (This mirrors
# scripts/step_import_check.py, the export-side oracle, which is fenced
# and untouched by this unit; the round trip under test here is a
# different one — FreeCAD -> kernel -> FreeCAD.)
#
# Counts are NOT compared. The kernel's boundary graph legitimately
# differs from OCC's on exactly the shapes M7-2 re-tessellates and on
# the pole/seam edges OCC adds back on import; the invariants that must
# survive a foreign round trip are validity and volume.
import os
import sys

import Part  # FreeCAD's Part workbench; freecadcmd puts it on sys.path

path = os.environ["STEP_FILE"]
shape = Part.Shape()
shape.read(path)

failures = []
print(f"reexport_check: {path}")
print(
    "  valid:", shape.isValid(),
    "solids:", len(shape.Solids),
    "faces:", len(shape.Faces),
    "edges:", len(shape.Edges),
    "volume_mm3:", shape.Volume,
)

if not shape.isValid():
    failures.append("shape.isValid() is False")

want_solids = os.environ.get("EXPECT_SOLIDS")
if want_solids is not None and int(want_solids) != len(shape.Solids):
    failures.append(f"solids: expected {want_solids}, got {len(shape.Solids)}")

vol = os.environ.get("EXPECT_VOLUME_MM3")
if vol is not None:
    rtol = float(os.environ.get("EXPECT_VOLUME_RTOL", "1e-9"))
    expected = float(vol)
    if abs(shape.Volume - expected) > rtol * abs(expected):
        failures.append(
            f"volume_mm3: expected {expected} (rtol {rtol}), got {shape.Volume}"
        )

if failures:
    for f in failures:
        print("  FAIL:", f)
    sys.exit(1)
print("  OK")
