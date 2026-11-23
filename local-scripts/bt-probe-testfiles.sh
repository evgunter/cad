#!/usr/bin/env bash
# Temporary investigation helper (NOT for merge).
# Test files that use the Probe SCALAR in code (not just in prose).
# Strips //-comment lines and //! doc lines before matching, so files that
# merely discuss "Probe 1 / probe points" are not swept up.
cd "$(dirname "$0")/.."
for f in $(grep -rl "Probe\|start_recording\|take_samples\|SampleOutcome\|MarginSample" --include='*.rs' crates/*/tests | sort); do
  hits=$(sed -E 's://.*::' "$f" \
    | grep -cE '\bProbe\b|\bstart_recording\b|\btake_samples\b|\bSampleOutcome\b|\bMarginSample\b')
  [ "$hits" -gt 0 ] && printf '%-56s %s\n' "$f" "$hits"
done
