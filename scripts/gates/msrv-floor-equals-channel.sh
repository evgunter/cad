#!/usr/bin/env bash
# msrv-floor-equals-channel.sh — `Cargo.toml`'s declared MSRV floor and
# `rust-toolchain.toml`'s pinned channel are THE SAME STRING.
# ONE home; ci.yml's "the MSRV floor is the pinned channel" step and
# local-scripts/ci-local.sh's discipline row both call this file.
#
# THE RULE. `[workspace.package] rust-version` is a floor a consumer's
# compiler must clear; `[toolchain] channel` is the compiler this
# workspace builds with (D9/L2's pin). They are different claims, and
# nothing reconciled them: the day the channel moves, the floor stays
# behind as a number nobody maintains and nothing compiles at. This
# gate settles that by holding them EQUAL.
#
# WHAT EQUALITY BUYS, and it is the whole argument for the gate: pinned
# equal, THE CHANNEL IS THE FLOOR. Every hosted job, every local gate
# row and `local-scripts/ci-local.sh` already build on the channel — at
# every lane and every eps point, on every run — so they already build
# at the floor. The promise `rust-version` makes to a consumer becomes
# exactly the promise CI already proves, with no new build row and no
# nightly. Without it the floor is untested by construction: no row
# compiles the workspace at a floor the channel has moved past.
#
# WHAT IT COSTS, written here so the deletion is deliberate when it
# comes. THIS GATE FORBIDS THE FLOOR FROM EVER LAGGING THE CHANNEL. The
# day this repository wants to say *"we build with 1.99 and still
# support 1.97"* — a real thing to want once **Q9** lands and something
# is published, since the workspace is `publish = false` and nameless
# until then — this gate is WRONG and must be deleted, not worked
# around. At that moment the question it closed comes back with a
# consumer attached to it: is the floor a promise, and what row compiles
# at it? That is the time to answer it and the wrong time to be
# surprised by it.
#
# WHY EXACT STRING EQUALITY, not a version comparison. The declaration
# is documentation of the pin, so what the gate holds is that it is a
# COPY of the pin — character for character. `1.97` and `1.97.0` name
# the same floor to cargo and are still a failure here, because a
# tolerance is a second rule to maintain and the only way the two
# spellings diverge is someone editing one file without reading the
# other, which is the event this gate is for.
#
# WHY A TOML PARSE, NOT A GREP — the argument is
# `kernel-serde-free.sh`'s header, and `rust-version` is one of the
# dotted-key fields it names. `rust-version = "1.97.0"` under a
# `[workspace.package]` header, `workspace.package.rust-version =
# "1.97.0"` as a bare dotted key, and a single-quoted literal string are
# the same document to cargo and three different regexes. A parser reads
# what the build reads, so the spelling stops mattering. The self-test
# plants the alternate spellings as PASSING fixtures, which is where
# that claim is checked rather than asserted.
#
# WHAT THIS DOES NOT PROVE: that either value is the right one. A PR
# moving both strings together to a channel that does not exist passes
# here and fails at the first `cargo` invocation, which is the right
# division of labour. Nor does it read a member's own `rust-version`: no
# member declares one — they inherit through `workspace = true` — and a
# member that overrode it would be making a per-crate floor claim this
# gate has no opinion about.
#
# WHEN THE CHANNEL STOPS BEING A VERSION. `channel = "stable"` or a
# dated nightly cannot be copied into `rust-version`, which cargo
# requires to be a bare version. Equality is then unsatisfiable, so the
# gate REFUSES rather than demanding a value cargo would reject — and
# that refusal is the same conversation as the deletion above, arriving
# from the other side.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

MANIFEST=Cargo.toml
TOOLCHAIN=rust-toolchain.toml

gate() {
  # A GATE THAT SCANNED NOTHING IS NOT A PASS, and this gate's subject
  # is two NAMED files: a renamed or deleted one would otherwise make it
  # green forever over a tree it never read.
  GATE_SCAN_NOUN="manifest"
  gate_require_file "$MANIFEST"
  gate_require_file "$TOOLCHAIN"
  GATE_SCAN_FILES=2

  local verdict
  # A parse failure, a missing key, a non-version channel or a missing
  # `tomllib` must FAIL, never pass quiet: a gate that cannot read its
  # subject has not cleared it. The `if` is what makes that a DIAGNOSIS
  # — a bare assignment dies here under errexit, and a CI reader gets
  # the reader's own sentence with no gate name and no `::error::`
  # annotation around it (S157).
  if ! verdict=$(python3 - "$MANIFEST" "$TOOLCHAIN" <<'PY'
import re, sys

GATE = "msrv-floor-equals-channel"

try:
    import tomllib
except ImportError:  # pragma: no cover - guarded, not silenced
    sys.exit(f"{GATE}: python3 has no tomllib (needs 3.11+) — "
             "the gate cannot read its subject, which is not a pass")

manifest, toolchain = sys.argv[1], sys.argv[2]


def load(path):
    try:
        with open(path, "rb") as fh:
            return tomllib.load(fh)
    except Exception as exc:
        sys.exit(f"{GATE}: {path} does not parse ({exc}) — "
                 "the gate cannot read its subject, which is not a pass")


def dig(doc, path, where, what):
    cur = doc
    for key in path:
        if not isinstance(cur, dict) or key not in cur:
            sys.exit(f"{GATE}: {where} has no `{'.'.join(path)}` — {what}")
        cur = cur[key]
    if not isinstance(cur, str):
        sys.exit(f"{GATE}: {where}'s `{'.'.join(path)}` is {type(cur).__name__}, "
                 "not a string — the gate cannot compare it to anything")
    return cur


floor = dig(load(manifest), ("workspace", "package", "rust-version"), manifest,
            "the declared MSRV floor is what this gate holds equal to the pinned "
            "channel, and a floor that is gone is not a floor this gate can check; "
            "restore it, or retire this gate deliberately")
channel = dig(load(toolchain), ("toolchain", "channel"), toolchain,
              "the pinned channel is what this gate holds the MSRV floor equal to; "
              "this workspace pins a compiler (D9/L2), and an unpinned one is a "
              "different decision than the one this gate enforces")

# A channel that is not a bare version cannot be copied into
# `rust-version` at all — see the header.
if not re.fullmatch(r"[0-9]+\.[0-9]+(\.[0-9]+)?", channel):
    sys.exit(f"{GATE}: {toolchain}'s channel `{channel}` is not a version number, so "
             "`rust-version` cannot equal it — cargo requires a bare version there. "
             "This gate's equality no longer has a meaning; decide what the floor "
             "promises (Q9) and delete or rewrite the gate deliberately")

if floor != channel:
    print(f'{manifest}: [workspace.package] rust-version = "{floor}"\n'
          f'{toolchain}: [toolchain] channel = "{channel}"', end="")
PY
  ); then
    gate_error "$(gate_name): could not read the two declarations under $PWD (the reader's own message is above, if it had one) — the gate decided nothing, which is not a pass"
    exit 1
  fi
  if [ -n "$verdict" ]; then
    echo "$verdict"
    gate_error "the declared MSRV floor and the pinned toolchain channel do not agree. They are held EQUAL so that every job compiling on the channel compiles at the floor — a floor that lags the channel is compiled by nothing. Move both strings together; or, if this repository now means to support a compiler older than the one it builds with, delete this gate deliberately and say what row proves the older floor (the gate's header, and Q9)."
    exit 1
  fi
  gate_ok "the declared MSRV floor is the pinned toolchain channel, so every job that builds on the channel builds at the floor"
}

# The subject is two root files, not `crates/*/src`, so the clean
# fixture is those two files and nothing else.
gate_plant_clean() {
  printf '[workspace]\nmembers = []\n\n[workspace.package]\nrust-version = "1.97.0"\n' \
    > "$1/$MANIFEST"
  printf '[toolchain]\nchannel = "1.97.0"\ncomponents = ["rustfmt"]\n' \
    > "$1/$TOOLCHAIN"
}

# THE CASE THIS GATE EXISTS FOR: the channel moves and the floor stays.
plant_channel_ahead() {
  printf '[toolchain]\nchannel = "1.98.0"\n' > "$1/$TOOLCHAIN"
}

# Equality is symmetric, and this direction is the one a `>=` comparison
# would have let through in the other order — a floor ahead of the
# channel is a promise nothing in this repository builds.
plant_floor_ahead() {
  printf '[workspace]\nmembers = []\n\n[workspace.package]\nrust-version = "1.99.0"\n' \
    > "$1/$MANIFEST"
}

# The same floor, spelled short. Cargo reads `1.97` and `1.97.0` as the
# same requirement; this gate does not, and the header says why.
plant_short_floor() {
  printf '[workspace]\nmembers = []\n\n[workspace.package]\nrust-version = "1.97"\n' \
    > "$1/$MANIFEST"
}

plant_no_floor() {
  printf '[workspace]\nmembers = []\n\n[workspace.package]\nedition = "2024"\n' \
    > "$1/$MANIFEST"
}

plant_no_channel() {
  printf '[toolchain]\ncomponents = ["rustfmt"]\n' > "$1/$TOOLCHAIN"
}

plant_no_toolchain_file() {
  rm -f "$1/$TOOLCHAIN"
}

plant_non_version_channel() {
  printf '[toolchain]\nchannel = "stable"\n' > "$1/$TOOLCHAIN"
}

plant_unparseable() {
  printf '[workspace.package\nrust-version = "1.97.0"\n' > "$1/$MANIFEST"
}

plant_floor_is_not_a_string() {
  printf '[workspace]\nmembers = []\n\n[workspace.package]\nrust-version = 1.97\n' \
    > "$1/$MANIFEST"
}

# THE PARSE-NOT-GREP CLAIM, checked rather than asserted. Both fixtures
# are the SAME DOCUMENT as the clean one to a TOML reader and a
# different one to every regex this gate could have been written with;
# both must stay green.
plant_dotted_spelling() {
  printf 'workspace.members = []\nworkspace.package.rust-version = "1.97.0"\n' \
    > "$1/$MANIFEST"
  printf 'toolchain.channel = "1.97.0"\n' > "$1/$TOOLCHAIN"
}

plant_literal_strings() {
  printf "[workspace]\nmembers = []\n\n[workspace.package]\nrust-version = '1.97.0'\n" \
    > "$1/$MANIFEST"
  printf "[toolchain]\nchannel = '1.97.0'\n" > "$1/$TOOLCHAIN"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_without_tool python3 "could not read the two declarations"
  gate_selftest_case "do not agree" plant_channel_ahead
  gate_selftest_case "do not agree" plant_floor_ahead
  gate_selftest_case "do not agree" plant_short_floor
  gate_selftest_case "has no \`workspace.package.rust-version\`" plant_no_floor
  gate_selftest_case "has no \`toolchain.channel\`" plant_no_channel
  gate_selftest_case "$TOOLCHAIN does not exist under" plant_no_toolchain_file
  gate_selftest_case "is not a version number" plant_non_version_channel
  gate_selftest_case "does not parse" plant_unparseable
  gate_selftest_case "not a string" plant_floor_is_not_a_string
  gate_selftest_passes "the dotted-key spelling of both values" plant_dotted_spelling
  gate_selftest_passes "single-quoted literal strings" plant_literal_strings
  printf '%s selftest OK: passes a clean fixture and two alternate TOML spellings of it, fires on a channel ahead of the floor, a floor ahead of the channel and a short-spelled floor, and diagnoses a missing key, a missing subject file, a non-version channel, a non-string floor, an unparseable manifest and a reader that cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
