#!/usr/bin/env python3
"""Strip FreeCAD's WALL-CLOCK and PATH metadata chunks from rendered PNGs.

FreeCAD's `view.saveImage()` (demos/render_freecad.py) stamps the
current time, and the path it was asked to write, into every PNG — so a
re-render of unchanged geometry still shows up as a modified file in
`git status`, and the same pixels written to two paths are two
different files:

  * a `tEXt` chunk keyed "Creation Time" — the wall clock, literally;
  * a `zTXt` chunk keyed "Description" — FreeCAD's MIBA XML, whose
    `<CreationDate>` makes even the *compressed* length wander by a
    byte or two;
  * a `tEXt` chunk keyed "Title" — the OUTPUT PATH the frame was
    rendered to. It is deterministic, but it is a fact about the
    invocation rather than about the image: it made a frame's bytes
    depend on WHERE it was written, so the same pixels staged under one
    directory name and published under another were not the same file.
    Dropping it (Ev, 2026-08-22 — "we already erase some PNG metadata
    so we can do the paths too") makes a frame comparable to any other
    frame of the same pixels, whatever path produced it. That is what
    lets a render be A/B-compared against a differently-routed one —
    e.g. batched vs unbatched — with a byte difference meaning a
    PIXEL difference and nothing else.

All three are ancillary chunks (lowercase first letter = safe to drop;
the image is bit-identical without them — verified by decoding the IDAT
stream on both sides of a strip). Removing them makes a re-render
byte-reproducible, so a dirty `git status` after `./render.sh` means
the *pixels* changed.

Only those three chunks go; every other byte of the file — chunk order,
IDAT framing, any other ancillary chunk (pHYs, sRGB, ...) — is carried
through verbatim. In particular the provenance signature
(`demos/check_render_provenance.py` reads the `Author` and `Software`
tEXt chunks) survives untouched: which renderer drew the pixels is
still recorded in the file, only where it was told to put them is not. Parsing is by the PNG chunk framing
(length/type/data/CRC), not by pattern matching on bytes, and every
CRC is checked on the way in: a malformed file is refused, never
silently rewritten.

The matplotlib lane (compose_montage.py) carries no timestamp, so its
byte changes are genuine signal — this tool leaves such files alone
(they simply have no matching chunks).

Usage:
    python strip_png_stamps.py <path|dir> [...]        strip in place
    python strip_png_stamps.py --check <path|dir> [...]  report only

Directories are scanned one level deep for `*.png`. `--check` writes
nothing and exits 1 if any file still carries a stamped chunk (that is
the idempotence assertion: a second pass must find none).
"""

import struct
import sys
import zlib
from pathlib import Path

SIGNATURE = b"\x89PNG\r\n\x1a\n"

# (chunk type, keyword) pairs to drop. Keyword = the NUL-terminated
# first field of tEXt/zTXt chunk data.
STAMP_CHUNKS = {
    (b"tEXt", b"Creation Time"),
    (b"zTXt", b"Description"),
    (b"tEXt", b"Title"),
}


def parse_chunks(blob, path):
    """Yield (type, data) for each chunk, verifying framing and CRCs."""
    if not blob.startswith(SIGNATURE):
        raise SystemExit(f"{path}: not a PNG (bad signature)")
    pos = len(SIGNATURE)
    seen_end = False
    while pos < len(blob):
        if seen_end:
            raise SystemExit(f"{path}: {len(blob) - pos} trailing bytes after IEND")
        if pos + 8 > len(blob):
            raise SystemExit(f"{path}: truncated chunk header at offset {pos}")
        (length,) = struct.unpack(">I", blob[pos : pos + 4])
        ctype = blob[pos + 4 : pos + 8]
        end = pos + 8 + length
        if end + 4 > len(blob):
            raise SystemExit(f"{path}: chunk {ctype!r} at {pos} runs past end of file")
        data = blob[pos + 8 : end]
        (crc,) = struct.unpack(">I", blob[end : end + 4])
        want = zlib.crc32(ctype + data) & 0xFFFFFFFF
        if crc != want:
            raise SystemExit(
                f"{path}: chunk {ctype!r} at {pos} has CRC {crc:#010x}, "
                f"computed {want:#010x}"
            )
        yield ctype, data
        seen_end = ctype == b"IEND"
        pos = end + 4
    if not seen_end:
        raise SystemExit(f"{path}: no IEND chunk")


def is_stamp(ctype, data):
    keyword = data.split(b"\x00", 1)[0]
    return (ctype, keyword) in STAMP_CHUNKS


def encode_chunk(ctype, data):
    return (
        struct.pack(">I", len(data))
        + ctype
        + data
        + struct.pack(">I", zlib.crc32(ctype + data) & 0xFFFFFFFF)
    )


def strip(path, check):
    """Strip `path` in place. Returns the list of dropped chunk types."""
    blob = path.read_bytes()
    kept, dropped = [], []
    for ctype, data in parse_chunks(blob, path):
        if is_stamp(ctype, data):
            dropped.append(ctype.decode("ascii"))
        else:
            kept.append(encode_chunk(ctype, data))
    if dropped and not check:
        path.write_bytes(SIGNATURE + b"".join(kept))
    return dropped


def collect(args):
    paths = []
    for arg in args:
        p = Path(arg)
        if p.is_dir():
            paths.extend(sorted(p.glob("*.png")))
        else:
            paths.append(p)
    return paths


def main():
    args = sys.argv[1:]
    check = "--check" in args
    targets = collect([a for a in args if a != "--check"])
    if not targets:
        raise SystemExit("usage: strip_png_stamps.py [--check] <path|dir> [...]")
    stamped = 0
    for path in targets:
        dropped = strip(path, check)
        if dropped:
            stamped += 1
            verb = "carries" if check else "stripped"
            print(f"  {verb} {path}: {', '.join(sorted(set(dropped)))}")
    noun = "file" if stamped == 1 else "files"
    if check:
        print(f"strip_png_stamps --check: {stamped}/{len(targets)} {noun} stamped")
        if stamped:
            raise SystemExit(1)
    else:
        print(f"strip_png_stamps: {stamped}/{len(targets)} {noun} stripped")


if __name__ == "__main__":
    main()
