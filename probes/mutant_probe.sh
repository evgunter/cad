#!/bin/sh
# Apply one mutant from probes/mutants.py, dump the reviewer probe's poses,
# restore. Usage: probes/mutant_probe.sh "<mutant name prefix>" out.jsonl
python3 - "$1" <<'PY'
import sys; sys.path.insert(0, "probes"); import mutants as m
for k,(f,o,n) in m.MUTANTS.items():
    if k.startswith(sys.argv[1]):
        s=open(f).read(); open(f+".orig","w").write(s); open(f,"w").write(s.replace(o,n)); open("/tmp/mutfile","w").write(f)
PY
PROBE_PER=${PROBE_PER:-10} CARGO_INCREMENTAL=0 cargo nextest run -p topo --lib --run-ignored only probe_ellipse_torus --no-capture 2>&1 | grep "^PROBE" > "$2"
f=$(cat /tmp/mutfile); mv "$f.orig" "$f"
