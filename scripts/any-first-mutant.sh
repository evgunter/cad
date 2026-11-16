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
# kept rung (as `climb_agreeing` and `climb_differing` do), runs every
# registered pin, and fails on each one that passes (it does not pin its
# site) or that no package ran (it names no test). A control run of the
# same pins without the mutant comes first, and a pin red there fails too:
# a red with another cause (a main red) says nothing about the mutant.
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

control=$(mktemp)
mutant=$(mktemp)
trap 'rm -f "$control" "$mutant"' EXIT
run() { # run LOG [RUSTFLAGS…]: every pin, its outcomes to LOG
  local log=$1
  shift
  set +e
  RUSTFLAGS="${RUSTFLAGS:-} $*" cargo nextest run "${args[@]}" --no-fail-fast \
    --run-ignored all --ignore-default-filter \
    --success-output never --failure-output never \
    -E "test(/::($filter)\$/)" >"$log" 2>&1
  set -e
}
# The control: every pin, plain. A pin red without the mutant is red for
# some other reason (a main red), so its red under the mutant says
# nothing about the mutant; it fails here instead.
run "$control"
# The mutant, in its own target dir so it does not rebuild the plain one.
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-target}/any-first-mutant" run "$mutant" --cfg any_first_mutant

python3 - "$control" "$mutant" $pins <<'PY'
import re, sys
pins = sys.argv[3:]
def outcomes(path, what):
    log = open(path, errors="replace").read()
    ran = {}
    for m in re.finditer(r"^\s*(PASS|FAIL|SIGABRT|SIGSEGV|TIMEOUT)\b.*?::([A-Za-z_0-9]+)\s*$", log, re.M):
        ran.setdefault(m.group(2), set()).add(m.group(1))
    if not ran:
        print(log[-4000:])
        print(f"any-first-mutant: the {what} run read no test (the build failed?)", file=sys.stderr)
        sys.exit(1)
    return ran
control = outcomes(sys.argv[1], "control")
mutant = outcomes(sys.argv[2], "mutant")
bad = []
for pin in pins:
    plain, mutated = control.get(pin), mutant.get(pin)
    if not plain or not mutated:
        bad.append(f"{pin}: no package ran it (a pin must name a test)")
    elif plain != {"PASS"}:
        bad.append(f"{pin}: red without the mutant, so its red under it pins nothing; fix it first")
    elif "PASS" in mutated:
        bad.append(f"{pin}: green under the first-refusal mutant, so it does not reach the answer past a kept rung")
if bad:
    print("\n".join(bad))
    print(f"any-first-mutant: {len(bad)} of {len(pins)} pins do not pin (above)", file=sys.stderr)
    sys.exit(1)
print(f"any-first-mutant OK: all {len(pins)} pins are green plain and red under the first-refusal mutant")
PY
