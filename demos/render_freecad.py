"""Headless FreeCAD renderer for the demo tour (#91 C10).

Runs under `freecadcmd` with QT_QPA_PLATFORM=offscreen — no display,
no Xvfb; `saveImage` goes through Coin's offscreen renderer (the
console's "No valid GL context found!" spam is the interactive
viewport failing, harmlessly). ONE freecadcmd session renders every
scene (startup is ~a minute; per-scene cost is small).

Reads <outdir>/scenes.json (written by the tour). Each scene body is
imported from OUR OWN STEP export where the analytic subset allowed
one (the F6 lane dogfooded end-to-end: export -> OCC import ->
render), else from the STL mesh (curved bodies until the M5 STEP
arms). Camera = the manifest's (elev, azim, up) — matplotlib
`view_init` semantics, so both renderers frame scenes identically.

Usage: QT_QPA_PLATFORM=offscreen freecadcmd render_freecad.py <outdir> <renderdir>
"""

import json
import math
import os
import sys
from pathlib import Path

import FreeCADGui as Gui

Gui.showMainWindow()

import FreeCAD as App  # noqa: E402
import Mesh  # noqa: E402
import Part  # noqa: E402

App.ParamGet("User parameter:BaseApp/Preferences/View").SetBool(
    "UseNavigationAnimation", False
)

WIDTH, HEIGHT = 1200, 900


def camera_rotation(elev, azim, up):
    """FreeCAD Rotation for matplotlib view_init(elev, azim) + up axis.

    Display frame: z vertical. up='y' bodies were authored y-up, so
    display (a, b, c) maps to world (a, c, -b); up='z' is identity.
    """
    el, az = math.radians(elev), math.radians(azim)
    # Camera position direction (scene -> camera) and up, display frame.
    pos_d = (math.cos(el) * math.cos(az), math.cos(el) * math.sin(az), math.sin(el))
    up_d = (0.0, 0.0, 1.0)
    if up == "y":
        to_world = lambda v: App.Vector(v[0], v[2], -v[1])  # noqa: E731
    else:
        to_world = lambda v: App.Vector(v[0], v[1], v[2])  # noqa: E731
    z_cam = to_world(pos_d)  # camera looks along -z_cam
    z_cam.normalize()
    x_cam = to_world(up_d).cross(z_cam)
    x_cam.normalize()
    y_cam = z_cam.cross(x_cam)
    m = App.Matrix(
        x_cam.x, y_cam.x, z_cam.x, 0,
        x_cam.y, y_cam.y, z_cam.y, 0,
        x_cam.z, y_cam.z, z_cam.z, 0,
        0, 0, 0, 1,
    )
    return App.Rotation(m)


def render_scene(scene, outdir, renderdir):
    doc = App.newDocument("scene")
    for body in scene["bodies"]:
        color = tuple(body["color"])
        if body.get("step"):
            before = set(o.Name for o in doc.Objects)
            Part.insert(str(outdir / body["step"]), doc.Name)
            new = [o for o in doc.Objects if o.Name not in before]
        else:
            before = set(o.Name for o in doc.Objects)
            Mesh.insert(str(outdir / body["stl"]), doc.Name)
            new = [o for o in doc.Objects if o.Name not in before]
        for obj in new:
            if hasattr(obj.ViewObject, "ShapeColor"):
                obj.ViewObject.ShapeColor = color
    doc.recompute()
    Gui.updateGui()

    view = Gui.activeDocument().activeView()
    view.setCameraType("Orthographic")
    v = scene["view"]
    rot = camera_rotation(v["elev"], v["azim"], v["up"])
    view.setCameraOrientation(rot.Q)
    Gui.updateGui()
    view.fitAll()
    Gui.updateGui()
    target = renderdir / f"{scene['name']}.png"
    view.saveImage(str(target), WIDTH, HEIGHT, "White")
    print(f"rendered {target}")
    App.closeDocument(doc.Name)
    return target


def main():
    outdir, renderdir = Path(sys.argv[-2]), Path(sys.argv[-1])
    renderdir.mkdir(parents=True, exist_ok=True)
    scenes = json.loads((outdir / "scenes.json").read_text())
    done = []
    for scene in scenes:
        done.append(render_scene(scene, outdir, renderdir))
    missing = [str(p) for p in done if not p.exists()]
    if missing:
        raise SystemExit(f"missing renders: {missing}")
    # Sentinel for render.sh: freecadcmd's Qt teardown can crash after
    # a fully successful run, so exit status alone is not the signal.
    (renderdir / ".freecad_ok").write_text(f"{len(done)}\n")
    print(f"freecad render complete: {len(done)} scenes")
    # Skip FreeCAD/Qt teardown (known offscreen destructor crash).
    sys.stdout.flush()
    os._exit(0)


main()
