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
# A HAND-WRITTEN CENSUS IS CHECKED IN BOTH DIRECTIONS, or it is only a
# census of what someone remembered. A listed sheet the tour did not
# write refuses below (a stale committed sheet is the failure mode the
# render lanes' provenance work exists to prevent). The other direction
# is the one that bites quietly: a sheet the tour DOES write and this
# list does not name is simply never published, and nothing downstream
# can tell — `render.yml`'s drift step runs `git status` over
# `renders-mc/`, which sees files that were copied and cannot see a
# file that never was. So the glob is read too, and an unlisted sheet
# is an error with the line to add.
SHEETS=(plate-density.svg chain-density.svg)

mkdir -p renders-mc

shopt -s nullglob
WROTE=(out/mc/*.svg)
shopt -u nullglob
for path in "${WROTE[@]}"; do
    sheet=$(basename "$path")
    listed=no
    for known in "${SHEETS[@]}"; do
        # `if` and not `[ … ] && listed=yes`: under `set -e` the latter
        # is the loop body's last command, so a sheet that does not
        # match the FIRST name would exit the script with 1.
        if [ "$sheet" = "$known" ]; then
            listed=yes
        fi
    done
    if [ "$listed" = no ]; then
        echo "render-mc.sh: the tour wrote $path and SHEETS does not name it, so it would" >&2
        echo "  never be published and nothing downstream could tell. Add it:" >&2
        echo "      SHEETS=(${SHEETS[*]} $sheet)" >&2
        exit 2
    fi
done

# REFUSE rather than leave the committed sheet standing. A stale sheet
# that nothing regenerated is exactly the failure mode the render
# lanes' provenance work exists to prevent, one lane over: the file
# would still be there, still look current, and describe a tour that no
# longer ran. Refusing on the FIRST missing sheet is deliberate — a run
# that wrote one sheet and not the other did not half-succeed, it
# failed partway through the tour.
#
# CHECKED BEFORE THE FIRST COPY, not as the loop reaches each one. When
# the check rode the copy loop, a missing SECOND sheet exited 2 with
# the first already overwritten — a partial publish, which is the state
# this refusal exists to avoid, reached by the refusal itself.
for sheet in "${SHEETS[@]}"; do
    if [ ! -f "out/mc/$sheet" ]; then
        echo "render-mc.sh: out/mc/$sheet is missing — run the tour first:" >&2
        echo "  cd demos/tour && cargo run --release -- ../out" >&2
        echo "  (nothing was published: the sheets are checked before any is copied)" >&2
        exit 2
    fi
done

for sheet in "${SHEETS[@]}"; do
    cp "out/mc/$sheet" "renders-mc/$sheet"
    echo "render-mc.sh: renders-mc/$sheet ($(wc -c < "renders-mc/$sheet") bytes)"
done
