#!/usr/bin/env bash
# Publish the Monte-Carlo density sheet -> renders-mc/plate-density.svg
#
# The fourth lane, and the second one with NO renderer dependency at
# all. `render.sh`'s two lanes draw 3-D and need headless FreeCAD;
# `render-uv.sh` and this one draw geometry that is already 2-D, so
# the tour writes the SVG itself and this script only publishes it.
#
# There is no compose step because there is nothing to tile: the cell
# is one sheet with two panels, and the tour lays it out where the
# numbers on it are measured. A second MC cell is when a composer gets
# written, not before.
#
# WHY IT IS SVG (the same three reasons as the uv lane): it is text, so
# an unchanged re-run is byte-identical and `git status` stays clean
# with none of the PNG lanes' wall-clock-stamp surgery; there is no
# second renderer for a provenance guard to tell apart, because the
# kernel is the only thing that could have drawn it; and 512 samples
# cost 1024 `<circle>` elements rather than 512 imported meshes.
#
# Inputs come from the tour, which must have run first:
#   cd demos/tour && cargo run --release -- ../out
set -euo pipefail
cd "$(dirname "$0")"

# Hosted is the default renderer; this refuses without the explicit
# preview-only override. See demos/hosted-render-guard.sh.
# shellcheck source=demos/hosted-render-guard.sh
. ./hosted-render-guard.sh
require_hosted_render "demos/render-mc.sh"

SRC=out/mc/plate-density.svg
DST=renders-mc/plate-density.svg

# REFUSE rather than leave the committed sheet standing. A stale sheet
# that nothing regenerated is exactly the failure mode the render
# lanes' provenance work exists to prevent, one lane over: the file
# would still be there, still look current, and describe a tour that
# no longer ran.
if [ ! -f "$SRC" ]; then
    echo "render-mc.sh: $SRC is missing — run the tour first:" >&2
    echo "  cd demos/tour && cargo run --release -- ../out" >&2
    exit 2
fi

mkdir -p renders-mc
cp "$SRC" "$DST"
echo "render-mc.sh: $DST ($(wc -c < "$DST") bytes)"
