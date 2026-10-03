#!/usr/bin/env bash
# Photograph the VIEWER -> renders-gui/ (cells + montage-gui.png)
#
# The fifth montage lane, and the first one whose subject is the
# APPLICATION rather than a body. The other four draw geometry: the two
# PNG lanes render solids, the uv lane draws each face's chart, the mc
# lane plots a sample cloud. This one opens documents in the real
# `viewer` binary on a virtual X server and screenshots what a user
# would see — the feature tree, the properties panel, and the status
# line's reading of the document's declarations.
#
# WHY A LANE AND NOT A LOCAL PASS. Same reason as every other one
# (demos/hosted-render-guard.sh): these are COMMITTED pixels, and a
# local pass puts this box's GL stack into the repo. The viewer draws
# through lavapipe, a software Vulkan rasteriser, so what it produces
# depends on the mesa the runner happens to carry — exactly the drift
# the guard exists to keep out of tracked files.
#
# WHAT IT NEEDS, and each package earns its line (crates/viewer/README.md
# carries the full account):
#
#   libxkbcommon-x11-0   without it winit panics BEFORE the window
#                        exists, so there is nothing to photograph
#   mesa-vulkan-drivers  supplies lavapipe, the software ICD wgpu lands
#                        on. `WGPU_BACKEND=gl` is a DEAD END: it refuses
#                        with CreateSurfaceError(Hal(FailedToCreate...))
#   libegl1 / libgl1-mesa-dri   the ICD's own loader path
#   xvfb                 the virtual X server
#   imagemagick          `import` takes the shot; `montage` tiles it
#
# `libEGL warning: DRI3 error` on stderr is EXPECTED NOISE on a virtual
# server with no DRI3 device, and is not a failure.
#
# BUILD RELEASE. In a debug build the viewer's tessellation and BVH
# index build take minutes, which is indistinguishable from a hang and
# would make every settle-poll below time out.
#
# Inputs come from the tour, which must have run first — this lane
# photographs the assembly stop's own document store:
#   cd demos/tour && cargo run --release -- ../out
set -euo pipefail
cd "$(dirname "$0")"

# Hosted is the default renderer; this refuses without the explicit
# preview-only override. See demos/hosted-render-guard.sh.
# shellcheck source=demos/hosted-render-guard.sh
. ./hosted-render-guard.sh
require_hosted_render "demos/render-gui.sh"

# THE LANE'S SIGNATURE, stamped into every cell AND into the sheet.
#
# `check_render_provenance.py` reads this exact line back out of this
# file and compares it against its own constant, so the two spellings
# cannot drift apart without the selftest saying so — the same
# arrangement `render-wild.sh`'s AUTHOR has, and for the same reason: a
# signature that drifts turns every committed cell in the lane into a
# violation at once.
#
# Stamping the SHEET too is a deliberate difference from the older
# lanes. There the contact sheets are matplotlib-composed and therefore
# carry no renderer signature, so the guard needs an exemption list for
# them by name. Here the sheet is tiled from cells this lane drew and
# then stamped like one, so it needs no exemption and the guard has one
# rule for the whole directory.
AUTHOR='pncad viewer lane (headless Xvfb + lavapipe screenshot of the real app)'

exec python3 render_gui.py --author "$AUTHOR" out renders-gui
