#!/usr/bin/env python3
"""Screenshot the viewer over the assembly store -> renders-gui/.

Driven by `render-gui.sh`, which owns the hosted-render guard and the
lane's signature. This file owns the mechanics: bring up a virtual X
server, open each document in the real `viewer` binary, wait for the
frame to STOP CHANGING, photograph it, stamp it, and tile the results.

# Why "wait until it stops changing" and not a sleep

The viewer evaluates, tessellates and builds a BVH before its first
real frame, and how long that takes depends on the document and on the
runner. A fixed sleep is therefore either a guess that races the app or
a guess that wastes minutes on every cell, and the failure mode of the
first one is a HALF-DRAWN COMMITTED PIXEL — the worst outcome this lane
has, because it looks like a rendering bug forever after.

So each cell polls: shoot, hash, shoot again, and accept the frame only
when two consecutive shots are byte-identical AND the window is not
still blank. That is self-timing (a fast document finishes fast), it
cannot accept a frame mid-paint, and it fails loudly on timeout instead
of committing whatever was on screen.

`crates/viewer/README.md` records the same technique for timing an
interaction, and its warning applies here too: `pkill -f viewer` kills
the invoking shell, whose own command line contains that string. This
file always kills by PID.

# What is photographed, and why these documents

The tour's assembly stop writes its whole part STORE to
`out/assembly/*.pncad` — leaf parts, the patterns over them, and the
mated bench that references them all. That store is the assembly
layer's own shape, so the lane opens every document in it rather than
curating a subset: which documents exist IS the story, and a curated
list would go stale the first time the stop gains a part.

Cells are named by what the document CONTAINS (its node kinds) rather
than by its content-address id, because an id is not a name a reader
can use — and the id is in the window's title bar anyway, which the
shot includes. Where two documents share a shape, and the bench's two
leaf parts do, the id comes back as a suffix; [`cell_names`] carries
why that is not optional.
"""

import argparse
import hashlib
import json
import os
import pathlib
import re
import struct
import subprocess
import sys
import time
import zlib

HERE = pathlib.Path(__file__).resolve().parent

#: The virtual display this lane runs on. High enough not to collide
#: with a developer's own :0/:1, and fixed rather than searched because
#: a searched display makes two concurrent passes silently share one
#: server and photograph each other's windows.
DISPLAY = ":99"
#: Screen geometry. Wide enough that the properties panel is not
#: truncated — the panel is the point of this lane, and a clipped panel
#: is a cell that shows less than the app does.
GEOMETRY = "1400x900x24"
#: How long one cell may take to settle before the lane fails. Generous
#: because a cold release binary on a loaded runner is slow; bounded
#: because a hang must not look like patience.
SETTLE_TIMEOUT_S = 180.0
#: Gap between settle polls. Each poll costs a full-screen grab, so
#: this trades responsiveness against load.
POLL_S = 1.5
#: Consecutive identical shots required. TWO is enough: the frames are
#: byte-compared, so a match already means the app painted nothing new
#: in a whole poll interval.
STABLE_SHOTS = 2


def run(cmd, **kw):
    """A checked subprocess call that reports the command on failure."""
    try:
        return subprocess.run(cmd, check=True, capture_output=True, **kw)
    except subprocess.CalledProcessError as exc:
        sys.exit(
            f"render_gui: {' '.join(map(str, cmd))} failed ({exc.returncode})\n"
            f"{exc.stderr.decode('utf-8', 'replace')}"
        )


def node_kinds(doc_path):
    """The node kinds a `.pncad` snapshot holds, in document order.

    The file is a one-line `id:` header followed by JSON, so the parse
    starts at the first brace. A document whose shape this cannot read
    is a FAILURE rather than a skip: a silently skipped document is a
    cell that quietly stops existing, which is the drift this lane's
    re-baseline would then commit.
    """
    text = doc_path.read_text(encoding="utf-8")
    try:
        blob = json.loads(text[text.index("{") :])
        nodes = blob["snapshot"]["nodes"]
    except (ValueError, KeyError) as exc:
        sys.exit(f"render_gui: {doc_path.name} is not a readable snapshot: {exc}")
    out = []
    for key in sorted(nodes, key=int):
        node = nodes[key]
        out.append(next(iter(node)) if isinstance(node, dict) else str(node))
    return out


def shape_name(kinds):
    """A readable name for a document's SHAPE, from its node kinds.

    `InstantiatePart x3 + Mate x2` becomes `instantiatepart3-mate2`.
    Purely a function of the document's shape, so two runs agree and a
    reader can tell which cell is which without opening the file.

    NOT injective over a real store, which is why [`cell_names`] exists
    rather than this being the whole answer: the bench's two leaf parts
    are both `Datum + Profile + Extrude`, so they share this name and
    one would silently overwrite the other.
    """
    counts = {}
    for k in kinds:
        counts[k] = counts.get(k, 0) + 1
    parts = []
    for kind in sorted(counts):
        short = re.sub(r"(?<!^)(?=[A-Z])", "", kind).lower()
        parts.append(short if counts[kind] == 1 else f"{short}{counts[kind]}")
    return "-".join(parts)


def cell_names(docs):
    """One name per document, collision-free, as `{path: name}`.

    [`shape_name`] alone is not enough and the failure mode is silent:
    two documents of the same shape would write the same file, so the
    lane would commit five cells for six documents and tile one of them
    twice. That is exactly what the first run of this lane did.

    So a name that is shared gets the document's own id appended —
    eight characters of a CONTENT ADDRESS, which is stable across runs
    and is the only thing that actually distinguishes two documents of
    identical shape. A name that is unique keeps the readable form,
    because most of the store is unique and an id in every filename
    would cost every reader something to buy one pair a suffix.
    """
    shapes = {doc: shape_name(node_kinds(doc)) for doc in docs}
    shared = {n for n in shapes.values() if list(shapes.values()).count(n) > 1}
    return {
        doc: (f"{name}-{doc.stem[:8]}" if name in shared else name)
        for doc, name in shapes.items()
    }


def encode_chunk(ctype, data):
    """One PNG chunk, framed and CRC'd (strip_png_stamps.py's twin)."""
    return (
        struct.pack(">I", len(data))
        + ctype
        + data
        + struct.pack(">I", zlib.crc32(ctype + data) & 0xFFFFFFFF)
    )


def stamp_author(path, author):
    """Insert the lane's `Author` tEXt chunk right after the header.

    Written here rather than asked of `import`, because ImageMagick's
    own text options land in chunks whose keyword depends on the build,
    and the provenance guard matches on the keyword. Writing the bytes
    makes the signature this lane's own fact.
    """
    blob = path.read_bytes()
    if blob[:8] != b"\x89PNG\r\n\x1a\n":
        sys.exit(f"render_gui: {path} is not a PNG")
    # IHDR is always the first chunk: 8 signature + 4 length + 4 type +
    # 13 data + 4 crc.
    cut = 8 + 4 + 4 + 13 + 4
    chunk = encode_chunk(b"tEXt", b"Author\x00" + author.encode("utf-8"))
    path.write_bytes(blob[:cut] + chunk + blob[cut:])


def window_geometry(env):
    """The viewer window's `WxH+X+Y`, or None if it cannot be read.

    There is no window manager on the virtual server, so the app's
    window is the only visible one and sits at the origin — but it is
    SMALLER than the screen (1280x800 of the 1400x900 above), and a
    root grab therefore carries a band of empty desktop on two sides.
    On a contact sheet that band is most of what a reader sees, so the
    final shot is cropped to what `xdotool` reports.

    Read at run time rather than hard-coded to the viewer's current
    default, because a default that changes would otherwise either clip
    the app or pad it, and the clipping case is silent.

    None when the answer is not exactly one window: the caller then
    keeps the full frame, which is worse-looking and still correct.
    """
    try:
        ids = subprocess.run(
            ["xdotool", "search", "--onlyvisible", "--name", "."],
            check=True, capture_output=True, env=env,
        ).stdout.split()
    except subprocess.CalledProcessError:
        return None
    if len(ids) != 1:
        return None
    try:
        info = subprocess.run(
            ["xdotool", "getwindowgeometry", ids[0]],
            check=True, capture_output=True, env=env,
        ).stdout.decode("utf-8", "replace")
    except subprocess.CalledProcessError:
        return None
    pos = re.search(r"Position:\s*(-?\d+),(-?\d+)", info)
    geo = re.search(r"Geometry:\s*(\d+)x(\d+)", info)
    if not (pos and geo):
        return None
    return f"{geo.group(1)}x{geo.group(2)}+{pos.group(1)}+{pos.group(2)}"


def shoot(dest, env):
    """One full-screen grab. Returns its bytes, or None if the grab
    failed — which happens while the server has no window mapped yet,
    and is an ordinary early-poll outcome rather than an error."""
    try:
        subprocess.run(
            ["import", "-window", "root", str(dest)],
            check=True,
            capture_output=True,
            env=env,
        )
    except subprocess.CalledProcessError:
        return None
    return dest.read_bytes() if dest.exists() else None


def looks_blank(png_bytes):
    """Whether a grab is the empty server — a frame with almost no
    entropy. Compressed size is the cheap proxy: a blank 1400x900 is a
    couple of kB, a drawn one is tens. The threshold only has to
    separate 'nothing yet' from 'a window with a panel in it'."""
    return len(png_bytes) < 8_000


def settle(dest, env, label):
    """Poll until the frame stops changing, then leave it at `dest`."""
    deadline = time.monotonic() + SETTLE_TIMEOUT_S
    previous, stable = None, 0
    while time.monotonic() < deadline:
        time.sleep(POLL_S)
        shot = shoot(dest, env)
        if shot is None or looks_blank(shot):
            previous, stable = None, 0
            continue
        digest = hashlib.sha256(shot).hexdigest()
        if digest == previous:
            stable += 1
            if stable >= STABLE_SHOTS - 1:
                return
        else:
            stable = 0
        previous = digest
    sys.exit(
        f"render_gui: {label} never settled in {SETTLE_TIMEOUT_S:.0f}s. The frame was "
        f"still changing (or still blank) at the deadline, so there is nothing this "
        f"lane is willing to commit. A debug viewer build is the usual cause."
    )


def photograph(viewer, doc, dest, env, label):
    """Open one document, wait for its frame, and leave a shot at `dest`."""
    proc = subprocess.Popen(
        [str(viewer), str(doc)],
        env=env,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    try:
        settle(dest, env, label)
        # The settle ran on ROOT grabs; the committed cell is the same
        # settled frame cropped to the app. Re-shooting here is one
        # extra grab of a frame already proven static.
        crop = window_geometry(env)
        if crop is not None:
            run(
                ["import", "-window", "root", "-crop", crop, "+repage", str(dest)],
                env=env,
            )
        if proc.poll() is not None:
            sys.exit(
                f"render_gui: the viewer exited ({proc.returncode}) while opening "
                f"{doc.name}; the frame at {dest} is not a picture of a running app."
            )
    finally:
        # By PID, never by name: `pkill -f viewer` matches this
        # process's own command line too (viewer README).
        proc.terminate()
        try:
            proc.wait(timeout=20)
        except subprocess.TimeoutExpired:
            proc.kill()
            proc.wait(timeout=20)


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--author", required=True, help="the lane's signature")
    ap.add_argument("outdir", help="the tour's output directory (holds assembly/)")
    ap.add_argument("lanedir", help="the committed lane directory")
    args = ap.parse_args()

    out = (HERE / args.outdir).resolve()
    lane = (HERE / args.lanedir).resolve()
    store = out / "assembly"
    if not store.is_dir():
        sys.exit(
            f"render_gui: {store} does not exist. This lane photographs the tour's "
            f"own assembly store, so the tour must have run first:\n"
            f"  cd demos/tour && cargo run --release -- ../{args.outdir}"
        )
    docs = sorted(store.glob("*.pncad"))
    if not docs:
        sys.exit(f"render_gui: {store} holds no .pncad documents")

    viewer = HERE.parent / "target" / "release" / "viewer"
    if not viewer.is_file():
        sys.exit(
            f"render_gui: {viewer} is missing. Build it RELEASE — a debug build "
            f"spends minutes in tessellation and every cell would time out:\n"
            f"  cargo build --release -p viewer --features app"
        )

    stage = out / "gui"
    stage.mkdir(parents=True, exist_ok=True)
    lane.mkdir(parents=True, exist_ok=True)

    xvfb = subprocess.Popen(
        ["Xvfb", DISPLAY, "-screen", "0", GEOMETRY],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    env = dict(os.environ, DISPLAY=DISPLAY)
    cells = []
    try:
        # The server needs a moment before `import` can find a root
        # window; a failed early grab is handled by the poll anyway,
        # this just avoids burning the first one.
        time.sleep(2.0)
        if xvfb.poll() is not None:
            sys.exit(
                f"render_gui: Xvfb exited immediately ({xvfb.returncode}). Another "
                f"server may already hold {DISPLAY}."
            )
        names = cell_names(docs)
        for doc in docs:
            name = names[doc]
            dest = stage / f"{name}.png"
            print(f"  [{name}] opening {doc.name[:8]}…", flush=True)
            photograph(viewer, doc, dest, env, name)
            stamp_author(dest, args.author)
            cells.append(dest)
            print(f"  [{name}] {dest.stat().st_size} bytes", flush=True)
    finally:
        xvfb.terminate()
        try:
            xvfb.wait(timeout=20)
        except subprocess.TimeoutExpired:
            xvfb.kill()

    for cell in cells:
        (lane / cell.name).write_bytes(cell.read_bytes())

    sheet = lane / "montage-gui.png"
    run(
        ["montage", *[str(c) for c in sorted(cells)], "-tile", "2x", "-geometry",
         "+8+8", "-background", "#1b1b1b", str(sheet)]
    )
    # The sheet is stamped like a cell, so the provenance guard needs no
    # exemption for it (render-gui.sh's AUTHOR comment carries why).
    stamp_author(sheet, args.author)
    print(f"render_gui: {len(cells)} cell(s) + {sheet.name} -> {lane}")


if __name__ == "__main__":
    main()
