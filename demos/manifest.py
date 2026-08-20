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

`--selftest` checks the camera convention and the walk against
synthetic manifests in both producers' shapes.
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


def _invert(axes):
    """Inverse of a signed permutation, as a signed permutation.

    `axes[d] == (src, sign)` reads `out[d] = sign * inp[src]`. Signs
    are +-1 and every source index appears exactly once, so the same
    equation rearranges to `inp[src] = sign * out[d]`.
    """
    inv = [None, None, None]
    for dst, (src, sign) in enumerate(axes):
        inv[src] = (dst, sign)
    return tuple(inv)


def world_to_display(up):
    """Axis spec taking world coordinates into the z-up display frame."""
    try:
        return _WORLD_TO_DISPLAY[up]
    except KeyError:
        raise SystemExit(
            f"scenes.json: unknown view up axis {up!r} "
            f"(known: {', '.join(sorted(_WORLD_TO_DISPLAY))})"
        ) from None


def display_to_world(up):
    """Axis spec taking display coordinates back to world coordinates."""
    return _invert(world_to_display(up))


def apply_axes(axes, v):
    """Apply an axis spec to one 3-vector, as a tuple."""
    return tuple(sign * v[src] for src, sign in axes)


class Body:
    """One body of a scene: the files and the look, as the manifest has them."""

    def __init__(self, d):
        self.stl = d["stl"]
        # `step` is the one manifest field whose VALUE is genuinely
        # optional: the tour writes a stem for every body (it fails
        # rather than emit one without a STEP export), the wild
        # generator writes null for every cell, because a wild cell's
        # STEP is an input fixture and not something this pipeline
        # exported. The key is always present; a consumer that imports
        # STEP has to know which producer it is reading, and says so
        # where it reads this.
        self.step = d["step"]
        self.color = tuple(d["color"])
        # Read, not defaulted. Both producers write `transparency`
        # unconditionally -- an agreement they keep independently,
        # with no shared type and no dependency edge between them (see
        # each emitter). Defaulting it here would be this reader
        # deciding what a body with no `transparency` looks like, and
        # no producer emits one; a producer that stopped writing it
        # would then render silently opaque instead of failing.
        self.transparency = d["transparency"]


class Scene:
    """One rendered scene: several bodies under one camera."""

    def __init__(self, d):
        self.name = d["name"]
        self.caption = d["caption"]
        # Read, not defaulted, for the same reason as `transparency`:
        # both producers write it for every scene.
        self.montage = d["montage"]
        view = d["view"]
        self.elev = view["elev"]
        self.azim = view["azim"]
        self.up = view["up"]
        self.bodies = [Body(b) for b in d["bodies"]]
        # Every scene in a manifest has at least one body, and every
        # body has an STL: both producers fail rather than emit
        # either empty, so no renderer carries a skip path.

    def world_to_display(self):
        """This scene's world -> display axis spec."""
        return world_to_display(self.up)

    def display_to_world(self):
        """This scene's display -> world axis spec."""
        return display_to_world(self.up)


def read_scenes(outdir):
    """Every scene of `<outdir>/scenes.json`, in manifest order."""
    text = (Path(outdir) / "scenes.json").read_text()
    return [Scene(s) for s in json.loads(text)]


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

    try:
        world_to_display("x")
    except SystemExit as e:
        assert "unknown view up axis" in str(e), e
    else:
        raise AssertionError("an unknown up axis must refuse")

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

    # A missing key is a refusal, not a default. This is the whole
    # reason `.get` is gone from the two fields above.
    for field in ("transparency", "montage"):
        broken = json.loads(json.dumps(doc))
        if field == "montage":
            del broken[0][field]
        else:
            del broken[0]["bodies"][0][field]
        with tempfile.TemporaryDirectory() as d:
            (Path(d) / "scenes.json").write_text(json.dumps(broken))
            try:
                read_scenes(d)
            except KeyError as e:
                assert field in str(e), (field, e)
            else:
                raise AssertionError(f"a manifest missing {field} must refuse")

    print("manifest selftest: ok")


if __name__ == "__main__":
    if sys.argv[1:] == ["--selftest"]:
        _selftest()
    else:
        raise SystemExit("usage: python manifest.py --selftest")
