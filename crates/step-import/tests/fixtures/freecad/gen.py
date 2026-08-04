# M7-2 substrate: author solids with FreeCAD 1.1.2 Part primitives and
# export each as STEP so the emitted dialect can be measured.
# Run: freecadcmd gen.py   (cwd = this directory)
import os

import FreeCAD
import Part

OUT = os.path.dirname(os.path.abspath(__file__))


def exp(shape, name):
    path = os.path.join(OUT, name + ".step")
    shape.exportStep(path)
    print("wrote", name, "solids", len(shape.Solids), "faces", len(shape.Faces),
          "edges", len(shape.Edges), "vol", shape.Volume)


# 1. box 1x1x1 (FreeCAD units are mm)
box = Part.makeBox(1, 1, 1)
exp(box, "box")

# 2. cylinder r=0.5 h=1
cyl = Part.makeCylinder(0.5, 1)
exp(cyl, "cylinder")

# 3a. truncated cone r1=1 r2=0.5 h=1
exp(Part.makeCone(1, 0.5, 1), "cone_trunc")

# 3b. apex cone r1=1 r2=0 h=1
exp(Part.makeCone(1, 0, 1), "cone_apex")

# 4. sphere r=1
exp(Part.makeSphere(1), "sphere")

# 5. torus R=1 r=0.25
exp(Part.makeTorus(1, 0.25), "torus")

# 6. box minus cylinder: 2x2x1 box, through-hole r=0.5 at center
b = Part.makeBox(2, 2, 1)
h = Part.makeCylinder(0.5, 1, FreeCAD.Vector(1, 1, 0))
exp(b.cut(h), "box_hole")

# 7. fuse of two boxes (offset so the union is non-trivial)
b1 = Part.makeBox(1, 1, 1)
b2 = Part.makeBox(1, 1, 1, FreeCAD.Vector(0.5, 0.5, 0))
exp(b1.fuse(b2), "fuse_boxes")

# 8. filleted box edge: fillet one edge of a unit box, r=0.25
fb = Part.makeBox(1, 1, 1)
# pick the vertical edge at x=0,y=0
edges = [e for e in fb.Edges
         if all(abs(v.Point.x) < 1e-9 and abs(v.Point.y) < 1e-9
                for v in e.Vertexes)]
exp(fb.makeFillet(0.25, edges), "box_fillet_edge")

# 9. spherical fillet corner: fillet the three edges meeting at the
# origin corner of a unit box -> spherical corner patch
cb = Part.makeBox(1, 1, 1)
corner = FreeCAD.Vector(0, 0, 0)
cedges = [e for e in cb.Edges
          if any(v.Point.distanceToPoint(corner) < 1e-9 for v in e.Vertexes)]
exp(cb.makeFillet(0.25, cedges), "box_fillet_corner")

# 10. two-solid compound
comp = Part.makeCompound([Part.makeBox(1, 1, 1),
                          Part.makeSphere(0.5, FreeCAD.Vector(3, 0, 0))])
exp(comp, "compound_two")

print("gen.py done")
