#!/usr/bin/env python3
"""Temporary investigation helper (NOT for merge).

Mirrors each member's `interval = [...]` forwarding into a `probe = [...]`
entry, so a workspace-wide `--features probe` lane builds uniformly — the
same rationale those manifests already state for `interval`.

geom-core is skipped: its `probe = []` is hand-written with the full
rationale.
"""

import pathlib
import re

NOTE = (
    "# Forwards geom-core's K-telemetry recording scalar so the\n"
    "# workspace-wide `--features probe` lane builds uniformly. Same shape\n"
    "# and same reason as the `interval` row above; see geom-core's manifest\n"
    "# for why Probe is opt-in (it is a `Real` instantiation, so it\n"
    "# monomorphizes every generic-over-`Real` body, and only the K\n"
    "# experiment consumes it).\n"
)

for path in sorted(pathlib.Path("crates").glob("*/Cargo.toml")):
    if path.parent.name == "geom-core":
        continue
    text = path.read_text()
    if "\nprobe = " in text:
        print(f"skip (already has probe): {path}")
        continue
    m = re.search(r"^interval = (\[[^\]]*\]|\[\s*\n(?:.*\n)*?\])", text, re.M)
    if not m:
        print(f"skip (no interval feature): {path}")
        continue
    body = m.group(1)
    probe_body = body.replace("interval", "probe")
    insertion = f"{NOTE}probe = {probe_body}\n"
    text = text[: m.end()] + "\n" + insertion.rstrip("\n") + text[m.end() :]
    path.write_text(text)
    print(f"added probe to {path}")
