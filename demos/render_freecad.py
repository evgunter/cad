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

`scene=NAME` restricts the pass to that scene; REPEAT it to render a
BATCH of scenes in one process (`scene=a scene=b scene=c`). Whatever
order the keywords come in, the scenes are rendered in scenes.json
order, so a batch's frames do not depend on how it was spelled.

That is how render.sh drives BOTH lanes: `CAD_RENDER_BATCH` scenes per
freecadcmd process (default 1 — one process per scene, as it has been
since #224) under a budget of that many per-scene budgets. The warm
session that deadlocked partway through in #224 was root-caused in
2026-08 to FreeCAD's notification area re-entering its own mutex under
the offscreen plugin, and is disabled below; the process boundary is
kept as what BOUNDS a future hang, which is why the batch is a knob
with a small default and not "render everything in one process".

Both selectors are BARE keywords, not --flags: freecadcmd's own option
parser rejects unknown dashed tokens and its --pass passthrough drops
them too (probed on 1.1.2), while bare positionals arrive in sys.argv
untouched. Camera = the manifest's (elev, azim, up), in matplotlib
`view_init` semantics: this renderer and the matplotlib one frame a
scene from one definition of that spec (`manifest`), one of them using
it forwards and this one using the inverse derived from it.

Usage: QT_QPA_PLATFORM=offscreen freecadcmd render_freecad.py [mesh|step] [scene=NAME]... <outdir> <renderdir>
"""

import math
import os
import sys
import traceback
from pathlib import Path

# freecadcmd runs this script with the render staging tree as its cwd,
# so the sibling module it shares with the other readers has to be
# found by this file's own location.
sys.path.insert(0, str(Path(__file__).resolve().parent))

import FreeCAD as App

import manifest

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

import FreeCADGui as Gui  # noqa: E402 — after the wedge above, which must run before Gui loads

Gui.showMainWindow()

# The E402 markers on all three: the GUI must exist before they import.
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
    # The E731 marker below: a one-expression rebasing of a triple into the
    # scene's world axes, used three lines down and nowhere else; a `def`
    # would separate it from the `axes` it closes over.
    to_world = lambda v: App.Vector(*manifest.apply_axes(axes, v))  # noqa: E731
    z_cam = to_world(pos_d)  # camera looks along -z_cam
    z_cam.normalize()
    up_w = to_world(up_d)
    # Straight-down views (elev = +-90): the display up vector and the
    # camera direction are parallel, so `up_w.cross(z_cam)` has no
    # length and any x_cam would do; pick the display y axis.
    #
    # The threshold is written in WORLD coordinates and names no `up`.
    # That is sound because the axis specs are isometries, so the
    # quantity here is the display-frame sin(elev) whatever the axis --
    # `manifest`'s selftest pins that property, which is as close as
    # anything gets to guarding this branch: NOTHING EXECUTES IT
    # without FreeCAD, and no lane in CI runs this file outside a full
    # render. The two committed scenes at elev 90
    # (`twisted_duct_shadow_z`, `silhouette3_shadow_z`) are what
    # exercise it, and they only do so in a render pass.
    if abs(up_w.dot(z_cam)) > 0.9999:
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
    skipped = {}
    for scene in scenes:
        objs = []
        for body in scene.bodies:
            before = set(o.Name for o in doc.Objects)
            if use_step:
                # `step` is nullable in the format, and since the tour
                # grew `SceneBody::step_at_frontier` its producer CAN
                # emit one: a body past the STEP writer's named subset
                # frontier (a multi-shell curved solid, whose
                # outward/void classifier has closed forms for planar
                # faces only) has no STEP to import. This lane's whole
                # subject is OCC re-tessellating OUR STEP, so there is
                # nothing here for it to say about such a body, and
                # substituting the STL would put a cell in this
                # montage that LOOKS like OCC evidence and is none.
                # So the body is skipped and named; its scene still
                # renders, from whatever else it carries.
                if body.step is None:
                    print(
                        f"skipped {body.stl} in scene {scene.name!r}: "
                        "no STEP (the writer's named subset frontier)"
                    )
                    skipped.setdefault(scene.name, 0)
                    skipped[scene.name] += 1
                    continue
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
    return by_scene, skipped


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
    # Every scene= keyword, not just the first: one of them is a scene,
    # several are a BATCH. The selection filters the manifest's own
    # list, so the render order is scenes.json order however the batch
    # was spelled -- a batch's frames must not depend on argv order.
    only = [a.split("=", 1)[1] for a in args if a.startswith("scene=")]
    pos = [
        a for a in args if a not in ("mesh", "step") and not a.startswith("scene=")
    ]
    outdir, renderdir = Path(pos[-2]), Path(pos[-1])
    renderdir.mkdir(parents=True, exist_ok=True)
    scenes = manifest.read_scenes(outdir)
    if only:
        wanted = list(dict.fromkeys(only))
        scenes = [s for s in scenes if s.name in set(wanted)]
        unknown = [n for n in wanted if n not in {s.name for s in scenes}]
        if unknown:
            raise SystemExit(f"unknown scene(s): {', '.join(unknown)}")
    doc = App.newDocument("scenes")
    by_scene, skipped = import_bodies(doc, scenes, outdir, use_step)
    # A scene every one of whose bodies was skipped renders BLANK, and a
    # blank cell is this lane's known crash signature. So it gets a
    # sidecar note, the same way a missing render gets a `.fail.txt`,
    # and `compose_montage` stamps the cell neutrally: a declared gate
    # must not be indistinguishable from a wedge.
    for name, n in skipped.items():
        if not by_scene[name]:
            (renderdir / f"{name}.note.txt").write_text(
                "no STEP — declared writer frontier\n"
                if n == 1
                else f"no STEP for any of {n} bodies — declared writer frontier\n"
            )
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


# WHY main() IS NOT CALLED BARE, AND WHY THE PRINT IS HERE.
#
# freecadcmd SWALLOWS the traceback of anything this script raises: it
# reports only "Unknown exception while processing file" and then treats
# whatever argv is left as documents to open ("File format not supported:
# .." — a symptom of the swallow, not the cause). Worse, the failure path
# then runs FreeCAD's document teardown, which SIGSEGVs offscreen
# (closeAllDocuments -> slotDeleteDocument -> setActiveDocument ->
# runString -> PyException::PyException) — it crashes while building the
# object that would have described the error. Ten render-lane failures
# were undiagnosable for exactly this reason.
#
# So the traceback is printed HERE, at the innermost point that still has
# it, and FLUSHED before the exception is allowed to continue outwards:
# anything that defers output until interpreter shutdown or teardown may
# never run at all. Then the exception is RE-RAISED unchanged — this
# handler makes the failure visible, it does not change what failing
# means, and it deliberately does not os._exit() over the teardown crash,
# which would hide a second, real bug.
#
# BaseException, not Exception: the script's own errors are SystemExit,
# and those are swallowed by freecadcmd just as thoroughly.
#
# The success path never reaches this: main() ends in os._exit(0).
def _print_traceback():
    """Put the current exception's traceback on stderr, NOW.

    Every arm below is blind and silent on purpose — that is what the
    noqa markers claim and this is the claim: it is the last thing that
    will ever describe this failure, and a reporter that raises on its
    way out reports nothing at all.
    """
    try:
        text = traceback.format_exc()
    except Exception:  # noqa: BLE001 — formatting failed; say SOMETHING anyway
        text = "render_freecad: exception whose traceback could not be formatted\n"
    try:
        # stdout first, so the per-scene log keeps its ordering.
        sys.stdout.flush()
    except Exception:  # noqa: BLE001, S110 — ordering is a nicety, the text is not
        pass
    try:
        sys.stderr.write(text)
        sys.stderr.flush()
        return
    except Exception:  # noqa: BLE001, S110 — fall through to the raw fd
        pass
    # Last resort: the raw fd, which has no Python-level buffer to lose.
    try:
        os.write(2, text.encode("utf-8", "replace"))
    except Exception:  # noqa: BLE001, S110 — nothing left to try, and no way to say so
        pass


try:
    main()
except BaseException:
    _print_traceback()
    raise
