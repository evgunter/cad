#!/usr/bin/env python3
"""The demo scene manifest, read in one place.

`<outdir>/scenes.json` is hand-rolled by the tour and by the wild
generator, and everything that draws a render walks it: the matplotlib
renderer, the FreeCAD renderer, the montage composer and `render.sh`'s
scene lister. **They walk it through here.** A second walk is a second
opinion about what a missing field means, and a second copy of the
`up` convention is two maps facing opposite ways with nothing
comparing them — neither shows up as a crash.

There is no schema and no validation layer, deliberately: the
producers stay hand-rolled JSON. A schema would not help with either
hazard above — it declares that `up` is `"y"` or `"z"` and says
nothing whatever about the transform that meaning implies.

Scope is `scenes.json`. The UV lane's `uv.json` has exactly one reader
(`compose_uv_montage.py`) and its walk stays there; a second home for
a format with one consumer would be the same mistake pointed the other
way.

Two script modes, both thin: `--selftest` checks the camera convention
and the walk against synthetic manifests in both producers' shapes,
and `--scene-names <outdir> [montage]` prints the manifest's scene
names in order for `render.sh` to loop over.
"""

import json
import sys
from pathlib import Path

# THE `up` CONVENTION, WRITTEN ONCE.
#
# A manifest view names which WORLD axis is display-up. The display
# frame is always z-up, because that is what matplotlib's
# `view_init(elev, azim)` means and every renderer frames from it.
#
# The convention is a signed permutation of the axes, given here in
# the world -> display direction ONLY: display axis `i` is `sign`
# times world axis `src`. `up: "z"` is the identity; `up: "y"` takes
# the y axis vertical, so (x, y, z) -> (x, -z, y).
#
# The other direction is DERIVED and never written down. A renderer
# that orients a mesh needs world -> display; one that places a camera
# needs display -> world, so both are genuinely wanted. Two
# hand-written maps facing opposite ways each read as correct in the
# file they live in, nothing local compares them, and a disagreement
# is a silently wrong camera rather than a crash.
_WORLD_TO_DISPLAY = {
    "z": ((0, 1), (1, 1), (2, 1)),
    "y": ((0, 1), (2, -1), (1, 1)),
}


def _check_signed_permutation(up, axes):
    """Refuse a table row `_invert` could not invert.

    Checked at import, not at use: the table is what a future edit
    touches, and an edit that is not a signed permutation makes the
    DERIVED direction quietly wrong rather than absent — `_invert`
    would leave a `None` in the tuple and it would surface as a
    `TypeError` in a renderer. One home for the convention is only
    worth having if it is a home the next person can edit safely.
    """
    if (
        len(axes) != 3
        or sorted(src for src, _ in axes) != [0, 1, 2]
        or any(sign not in (1, -1) for _, sign in axes)
    ):
        raise SystemExit(
            f"manifest.py: _WORLD_TO_DISPLAY[{up!r}] = {axes!r} is not a "
            f"signed permutation. Each row must name axes 0, 1 and 2 exactly "
            f"once, each with sign +1 or -1; the display -> world direction "
            f"is derived by inverting it, and nothing else has an inverse."
        )


for _up, _axes in _WORLD_TO_DISPLAY.items():
    _check_signed_permutation(_up, _axes)
del _up, _axes


def _invert(axes):
    """Inverse of a signed permutation, as a signed permutation.

    `axes[d] == (src, sign)` reads `out[d] = sign * inp[src]`. Signs
    are +-1 and every source index appears exactly once (checked at
    import), so the same equation rearranges to `inp[src] = sign * out[d]`.
    """
    inv = [None, None, None]
    for dst, (src, sign) in enumerate(axes):
        inv[src] = (dst, sign)
    return tuple(inv)


def world_to_display(up, where="a scene"):
    """Axis spec taking world coordinates into the z-up display frame."""
    try:
        return _WORLD_TO_DISPLAY[up]
    except KeyError:
        raise SystemExit(
            f"scenes.json: {where} names view up axis {up!r}, which this "
            f"reader has no convention for (known: "
            f"{', '.join(sorted(_WORLD_TO_DISPLAY))}). Adding an axis means "
            f"adding its world -> display row to _WORLD_TO_DISPLAY in "
            f"demos/manifest.py; both directions follow from it."
        ) from None


def display_to_world(up, where="a scene"):
    """Axis spec taking display coordinates back to world coordinates."""
    return _invert(world_to_display(up, where))


def _field(d, key, where):
    """One required manifest field, or a refusal that says where it was.

    Every key this reader names is written unconditionally by both
    producers, so a missing one is a broken or stale manifest and not a
    shape to default. The refusal has to carry enough to act on — which
    scene, which body, which key — because the reader is the only thing
    standing between a hand-edited manifest and a renderer that draws
    something plausible.
    """
    try:
        return d[key]
    except (KeyError, TypeError):
        raise SystemExit(
            f"scenes.json: {where} has no {key!r}. Both producers (the tour "
            f"and the wild generator) write it for every entry, so this "
            f"manifest is stale or hand-edited — re-run the generator that "
            f"wrote it."
        ) from None


def apply_axes(axes, v):
    """Apply an axis spec to one 3-vector, as a tuple."""
    return tuple(sign * v[src] for src, sign in axes)


class Body:
    """One body of a scene: the files and the look, as the manifest has them."""

    def __init__(self, d, where):
        self.stl = _field(d, "stl", where)
        # `step` is the one manifest field whose VALUE is genuinely
        # optional: the tour writes a stem for every body (it fails
        # rather than emit one without a STEP export), the wild
        # generator writes null for every cell, because a wild cell's
        # STEP is an input fixture and not something this pipeline
        # exported. The key is always present; a consumer that imports
        # STEP has to know which producer it is reading, and says so
        # where it reads this.
        self.step = _field(d, "step", where)
        self.color = tuple(_field(d, "color", where))
        # Read, not defaulted. Both producers write `transparency`
        # unconditionally -- an agreement they keep independently,
        # with no shared type and no dependency edge between them (see
        # each emitter). Defaulting it here would be this reader
        # deciding what a body with no `transparency` looks like, and
        # no producer emits one; a producer that stopped writing it
        # would then render silently opaque instead of failing.
        self.transparency = _field(d, "transparency", where)


class Scene:
    """One rendered scene: one or more bodies under one camera."""

    def __init__(self, d, ordinal):
        # `name` first, and located by ORDINAL, because it is what
        # every later refusal in this scene is located by.
        self.name = _field(d, "name", f"scene #{ordinal}")
        where = f"scene {self.name!r}"
        self.caption = _field(d, "caption", where)
        # Read, not defaulted, for the same reason as `transparency`:
        # both producers write it for every scene.
        self.montage = _field(d, "montage", where)
        view = _field(d, "view", where)
        self.elev = _field(view, "elev", f"{where}'s view")
        self.azim = _field(view, "azim", f"{where}'s view")
        self.up = _field(view, "up", f"{where}'s view")
        # The up axis is resolved HERE rather than at the first camera:
        # this is where the scene's name is in hand, and a manifest
        # naming an axis no renderer can build should fail before any
        # frame is drawn rather than partway through a pass.
        world_to_display(self.up, where)
        # Never empty, and every body in it has an STL: both producers
        # fail rather than emit either, so no renderer needs a skip
        # path or an empty-scene path.
        self.bodies = [
            Body(b, f"{where} body {i}")
            for i, b in enumerate(_field(d, "bodies", where))
        ]

    def world_to_display(self):
        """This scene's world -> display axis spec."""
        return world_to_display(self.up, f"scene {self.name!r}")

    def display_to_world(self):
        """This scene's display -> world axis spec."""
        return display_to_world(self.up, f"scene {self.name!r}")


def read_scenes(outdir):
    """Every scene of `<outdir>/scenes.json`, in manifest order."""
    text = (Path(outdir) / "scenes.json").read_text()
    return [Scene(s, i) for i, s in enumerate(json.loads(text))]


def _selftest():
    # The convention itself, pinned against the two spellings this
    # module replaced -- so a future edit to _WORLD_TO_DISPLAY has to
    # mean to change the cameras.
    assert apply_axes(world_to_display("y"), (1.0, 2.0, 3.0)) == (1.0, -3.0, 2.0)
    assert apply_axes(display_to_world("y"), (1.0, 2.0, 3.0)) == (1.0, 3.0, -2.0)
    assert apply_axes(world_to_display("z"), (1.0, 2.0, 3.0)) == (1.0, 2.0, 3.0)
    assert apply_axes(display_to_world("z"), (1.0, 2.0, 3.0)) == (1.0, 2.0, 3.0)

    # The derivation is an inverse, in both compositions, on a vector
    # with three distinct nonzero coordinates (so no axis swap or sign
    # flip can hide in it).
    v = (2.0, -5.0, 11.0)
    for up in _WORLD_TO_DISPLAY:
        fwd, back = world_to_display(up), display_to_world(up)
        assert apply_axes(back, apply_axes(fwd, v)) == v, up
        assert apply_axes(fwd, apply_axes(back, v)) == v, up

    # An unknown axis refuses, and the refusal locates itself and says
    # what to do about it.
    try:
        world_to_display("x", "scene 'vase'")
    except SystemExit as e:
        for want in ("scene 'vase'", "'x'", "y, z", "_WORLD_TO_DISPLAY"):
            assert want in str(e), (want, str(e))
    else:
        raise AssertionError("an unknown up axis must refuse")

    # A table row that is not a signed permutation refuses AT IMPORT,
    # because that is the edit this module invites and the one that
    # would make the derived direction quietly wrong.
    for bad in (
        ((0, 1), (0, -1), (2, 1)),  # 1 missing, 0 twice
        ((0, 1), (1, 2), (2, 1)),  # a sign that is not +-1
        ((0, 1), (1, 1)),  # two axes
    ):
        try:
            _check_signed_permutation("bad", bad)
        except SystemExit as e:
            assert "signed permutation" in str(e), e
        else:
            raise AssertionError(f"{bad!r} must refuse")

    # THE FREECAD CAMERA'S FALLBACK, PINNED AS FAR AS IT CAN BE HERE.
    #
    # `render_freecad.camera_rotation` picks a substitute up vector
    # when `abs(up_w . z_cam) > 0.9999` -- a threshold written in WORLD
    # coordinates with no `up` in it. That is only sound because these
    # specs are ISOMETRIES: a signed permutation preserves dot
    # products, so the quantity tested is the display-frame one
    # (sin(elev)) whatever the up axis, and the branch fires on exactly
    # the same views for every scene. The branch itself needs FreeCAD
    # to execute and no gate here can reach it; this is the property it
    # rests on, and it is checkable. TWO THINGS HOLD IT: the
    # import-time check keeps every table row a signed permutation, and
    # these rows catch `apply_axes` itself ceasing to be an isometry
    # (measured: scaling one output component reds both up axes here).
    # Integer coordinates, so the three products sum exactly in any
    # order.
    def dot(a, b):
        return sum(x * y for x, y in zip(a, b, strict=True))

    pairs = (
        ((2.0, -5.0, 11.0), (3.0, 7.0, -2.0)),
        ((0.0, 0.0, 1.0), (4.0, -6.0, 9.0)),  # display up against a camera dir
        ((1.0, 0.0, 0.0), (0.0, 1.0, 0.0)),
    )
    for up in _WORLD_TO_DISPLAY:
        back = display_to_world(up)
        for a, b in pairs:
            assert dot(apply_axes(back, a), apply_axes(back, b)) == dot(a, b), up

    # The walk, over one scene in each producer's shape: the tour's
    # (a STEP stem, a transparent body, off the montage) and the wild
    # generator's (null STEP, opaque, on the montage).
    import tempfile

    doc = [
        {
            "name": "tour_stop",
            "caption": "a tour stop",
            "montage": False,
            "view": {"elev": 22.0, "azim": -60.0, "up": "y"},
            "bodies": [
                {
                    "stl": "a.stl",
                    "step": "a.step",
                    "color": [0.1, 0.2, 0.3],
                    "transparency": 40,
                }
            ],
        },
        {
            "name": "wild_cell",
            "caption": "a wild cell",
            "montage": True,
            "view": {"elev": 30.0, "azim": -50.0, "up": "z"},
            "bodies": [
                {
                    "stl": "b.stl",
                    "step": None,
                    "color": [0.4, 0.5, 0.6],
                    "transparency": 0,
                }
            ],
        },
    ]
    with tempfile.TemporaryDirectory() as d:
        (Path(d) / "scenes.json").write_text(json.dumps(doc))
        scenes = read_scenes(d)
    assert [s.name for s in scenes] == ["tour_stop", "wild_cell"], scenes
    assert [s.montage for s in scenes] == [False, True]
    tour, wild = scenes
    assert (tour.elev, tour.azim, tour.up) == (22.0, -60.0, "y")
    assert tour.world_to_display() == world_to_display("y")
    assert tour.display_to_world() == display_to_world("y")
    assert tour.bodies[0].step == "a.step"
    assert tour.bodies[0].color == (0.1, 0.2, 0.3)
    assert tour.bodies[0].transparency == 40
    assert wild.bodies[0].step is None
    assert wild.bodies[0].transparency == 0

    # A missing key is a refusal, not a default -- the whole reason
    # `.get` is gone from `transparency` and `montage`. What is pinned
    # is the MESSAGE, not the exception type: a refusal a reader cannot
    # act on is the defect this module exists to remove, and pinning
    # `KeyError` would make improving it look like a regression.
    def drop(path):
        broken = json.loads(json.dumps(doc))
        node = broken
        for step in path[:-1]:
            node = node[step]
        del node[path[-1]]
        return broken

    for path, wanted in (
        ([0, "bodies", 0, "transparency"], ("scene 'tour_stop' body 0", "'transparency'")),
        ([0, "montage"], ("scene 'tour_stop'", "'montage'")),
        ([0, "view", "up"], ("scene 'tour_stop'", "view", "'up'")),
        ([0, "name"], ("scene #0", "'name'")),
        ([1, "bodies", 0, "step"], ("scene 'wild_cell' body 0", "'step'")),
    ):
        with tempfile.TemporaryDirectory() as d:
            (Path(d) / "scenes.json").write_text(json.dumps(drop(path)))
            try:
                read_scenes(d)
            except SystemExit as e:
                for want in (*wanted, "re-run the generator"):
                    assert want in str(e), (path, want, str(e))
            else:
                raise AssertionError(f"a manifest missing {path} must refuse")

    print("manifest selftest: ok")


def _scene_names(argv):
    """`render.sh`'s scene loop, as a mode of the reader rather than as
    an inline `python -c` in the shell script.

    It lived in the shell as a four-line snippet, which made it the one
    consumer that reached this module through the CURRENT DIRECTORY
    rather than through its own location -- `python -c` puts cwd on
    `sys.path`, `python <script>` puts the script's directory there.
    Both happened to work because `render.sh` cd's to `demos/` at the
    top, two hundred lines above the snippet. As a mode here it is a
    script like the others, so one mechanism covers every consumer that
    can use one; `render_freecad.py` is the single exception and says
    why at its own `sys.path` line.
    """
    if not argv or len(argv) > 2 or (len(argv) == 2 and argv[1] != "montage"):
        raise SystemExit("usage: python manifest.py --scene-names <outdir> [montage]")
    montage_only = len(argv) == 2
    for scene in read_scenes(argv[0]):
        if scene.montage or not montage_only:
            print(scene.name)


if __name__ == "__main__":
    mode, rest = (sys.argv[1:2] or [None])[0], sys.argv[2:]
    if mode == "--selftest" and not rest:
        _selftest()
    elif mode == "--scene-names":
        _scene_names(rest)
    else:
        raise SystemExit(
            "usage:  python manifest.py --selftest\n"
            "        python manifest.py --scene-names <outdir> [montage]"
        )
