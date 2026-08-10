#!/usr/bin/env bash
# Compose the UV trim-loop contact sheet -> renders-uv/montage-uv.svg
#
# The third montage lane. The other two need an external renderer
# (`render.sh`: headless FreeCAD, matplotlib as an uncommittable
# fallback) because they draw 3-D. This one draws each face's own
# (u, v) chart, which is already 2-D — so it has NO renderer
# dependency at all: the tour writes the per-face SVGs, and this
# script tiles them with Python's standard library. No venv, no
# numpy/matplotlib, no freecadcmd, no provenance guard, no PNG
# wall-clock stamps to strip. The sheet is SVG, so an unchanged
# re-run is byte-identical.
#
# Inputs come from the tour, which must have run first:
#   cd demos/tour && cargo run --release -- ../out
set -euo pipefail
cd "$(dirname "$0")"

# Hosted is the default renderer; this refuses without the explicit
# preview-only override. See demos/hosted-render-guard.sh.
# shellcheck source=demos/hosted-render-guard.sh
. ./hosted-render-guard.sh
require_hosted_render "demos/render-uv.sh"

exec python3 compose_uv_montage.py out renders-uv
