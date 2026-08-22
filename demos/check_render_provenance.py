#!/usr/bin/env python3
"""Guard: every committed per-scene render must carry ITS LANE'S
renderer signature — FreeCAD-authored in the two tour lanes,
wild-lane-stamped matplotlib in the wild-corpus lane.

WHY (the #221 incident). `render.sh`'s kernel lane has a matplotlib
fallback (`render.py`) for hosts without FreeCAD. It used to write the
SAME directory the FreeCAD renderer writes — `demos/renders/` — so a
fallback frame was indistinguishable from a real one at the filesystem
level, and one of them silently reached a committed montage cell
(PR #221's repair note). Two layers now make that structurally
impossible:

  1. the fallback writes only the gitignored `demos/renders-preview/`
     tree (`render.sh`), so it CANNOT land in a committed path;
  2. this guard — run by both `render.sh` lanes before the montage is
     composed, and by the gate — refuses any committed render that is
     not FreeCAD-authored, naming the file.

Layer 2 is the one that holds if layer 1 is ever bypassed (a hand
copy, a stray `python render.py out renders`, a future edit to the
routing).

HOW. FreeCAD's `view.saveImage()` stamps three `tEXt` chunks into
every PNG it writes:

    Author    FreeCAD (https://www.freecad.org)
    Software  FreeCAD
    Title     <the path it was asked to write>

matplotlib stamps exactly one:

    Software  Matplotlib version3.10.3, https://matplotlib.org/

`strip_png_stamps.py` drops only the two WALL-CLOCK chunks (`tEXt`
"Creation Time", `zTXt` "Description") — the three above are
deterministic and survive the strip, so they are a stable signature of
which renderer drew the pixels. This guard reads Author + Software.
(Title is deterministic too, but it records the path passed at render
time, which is a render-invocation detail, not provenance — so it is
not asserted.)

SHEETS ARE EXEMPT — AND WHY THAT IS SAFE. The two montage sheets
(`renders/montage.png`, `renders-freecad/montage-freecad.png`) are
composed BY matplotlib on purpose: `compose_montage.py` lays the
FreeCAD-rendered cells out on a grid with captions and a banner. Their
`Software: Matplotlib` is correct, not a fallback, so the guard cannot
demand a FreeCAD signature of them. It does the next best thing: the
exemption is a positive assertion, not a hole. A file is exempt IFF
its name is one of the two known sheets, and each sheet that exists
must carry the matplotlib signature — so the exemption cannot be used
to smuggle anything in (rename a fallback frame to `montage.png` and
it stops being a scene cell that any sheet or README references).

THE WILD LANE INVERTS THE RULE, ON PURPOSE. `renders-wild/` (the
wild-corpus montage, `render-wild.sh`) is FreeCAD-FREE by ratified
scope: its cells are the kernel's own tessellation of licensed
third-party STEP, drawn by `render.py` — matplotlib as the PRIMARY
renderer, not a fallback. There the guard demands the matplotlib
`Software` chunk PLUS the wild lane's own `Author` stamp
(`render.py --author=...`), so the lane's contract is still a positive
per-file assertion: a tour fallback frame (matplotlib but unstamped)
is refused in the wild lane exactly as it is in `renders/`, and a
FreeCAD frame is refused there too — whatever lands in a committed
wild cell must be the wild pipeline's own output.

The sheet's own pixels are covered INDIRECTLY, which is the honest
statement of this guard's reach: a sheet is only ever composed from
the cells sitting next to it, and `render.sh` runs this guard over
those cells in the same pass, immediately before composing. So a sheet
produced by the pipeline is composed from a certified-FreeCAD cell
set. What the guard cannot see is a STALE sheet — cells fixed
afterwards without recomposing. That is a `git status` / re-render
question (both lanes are byte-reproducible after the stamp strip), not
a provenance one.

Usage:
    python3 check_render_provenance.py [<dir>...]  # default: both lanes
    python3 check_render_provenance.py --selftest  # the guard's own test

Exit status is 0 when every PNG checks out, 1 (with every offending
file named) otherwise. Stdlib only — no venv, no numpy — so the gate
can run it anywhere.
"""

import re
import struct
import sys
import zlib
from pathlib import Path

from strip_png_stamps import parse_chunks

HERE = Path(__file__).resolve().parent

# The committed render trees, one per montage lane (render.sh /
# render-wild.sh). A directory NAMED `renders-wild` runs under the
# wild lane's rules (matplotlib + wild Author stamp); every other lane
# demands FreeCAD authorship.
LANE_DIRS = ("renders", "renders-freecad", "renders-wild")
WILD_LANE = "renders-wild"

# The matplotlib-COMPOSED contact sheets — the only exempt names, and
# they must actually be matplotlib-composed. This set is the exemption
# list; the sheet names themselves are decided by the render scripts'
# `--montage=` arguments and compose_montage.py's default, and
# `sheet_names_in_scripts()` reads them back out of those files so the
# agreement is CHECKED (the selftest's last case) rather than asked for
# in a comment.
SHEETS = {"montage.png", "montage-freecad.png", "montage-wild.png"}

FREECAD_AUTHOR = "FreeCAD (https://www.freecad.org)"
FREECAD_SOFTWARE = "FreeCAD"
MATPLOTLIB_SOFTWARE_PREFIX = "Matplotlib"
# The wild lane's own signature. Two spellings, and they must agree:
# this constant and `render-wild.sh`'s AUTHOR, which it passes to
# `render.py --author`. `wild_author_in_script()` reads the second one
# and the selftest compares them, because a signature that drifts turns
# every committed wild cell into a violation at once.
WILD_AUTHOR = "pncad wild-corpus lane (kernel tessellation of licensed third-party STEP)"


def sheet_names_in_scripts(root=None):
    """Every sheet name the render scripts and the composer can write.

    `--montage=NAME` in either render script, plus `compose_montage.py`'s
    own default for the invocation that passes no such flag.
    """
    root = root or HERE
    names = set(
        re.findall(
            r"--montage=(\S+)", (root / "render.sh").read_text()
        )
    ) | set(
        re.findall(
            r"--montage=(\S+)", (root / "render-wild.sh").read_text()
        )
    )
    (default,) = re.findall(
        r'^\s*montage_name = "([^"]+)"', (root / "compose_montage.py").read_text(),
        re.MULTILINE,
    )
    return names | {default}


def wild_author_in_script(root=None):
    """The Author string `render-wild.sh` actually stamps into its cells."""
    root = root or HERE
    (author,) = re.findall(
        r"^AUTHOR='([^']*)'$", (root / "render-wild.sh").read_text(), re.MULTILINE
    )
    return author


def text_chunks(path):
    """Return {keyword: value} for the file's `tEXt` chunks.

    Framing and CRCs are verified on the way in (parse_chunks refuses a
    malformed file rather than guessing).
    """
    out = {}
    for ctype, data in parse_chunks(path.read_bytes(), path):
        if ctype != b"tEXt":
            continue
        keyword, _, value = data.partition(b"\x00")
        out[keyword.decode("latin-1")] = value.decode("latin-1")
    return out


def describe(text):
    """One-line renderer identification for an error message."""
    software = text.get("Software")
    if software is None:
        return "no Software tEXt chunk (unknown renderer)"
    return f"Software: {software!r}, Author: {text.get('Author')!r}"


def check_file(path, wild=False):
    """Return a violation string, or None if the file is in order.

    `wild` selects the wild lane's rules: matplotlib + the wild
    `Author` stamp instead of FreeCAD authorship.
    """
    text = text_chunks(path)
    software = text.get("Software", "")
    if path.name in SHEETS:
        # Exempt from the per-cell renderer requirement, but not
        # unchecked: a sheet IS a matplotlib composition, so say so.
        if not software.startswith(MATPLOTLIB_SOFTWARE_PREFIX):
            return (
                f"{path}: montage sheet is not matplotlib-composed "
                f"({describe(text)}) — compose_montage.py draws the "
                f"sheets, so this file is not what its name claims"
            )
        return None
    if wild:
        # Wild-corpus cell: render.py IS the renderer here (the lane
        # is FreeCAD-free by scope), so demand matplotlib PLUS the
        # lane's own Author stamp — an unstamped matplotlib frame is
        # some OTHER lane's fallback output, not a wild cell.
        if not software.startswith(MATPLOTLIB_SOFTWARE_PREFIX) or text.get(
            "Author"
        ) != WILD_AUTHOR:
            return (
                f"{path}: not a wild-lane render ({describe(text)}); "
                f"expected Software: {MATPLOTLIB_SOFTWARE_PREFIX}*, "
                f"Author: {WILD_AUTHOR!r} (re-run demos/render-wild.sh)"
            )
        return None
    if software.startswith(MATPLOTLIB_SOFTWARE_PREFIX):
        return (
            f"{path}: MATPLOTLIB FALLBACK FRAME in a committed render "
            f"path ({describe(text)}). The fallback renders to the "
            f"gitignored demos/renders-preview/ tree; this file must "
            f"come from FreeCAD (re-run ./render.sh with a working "
            f"FREECADCMD)"
        )
    if software != FREECAD_SOFTWARE or text.get("Author") != FREECAD_AUTHOR:
        return (
            f"{path}: not FreeCAD-authored ({describe(text)}); expected "
            f"Software: {FREECAD_SOFTWARE!r}, Author: {FREECAD_AUTHOR!r}"
        )
    return None


def check_dirs(dirs):
    """Check every `*.png` in each directory. Returns (violations, count)."""
    violations, checked = [], 0
    for d in dirs:
        d = Path(d)
        if not d.is_dir():
            violations.append(f"{d}: render directory does not exist")
            continue
        pngs = sorted(d.glob("*.png"))
        if not pngs:
            violations.append(f"{d}: no PNGs — a committed render tree is empty")
            continue
        wild = d.name == WILD_LANE
        for png in pngs:
            checked += 1
            bad = check_file(png, wild=wild)
            if bad:
                violations.append(bad)
    return violations, checked


# ---- self-test -----------------------------------------------------
# The guard's own test: synthesize the three cases as real PNGs (chunk
# framing + CRCs, so they go through the same parser the committed
# files do) and assert the verdicts. Stdlib only — no renderer needed,
# so it runs on every host the gate runs on.


def _chunk(ctype, data):
    return (
        struct.pack(">I", len(data))
        + ctype
        + data
        + struct.pack(">I", zlib.crc32(ctype + data) & 0xFFFFFFFF)
    )


def _tiny_png(path, texts):
    """A valid 1x1 greyscale PNG carrying the given tEXt chunks."""
    ihdr = struct.pack(">IIBBBBB", 1, 1, 8, 0, 0, 0, 0)
    idat = zlib.compress(b"\x00\x00")
    blob = [b"\x89PNG\r\n\x1a\n", _chunk(b"IHDR", ihdr)]
    for keyword, value in texts:
        blob.append(_chunk(b"tEXt", keyword.encode() + b"\x00" + value.encode()))
    blob += [_chunk(b"IDAT", idat), _chunk(b"IEND", b"")]
    path.write_bytes(b"".join(blob))


FREECAD_TEXTS = [
    ("Author", FREECAD_AUTHOR),
    ("Software", FREECAD_SOFTWARE),
    ("Title", "renders/selftest.png"),
]
MATPLOTLIB_TEXTS = [
    ("Software", "Matplotlib version3.10.3, https://matplotlib.org/")
]
WILD_TEXTS = [*MATPLOTLIB_TEXTS, ("Author", WILD_AUTHOR)]


def selftest():
    import tempfile

    with tempfile.TemporaryDirectory() as tmp:
        d = Path(tmp) / "renders"
        d.mkdir()
        _tiny_png(d / "good_cell.png", FREECAD_TEXTS)
        _tiny_png(d / "montage.png", MATPLOTLIB_TEXTS)
        violations, checked = check_dirs([d])
        assert checked == 2, checked
        assert violations == [], violations

        # A fallback frame in a cell path: refused, and named.
        _tiny_png(d / "fallback_cell.png", MATPLOTLIB_TEXTS)
        violations, _ = check_dirs([d])
        assert len(violations) == 1, violations
        assert "fallback_cell.png" in violations[0], violations
        assert "MATPLOTLIB FALLBACK FRAME" in violations[0], violations
        (d / "fallback_cell.png").unlink()

        # An unstamped cell (unknown renderer): refused too.
        _tiny_png(d / "mystery.png", [])
        violations, _ = check_dirs([d])
        assert len(violations) == 1 and "mystery.png" in violations[0], violations
        (d / "mystery.png").unlink()

        # The sheet exemption is not a hole: a sheet must still BE a
        # matplotlib composition.
        _tiny_png(d / "montage.png", FREECAD_TEXTS)
        violations, _ = check_dirs([d])
        assert len(violations) == 1 and "montage.png" in violations[0], violations

        # An empty / missing lane directory is a violation, not a pass.
        violations, _ = check_dirs([Path(tmp) / "nope"])
        assert len(violations) == 1 and "does not exist" in violations[0], violations

        # ---- the wild lane's inverted rules ------------------------
        w = Path(tmp) / "renders-wild"
        w.mkdir()
        # A stamped wild cell and the wild sheet: both in order.
        _tiny_png(w / "wild_cell.png", WILD_TEXTS)
        _tiny_png(w / "montage-wild.png", MATPLOTLIB_TEXTS)
        violations, checked = check_dirs([w])
        assert checked == 2, checked
        assert violations == [], violations
        # An UNSTAMPED matplotlib frame (another lane's fallback
        # output) is refused as a wild cell.
        _tiny_png(w / "fallback_cell.png", MATPLOTLIB_TEXTS)
        violations, _ = check_dirs([w])
        assert len(violations) == 1, violations
        assert "fallback_cell.png" in violations[0], violations
        assert "not a wild-lane render" in violations[0], violations
        (w / "fallback_cell.png").unlink()
        # A FreeCAD frame is refused in the wild lane too.
        _tiny_png(w / "freecad_cell.png", FREECAD_TEXTS)
        violations, _ = check_dirs([w])
        assert len(violations) == 1, violations
        assert "freecad_cell.png" in violations[0], violations
        (w / "freecad_cell.png").unlink()
        # And a stamped wild frame is refused OUTSIDE its lane: in
        # `renders/` it is still a matplotlib frame in a FreeCAD lane.
        _tiny_png(d / "stray_wild.png", WILD_TEXTS)
        violations, _ = check_dirs([d])
        # (d still holds the bad sheet from the exemption case above.)
        assert any(
            "stray_wild.png" in v and "MATPLOTLIB FALLBACK FRAME" in v
            for v in violations
        ), violations
    # ---- the two cross-file agreements this module asserts ---------
    # Both used to be comments asking a future editor to keep files in
    # sync. Read them instead: a sheet name only the scripts know would
    # make the exemption list wrong (a real sheet refused as a fallback
    # frame), and a drifted Author string would fail every committed
    # wild cell at once.
    in_scripts = sheet_names_in_scripts()
    assert in_scripts == SHEETS, (in_scripts, SHEETS)
    stamped = wild_author_in_script()
    assert stamped == WILD_AUTHOR, (stamped, WILD_AUTHOR)

    print("check_render_provenance --selftest: 11 cases OK")


def main():
    args = sys.argv[1:]
    if args == ["--selftest"]:
        selftest()
        return
    dirs = args or [HERE / d for d in LANE_DIRS]
    violations, checked = check_dirs(dirs)
    if violations:
        print(
            f"RENDER PROVENANCE FAILED — {len(violations)} problem(s) in "
            f"{checked} PNG(s):",
            file=sys.stderr,
        )
        for v in violations:
            print(f"  {v}", file=sys.stderr)
        raise SystemExit(1)
    print(
        f"check_render_provenance: {checked} PNG(s) carry their lane's "
        f"renderer signature (sheets exempt)"
    )


if __name__ == "__main__":
    main()
