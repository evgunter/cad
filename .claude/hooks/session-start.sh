#!/usr/bin/env bash
# SessionStart hook — provision a Claude Code on the web container so the
# gate is runnable the moment a session opens.
#
# WHAT "RUNNABLE" MEANS HERE: the rows of .github/workflows/ci.yml that a
# session actually reruns by hand — `cargo fmt --check`, `cargo clippy
# ... -D warnings`, `cargo nextest run`, `cargo test --doc`, the tripwire
# scripts, `crates/pncad-py/run-python-tests.sh`, and the admesh
# watertight check. Everything those need that a bare container lacks is
# installed below — plus the demo renderers' Python venv, so a scene can
# be previewed without a cold install mid-session (§6 for why that is the
# only render prerequisite worth warming here).
#
# All of those were run end to end on a hosted container on 2026-08-14
# and were green: fmt, clippy -D warnings, `cargo nextest run` (geom-core,
# 267 passed), `cargo test --doc` (pncad, 33 passed),
# run-python-tests.sh (132 passed, with `ty` present so the stub lattice
# is MEASURED rather than skipped), check_admesh.sh, and a full preview
# montage. That list is what "provisioned" is supposed to mean; a session
# finding one of them broken should suspect the container, not the row.
#
# REMOTE ONLY. Local machines are provisioned by hand and carry
# machine-local tuning this script must not second-guess (see
# local-scripts/setup-build-env.sh); the guard below makes this a no-op
# outside Claude Code on the web.
#
# SYNCHRONOUS, deliberately: the session must not open onto a half-built
# toolchain, because the first thing an agent does here is compile.
#
# IDEMPOTENT: every step is a no-op when its output is already present, so
# a resume/clear/compact re-fire costs seconds.
#
# NO WARM BUILD (2026-08-15). This hook used to end with `cargo build
# --workspace --all-targets`, on the belief that the container is
# snapshotted after the hook and later sessions would open onto a warm
# target/. They do not: a session opens on a fresh clone with an empty
# target/ regardless, so the build bought nothing that outlived the
# session that paid for it. Within a session it is worse than
# neutral: the same compile happens either way, and doing it HERE spends
# minutes before the agent can type, hides the output behind hook capture,
# and compiles the whole workspace when the first real command usually
# wants one crate. So provisioning here is confined to what a bare
# container LACKS — tools it has no copy of, and the registry warm that
# keeps the first build from also being a download. Do not re-add a build
# step without a measurement showing target/ actually survives the
# session.
#
# NO SINGLE TOOL MAY COST THE SESSION ITS COMPILER (2026-08-14). This
# script used to die on the first failed download: `set -e` plus a
# `curl | tar` pipeline meant one unreachable host aborted the hook where
# it stood, and everything AFTER that point — the remaining tools, the
# registry warm — simply never ran. That is exactly backwards. The rows
# differ in how much they cost when absent:
#
#   * the COMPILER is the session. Nothing here may stand between the
#     agent and `cargo build`;
#   * a TEST-RUNNER or CHECKER (nextest, maturin, ty, admesh) costs its
#     own gate row and nothing else. A session missing one can still
#     build, still `cargo test`, still read code.
#
# So every network-dependent tool below is NON-FATAL and LOUD: it warns
# on stderr naming the tool and what it costs, records itself in
# DEGRADED, and the hook prints the whole list again at the end, on
# STDOUT — the channel a session actually reads (see that block). Fail-loud
# is preserved — what is dropped is fail-EARLY, which here only ever
# punished the steps that matter most. The hook still exits nonzero if the
# toolchain itself fails.
#
# EGRESS IS POLICY-FILTERED, AND THE FILTER DIFFERS PER ENVIRONMENT.
# Outbound HTTPS goes through an agent proxy enforcing an allow-list, so
# "the download failed" here usually means "your organization does not
# permit that host", not "the network is flaky". As measured on
# `cloud_default`, 2026-08-14:
#
#   DENIED   get.nexte.st — the gateway answers 403 to CONNECT
#   allowed  github.com (release assets), crates.io + static.crates.io,
#            pypi.org, the Ubuntu apt mirrors
#
# DIAGNOSING THE NEXT ONE: `curl -sS "$HTTPS_PROXY/__agentproxy/status"`.
# Its `recentRelayFailures` names the host AND the reason, which is the
# only place to get it — curl hides the response body on a failed
# CONNECT, so at the call site a policy denial is indistinguishable from
# a dead mirror (that is precisely how the get.nexte.st breakage read
# before the status endpoint was consulted). /root/.ccr/README.md has the
# other failure classes.
#
# WHAT TO DO ABOUT A DENIAL: report the blocked host; do not tunnel
# around the policy. Reaching for a DIFFERENT OFFICIAL SOURCE OF THE SAME
# ARTIFACT is not tunnelling around it — that is what the nextest
# fallback below is, and the distinction is the publisher, not the
# hostname. A random mirror of a binary would not qualify.
set -euo pipefail

if [ "${CLAUDE_CODE_REMOTE:-}" != "true" ]; then
  exit 0
fi

repo="${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"
cd "$repo"

say() { printf '\n== %s\n' "$1"; }

# WHAT EACH ABSENCE COSTS, WRITTEN ONCE. The same tool goes missing down
# two or three different branches — its pin unread, its download denied,
# its interpreter absent — and every branch owes the session the same
# sentence about what it can no longer run. Three of these were spelled
# twice and one of them three times, a dozen lines apart, with nothing
# comparing the copies; a cost sentence that drifts between branches
# describes a different container depending on how the tool went missing.
COST_NEXTEST="\`cargo nextest run\` is unavailable; use \`cargo test\` instead (slower, no per-test process isolation, and NOT what ci.yml runs). \`cargo test --doc\` is unaffected."
COST_MATURIN="the WHEEL path of crates/pncad-py is unavailable; run-python-tests.sh still runs by staging the plain cdylib, which is the degraded-box path it was written for."
COST_TY="tests/test_ty.py will skip — it says so loudly, so the stub lattice is UNMEASURED rather than silently passing."
# The prefix a PIN-UNREAD branch adds to the sentence above. NAME is
# substituted at the call site (`${UNREAD_PIN/NAME/TY_VERSION}`), so the
# three branches cannot disagree about what a refused read means.
UNREAD_PIN="ci.yml's NAME could not be read — the refusal is above, and nothing is installed unpinned."

# What this session does NOT have, and what each absence costs. Filled by
# warn(), replayed by the final summary — so the cost of a degraded
# container is stated once where it happens and once where an agent
# reading the tail of the hook output will actually see it.
DEGRADED=()
warn() {
  local tool=$1 cost=$2
  DEGRADED+=("$tool — $cost")
  echo "WARNING: ${tool} is unavailable in this container." >&2
  echo "         Cost: ${cost}" >&2
  echo "         Every other gate row is unaffected." >&2
}

# ONE PINNED TOOL VERSION, READ FROM ci.yml RATHER THAN RESTATED HERE.
# `.github/workflows/ci.yml`'s workflow-level `env:` block is the single
# source of truth for the tool versions IT NAMES — the three below among
# them — and `scripts/ci-pin.py` is the one reader of it, anchored to that
# block, so a per-job pin of the same name cannot be picked up by
# accident. It is NOT every tool version CI installs: FreeCAD, the 3.12
# interpreter and this container's render venv are all pinned outside it
# and reconciled by nothing, which is a filed defect
# (`work/ciw/tool-versions-outside-the-env-block-have-no-source-of-truth`)
# and not a licence to restate the three that do have a source.
#
# The three versions below used to be shell literals in this file. Nothing
# compared them to the block, and nothing can be MADE to: every hosted job
# deletes `.claude/` at checkout, so a checker about this file would pass
# on hosted CI and red only on a developer's box — a check whose verdict
# depends on which half runs it. Reading the value is the repair that
# works from both halves.
#
# WHAT A FAILED READ DOES, WHICH IS THIS FUNCTION'S REASON FOR EXISTING.
# `ci-pin.py` refuses bluntly and exits 2 — a pin renamed, a second pin at
# any indentation, the workflow moved — and `python3` itself is one more
# thing a bare container might not have at this point in the hook. Each of
# the three pins feeds a TEST-RUNNER OR CHECKER in the taxonomy at the top
# of this file: cargo-nextest, maturin, `ty`. Each costs its own gate row
# and nothing else. So a refusal is handled exactly as a failed download
# is — THE TOOL IS NOT INSTALLED, the absence goes into DEGRADED and is
# replayed at the end, and the session still gets its compiler.
#
# The three other answers, and why each is worse:
#
#   * FALL BACK TO A LITERAL re-creates the defect being removed. A
#     version this file states and nothing compares to ci.yml is exactly
#     what the read replaced, and a last-resort copy drifts on the same
#     schedule as a first-resort one.
#   * INSTALL UNPINNED — whatever `latest` resolves to — is the silent
#     wrong-version install this whole class is about, and it would be
#     silent HERE, in the one lane no gate of record can see.
#   * FAIL THE HOOK denies the session its compiler over a checker. This
#     script's own rule, argued at length above, is that no single tool
#     may cost the session its toolchain; a pin that cannot be read is a
#     smaller event than a mirror outage, not a larger one.
#
# The refusal is reprinted verbatim: `ci-pin.py` names the lines it saw,
# and that text is the diagnosis a session needs in order to fix the
# workflow it names.
#
# THE TWO STREAMS STAY APART, AND THAT IS NOT A STYLE CHOICE. Folding
# stderr into the answer (`2>&1`) makes any noise on a SUCCESSFUL read
# part of the version — a deprecation warning, a $PYTHONWARNINGS line, a
# pyenv or asdf shim banner, anything a `sitecustomize` prints. Nothing
# would install unpinned, because a URL built from that answer 404s at
# both sources; what it destroys is the DIAGNOSIS. The session would be
# told the mirror was unreachable when the truth is that this read was
# dirty, and every sentence below promising "pin unread" would be wrong
# at the moment it mattered. So the error text is captured separately and
# printed only on the refusal branch.
#
# AND THE ANSWER HAS A SHAPE: exactly one line, no whitespace in it.
# `ci-pin.py` refuses a scalar with whitespace, so this is a second
# check of that contract rather than the first — and it is here because
# THIS file installs from the answer: the "already installed" tests below
# are `grep -qw "$version"`, and `grep` over a multi-line pattern is an
# OR across its lines, so one noisy line sharing a word with
# `cargo nextest --version` would report the pin as present when it is
# not. A shape this file depends on is checked where it is depended on.
ci_pin() {
  local name=$1 out err rc=0
  err=$(mktemp)
  out=$(python3 "$repo/scripts/ci-pin.py" "$name" 2>"$err") || rc=$?
  if [ "$rc" -eq 0 ] && [ -n "$out" ] && [ "$out" = "${out%%[[:space:]]*}" ]; then
    rm -f "$err"
    printf '%s\n' "$out"
    return 0
  fi
  echo "WARNING: cannot read ${name} from .github/workflows/ci.yml." >&2
  if [ "$rc" -eq 0 ]; then
    echo "         It exited 0 and printed something that is not a version pin:" >&2
    printf '%s\n' "${out:-(nothing at all)}" | sed 's/^/           /' >&2
  fi
  sed 's/^/         /' "$err" >&2
  echo "         Nothing is installed unpinned; the tool it pins is skipped." >&2
  rm -f "$err"
  return 1
}

# cargo-nextest: a dev/CI tool only, never in a shipped build graph.
# Installed from the official get.nexte.st prebuilt, same as the in-repo
# composite action, not from a third-party channel...
NEXTEST_VERSION="$(ci_pin NEXTEST_VERSION)" || NEXTEST_VERSION=
# ...with the project's OWN GitHub release asset as the second source.
# get.nexte.st is a redirector in front of exactly these assets, so this
# is the same artifact from the same publisher, not a third-party mirror
# — the only thing that changes is which hostname the request names.
#
# It exists because a hosted container's egress is policy-filtered per
# environment, and on this one get.nexte.st is DENIED (the gateway
# answers 403 to CONNECT) while github.com is allowed. That denial is
# what used to end the hook 14 seconds in. Trying the redirector first
# keeps hosted-CI parity and costs ~1s when it is blocked: curl does not
# retry a 403, so the fallback is reached immediately.
#
# EMPTY WHEN THE PIN WAS NOT READ, because a URL built out of an unread
# version names an artifact nobody asked for. The install step below
# refuses on the version first, so this array is never reached empty.
NEXTEST_URLS=()
if [ -n "$NEXTEST_VERSION" ]; then
  NEXTEST_URLS=(
    "https://get.nexte.st/${NEXTEST_VERSION}/linux"
    "https://github.com/nextest-rs/nextest/releases/download/cargo-nextest-${NEXTEST_VERSION}/cargo-nextest-${NEXTEST_VERSION}-x86_64-unknown-linux-gnu.tar.gz"
  )
fi
# The other two hosted pins, read the same way: maturin builds the abi3
# wheel, `ty` is the static stub checker behind tests/test_ty.py. Both
# dev/CI tools only.
MATURIN_VERSION="$(ci_pin MATURIN_VERSION)" || MATURIN_VERSION=
TY_VERSION="$(ci_pin TY_VERSION)" || TY_VERSION=
# ci.yml pins the interpreter at 3.12 — the version the suite was written
# and validated on. The container's default `python3` is 3.11, so name 3.12
# explicitly rather than inheriting whatever `python3` happens to be.
PY=/usr/bin/python3.12
# Outside the repo on purpose: a venv under target/ would be destroyed by
# `cargo clean` and confuse the wheel-vs-staged-cdylib install paths.
VENV="$HOME/.cache/pncad-py/venv"

# curl | tar, with each candidate URL tried in turn. Extracts into a temp
# directory first: `curl | tar` in a pipeline puts a HALF-WRITTEN binary
# on PATH when the transfer dies mid-stream (and, under `pipefail`,
# reports curl's status through tar's noise), which is a worse failure
# than not installing at all — a truncated `cargo-nextest` looks
# installed and fails at the point of use. Nothing is moved into
# ~/.cargo/bin until the whole archive has been extracted.
#
# $1: binary name inside the archive, $2...: candidate URLs.
fetch_tool() {
  local name=$1 url tmp rc
  shift
  tmp=$(mktemp -d)
  for url in "$@"; do
    rc=0
    curl --proto '=https' --tlsv1.2 -LsSf --retry 3 --retry-delay 2 "$url" \
      | tar zxf - -C "$tmp" "$name" || rc=$?
    if [ "$rc" -eq 0 ] && [ -s "$tmp/$name" ]; then
      install -m 0755 "$tmp/$name" "${CARGO_HOME:-$HOME/.cargo}/bin/$name"
      rm -rf "$tmp"
      return 0
    fi
    echo "  (source unusable: $url)" >&2
    rm -f "$tmp/$name"
  done
  rm -rf "$tmp"
  return 1
}

# ---------------------------------------------------------------------------
# 1. The pinned compiler (D9 / L2) plus rustfmt and clippy.
#
# `rustup show active-toolchain` honours rust-toolchain.toml — it installs
# the pinned channel AND its components list on first call, and prints in
# under a second afterwards. Doing it here rather than letting the first
# `cargo` invocation trigger it keeps the download out of the middle of an
# agent's first command, where it interleaves with real output.
# ---------------------------------------------------------------------------
say "rust toolchain (rust-toolchain.toml)"
rustup show active-toolchain
cargo --version
cargo fmt --version
cargo clippy --version

# ---------------------------------------------------------------------------
# 2. cargo-nextest — the test runner every hosted test row uses.
#
# Note for whoever runs the suite: nextest DOES NOT RUN DOC-TESTS, and this
# workspace has real ones (docs/GUIDE.md's Rust blocks are doctests of
# `pncad`). `cargo test --doc` is a separate row in ci.yml for that reason.
# ---------------------------------------------------------------------------
say "cargo-nextest ${NEXTEST_VERSION:-(pin unread)}"
# An empty version is a REFUSED READ, never "any version will do": every
# test below it compares against the pin, and `grep -qw ""` would call any
# cargo-nextest on PATH a match.
if [ -z "$NEXTEST_VERSION" ]; then
  warn "cargo-nextest (pin unread)" "${UNREAD_PIN/NAME/NEXTEST_VERSION} ${COST_NEXTEST}"
elif cargo nextest --version 2>/dev/null | grep -qw "${NEXTEST_VERSION}"; then
  echo "already installed: $(cargo nextest --version)"
# --retry (inside fetch_tool): ~10 hosted jobs per run taught us that an
# unretried fetch turns one bad minute on a CDN edge into a whole-gate
# failure (.github/actions/install-nextest carries the same reasoning).
elif fetch_tool cargo-nextest "${NEXTEST_URLS[@]}"; then
  cargo nextest --version
else
  warn "cargo-nextest ${NEXTEST_VERSION}" "${COST_NEXTEST}"
fi

# ---------------------------------------------------------------------------
# 3. The Python row's two pinned tools (ci.yml's `python-suite` job).
#
# maturin: the official GitHub-release prebuilt at the pinned URL, same
# idiom as nextest — no third-party action, no pip involved. It is only
# needed for the WHEEL path; crates/pncad-py/run-python-tests.sh works
# without it by staging the plain cdylib, which is what makes that script
# runnable on a degraded box.
#
# ty: the static half of the §L4 type story. Without it test_ty.py skips
# LOUDLY (an honest "this environment has no type checker", never a pass)
# — so installing it is the difference between the stub lattice being
# MEASURED in a session and merely not-failing.
# ---------------------------------------------------------------------------
say "maturin ${MATURIN_VERSION:-(pin unread)}"
if [ -z "$MATURIN_VERSION" ]; then
  warn "maturin (pin unread)" "${UNREAD_PIN/NAME/MATURIN_VERSION} ${COST_MATURIN}"
elif maturin --version 2>/dev/null | grep -qw "${MATURIN_VERSION}"; then
  echo "already installed: $(maturin --version)"
elif fetch_tool maturin \
    "https://github.com/PyO3/maturin/releases/download/v${MATURIN_VERSION}/maturin-x86_64-unknown-linux-musl.tar.gz"; then
  maturin --version
else
  warn "maturin ${MATURIN_VERSION}" "${COST_MATURIN}"
fi

say "ty ${TY_VERSION:-(pin unread)} (static stub checker, python 3.12 venv)"
if [ -z "$TY_VERSION" ]; then
  warn "ty (pin unread)" "${UNREAD_PIN/NAME/TY_VERSION} ${COST_TY}"
elif [ -x "$VENV/bin/ty" ] && "$VENV/bin/ty" --version 2>/dev/null | grep -qw "${TY_VERSION}"; then
  echo "already installed: $("$VENV/bin/ty" --version)"
elif [ ! -x "$PY" ]; then
  warn "ty ${TY_VERSION} (no ${PY})" "${COST_TY}"
elif "$PY" -m venv --clear "$VENV" \
     && "$VENV/bin/python" -m pip install -q --disable-pip-version-check "ty==${TY_VERSION}"; then
  "$VENV/bin/ty" --version
else
  warn "ty ${TY_VERSION}" "${COST_TY}"
fi

# PNCAD_TY is how tests/test_ty.py finds a checker that is not on PATH.
# Exporting it here means the suite measures the lattice whether it is run
# via run-python-tests.sh or by hand, with no per-session setup.
if [ -x "$VENV/bin/ty" ] && [ -n "${CLAUDE_ENV_FILE:-}" ]; then
  echo "export PNCAD_TY=\"$VENV/bin/ty\"" >> "$CLAUDE_ENV_FILE"
  echo "exported PNCAD_TY for the session"
fi

# ---------------------------------------------------------------------------
# 4. admesh — the external watertight/manifold oracle for exported STLs
#    (scripts/check_admesh.sh, M2 PR 7). Independent of our mesh code on
#    purpose, which is the whole point of the row.
#
# NON-FATAL: the check is one row of the local gate, not a prerequisite for
# building or testing, and apt is the one step here that depends on a
# package mirror. A session without it loses that row and nothing else, so
# a mirror outage must not cost the session its whole startup.
# ---------------------------------------------------------------------------
say "admesh (external watertight oracle)"
if command -v admesh >/dev/null 2>&1; then
  echo "already installed: $(admesh --version 2>&1 | head -1)"
else
  sudo=""
  if [ "$(id -u)" -ne 0 ] && command -v sudo >/dev/null 2>&1; then
    sudo="sudo"
  fi
  if (DEBIAN_FRONTEND=noninteractive $sudo apt-get update -qq \
      && DEBIAN_FRONTEND=noninteractive $sudo apt-get install -y -qq admesh); then
    admesh --version 2>&1 | head -1
  else
    warn "admesh" \
      "scripts/check_admesh.sh cannot run, so exported STLs lose their EXTERNAL watertight/manifold oracle. The kernel's own mesh assertions still run."
  fi
fi

# ---------------------------------------------------------------------------
# 5. Warm the cargo registry for every cargo root in the tree.
#
# benches/, demos/, tools/ and interval-transcendentals/ are
# workspace-EXCLUDED on purpose (see the root Cargo.toml header) —
# `--workspace` never reaches them, so each needs its own fetch.
# --locked: the lockfiles are checked in and a session must never
# silently resolve something new.
#
# THE ROSTER IS DERIVED, NOT SPELLED. `scripts/doc-gate.sh --print-roots`
# is this tree's one derivation of "every cargo root", and a second
# hand-written copy of a derived list is the defect the rest of this file
# was just rewritten to remove, with the digits taken out. This loop HAD
# that defect: it named five roots where the derivation gives eight —
# `benches`, `tools/tess-lint` and `tools/tess-meter` had landed and no
# session warmed them, silently, because a list nothing compares to its
# source cannot report that it is short.
#
# PER-ROOT NON-FATAL, and specifically because of --locked: an excluded
# demo root that path-depends on the kernel has a lockfile that must track
# the kernel's dependency set, so it goes stale whenever a kernel dep lands
# without that lock being refreshed (demos/wild is in exactly that state as
# this hook lands). That is a real thing to fix in the repo, but it is not
# a reason to deny the session its toolchain — the warning names the root,
# and `cargo fetch` (no --locked) inside it is the fix.
# ---------------------------------------------------------------------------
say "cargo fetch (workspace + the excluded roots)"
# A failed derivation warms the workspace root and says so, for the same
# reason a failed pin read skips its tool: a roster this file cannot
# derive is not a roster it may invent.
roots=$(bash "$repo/scripts/doc-gate.sh" --print-roots 2>/dev/null) || roots=
if [ -z "$roots" ]; then
  echo "WARNING: scripts/doc-gate.sh --print-roots did not answer." >&2
  echo "         Only the workspace root is pre-warmed; every excluded root's deps are cold." >&2
  roots="."
fi
while IFS= read -r root; do
  [ -n "$root" ] || continue
  if ( cd "$root" && cargo fetch --locked >/dev/null 2>&1 ); then
    echo "fetched: $root"
  else
    echo "WARNING: cargo fetch --locked failed in ${root} (stale lockfile?)" >&2
    echo "         Its deps are not pre-warmed; the kernel workspace is." >&2
  fi
done <<< "$roots"

# ---------------------------------------------------------------------------
# 6. The demo renderers' Python venv (numpy + matplotlib, pinned by
#    demos/render.sh and demos/render-wild.sh).
#
# NOT the committed sheets: hosted CI renders those (render.yml), and
# demos/hosted-render-guard.sh refuses a local pass without the drift
# sentence precisely so this box's GL stack cannot reach tracked pixels.
# What this warms is the PREVIEW path an agent uses to LOOK at a scene it
# is shaping — plus the wild lane, where matplotlib is the primary
# renderer rather than a fallback.
#
# Worth pre-warming because it is the one render prerequisite that is a
# cold download: FreeCAD is deliberately absent (a ~1 GB AppImage whose
# whole purpose here is the hosted OCC reference lane), so on this
# container the tour's KERNEL lane cannot draw at all — it exits 1,
# naming the missing binary. A preview here is `./render.sh
# --matplotlib`, which asks for this venv's renderer by name and writes
# only the ignored preview tree. Nothing selects that automatically:
# a pass that drew nothing must not be able to look like one that
# drew everything.
#
# The two things a preview pass needs beyond this venv, neither of which
# belongs in a hook (both are per-scene work, not provisioning):
#
#   * THE DRIFT SENTENCE. demos/hosted-render-guard.sh refuses without
#     CAD_RENDER_LOCAL_OVERRIDE=i-accept-local-render-drift, spelled out
#     in full on purpose. Frames still land in the gitignored
#     demos/renders-preview/ and the committed lanes stay untouched.
#   * THE TOUR OUTPUT, and it is `cd demos/tour && cargo run --release --
#     ../out`. Note ../out: the bare `out` that reads naturally writes to
#     demos/tour/out, where render.sh will not find scenes.json and dies
#     on a FileNotFoundError several minutes into a pass.
# ---------------------------------------------------------------------------
say "demo render venv (numpy + matplotlib)"
if [ -x demos/.venv/bin/python ]; then
  echo "already present: demos/.venv"
elif ( cd demos && if command -v uv >/dev/null 2>&1; then
         uv venv --python 3.12 .venv \
           && uv pip install -q --python .venv/bin/python \
                'numpy==2.2.6' 'matplotlib==3.10.3'
       else
         "$PY" -m venv .venv \
           && .venv/bin/pip install -q 'numpy==2.2.6' 'matplotlib==3.10.3'
       fi ); then
  demos/.venv/bin/python -c 'import matplotlib, numpy; print("numpy", numpy.__version__, "matplotlib", matplotlib.__version__)'
else
  rm -rf demos/.venv
  warn "the demo render venv" \
    "demos/render*.sh cannot draw a preview frame. The scripts rebuild the venv themselves on first run, so this only costs the retry."
fi

# ---------------------------------------------------------------------------
# The degraded-row summary. A warning that scrolled past 200 lines of
# cargo output is a warning nobody reads, so every absence is restated
# here, at the end, where the session actually starts.
#
# ON STDOUT, AND THE CHANNEL IS THE POINT. A SessionStart hook's STDOUT is
# what reaches the session's context; its stderr is a log the agent may
# never be shown. Every per-tool `warn()` above stays on stderr, where it
# belongs — it is a running commentary beside the step that produced it —
# but this block is the one sentence the session has to be holding when
# it starts work, and under `exit 0` a summary on stderr is "loud" only in
# the sense that something wrote it down somewhere. That was survivable
# while every DEGRADED row meant a download had visibly failed; it is not
# survivable now that a row can mean A PIN WAS NOT READ, because the
# session's next move (bump the pin, fix the workflow) is a repair only
# the agent can make and only if it hears about it.
# ---------------------------------------------------------------------------
if [ "${#DEGRADED[@]}" -gt 0 ]; then
  {
    echo
    echo "================================================================"
    echo " DEGRADED CONTAINER — the compiler is fine, but"
    echo " ${#DEGRADED[@]} gate row(s) cannot run in this session:"
    echo
    printf '   * %s\n' "${DEGRADED[@]}"
    echo
    echo " Hosted CI runs the full gate regardless; this only bounds what"
    echo " can be checked BEFORE pushing. If a tool was denied by egress"
    echo " policy, report the blocked host rather than routing around it."
    echo "================================================================"
  }
fi

say "ready"
