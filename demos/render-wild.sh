#!/usr/bin/env bash
# Render the wild-corpus montage (demos/wild/) — KERNEL-TESSELLATION
# LANE ONLY, and FreeCAD-FREE by ratified scope: the cells are
# third-party STEP files imported by `step-import` and tessellated by
# the kernel's own tessellator, drawn by `render.py` (the numpy +
# matplotlib STL renderer). matplotlib is the PRIMARY renderer here,
# not a fallback — there is deliberately no FreeCAD import and no OCC
# comparison lane for these files.
#
#   cd wild && cargo run --release -- out   # import + tessellate + STL + scenes.json
#   cd .. && ./render-wild.sh               # cells + sheet -> renders-wild/
#
# The cell set is governed by docs/WILD-CORPUS-LICENSES.md (the
# license audit): the generator pins it (8 cells) and fails loud on
# drift, and the attribution block rides demos/README.md.
#
# The lane keeps the committed-tree contracts of render.sh:
#   * STAGING — cells render into out/stage/renders-wild/ (untracked)
#     and are published to renders-wild/ only when the whole pass
#     succeeded, so a failed pass leaves the committed tree untouched;
#   * PROVENANCE — every cell is stamped with the wild lane's Author
#     signature (render.py --author), and check_render_provenance.py
#     certifies the published cells BEFORE the sheet is composed. A
#     tour fallback frame (matplotlib, unstamped) or a FreeCAD frame
#     cannot pass as a wild cell;
#   * BYTE-STABILITY — matplotlib stamps no wall clock, so an
#     unchanged re-render is byte-identical and a dirty `git status`
#     after a re-render means the pixels changed.
set -euo pipefail
cd "$(dirname "$0")"

VENV=.venv
if [ ! -x "$VENV/bin/python" ]; then
    if command -v uv >/dev/null 2>&1; then
        uv venv --python 3.12 "$VENV"
        uv pip install --python "$VENV/bin/python" \
            'numpy==2.2.6' 'matplotlib==3.10.3'
    else
        python3 -m venv "$VENV"
        "$VENV/bin/pip" install 'numpy==2.2.6' 'matplotlib==3.10.3'
    fi
fi

WILD_OUT=wild/out
RD=renders-wild
STAGE=out/stage/$RD
# Keep in sync with check_render_provenance.py's WILD_AUTHOR.
AUTHOR='pncad wild-corpus lane (kernel tessellation of licensed third-party STEP)'

if [ ! -f "$WILD_OUT/scenes.json" ]; then
    echo "no $WILD_OUT/scenes.json — run the generator first:" >&2
    echo "  (cd demos/wild && cargo run --release -- out)" >&2
    exit 1
fi

rm -rf "$STAGE"
mkdir -p "$STAGE"

# Every cell, stamped with the lane's provenance signature. render.py
# fails loud on a missing/broken STL; set -e ends the pass with the
# committed tree untouched.
"$VENV/bin/python" render.py "$WILD_OUT" "$STAGE" "--author=$AUTHOR"

# Publish the completed pass WHOLESALE (the pass rendered every
# pinned cell, so the lane directory goes from one whole pass to the
# next — a renamed or retired cell cannot leave a stale frame behind),
# certify it, then compose the sheet from the certified cells (the
# render.sh ordering, lane for lane).
mkdir -p "$RD"
rm -f "$RD"/*.png
mv -f "$STAGE"/*.png "$RD/"
"$VENV/bin/python" check_render_provenance.py "$RD"
exec "$VENV/bin/python" compose_montage.py "$WILD_OUT" "$RD" \
    --montage=montage-wild.png \
    '--title=Wild corpus — third-party STEP imported and tessellated by the kernel' \
    '--banner=kernel render — step-import + the kernel'\''s own tessellation; no FreeCAD/OCC lane (licenses: docs/WILD-CORPUS-LICENSES.md)'
