# External STEP import acceptance (M4 PR 7) — run under FreeCAD's
# headless interpreter by scripts/check_step.sh:
#
#   STEP_FILE=<file.step> freecadcmd scripts/step_import_check.py
#
# The file path arrives via $STEP_FILE, NOT as a positional argument:
# freecadcmd auto-opens every positional file it recognizes, so a
# positional .step path would be imported a SECOND time as a document —
# duplicate work, and duplicated/confusing logs when a file is corrupt.
#
# Expectations arrive via EXPECT_* environment variables (set by
# check_step.sh from the fixture's .expect sidecar):
#   EXPECT_SOLIDS / EXPECT_SHELLS / EXPECT_FACES / EXPECT_EDGES /
#   EXPECT_VERTICES  — exact integer counts;
#   EXPECT_VOLUME_MM3 — total volume in FreeCAD's internal mm^3
#   (our files are in metres; OCC scales on import), checked to
#   EXPECT_VOLUME_RTOL relative tolerance (default 1e-9).
#
# Exit status: 0 iff the shape imports, is valid, and matches every
# provided expectation.
import os
import sys

import Part  # FreeCAD's Part workbench; freecadcmd puts it on sys.path

path = os.environ["STEP_FILE"]
shape = Part.Shape()
shape.read(path)

failures = []


def expect_count(name, actual):
    raw = os.environ.get("EXPECT_" + name)
    if raw is None:
        return
    if int(raw) != actual:
        failures.append(f"{name.lower()}: expected {raw}, got {actual}")


print(f"step_import_check: {path}")
print(
    "  valid:", shape.isValid(),
    "solids:", len(shape.Solids),
    "shells:", len(shape.Shells),
    "faces:", len(shape.Faces),
    "edges:", len(shape.Edges),
    "vertices:", len(shape.Vertexes),
    "volume_mm3:", shape.Volume,
)

if not shape.isValid():
    failures.append("shape.isValid() is False")
expect_count("SOLIDS", len(shape.Solids))
expect_count("SHELLS", len(shape.Shells))
expect_count("FACES", len(shape.Faces))
expect_count("EDGES", len(shape.Edges))
expect_count("VERTICES", len(shape.Vertexes))

vol = os.environ.get("EXPECT_VOLUME_MM3")
if vol is not None:
    rtol = float(os.environ.get("EXPECT_VOLUME_RTOL", "1e-9"))
    expected = float(vol)
    if abs(shape.Volume - expected) > rtol * abs(expected):
        failures.append(f"volume_mm3: expected {expected} (rtol {rtol}), got {shape.Volume}")

if failures:
    for f in failures:
        print("  FAIL:", f)
    sys.exit(1)
print("  OK")
