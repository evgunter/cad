#!/usr/bin/env python3
"""Open CASCADE's own statement of a sphere face that reaches a pole.

Written with OCP (`pip install cadquery-ocp==7.8.1.1.post1`, Open
CASCADE 7.8.1), NOT with FreeCAD: FreeCAD was not runnable where these
were made. FreeCAD's `exportStep` writes through the same
`STEPControl_Writer`, and the settings below are the ones that make it
write FreeCAD's dialect (AP214, no pcurves: the committed `../freecad/`
corpus carries no SEAM_CURVE or SURFACE_CURVE). Regeneration is manual
and never a test dependency; the suites read the committed files.

  * cap.step: a R = 10 mm ball with everything below z = R sin 0.5 cut
    away. OCC states the cap face as its closed rim plus ONE seam
    meridian from the rim vertex to a pole vertex of valence 1, used
    twice by the face; no degenerate pole edge.
  * side_cut.step: the same ball with the cap x > 8 cut away. The cut
    crosses OCC's seam, so the sphere face's loop is the cut circle (two
    arcs) with a seam piece from each of its two seam crossings to a
    pole vertex of valence 1, one per pole.
  * side_cut_ring.step: the ball with the cap y > 8 cut away, which
    misses the seam. OCC states the sphere face as a VERTEX_LOOP plus
    the cut circle as a second bound: a sphere with a hole and no edge
    at its poles.

Run: python3 gen_occ.py   (cwd = this directory)
"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt
from OCP.Interface import Interface_Static
from OCP.STEPControl import STEPControl_AsIs, STEPControl_Controller, STEPControl_Writer
import math

STEPControl_Controller.Init_s()
Interface_Static.SetCVal_s("write.step.schema", "AP214IS")
Interface_Static.SetIVal_s("write.surfacecurve.mode", 0)


def out(shape, name):
    w = STEPControl_Writer()
    w.Transfer(shape, STEPControl_AsIs)
    w.Write(name)


R = 10.0
ball = BRepPrimAPI_MakeSphere(R).Shape()


def cut(a, b):
    return BRepAlgoAPI_Cut(ball, BRepPrimAPI_MakeBox(gp_Pnt(*a), gp_Pnt(*b)).Shape()).Shape()


out(cut((-20, -20, -20), (20, 20, R * math.sin(0.5))), "cap.step")
out(cut((8, -20, -20), (20, 20, 20)), "side_cut.step")
out(cut((-20, 8, -20), (20, 20, 20)), "side_cut_ring.step")
