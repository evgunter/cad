#!/usr/bin/env bash
# Publish the Monte-Carlo density sheets -> renders-mc/*.svg
#
# The fourth lane, and the second one with NO renderer dependency at
# all. `render.sh`'s two lanes draw 3-D and need headless FreeCAD;
# `render-uv.sh` and this one draw geometry that is already 2-D, so
# the tour writes the SVGs itself and this script only publishes them.
#
# There is still no compose step, and the second MC cell is why rather
# than why not. This header used to say "a second MC cell is when a
# composer gets written": there are two now (the two-hole plate and the
# four-link chain), and a composer would still be tiling two sheets
# that are already laid out where their own numbers are measured, at
# two different aspect ratios, for a montage nothing reads. What the
# second cell actually needed was for the ONE sheet in `SHEETS` to
# become a list.
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

# Every sheet the tour's MC cells write, by basename. One line per
# cell: the tour names the file and this publishes it under the same
# name, so a third cell is a third line here and nothing else.
#
# publish_lane.py checks this census against what the tour wrote, in
# both directions, before it publishes anything.
SHEETS=(plate-density.svg chain-density.svg)

names=()
for sheet in "${SHEETS[@]}"; do
    names+=("${sheet%.svg}")
done
python3 publish_lane.py --frame=.svg --no-provenance out/mc renders-mc "${names[@]}"
