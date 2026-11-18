#!/usr/bin/env python3
"""Open CASCADE's own statement of a solid with cavities.

Written with OCP (`pip install cadquery-ocp`, Open CASCADE 7.8), with
the settings `../occ/gen_occ.py` uses (AP214, no pcurves). Open
CASCADE writes a solid with cavities as one BREP_WITH_VOIDS, each
cavity an ORIENTED_CLOSED_SHELL(.F.) over its shell written reversed;
these files are the importer's check against a writer other than the
kernel's own. Regeneration is manual and never a test dependency; the
suites read the committed files. Units are millimetres.

  * box_cavity.step: [0, 30]^3 minus [10, 20]^3, V = 26000 mm^3.
  * two_cavities.step: [0, 30]^3 minus [5, 10]^3 and [20, 25]^3,
    V = 27000 - 2 * 125 = 26750 mm^3.
  * cylinder_sphere_cavity.step: a cylinder of radius 8 and height 12
    on the z axis minus a ball of radius 3 at its centre,
    V = pi (8^2 * 12 - 4/3 * 3^3) = 732 pi mm^3.

Run: python3 gen_occ_voids.py   (cwd = this directory)
"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt
from OCP.Interface import Interface_Static
from OCP.STEPControl import STEPControl_AsIs, STEPControl_Controller, STEPControl_Writer

STEPControl_Controller.Init_s()
Interface_Static.SetCVal_s("write.step.schema", "AP214IS")
Interface_Static.SetIVal_s("write.surfacecurve.mode", 0)


def out(shape, name):
    w = STEPControl_Writer()
    w.Transfer(shape, STEPControl_AsIs)
    w.Write(name)


def box(a, b):
    return BRepPrimAPI_MakeBox(gp_Pnt(*a), gp_Pnt(*b)).Shape()


def cut(a, b):
    return BRepAlgoAPI_Cut(a, b).Shape()


block = box((0, 0, 0), (30, 30, 30))
out(cut(block, box((10, 10, 10), (20, 20, 20))), "box_cavity.step")
out(
    cut(cut(block, box((5, 5, 5), (10, 10, 10))), box((20, 20, 20), (25, 25, 25))),
    "two_cavities.step",
)
cylinder = BRepPrimAPI_MakeCylinder(8.0, 12.0).Shape()
ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 6), 3.0).Shape()
out(cut(cylinder, ball), "cylinder_sphere_cavity.step")
