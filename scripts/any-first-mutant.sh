#!/usr/bin/env bash
# any-first-mutant.sh — every pin `scripts/gates/any-climb-pins.counts`
# names goes RED under the first-refusal mutant. The nightly runs it
# (`.github/workflows/nightly.yml`, job `any-first-mutant`); by hand, run
# it from the repository root.
#
# THE RULE. A pin for an `Answering::Any` ladder is a test that reads the
# production function with a reading kept ahead of an answer, red under a
# first-refusal loop. The gate checks that each pin names a test; only
# running it under that loop checks that it reaches the answer past a
# kept rung. So this builds with `--cfg any_first_mutant`, under which
# `geom_core::ladder::climb_evidence` refuses an `Any` ladder at its first
# kept rung, runs every registered pin, and fails on each one that passes
# (it does not pin its site) or that no package ran (it names no test).
#
# The mutant build goes to its own target dir
# (`${CARGO_TARGET_DIR:-target}/any-first-mutant`), so it does not
# rebuild the plain one. Slow tests a pin names run too
# (`--run-ignored all`, `--ignore-default-filter`).
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

REGISTER=scripts/gates/any-climb-pins.counts

# The pins, and the package of the crate each is declared in.
plan=$(python3 - "$REGISTER" <<'PY'
import glob, re, sys, tomllib
pins = []
with open(sys.argv[1]) as fh:
    for raw in fh:
        line = raw.split("#", 1)[0].split()
        if len(line) == 5 and line[3] == "pin" and line[4] not in pins:
            pins.append(line[4])
packages = set()
for pin in pins:
    pat = re.compile(r"\bfn\s+" + re.escape(pin) + r"\b")
    for p in glob.glob("crates/*/**/*.rs", recursive=True):
        with open(p, errors="replace") as fh:
            if pat.search(fh.read()):
                with open("/".join(p.split("/")[:2]) + "/Cargo.toml", "rb") as m:
                    packages.add(tomllib.load(m)["package"]["name"])
print(" ".join(sorted(packages)))
print(" ".join(pins))
PY
)
packages=$(sed -n 1p <<<"$plan")
pins=$(sed -n 2p <<<"$plan")
if [ -z "$pins" ]; then
  echo "any-first-mutant: $REGISTER names no pin" >&2
  exit 1
fi
args=()
for p in $packages; do args+=(-p "$p"); done
filter=$(tr ' ' '|' <<<"$pins")

target="${CARGO_TARGET_DIR:-target}/any-first-mutant"
log=$(mktemp)
trap 'rm -f "$log"' EXIT
set +e
RUSTFLAGS="${RUSTFLAGS:-} --cfg any_first_mutant" CARGO_TARGET_DIR="$target" \
  cargo nextest run "${args[@]}" --no-fail-fast --run-ignored all --ignore-default-filter \
  --success-output never --failure-output never \
  -E "test(/::($filter)\$/)" >"$log" 2>&1
set -e

python3 - "$log" $pins <<'PY'
import re, sys
log = open(sys.argv[1], errors="replace").read()
pins = sys.argv[2:]
ran = {}
for m in re.finditer(r"^\s*(PASS|FAIL|SIGABRT|SIGSEGV|TIMEOUT)\b.*?::([A-Za-z_0-9]+)\s*$", log, re.M):
    ran.setdefault(m.group(2), set()).add(m.group(1))
bad = []
for pin in pins:
    got = ran.get(pin)
    if not got:
        bad.append(f"{pin}: no package ran it (a pin must name a test)")
    elif "PASS" in got:
        bad.append(f"{pin}: green under the first-refusal mutant, so it does not reach the answer past a kept rung")
if not ran:
    print(log[-4000:])
    print("any-first-mutant: the mutant run read no test (the build failed?)", file=sys.stderr)
    sys.exit(1)
if bad:
    print("\n".join(bad))
    print(f"any-first-mutant: {len(bad)} of {len(pins)} pins do not pin (above)", file=sys.stderr)
    sys.exit(1)
print(f"any-first-mutant OK: all {len(pins)} pins are red under the first-refusal mutant")
PY
