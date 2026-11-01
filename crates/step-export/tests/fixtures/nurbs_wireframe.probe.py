# Per-fixture geometric probe, run by scripts/check_step.sh under
# FreeCAD's headless interpreter with $STEP_FILE set (the generic
# sidecar hook: any <fixture>.probe.py next to a .step is executed
# after the count/volume checks). Exit 0 iff every assertion holds.
#
# What it proves, and why counts alone would not: the fixture's single
# curve is the writer's RATIONAL_B_SPLINE_CURVE complex instance, and
# the control net + weights (1, 1/sqrt 2, 1) are the NURBS Book
# Eq. 7.33 exact quarter circle of radius 1 m about the origin in the
# z = 0 plane. So an INDEPENDENT reader reconstructing the record
# correctly must land every point of that arc on the unit circle. A
# reader that merely parsed the record without honouring the weights
# would trace the control polygon's parabola instead and miss by ~8%.
import math
import os
import sys

import Part  # FreeCAD's Part workbench; freecadcmd puts it on sys.path

shape = Part.Shape()
shape.read(os.environ["STEP_FILE"])

failures = []
if len(shape.Edges) != 1:
    print("  FAIL: expected exactly one edge")
    sys.exit(1)

edge = shape.Edges[0]
curve = edge.Curve
kind = type(curve).__name__
# OCC must have reconstructed a RATIONAL degree-2 B-spline -- not a
# polynomial one (weights dropped) and not a converted analytic circle.
if kind != "BSplineCurve":
    failures.append(f"curve class: expected BSplineCurve, got {kind}")
else:
    if not curve.isRational():
        failures.append("curve came back non-rational: the weights were dropped")
    if curve.Degree != 2:
        failures.append(f"degree: expected 2, got {curve.Degree}")
    if curve.NbPoles != 3:
        failures.append(f"poles: expected 3, got {curve.NbPoles}")
    weights = list(curve.getWeights())
    expected = [1.0, math.sqrt(0.5), 1.0]
    # Exact equality: these are the file's own reals, and the Part 21
    # tokens round-trip to identical bits.
    if weights != expected:
        failures.append(f"weights: expected {expected}, got {weights}")

# FreeCAD works in mm; our file is in metres, so the unit arc is r=1000.
radius_mm = 1000.0
first, last = edge.ParameterRange
worst_radial = 0.0
worst_z = 0.0
for i in range(1001):
    t = first + (last - first) * i / 1000.0
    p = curve.value(t)
    worst_radial = max(worst_radial, abs(math.hypot(p.x, p.y) - radius_mm) / radius_mm)
    worst_z = max(worst_z, abs(p.z))

print(f"  nurbs probe: worst radial rel dev {worst_radial:.3e}, worst |z| {worst_z:.3e} mm, "
      f"length {edge.Length!r} vs exact {math.pi / 2 * radius_mm!r}")

# 1e-12 is three orders of margin over what OCC actually achieves
# (~3.4e-16, i.e. a couple of ulps) and still four orders tighter than
# any approximation of a conic by a non-rational spline could reach.
if worst_radial > 1e-12:
    failures.append(f"radial deviation {worst_radial} exceeds 1e-12 -- not the exact conic")
if worst_z > 1e-9:
    failures.append(f"out-of-plane deviation {worst_z} mm")
if abs(edge.Length - math.pi / 2 * radius_mm) > 1e-6:
    failures.append(f"arc length {edge.Length} is not the quarter circumference")

if failures:
    for f in failures:
        print("  FAIL:", f)
    sys.exit(1)
print("  nurbs probe OK")
