#!/usr/bin/env bash
# Render the demo-tour STLs (demos/out/) to PNGs (demos/renders/).
# Creates a demo-local Python venv on first run (numpy + matplotlib,
# pinned; both releases are years past the repo's 2-week age policy).
# Prefers uv (fast, brings its own CPython); falls back to python3.
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

exec "$VENV/bin/python" render.py out renders
