"""Headless FreeCAD renderer for the demo tour (#91 C10).

Runs under `freecadcmd` with QT_QPA_PLATFORM=offscreen — no display,
no Xvfb; `saveImage` goes through Coin's offscreen renderer (the
console's "No valid GL context found!" spam is the interactive
viewport failing, harmlessly). ONE freecadcmd session renders every
scene (startup is ~a minute; per-scene cost is small).

Reads <outdir>/scenes.json (written by the tour). TWO source modes,
one per montage lane (#159: "our tessellation vs FreeCAD"):

  default      — import each body's STL mesh: the facets on screen are
                 the KERNEL'S OWN tessellation (the kernel montage lane).
                 Since M5 PR 13 every body exports STEP, so the old
                 prefer-STEP rule would have made this lane 100% OCC
                 tessellation — the STL source is now unconditional.
  step         — import each body's OWN STEP export and let OCC
                 re-tessellate it: the FreeCAD/OCC reference lane
                 (export -> OCC import -> render, dogfooded end-to-end).

`scene=NAME` renders exactly one scene (the STEP lane drives one
freecadcmd process per scene with a timeout — a stalled import then
costs one cell, not the montage). Both selectors are BARE keywords,
not --flags: freecadcmd's own option parser rejects unknown dashed
tokens and its --pass passthrough drops them too (probed on 1.1.2),
while bare positionals arrive in sys.argv untouched. Camera = the
manifest's (elev, azim, up) — matplotlib `view_init` semantics, so
all renderers frame scenes identically.

Usage: QT_QPA_PLATFORM=offscreen freecadcmd render_freecad.py [step] [scene=NAME] <outdir> <renderdir>
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
from pivy import coin  # noqa: E402

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
    up_w = to_world(up_d)
    if abs(up_w.dot(z_cam)) > 0.9999:  # straight-down views: fall back
        up_w = to_world((0.0, 1.0, 0.0))
    x_cam = up_w.cross(z_cam)
    x_cam.normalize()
    y_cam = z_cam.cross(x_cam)
    m = App.Matrix(
        x_cam.x, y_cam.x, z_cam.x, 0,
        x_cam.y, y_cam.y, z_cam.y, 0,
        x_cam.z, y_cam.z, z_cam.z, 0,
        0, 0, 0, 1,
    )
    return App.Rotation(m)


def import_bodies(doc, scenes, outdir, use_step):
    """Import every scene's bodies into ONE document, hidden.

    One document for the whole session: per-scene newDocument/
    closeDocument cycling races the (event-loop-deferred) view
    provider setup offscreen — observed as blank frames and hangs —
    while a single warm document with visibility toggling is stable.
    (The STEP lane still gets process-level isolation: render.sh runs
    one freecadcmd per scene, each with this one warm document.)
    Returns {scene name: [objects]}.
    """
    by_scene = {}
    for scene in scenes:
        objs = []
        for body in scene["bodies"]:
            before = set(o.Name for o in doc.Objects)
            if use_step:
                if not body.get("step"):
                    raise SystemExit(
                        f"scene {scene['name']}: no STEP export for {body['stl']}"
                    )
                Part.insert(str(outdir / body["step"]), doc.Name)
            else:
                Mesh.insert(str(outdir / body["stl"]), doc.Name)
            new = [o for o in doc.Objects if o.Name not in before]
            for obj in new:
                if hasattr(obj.ViewObject, "ShapeColor"):
                    obj.ViewObject.ShapeColor = tuple(body["color"])
                obj.ViewObject.Visibility = False
            objs.extend(new)
        by_scene[scene["name"]] = objs
    doc.recompute()
    Gui.updateGui()
    return by_scene


def render_scene(scene, objs, view, renderdir):
    for obj in objs:
        obj.ViewObject.Visibility = True
    Gui.updateGui()
    v = scene["view"]
    rot = camera_rotation(v["elev"], v["azim"], v["up"])
    # Set the camera node's orientation FIELD directly (pivy):
    # `view.setCameraOrientation` runs the navigation style's ANIMATED
    # transition, and offscreen `saveImage` can capture mid-flight --
    # observed as obliquely-off frames late in a long session.
    cam = view.getCameraNode()
    cam.orientation.setValue(coin.SbRotation(*rot.Q))
    Gui.updateGui()
    view.fitAll()
    Gui.updateGui()
    target = renderdir / f"{scene['name']}.png"
    view.saveImage(str(target), WIDTH, HEIGHT, "White")
    for obj in objs:
        obj.ViewObject.Visibility = False
    Gui.updateGui()
    print(f"rendered {target}")
    return target


def main():
    args = sys.argv[1:]
    use_step = "step" in args
    only = next((a.split("=", 1)[1] for a in args if a.startswith("scene=")), None)
    pos = [a for a in args if a != "step" and not a.startswith("scene=")]
    outdir, renderdir = Path(pos[-2]), Path(pos[-1])
    renderdir.mkdir(parents=True, exist_ok=True)
    scenes = json.loads((outdir / "scenes.json").read_text())
    if only is not None:
        scenes = [s for s in scenes if s["name"] == only]
        if not scenes:
            raise SystemExit(f"unknown scene: {only}")
    doc = App.newDocument("scenes")
    by_scene = import_bodies(doc, scenes, outdir, use_step)
    view = Gui.activeDocument().activeView()
    view.setCameraType("Orthographic")
    done = []
    for scene in scenes:
        done.append(render_scene(scene, by_scene[scene["name"]], view, renderdir))
    missing = [str(p) for p in done if not p.exists()]
    if missing:
        raise SystemExit(f"missing renders: {missing}")
    # Sentinel for render.sh (full kernel-lane pass only): freecadcmd's
    # Qt teardown can crash after a fully successful run, so exit
    # status alone is not the signal. Single-scene STEP-lane runs are
    # judged by their PNG existing instead.
    if only is None and not use_step:
        (renderdir / ".freecad_ok").write_text(f"{len(done)}\n")
    print(f"freecad render complete: {len(done)} scenes")
    # Skip FreeCAD/Qt teardown (known offscreen destructor crash).
    sys.stdout.flush()
    os._exit(0)


main()
