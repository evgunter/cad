#!/usr/bin/env python3
"""Temporary investigation helper (NOT for merge).

For each Probe-touching test file, report every `#[test]` item and whether
its body mentions the Probe scalar — so a whole-file gate is only used
where EVERY test needs Probe, and mixed files get split instead of
silently losing their non-Probe coverage from the default lane.
"""

import pathlib
import re

PROBE = re.compile(r"\bProbe\b|\bstart_recording\b|\btake_samples\b|\bSampleOutcome\b|\bMarginSample\b")

FILES = [
    "crates/editor-core/tests/m4_pr8_k_probe.rs",
    "crates/editor-core/tests/m5_pr5_corpus.rs",
    "crates/geom-brep/tests/rim_dim_review_probes.rs",
    "crates/geom-brep/tests/rim_dim_scale_twins.rs",
    "crates/profile/tests/review_m2_pr2.rs",
    "crates/profile/tests/review_s2.rs",
    "crates/profile/tests/scalar_channels.rs",
    "crates/profile/tests/validate_ok.rs",
    "crates/sweep/tests/k_report.rs",
    "crates/topo/tests/probe_census.rs",
    "crates/topo/tests/review_m3_pr2.rs",
    "crates/topo/tests/rim_dim_boolean_twins.rs",
    "crates/topo/tests/rim_dim_review_probes.rs",
]

for f in FILES:
    src = pathlib.Path(f).read_text()
    # split on test attributes; crude but enough to classify
    parts = re.split(r"\n(?=\s*#\[(?:test|ignore)\])", src)
    tests, probe_tests = 0, 0
    for part in parts[1:]:
        name = re.search(r"fn\s+(\w+)", part)
        if not name:
            continue
        tests += 1
        # body = up to the next test attribute (already split)
        body = re.sub(r"^\s*///.*$", "", part, flags=re.M)
        if PROBE.search(body):
            probe_tests += 1
    verdict = (
        "WHOLE-FILE ok" if tests and probe_tests == tests
        else "SPLIT — has non-Probe tests" if tests else "no #[test] items"
    )
    print(f"{f:<52} tests={tests:<3} probe={probe_tests:<3} {verdict}")
