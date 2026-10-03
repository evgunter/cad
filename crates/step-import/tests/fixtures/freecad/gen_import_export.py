# Probe the Import.export path (what GUI users hit) vs shape.exportStep:
# document objects, named parts, two-body document.
import os

import FreeCAD
import Import
import Part

OUT = os.path.dirname(os.path.abspath(__file__))

doc = FreeCAD.newDocument("m72")
o1 = doc.addObject("Part::Feature", "BoxA")
o1.Shape = Part.makeBox(1, 1, 1)
Import.export([o1], os.path.join(OUT, "box_importexport.step"))

o2 = doc.addObject("Part::Feature", "SphereB")
o2.Shape = Part.makeSphere(0.5, FreeCAD.Vector(3, 0, 0))
Import.export([o1, o2], os.path.join(OUT, "twobody_importexport.step"))
print("import-export done")
