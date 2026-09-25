#!/usr/bin/env bash
# msrv-floor-equals-channel.sh — EVERY declared MSRV floor in this tree
# is the string `rust-toolchain.toml` pins as the channel.
# ONE home; ci.yml's "the MSRV floor is the pinned channel" step and
# local-scripts/ci-local.sh's discipline row both call this file.
#
# THE RULE. `rust-version` is a floor a consumer's compiler must clear;
# `[toolchain] channel` is the compiler this workspace builds with
# (D9/L2's pin). They are different claims, and nothing reconciled them:
# the day the channel moves, a floor stays behind as a number nobody
# maintains and nothing compiles at. This gate settles that by holding
# them EQUAL.
#
# WHAT EQUALITY BUYS, and it is the whole argument for the gate: pinned
# equal, THE CHANNEL IS THE FLOOR, so every job that builds at all
# builds at the floor. That is the hedged form and it is the true one —
# the `discipline` job this gate runs in invokes no cargo, `ci.yml` sets
# `RUSTUP_TOOLCHAIN: stable` for its pinned tool installs, and a
# TIER=docs run has no build jobs at all. What the equality removes is
# the case where a floor is compiled by NOTHING: without it, no row
# compiles the workspace at a floor the channel has moved past; with it,
# every build row there is — at every lane and every eps point — is a
# build at the floor. No new build row, no nightly.
#
# AND THE FLOOR IS NOT INERT. `Cargo.toml` sets `resolver = "3"`, which
# makes `rust-version` an INPUT TO DEPENDENCY VERSION SELECTION: cargo
# prefers versions whose own `rust-version` the floor clears. So the
# declaration is read by the build, not only by a reader — which is why
# the floor tracking the channel is coherent rather than merely tidy.
# `Cargo.lock` is committed, so that preference is consulted when the
# lockfile is REGENERATED, a deliberate act, and not on an ordinary
# build. Written here because the repository records it nowhere else.
#
# WHAT IT COSTS, written here so the deletion is deliberate when it
# comes. THIS GATE FORBIDS A FLOOR FROM EVER LAGGING THE CHANNEL. The
# day this repository wants to say *"we build with 1.99 and still
# support 1.97"* — a real thing to want once **Q9** lands and something
# is published, since the workspace is `publish = false` and nameless
# until then — this gate is WRONG and must be deleted, not worked
# around. At that moment the question it closed comes back with a
# consumer attached to it: is the floor a promise, and what row compiles
# at it? That is the time to answer it and the wrong time to be
# surprised by it.
#
# THE SUBJECT IS EVERY MANIFEST, NOT THE WORKSPACE MANIFEST. Three
# files in this tree declare a floor, and only one of them is inherited
# from: `Cargo.toml`'s `[workspace.package]`, and `[package]` entries in
# `benches/Cargo.toml` and `interval-transcendentals/Cargo.toml`, both
# of which are in `Cargo.toml`'s `exclude` list and inherit nothing. A
# gate holding only the workspace declaration would force that one up on
# a channel bump and leave the other two behind silently — the failure
# this gate exists to prevent, one directory over, and
# `interval-transcendentals` is in the kernel's build closure through
# `geom-core`'s dependency on it — every build — so its floor is a live
# claim.
#
# HOW THE MANIFEST SET IS DERIVED, and why not from
# `scripts/doc-gate.sh --print-roots`. That script derives the CARGO
# ROOTS — `.` plus one directory per root outside the workspace — and
# `docs/prompts/implementer-discipline.md` §2 rightly says not to carry
# that list in your head. This gate reads a SUPERSET of it and needs no
# list either: every `Cargo.toml` in the tree, `target/` and `.git/`
# pruned. Every cargo root's manifest is a `Cargo.toml` in the tree, so
# a fifth root is covered the day it lands whether or not anyone
# remembers this gate; and a MEMBER that stops inheriting and writes its
# own literal floor is covered too, which the root set would not see.
# The cost of the superset is that this gate needs neither `cargo` nor
# `git` — it stays a file read in a row that is otherwise greps, and
# every self-test fixture is a directory rather than an initialised
# repository.
#
# WHAT IT READS IN EACH MANIFEST: a LITERAL `rust-version`, under
# `[workspace.package]` or `[package]`. `rust-version.workspace = true`
# parses as a table and is skipped — it is an inheritance, not a
# declaration, and holding the value it inherits would be holding one
# fact twice.
#
# WHY EXACT STRING EQUALITY, not a version comparison. A declaration is
# documentation of the pin, so what the gate holds is that it is a COPY
# of the pin — character for character. `1.97` and `1.97.0` name the
# same floor to cargo and are still a failure here, because a tolerance
# is a second rule to maintain and the only way the spellings diverge is
# someone editing one file without reading the others, which is the
# event this gate is for.
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
# AND THE SPELLINGS IT BLESSES ARE A SUPERSET OF WHAT ONE OTHER READER
# OF THE SAME FIELD ACCEPTS. `local-scripts/seal-oracle.sh` reads
# `[toolchain] channel` with a sed that requires a double-quoted value
# at column 0, and REFUSES an indented or single-quoted one — its own
# declared gap, and a refusal rather than a misread. So a
# `rust-toolchain.toml` this gate passes can still make that script
# refuse until `TOOLCHAIN=` is set. That divergence is real and is
# filed, not fenced away here:
# `work/ciw/seal-oracle-refuses-toml-spellings-the-msrv-gate-blesses.md`.
#
# WHAT THIS DOES NOT PROVE: that any of the values is RIGHT. Every
# string moved together to a channel that does not exist passes here and
# fails at the first `cargo` invocation, which is the right division of
# labour.
#
# WHEN THE CHANNEL STOPS BEING A VERSION. `channel = "stable"`, a dated
# nightly, or rustup's legacy bare-toolchain-name file cannot be copied
# into `rust-version`, which cargo requires to be a bare version.
# Equality is then unsatisfiable, so the gate REFUSES rather than
# demanding a value cargo would reject — and that refusal is the same
# conversation as the deletion above, arriving from the other side.
#
# A REFUSAL IS NOT A DEAD READER, and the two get different sentences.
# `lib.sh`'s `gate_reader_died_refusal` block draws exactly this line: a
# reader that COULD NOT RUN is one fact, and a refusal about something
# the gate READ and could not place is another. A missing key, a
# non-string floor and a non-version channel are all decisions made over
# documents this gate read successfully; a parse failure, an unreadable
# file and a missing `tomllib` are not. The self-test asserts which
# framing each case gets, so the two cannot quietly become one.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

MANIFEST=Cargo.toml
TOOLCHAIN=rust-toolchain.toml

gate() {
  # A GATE THAT SCANNED NOTHING IS NOT A PASS. The two NAMED subjects
  # are guarded first — a renamed one would otherwise make the gate
  # green forever over a tree it never read — and the derived manifest
  # list is checked for emptiness after.
  GATE_SCAN_NOUN="manifest"
  gate_require_file "$MANIFEST"
  gate_require_file "$TOOLCHAIN"

  # `find` CAPTURED AND CHECKED, not piped into `mapfile`: inside
  # `< <(…)` a reader that died is invisible, the loop runs zero times
  # and the gate reports green over a tree it never walked — the shape
  # `lib.sh` exists to refuse.
  local found
  if ! found=$(find . \( -name target -o -name .git \) -prune -o -name Cargo.toml -print); then
    gate_error "$(gate_name): find failed under $PWD, so the gate cannot tell which manifests this tree holds — an empty list would skip every declared floor and still report green"
    exit 1
  fi
  if [ -z "$found" ]; then
    gate_error "$(gate_name): no Cargo.toml under $PWD although one was just required — the manifest walk scanned nothing, which is not a pass"
    exit 1
  fi
  local -a manifests=()
  mapfile -t manifests < <(printf '%s\n' "$found" | sed 's#^\./##' | sort)
  GATE_SCAN_FILES=${#manifests[@]}

  # THE READER'S THREE ANSWERS, told apart by exit status: 0 decided
  # (stdout holds the disagreements, empty means none), 3 refused about
  # what it read, anything else died before it could decide.
  local verdict rc=0
  verdict=$(python3 - "$TOOLCHAIN" "${manifests[@]}" <<'PY'
import re
import sys

GATE = "msrv-floor-equals-channel"

try:
    import tomllib
except ImportError:  # pragma: no cover - guarded, not silenced
    print(f"{GATE}: python3 has no tomllib (needs 3.11+) — the gate cannot "
          "read its subjects, which is not a pass", file=sys.stderr)
    raise SystemExit(2) from None


def died(msg):
    print(f"{GATE}: {msg}", file=sys.stderr)
    raise SystemExit(2)


def refuse(msg):
    print(f"{GATE}: {msg}", file=sys.stderr)
    raise SystemExit(3)


def read_text(path):
    """An ABSENT file and a MALFORMED one are different facts."""
    try:
        with open(path, "rb") as fh:
            raw = fh.read()
    except OSError as exc:
        died(f"{path} could not be opened ({exc}) — the gate cannot read its "
             "subjects, which is not a pass")
    try:
        return raw.decode("utf-8")
    except UnicodeDecodeError as exc:
        died(f"{path} is not UTF-8 ({exc}) — the gate cannot read its "
             "subjects, which is not a pass")
    return None


def load(path):
    try:
        return tomllib.loads(read_text(path))
    except tomllib.TOMLDecodeError as exc:
        died(f"{path} does not parse ({exc}) — the gate cannot read its "
             "subjects, which is not a pass")
    return None


def dig(doc, *path):
    cur = doc
    for key in path:
        if not isinstance(cur, dict) or key not in cur:
            return None
        cur = cur[key]
    return cur


def channel_of(path):
    text = read_text(path)
    try:
        doc = tomllib.loads(text)
    except tomllib.TOMLDecodeError as exc:
        body = [ln for ln in text.splitlines()
                if ln.strip() and not ln.lstrip().startswith("#")]
        if len(body) == 1 and "=" not in body[0] and "[" not in body[0]:
            refuse(f"{path} is rustup's LEGACY bare-toolchain-name form "
                   f"({body[0].strip()!r}), not a [toolchain] table. This "
                   "workspace pins its compiler as TOML (D9/L2) and this "
                   "gate compares the table's `channel`; a bare name is a "
                   "different decision, made deliberately or not at all")
        died(f"{path} does not parse ({exc}) — the gate cannot read its "
             "subjects, which is not a pass")
    got = dig(doc, "toolchain", "channel")
    if got is None:
        refuse(f"{path} declares no `toolchain.channel` — the pinned channel "
               "is what this gate holds every MSRV floor equal to, and an "
               "unpinned compiler is a different decision than this gate's")
    if not isinstance(got, str):
        refuse(f"{path}'s `toolchain.channel` is {type(got).__name__}, not a "
               "string — the gate cannot compare it to anything")
    if not re.fullmatch(r"[0-9]+\.[0-9]+(\.[0-9]+)?", got):
        refuse(f"{path}'s channel `{got}` is not a version number, so a "
               "`rust-version` cannot equal it — cargo requires a bare "
               "version there. This gate's equality no longer has a meaning; "
               "decide what the floor promises (Q9) and delete or rewrite "
               "the gate deliberately")
    return got


def floors(path):
    """Every LITERAL rust-version one manifest declares."""
    doc = load(path)
    for table, where in (("workspace", "[workspace.package]"),
                         ("package", "[package]")):
        got = (dig(doc, "workspace", "package", "rust-version")
               if table == "workspace" else dig(doc, "package", "rust-version"))
        if got is None:
            continue
        # `rust-version.workspace = true` is an inheritance, not a
        # declaration — the value it inherits is already the subject.
        if isinstance(got, dict):
            continue
        if not isinstance(got, str):
            refuse(f"{path}'s {where} rust-version is {type(got).__name__}, "
                   "not a string — the gate cannot compare it to anything")
        yield where, got


toolchain, manifests = sys.argv[1], sys.argv[2:]
channel = channel_of(toolchain)

declared = [(p, where, got) for p in manifests for where, got in floors(p)]

# THE ANCHOR. The workspace manifest's inherited floor is the one every
# member stands on; if it is gone the gate is reading a tree that is not
# the one it was written for, and a quiet green over a vanished subject
# is the thing this directory refuses.
if not any(p == "Cargo.toml" and where == "[workspace.package]"
           for p, where, _ in declared):
    refuse("Cargo.toml declares no [workspace.package] rust-version — that "
           "is the floor every member inherits and the one this gate is "
           "anchored to; restore it, or retire this gate deliberately")

bad = [f'{p}: {where} rust-version = "{got}"'
       for p, where, got in declared if got != channel]
if bad:
    bad.append(f'{toolchain}: [toolchain] channel = "{channel}"')
    print("\n".join(bad), end="")
PY
  ) || rc=$?
  case "$rc" in
    0) ;;
    3) gate_error "$(gate_name): the gate READ its subjects and refuses to decide on them (the refusal is above) — that is a decision about what this tree declares, not a reader that could not run"
       exit 1 ;;
    *) gate_error "$(gate_name): the gate could not read its subjects under $PWD (the reader's own message is above, if it had one) — it decided nothing, which is not a pass"
       exit 1 ;;
  esac
  if [ -n "$verdict" ]; then
    echo "$verdict"
    gate_error "a declared MSRV floor and the pinned toolchain channel do not agree. They are held EQUAL so that every job that builds at all builds at the floor — a floor that lags the channel is compiled by nothing. Move every string together; or, if this repository now means to support a compiler older than the one it builds with, delete this gate deliberately and say what row proves the older floor (the gate's header, and Q9)."
    exit 1
  fi
  gate_ok "every declared MSRV floor is the pinned toolchain channel, so every job that builds at all builds at the floor"
}

# The clean fixture is the shape of the real tree: a workspace manifest
# whose `[workspace.package]` floor members inherit, a SECOND CARGO ROOT
# outside the workspace declaring its own `[package]` floor, and an
# inheriting member. All three shapes are in the scan, so the drift
# planters below each aim at one of them.
gate_plant_clean() {
  printf '[workspace]\nmembers = ["crates/clean"]\nexclude = ["outside"]\n\n[workspace.package]\nrust-version = "1.97.0"\n' \
    > "$1/$MANIFEST"
  printf '[toolchain]\nchannel = "1.97.0"\ncomponents = ["rustfmt"]\n' \
    > "$1/$TOOLCHAIN"
  mkdir -p "$1/outside" "$1/crates/clean"
  printf '[workspace]\n\n[package]\nname = "outside"\nrust-version = "1.97.0"\n' \
    > "$1/outside/$MANIFEST"
  printf '[package]\nname = "clean"\nrust-version.workspace = true\n' \
    > "$1/crates/clean/$MANIFEST"
}

# THE CASE THIS GATE EXISTS FOR: the channel moves and the floors stay.
plant_channel_ahead() {
  printf '[toolchain]\nchannel = "1.98.0"\n' > "$1/$TOOLCHAIN"
}

# THE CASE THE FIRST VERSION OF THIS GATE MISSED: an excluded root's
# hand-written floor drifts while the workspace declaration is correct.
# Three such floors exist in the real tree and only one was held.
plant_outside_root_drifts() {
  printf '[workspace]\n\n[package]\nname = "outside"\nrust-version = "1.96.0"\n' \
    > "$1/outside/$MANIFEST"
}

# A MEMBER THAT STOPS INHERITING. No member declares a literal floor
# today; the first one to do it is covered rather than fenced away.
plant_member_overrides_the_floor() {
  printf '[package]\nname = "clean"\nrust-version = "1.95.0"\n' \
    > "$1/crates/clean/$MANIFEST"
}

# Equality is symmetric, and this direction is the one a `>=` comparison
# would have let through in the other order — a floor ahead of the
# channel is a promise nothing in this repository builds.
plant_floor_ahead() {
  printf '[workspace]\nmembers = ["crates/clean"]\nexclude = ["outside"]\n\n[workspace.package]\nrust-version = "1.99.0"\n' \
    > "$1/$MANIFEST"
}

# The same floor, spelled short. Cargo reads `1.97` and `1.97.0` as the
# same requirement; this gate does not, and the header says why.
plant_short_floor() {
  printf '[workspace]\nmembers = ["crates/clean"]\nexclude = ["outside"]\n\n[workspace.package]\nrust-version = "1.97"\n' \
    > "$1/$MANIFEST"
}

plant_no_floor() {
  printf '[workspace]\nmembers = ["crates/clean"]\nexclude = ["outside"]\n\n[workspace.package]\nedition = "2024"\n' \
    > "$1/$MANIFEST"
}

plant_no_channel() {
  printf '[toolchain]\ncomponents = ["rustfmt"]\n' > "$1/$TOOLCHAIN"
}

# ONE CASE PER NAMED SUBJECT. The first version of this gate planted the
# toolchain file's disappearance and not the manifest's, so deleting the
# `gate_require_file "$MANIFEST"` line left the self-test green while
# its summary claimed a missing subject file was diagnosed.
plant_no_manifest_file() {
  rm -f "$1/$MANIFEST"
}

plant_no_toolchain_file() {
  rm -f "$1/$TOOLCHAIN"
}

plant_non_version_channel() {
  printf '[toolchain]\nchannel = "stable"\n' > "$1/$TOOLCHAIN"
}

plant_legacy_toolchain() {
  printf '# the pin\n1.97.0\n' > "$1/$TOOLCHAIN"
}

plant_unparseable() {
  printf '[workspace.package\nrust-version = "1.97.0"\n' > "$1/$MANIFEST"
}

plant_floor_is_not_a_string() {
  printf '[workspace]\nmembers = ["crates/clean"]\nexclude = ["outside"]\n\n[workspace.package]\nrust-version = 1.97\n' \
    > "$1/$MANIFEST"
}

# THE PARSE-NOT-GREP CLAIM, checked rather than asserted. Both fixtures
# are the SAME DOCUMENT as the clean one to a TOML reader and a
# different one to every regex this gate could have been written with;
# both must stay green.
plant_dotted_spelling() {
  printf 'workspace.members = ["crates/clean"]\nworkspace.exclude = ["outside"]\nworkspace.package.rust-version = "1.97.0"\n' \
    > "$1/$MANIFEST"
  printf 'toolchain.channel = "1.97.0"\n' > "$1/$TOOLCHAIN"
}

plant_literal_strings() {
  printf "[workspace]\nmembers = [\"crates/clean\"]\nexclude = [\"outside\"]\n\n[workspace.package]\nrust-version = '1.97.0'\n" \
    > "$1/$MANIFEST"
  printf "[toolchain]\nchannel = '1.97.0'\n" > "$1/$TOOLCHAIN"
}

# WHERE THE WALK STOPS. A manifest cargo never builds — one left in a
# build directory — must not red the gate, or every developer with a
# warm `target/` gets a different answer from CI.
plant_stale_floor_under_target() {
  mkdir -p "$1/target/package/old-0.1.0"
  printf '[package]\nname = "old"\nrust-version = "1.42.0"\n' \
    > "$1/target/package/old-0.1.0/$MANIFEST"
}

gate_selftest() {
  gate_selftest_clean
  gate_selftest_without_tool python3 "could not read its subjects"
  gate_selftest_case "do not agree" plant_channel_ahead
  gate_selftest_case --also 'outside/Cargo.toml: [package]' \
    "do not agree" plant_outside_root_drifts
  gate_selftest_case --also 'crates/clean/Cargo.toml: [package]' \
    "do not agree" plant_member_overrides_the_floor
  gate_selftest_case "do not agree" plant_floor_ahead
  gate_selftest_case "do not agree" plant_short_floor
  # THE TWO FRAMINGS, ASSERTED PER CASE. Without `--also` a refusal and
  # a dead reader are told apart by nothing, and the sentence the
  # Actions UI shows against the failing step is the wrong one.
  gate_selftest_case --also "refuses to decide" \
    "declares no [workspace.package] rust-version" plant_no_floor
  gate_selftest_case --also "refuses to decide" \
    "declares no \`toolchain.channel\`" plant_no_channel
  gate_selftest_case --also "refuses to decide" \
    "is not a version number" plant_non_version_channel
  gate_selftest_case --also "refuses to decide" \
    "LEGACY bare-toolchain-name form" plant_legacy_toolchain
  gate_selftest_case --also "refuses to decide" \
    "not a string" plant_floor_is_not_a_string
  gate_selftest_case --also "could not read its subjects" \
    "does not parse" plant_unparseable
  gate_selftest_case "$MANIFEST does not exist under" plant_no_manifest_file
  gate_selftest_case "$TOOLCHAIN does not exist under" plant_no_toolchain_file
  gate_selftest_passes "the dotted-key spelling of every value" plant_dotted_spelling
  gate_selftest_passes "single-quoted literal strings" plant_literal_strings
  gate_selftest_passes "a stale manifest under target/" plant_stale_floor_under_target
  printf '%s selftest OK: passes a three-manifest fixture (workspace floor, an excluded root of its own, an inheriting member), two alternate TOML spellings of it and a stale manifest under target/ the walk must not read; fires on a channel ahead of the floors, on an EXCLUDED ROOT and on a MEMBER drifting alone, on a floor ahead of the channel and on a short-spelled floor; and diagnoses — with the refusal framing where it read its subjects and the dead-reader framing where it did not — a missing floor, a missing channel, a non-version channel, rustup\47s legacy bare-name file, a non-string floor, an unparseable manifest, either named subject gone, and a reader that cannot run\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
