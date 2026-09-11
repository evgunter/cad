#!/usr/bin/env python3
"""Compose the UV trim-loop contact sheet from the tour's per-face SVGs.

The third montage lane, beside `renders/montage.png` (the kernel's own
tessellation) and `renders-freecad/montage-freecad.png` (OCC's). Unlike
those two this lane draws no 3-D at all: each cell is one face's `(u, v)`
chart domain with its trim loops on it, which needs no camera, no
projection and no silhouettes — and therefore no renderer. Python
stdlib only: no venv, no numpy, no matplotlib, no FreeCAD.

The sheet is SVG, not PNG, on purpose: it is text, so a re-run that
changes nothing produces a byte-identical file and `git status` stays
clean without the wall-clock-stamp surgery the PNG lanes need
(`strip_png_stamps.py`).

SELECTION, STATED. `out/uv.json` carries EVERY face of every tour body
— four figures, and growing with the corpus. A sheet with one cell per
face is not a sheet, so this
composer takes one representative per (body, chart kind) among the
CURVED charts — the richest one, by distinct pcurve forms then loop
count then face ordinal — and every face whose trim walk FAILED,
unconditionally. Planar charts are dropped as a class: a plane chart's
picture is the face's own outline, which the two 3-D lanes already
show. Every count is printed; nothing is truncated silently.

Usage: python3 compose_uv_montage.py <outdir> <sheetdir>
       python3 compose_uv_montage.py --selftest
"""

import json
import re
import sys
import tempfile
from pathlib import Path

COLS = 4
CELL_W = 380
CELL_H = 334
MARGIN = 18
# Title block + the legend, which is `len(LEGEND)` two-per-row entries
# ending at `MARGIN + 78 + (rows - 1) * 15`; the cells start below that.
HEADER = 145

# The exact opening tag `uvdump.rs` writes. Matched rather than parsed:
# the emitter and this composer are one format, and a cell that does not
# match is a contract break worth failing on, not worth guessing around.
ROOT_RE = re.compile(
    r'^<svg xmlns="http://www\.w3\.org/2000/svg" '
    r'width="(\d+)" height="(\d+)" viewBox="0 0 \1 \2">\n'
)

HERE = Path(__file__).resolve().parent
# The emitter this sheet composes. `legend_colors_in_emitter()` reads it.
EMITTER = HERE / "tour" / "src" / "uvdump.rs"

# (swatch color, dashed?, text) — laid out two per row under the title.
#
# Every swatch color here is a stroke `uvdump.rs` actually writes, and
# the selftest checks that by reading the emitter: a legend that names
# a color no cell contains teaches the reader something false, which is
# what the fourth row used to do (a grey `#777777` swatch, while a
# derived pcurve is dashed in its own FORM's color).
LEGEND = [
    ("#1f4e79", False, "harmonic — the closed-form chart image"),
    ("#b03060", True, "the chart’s periodic seam (u = k·2π)"),
    ("#1b7a3d", False, "isoline — the exact u/v-const image (NURBS walls)"),
    ("#1f4e79", True, "dashed, in the form’s own color — pcurve DERIVED, not cached"),
    ("#6b2fa0", False, "isoarc — the rational cap rim, uneven in u"),
    ("#333", False, "dot — a loop junction;  arrow — traversal direction"),
    ("#c1590a", False, "fitted — the rung-3 SSI-trace chart projection"),
    ("#a01c3c", False, "general — a curve-in-UV with no construction provenance"),
]


def _hex(color):
    """`#abc` and `#AABBCC` are the same color; compare colors, not spellings."""
    c = color.lstrip("#").lower()
    return "".join(ch * 2 for ch in c) if len(c) == 3 else c


def emitter_colors(emitter=None):
    """(every stroke literal, the pcurve-FORM palette) in uvdump.rs.

    The form palette is the `const COLOR_*` block — one entry per
    `Pcurve` form, which is exactly what this legend enumerates.
    """
    src = (emitter or EMITTER).read_text()
    strokes = {_hex(c) for c in re.findall(r'"(#[0-9a-fA-F]{3,6})"', src)}
    forms = {
        _hex(c) for c in re.findall(r'const COLOR_\w+: &str = "(#[0-9a-fA-F]{3,6})"', src)
    }
    return strokes, forms


def legend_agrees_with_emitter(emitter=None):
    """Both directions of the legend/emitter mirror. Returns (unused, unlisted).

    Read rather than asked for in a comment: this legend mirrors literals
    in another language in another file, and it was the one mirror here
    that nothing verified while the cell SIZE was verified hard
    (`ROOT_RE` and the CELL_W/CELL_H refusal below).

    - `unused` — legend swatches the emitter never strokes. That is the
      one that had already drifted: a grey `#777777` swatch for "derived",
      while a derived pcurve is dashed in its own FORM's color.
    - `unlisted` — pcurve-form colors the emitter defines and the legend
      omits, so a fourth form cannot be added silently.

    **Blind spot, stated.** The reverse direction covers the `const
    COLOR_*` palette only, NOT every stroke in the file: `#b8b8b8` (cell
    border) and `#d0d0d0` (axis box) are chrome and have no business in a
    legend, so a whole-set comparison would fail on correct code. A new
    form color written INLINE rather than as a `const` is therefore still
    invisible here — and it would be invisible to the emitter's own
    `match tr.form` too, which is where that would be caught first.
    """
    strokes, forms = emitter_colors(emitter)
    listed = {_hex(color) for color, _, _ in LEGEND}
    return sorted(listed - strokes), sorted(forms - listed)


def pick(entries):
    """One representative per (body, chart), plus every failed walk.

    Returns (cells, dropped_curved, dropped_planar).
    """
    curved = [e for e in entries if e["curved"] and not e["note"]]
    planar = [e for e in entries if not e["curved"]]
    failed = [e for e in entries if e["note"]]

    groups = {}
    for e in curved:
        groups.setdefault((e["body"], e["chart"]), []).append(e)
    reps = []
    for key in sorted(groups):
        # Richest cell of the group: most distinct pcurve forms, then
        # most loops, then the lowest face ordinal for determinism.
        best = sorted(
            groups[key],
            key=lambda e: (-len(e["forms"]), -e["loops"], e["face"]),
        )[0]
        reps.append(best)
    # Failures first — they are the reason this lane exists.
    cells = sorted(failed, key=lambda e: (e["body"], e["face"])) + reps
    return cells, len(curved) - len(reps), len(planar)


def cell_svg(path, x, y):
    """One cell, placed at (x, y).

    A translate group rather than a nested `<svg x= y=>`: both are valid
    SVG 1.1 and browsers render either, but ImageMagick's built-in
    renderer silently ignores the nested form's placement and stacks
    every cell at the origin. The cells are already drawn at exactly
    CELL_W x CELL_H with nothing outside that box, so the nested
    element's only extra service — clipping — buys nothing here.
    """
    text = path.read_text()
    m = ROOT_RE.match(text)
    if not m:
        raise SystemExit(
            f"{path}: does not start with the cell root tag uvdump.rs writes "
            f"— the emitter and this composer are one format, so this is a "
            f"contract break, not something to guess around"
        )
    w, h = int(m.group(1)), int(m.group(2))
    if (w, h) != (CELL_W, CELL_H):
        raise SystemExit(f"{path}: cell is {w}x{h}, this sheet lays out {CELL_W}x{CELL_H}")
    body = text[m.end() :]
    body = body.rstrip()
    if not body.endswith("</svg>"):
        raise SystemExit(f"{path}: cell does not end with </svg>")
    body = body[: -len("</svg>")]
    return f'<g transform="translate({x}, {y})">\n{body}</g>\n'


def esc(s):
    return s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def entry(body, chart, face, curved=True, forms=("harmonic",), loops=1, note=None):
    """A manifest entry, for the selftest."""
    return {
        "body": body, "chart": chart, "face": face, "curved": curved,
        "forms": list(forms), "loops": loops, "note": note,
        "svg": f"{body}.f{face:03}.svg",
    }


def selftest():
    """The two claims the sheet makes about what it shows.

    Worth a test because both are claims about what is NOT on the sheet,
    and a silent drop is the failure mode this lane exists to prevent.
    The emitter's own failure cell (`uvdump::draw_failure`) has no live
    fixture: no face in the M7 corpus fails its trim walk, and forging
    one would mean hand-building a corrupt body — so the arm is checked
    here only from the composer's side, which is where a real failure
    would have to survive to reach a reader.
    """
    # 1. One representative per (body, chart); planar dropped as a class;
    #    every failure kept and sorted first, whatever its chart.
    entries = [
        entry("a", "cylinder", 0),
        entry("a", "cylinder", 1, forms=("harmonic", "isoline")),  # richer -> wins
        entry("a", "torus", 2),
        entry("b", "plane", 0, curved=False),
        entry("b", "nurbs", 1, note="walk exploded"),
    ]
    cells, dropped_curved, dropped_planar = pick(entries)
    assert [(c["body"], c["chart"], c["face"]) for c in cells] == [
        ("b", "nurbs", 1), ("a", "cylinder", 1), ("a", "torus", 2)
    ], cells
    assert (dropped_curved, dropped_planar) == (1, 1), (dropped_curved, dropped_planar)

    # 2. A cell that does not match the emitter's root tag fails loud
    #    rather than being guessed around or skipped.
    with tempfile.TemporaryDirectory() as d:
        good = Path(d) / "good.svg"
        good.write_text(
            f'<svg xmlns="http://www.w3.org/2000/svg" width="{CELL_W}" '
            f'height="{CELL_H}" viewBox="0 0 {CELL_W} {CELL_H}">\n<rect/>\n</svg>\n'
        )
        placed = cell_svg(good, 7, 9)
        assert placed.startswith('<g transform="translate(7, 9)">'), placed
        assert "<rect/>" in placed and placed.rstrip().endswith("</g>"), placed

        bad = Path(d) / "bad.svg"
        bad.write_text('<svg width="10" height="10">\n</svg>\n')
        try:
            cell_svg(bad, 0, 0)
        except SystemExit:
            pass
        else:
            raise AssertionError("a non-conforming cell must fail loud")

    # 3. The legend and the emitter agree, in both directions.
    if not EMITTER.exists():  # a checkout without the tour source
        raise SystemExit(f"{EMITTER} not found — the legend check cannot run")
    unused, unlisted = legend_agrees_with_emitter()
    assert not unused, f"legend swatches the emitter never strokes: {unused}"
    assert not unlisted, f"pcurve-form colors missing from the legend: {unlisted}"

    # And the comparison is over colors, not spellings: `#333` is the
    # color `#333333`, so a cosmetic re-spelling in the emitter must not
    # fail this gate.
    assert _hex("#333") == _hex("#333333") == "333333"

    print("compose_uv_montage selftest: ok (3 cases)")


def main():
    if sys.argv[1:] == ["--selftest"]:
        selftest()
        return
    if len(sys.argv) != 3:
        raise SystemExit(__doc__)
    outdir, sheetdir = Path(sys.argv[1]), Path(sys.argv[2])
    manifest = outdir / "uv.json"
    if not manifest.exists():
        raise SystemExit(
            f"{manifest} not found — run the tour first:\n"
            f"  cd demos/tour && cargo run --release -- ../{outdir.name}"
        )
    entries = json.loads(manifest.read_text())
    cells, dropped_curved, dropped_planar = pick(entries)
    if not cells:
        raise SystemExit("no cells selected — the tour produced no curved faces")

    rows = (len(cells) + COLS - 1) // COLS
    width = MARGIN * 2 + COLS * CELL_W
    height = MARGIN * 2 + HEADER + rows * CELL_H

    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" '
        f'viewBox="0 0 {width} {height}">',
        f'<rect x="0" y="0" width="{width}" height="{height}" fill="#ffffff"/>',
        f'<text x="{MARGIN}" y="{MARGIN + 22}" font-family="sans-serif" font-size="19" '
        f'font-weight="bold" fill="#111">UV trim loops — a face is a bounded patch of an '
        f'UNBOUNDED surface; this is that boundary, drawn in the surface’s own chart</text>',
        f'<text x="{MARGIN}" y="{MARGIN + 41}" font-family="sans-serif" font-size="12" '
        f'fill="#555">Kernel-only lane: no camera, no projection, no silhouettes, no external '
        f'renderer. Compare renders/montage.png (our tessellation) and '
        f'renders-freecad/montage-freecad.png (OCC’s).</text>',
        f'<text x="{MARGIN}" y="{MARGIN + 58}" font-family="monospace" font-size="11" '
        f'fill="#555">{len(cells)} cell(s) = one representative per (body, chart) + every failed '
        f'walk; {dropped_curved} further curved and {dropped_planar} planar face(s) not shown '
        f'(all {len(entries)} are in out/uv/)</text>',
    ]
    for i, (color, dashed, text) in enumerate(LEGEND):
        x = MARGIN + (i % 2) * 400
        y = MARGIN + 78 + (i // 2) * 15
        dash = ' stroke-dasharray="5 3"' if dashed else ""
        parts.append(
            f'<line x1="{x}" y1="{y - 4}" x2="{x + 22}" y2="{y - 4}" stroke="{color}" '
            f'stroke-width="2.4"{dash}/>'
            f'<text x="{x + 28}" y="{y}" font-family="monospace" font-size="10.5" '
            f'fill="#444">{esc(text)}</text>'
        )

    uvdir = outdir / "uv"
    for i, e in enumerate(cells):
        x = MARGIN + (i % COLS) * CELL_W
        y = MARGIN + HEADER + (i // COLS) * CELL_H
        parts.append(cell_svg(uvdir / e["svg"], x, y))
    parts.append("</svg>\n")

    sheetdir.mkdir(parents=True, exist_ok=True)
    sheet = sheetdir / "montage-uv.svg"
    sheet.write_text("\n".join(parts))
    print(
        f"wrote {sheet} — {len(cells)} cell(s) in {rows} row(s); "
        f"{dropped_curved} curved + {dropped_planar} planar face(s) not on the sheet "
        f"(all {len(entries)} SVGs are in {uvdir}/)"
    )
    failed = [e for e in cells if e["note"]]
    if failed:
        print(f"{len(failed)} face(s) could not be walked — first cells on the sheet:")
        for e in failed:
            print(f"  {e['body']} face {e['face']} ({e['chart']}): {e['note']}")


if __name__ == "__main__":
    main()
