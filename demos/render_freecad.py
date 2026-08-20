"""Headless FreeCAD renderer for the demo tour (#91 C10).

Runs under `freecadcmd` with QT_QPA_PLATFORM=offscreen — no display,
no Xvfb; `saveImage` goes through Coin's offscreen renderer (the
console's "No valid GL context found!" spam is the interactive
viewport failing, harmlessly).

Reads <outdir>/scenes.json (written by the tour). TWO source modes,
one per montage lane (#159: "our tessellation vs FreeCAD"):

  mesh         — import each body's STL mesh: the facets on screen are
                 the KERNEL'S OWN tessellation (the kernel montage
                 lane); the default when no mode keyword is given.
                 Since M5 PR 13 every body exports STEP, so the old
                 prefer-STEP rule would have made this lane 100% OCC
                 tessellation — the STL source is now unconditional.
  step         — import each body's OWN STEP export and let OCC
                 re-tessellate it: the FreeCAD/OCC reference lane
                 (export -> OCC import -> render, dogfooded end-to-end).

`scene=NAME` renders exactly one scene, which is how render.sh drives
BOTH lanes: one freecadcmd process per scene under a per-scene budget,
because a warm session that renders many scenes deadlocks partway
through (#224 follow-up). Rendering several scenes in one invocation
still works, for hand runs.

Both selectors are BARE keywords, not --flags: freecadcmd's own option
parser rejects unknown dashed tokens and its --pass passthrough drops
them too (probed on 1.1.2), while bare positionals arrive in sys.argv
untouched. Camera = the manifest's (elev, azim, up), in matplotlib
`view_init` semantics: this renderer and the matplotlib one frame a
scene from one definition of that spec (`manifest`), one of them using
it forwards and this one using the inverse derived from it.

Usage: QT_QPA_PLATFORM=offscreen freecadcmd render_freecad.py [mesh|step] [scene=NAME] <outdir> <renderdir>
"""

import math
import os
import sys
from pathlib import Path

# freecadcmd runs this script with the render staging tree as its cwd,
# so the sibling module it shares with the other readers has to be
# found by this file's own location.
sys.path.insert(0, str(Path(__file__).resolve().parent))

import manifest  # noqa: E402

import FreeCAD as App  # noqa: E402

# THE WEDGE, AND WHY THIS IS THE FIRST THING THIS FILE DOES.
#
# FreeCAD's notification area self-deadlocks under the offscreen
# platform plugin, and that deadlock is the "renders stall at a random
# scene" pathology this repo has worked around for months
# (memories/freecad-render-lane.md: wchan = futex_do_wait, a different
# scene each time, on an idle box as well as a loaded one). Caught in
# the act on a hosted runner (main-thread backtrace, 2026-08-10):
#
#   Gui.updateGui()
#     -> a queued QTimer fires
#     -> Gui::NotificationArea::showInNotificationArea()   TAKES the lock
#     -> NotificationBox::showText -> QWidget::setVisible -> raise()
#     -> QPlatformWindow::raise() warns "This plugin does not support
#        raise()" -- the offscreen plugin cannot raise a window
#     -> FreeCAD routes every Qt message into its own Console
#     -> NotificationAreaObserver::sendLog
#     -> Gui::NotificationArea::pushNotification()         RETAKES it
#     -> non-recursive mutex, same thread: deadlock, forever.
#
# So it needs a pending notification AND a Qt warning emitted while that
# notification is being shown. The tour supplies the first (Part's "STEP
# import is deprecated" warning is itself a notification) and the
# offscreen plugin supplies the second on every single raise() -- which
# is why it is frequent here and unheard-of in an interactive session.
# It is a FreeCAD bug, not a misuse: nothing this script does can make
# re-entering that lock safe.
#
# Disabling the notification area removes the observer that closes the
# loop. Both keys go off: the area itself, and the non-intrusive popup
# whose show() is what emits the fatal warning.
#
# ORDER MATTERS: this runs before FreeCADGui is imported, because the
# notification area is constructed with the main window.
#
# SIDE EFFECT, STATED: FreeCAD parameters are global to the user's
# config, so this turns the notification area off for interactive
# FreeCAD too, on any machine that has run a render. Restoring it is one
# checkbox (Preferences -> General -> Notification Area). Isolating the
# render behind its own user-cfg would avoid that and would make the
# committed pixels independent of a developer's accumulated preferences
# -- worth doing, not done here.
_notify = App.ParamGet("User parameter:BaseApp/Preferences/NotificationArea")
_notify.SetBool("NotificationAreaEnabled", False)
_notify.SetBool("NonIntrusiveNotificationsEnabled", False)

import FreeCADGui as Gui  # noqa: E402

Gui.showMainWindow()

import Mesh  # noqa: E402
import Part  # noqa: E402
from pivy import coin  # noqa: E402

App.ParamGet("User parameter:BaseApp/Preferences/View").SetBool(
    "UseNavigationAnimation", False
)

WIDTH, HEIGHT = 1200, 900


def camera_rotation(scene):
    """FreeCAD Rotation for a scene's matplotlib-style (elev, azim, up).

    The camera is placed in the z-up DISPLAY frame and mapped back to
    world through the scene's own axis spec, so this file states no
    part of the up convention itself — `manifest` defines it in the
    forward direction and derives this one.
    """
    el, az = math.radians(scene.elev), math.radians(scene.azim)
    # Camera position direction (scene -> camera) and up, display frame.
    pos_d = (math.cos(el) * math.cos(az), math.cos(el) * math.sin(az), math.sin(el))
    up_d = (0.0, 0.0, 1.0)
    axes = scene.display_to_world()
    to_world = lambda v: App.Vector(*manifest.apply_axes(axes, v))  # noqa: E731
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

    One document per invocation: per-scene newDocument/closeDocument
    cycling races the (event-loop-deferred) view provider setup
    offscreen — observed as blank frames and hangs — while a single
    warm document with visibility toggling is stable. Isolation
    between scenes is process-level instead: render.sh runs one
    freecadcmd per scene, each with this one warm document.
    Returns {scene name: [objects]}.
    """
    by_scene = {}
    for scene in scenes:
        objs = []
        for body in scene.bodies:
            before = set(o.Name for o in doc.Objects)
            if use_step:
                # No `step is None` guard, deliberately. `step` is
                # nullable in the format and `manifest` says so, but
                # this file's producer is the TOUR (the docstring
                # above says so, and `render.sh` is the only thing
                # that drives it), and the tour fails rather than emit
                # a body without a STEP export. The wild generator's
                # null-STEP manifests are drawn by `render.py`, which
                # never reads the field at all. Guarding here would be
                # a guard against a state this reader's producer
                # cannot emit.
                Part.insert(str(outdir / body.step), doc.Name)
            else:
                Mesh.insert(str(outdir / body.stl), doc.Name)
            new = [o for o in doc.Objects if o.Name not in before]
            for obj in new:
                if hasattr(obj.ViewObject, "ShapeColor"):
                    obj.ViewObject.ShapeColor = body.color
                # Scene-carried transparency (0 = opaque). Set only
                # when asked, so every pre-existing cell's view
                # properties are untouched.
                if body.transparency and hasattr(obj.ViewObject, "Transparency"):
                    obj.ViewObject.Transparency = int(body.transparency)
                obj.ViewObject.Visibility = False
            objs.extend(new)
        by_scene[scene.name] = objs
    doc.recompute()
    Gui.updateGui()
    return by_scene


def render_scene(scene, objs, view, renderdir):
    for obj in objs:
        obj.ViewObject.Visibility = True
    Gui.updateGui()
    rot = camera_rotation(scene)
    # Set the camera node's orientation FIELD directly (pivy):
    # `view.setCameraOrientation` runs the navigation style's ANIMATED
    # transition, and offscreen `saveImage` can capture mid-flight --
    # observed as obliquely-off frames late in a long session.
    cam = view.getCameraNode()
    cam.orientation.setValue(coin.SbRotation(*rot.Q))
    Gui.updateGui()
    view.fitAll()
    Gui.updateGui()
    target = renderdir / f"{scene.name}.png"
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
    pos = [
        a for a in args if a not in ("mesh", "step") and not a.startswith("scene=")
    ]
    outdir, renderdir = Path(pos[-2]), Path(pos[-1])
    renderdir.mkdir(parents=True, exist_ok=True)
    scenes = manifest.read_scenes(outdir)
    if only is not None:
        scenes = [s for s in scenes if s.name == only]
        if not scenes:
            raise SystemExit(f"unknown scene: {only}")
    doc = App.newDocument("scenes")
    by_scene = import_bodies(doc, scenes, outdir, use_step)
    view = Gui.activeDocument().activeView()
    view.setCameraType("Orthographic")
    done = []
    for scene in scenes:
        done.append(render_scene(scene, by_scene[scene.name], view, renderdir))
    missing = [str(p) for p in done if not p.exists()]
    if missing:
        raise SystemExit(f"missing renders: {missing}")
    # No success sentinel: freecadcmd's Qt teardown can crash after a
    # fully successful run, so render.sh judges every scene by its PNG
    # existing, never by the exit status.
    print(f"freecad render complete: {len(done)} scenes")
    # Skip FreeCAD/Qt teardown (known offscreen destructor crash).
    sys.stdout.flush()
    os._exit(0)


main()
