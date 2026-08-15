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
# be previewed without a cold install mid-session (§7 for why that is the
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
# A HOOK, NOT A SETUP SCRIPT, AND NOTHING HERE IS EVER CACHED. Claude
# Code on the web provisions a container from two places and they are not
# interchangeable:
#
#   SETUP SCRIPT — pasted into the environment dialog at claude.ai/code.
#     Runs once per environment cache, BEFORE Claude Code launches and
#     BEFORE THE REPO IS CLONED. Its filesystem is then snapshotted and
#     reused, so what it installs is free for later sessions.
#   SESSIONSTART HOOK — this file, registered in .claude/settings.json.
#     Runs after the clone, with the repo on disk, on every session and
#     every resume/clear/compact. Never snapshotted; its cost is paid in
#     full each time.
#
# THIS REPO USES THE HOOK ONLY, deliberately. The comment that used to sit
# here claimed the container was snapshotted once the hook finished. It is
# not — that is the setup script's property, and believing otherwise is
# what made a 3-minute cold start look like a broken cache. The setup
# script was measured against this repo and rejected: it runs before the
# clone, so the step that actually costs the time (§6, the warm build)
# has no repo to build and could not be cached by it anyway, leaving only
# the tool installs of §2-§4 — worth well under a minute against the
# standing cost of a second copy of this script, pasted into a web form,
# free to drift from the one in git. What the hook cannot cache it
# instead gets out of the way: §6 hands the session back while the build
# runs.
#
# Corollary, since it has now cost two rounds of confusion: DO NOT paste
# this file into the Setup script box. It assumes the repo throughout —
# without one it resolves to `/`, reads no rust-toolchain.toml, cd's into
# demo roots that do not exist, and exits 101 on `cargo build`, which in
# that slot means the session does not start at all.
#
# REMOTE ONLY. Local machines are provisioned by hand and carry
# machine-local tuning this script must not second-guess (see
# local-scripts/setup-build-env.sh); the guard below makes this a no-op
# outside Claude Code on the web.
#
# THE WARM BUILD IS BACKGROUNDED, WITH A GRACE PERIOD. It is the
# expensive step (~2.6 min cold, ~0.2 s warm) and, because hooks are never
# snapshotted, a fresh session pays it in full. §6 launches it detached
# and waits WARM_BUILD_GRACE seconds: a warm tree finishes inside that
# window and is reported inline exactly as before, so resume/clear/compact
# are unchanged, while a cold tree hands the session back immediately and
# keeps compiling. §6 carries the sentinel protocol and the reasons this
# is not simply `nohup ... &`.
#
# IDEMPOTENT: every step is a no-op when its output is already present, so
# a resume/clear/compact re-fire costs seconds.
#
# NO SINGLE TOOL MAY COST THE SESSION ITS COMPILER (2026-08-14). This
# script used to die on the first failed download: `set -e` plus a
# `curl | tar` pipeline meant one unreachable host aborted the hook where
# it stood, and everything AFTER that point — the registry warm, the
# `cargo build` that is the whole reason for the container snapshot —
# simply never ran. That is exactly backwards. The rows differ in how
# much they cost when absent:
#
#   * the COMPILER and the WARM BUILD are the session. Nothing here may
#     stand between the agent and `cargo build`;
#   * a TEST-RUNNER or CHECKER (nextest, maturin, ty, admesh) costs its
#     own gate row and nothing else. A session missing one can still
#     build, still `cargo test`, still read code.
#
# So every network-dependent tool below is NON-FATAL and LOUD: it warns
# on stderr naming the tool and what it costs, records itself in
# DEGRADED, and the hook prints the whole list again at the end. Fail-loud
# is preserved — what is dropped is fail-EARLY, which here only ever
# punished the steps that matter most. The hook still exits nonzero if
# the toolchain or the warm build itself fails.
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

# Pinned to hosted CI's version (ci.yml is the single source of truth for
# it; keep the two in sync). A dev/CI tool only — never in a shipped build
# graph. Installed from the official get.nexte.st prebuilt, same as the
# in-repo composite action, not from a third-party channel.
NEXTEST_VERSION=0.9.140
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
NEXTEST_URLS=(
  "https://get.nexte.st/${NEXTEST_VERSION}/linux"
  "https://github.com/nextest-rs/nextest/releases/download/cargo-nextest-${NEXTEST_VERSION}/cargo-nextest-${NEXTEST_VERSION}-x86_64-unknown-linux-gnu.tar.gz"
)
# The other two hosted pins, same single-source-of-truth rule (ci.yml's
# `env:` block): maturin builds the abi3 wheel, `ty` is the static stub
# checker behind tests/test_ty.py. Both dev/CI tools only.
MATURIN_VERSION=1.14.1
TY_VERSION=0.0.39
# ci.yml pins the interpreter at 3.12 — the version the suite was written
# and validated on. The container's default `python3` is 3.11, so name 3.12
# explicitly rather than inheriting whatever `python3` happens to be.
PY=/usr/bin/python3.12
# Outside the repo on purpose: a venv under target/ would be destroyed by
# `cargo clean` and confuse the wheel-vs-staged-cdylib install paths.
VENV="$HOME/.cache/pncad-py/venv"

say() { printf '\n== %s\n' "$1"; }

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
#
# The image ships a different default (stable / cargo 1.94.1 as of
# writing), so this step is also what makes the versions printed below the
# PINNED ones rather than the container's.
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
say "cargo-nextest ${NEXTEST_VERSION}"
if cargo nextest --version 2>/dev/null | grep -qw "${NEXTEST_VERSION}"; then
  echo "already installed: $(cargo nextest --version)"
# --retry (inside fetch_tool): ~10 hosted jobs per run taught us that an
# unretried fetch turns one bad minute on a CDN edge into a whole-gate
# failure (.github/actions/install-nextest carries the same reasoning).
elif fetch_tool cargo-nextest "${NEXTEST_URLS[@]}"; then
  cargo nextest --version
else
  warn "cargo-nextest ${NEXTEST_VERSION}" \
    "\`cargo nextest run\` is unavailable; use \`cargo test\` instead (slower, no per-test process isolation, and NOT what ci.yml runs). \`cargo test --doc\` is unaffected."
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
say "maturin ${MATURIN_VERSION}"
if maturin --version 2>/dev/null | grep -qw "${MATURIN_VERSION}"; then
  echo "already installed: $(maturin --version)"
elif fetch_tool maturin \
    "https://github.com/PyO3/maturin/releases/download/v${MATURIN_VERSION}/maturin-x86_64-unknown-linux-musl.tar.gz"; then
  maturin --version
else
  warn "maturin ${MATURIN_VERSION}" \
    "the WHEEL path of crates/pncad-py is unavailable; run-python-tests.sh still runs by staging the plain cdylib, which is the degraded-box path it was written for."
fi

say "ty ${TY_VERSION} (static stub checker, python 3.12 venv)"
if [ -x "$VENV/bin/ty" ] && "$VENV/bin/ty" --version 2>/dev/null | grep -qw "${TY_VERSION}"; then
  echo "already installed: $("$VENV/bin/ty" --version)"
elif [ ! -x "$PY" ]; then
  warn "ty ${TY_VERSION} (no ${PY})" \
    "tests/test_ty.py will skip — it says so loudly, so the stub lattice is UNMEASURED rather than silently passing."
elif "$PY" -m venv --clear "$VENV" \
     && "$VENV/bin/python" -m pip install -q --disable-pip-version-check "ty==${TY_VERSION}"; then
  "$VENV/bin/ty" --version
else
  warn "ty ${TY_VERSION}" \
    "tests/test_ty.py will skip — it says so loudly, so the stub lattice is UNMEASURED rather than silently passing."
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
  # ONLY THE UBUNTU SOURCES. The base image ships third-party PPAs it
  # does not need for our purposes (deadsnakes, ondrej/php), and those
  # live on ppa.launchpadcontent.net, which the Trusted allow-list does
  # NOT cover — it lists ppa.launchpad.net, a different host. A plain
  # `apt-get update` therefore exits nonzero on their 403s and takes
  # admesh down with it even though archive.ubuntu.com answered fine;
  # that is precisely what happened on the 2026-08-15 setup-script run.
  # Pointing sourcelist at ubuntu.sources with an empty sourceparts
  # limits the refresh to the mirrors we actually install from, so a
  # denied PPA is no longer admesh's problem. Nothing is bypassed: the
  # blocked host is simply not consulted.
  if (DEBIAN_FRONTEND=noninteractive $sudo apt-get update -qq \
        -o Dir::Etc::sourcelist=/etc/apt/sources.list.d/ubuntu.sources \
        -o Dir::Etc::sourceparts=/dev/null \
        -o APT::Get::List-Cleanup=0 \
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
# demos/, tools/ and interval-transcendentals/ are workspace-EXCLUDED on
# purpose (see the root Cargo.toml header) — `--workspace` never reaches
# them, so each needs its own fetch. --locked: the lockfiles are checked
# in and a session must never silently resolve something new.
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
for root in . interval-transcendentals tools/k-lint demos/tour demos/wild; do
  if ( cd "$root" && cargo fetch --locked >/dev/null 2>&1 ); then
    echo "fetched: $root"
  else
    echo "WARNING: cargo fetch --locked failed in ${root} (stale lockfile?)" >&2
    echo "         Its deps are not pre-warmed; the kernel workspace is." >&2
  fi
done

# ---------------------------------------------------------------------------
# 6. Warm target/ so the first real command is incremental.
#
# --all-targets covers the test binaries too, which is what the nextest
# rows need; the doc-test and clippy rows reuse the dependency artifacts.
# Measured on cloud_default, 2026-08-15: 154 s cold with the registry
# already warmed by §5, ~0.2 s warm.
#
# THE SNAPSHOT DOES NOT COVER THIS, contrary to what this comment used to
# claim. Only the SETUP SCRIPT's filesystem is snapshotted, and the setup
# script runs before the clone — there is no target/ to snapshot and no
# repo to build. Hook output is never cached. So this cost is real, and
# paid in full, by every fresh session in this repo.
#
# DETACHED, BEHIND A GRACE PERIOD. The session is handed back as soon as
# the build is demonstrably going to take a while, so the minutes an
# agent spends reading DESIGN.md, the milestone plan and memories before
# its first compile are no longer spent waiting. The grace period is what
# keeps that from being a regression on the common case: a warm tree
# finishes in ~0.2 s, well inside the window, and is reported inline
# exactly as a synchronous build was.
#
# TWO THINGS BACKGROUNDING BREAKS, AND WHAT IS DONE ABOUT EACH:
#
#   * A FAILED BUILD CAN NO LONGER FAIL THE HOOK. That cuts straight
#     against this script's fail-loud posture, so the build's outcome is
#     not left in a pipe nobody reads: it goes to a SENTINEL file whose
#     entire contents are one of `running <pid>`, `ok <n>s`, or
#     `failed rc=<n> after <n>s`, with the full log beside it. The notice
#     printed below names both paths, and hook output is delivered to the
#     agent as session context, so the instruction to check them arrives
#     with the session rather than living in a comment.
#   * CARGO HOLDS AN EXCLUSIVE LOCK ON target/. A `cargo` command issued
#     while the background build runs does not fail — it prints
#     "Blocking waiting for file lock on build directory" and waits — but
#     it can sit there for minutes with no other output, which reads as a
#     hang. That is why the notice says so explicitly instead of leaving
#     it to be rediscovered.
#
# RE-FIRING IS SAFE. SessionStart fires again on resume/clear/compact,
# which can land in the middle of a cold build; the guard below adopts a
# live build rather than starting a second one to contend for the lock.
# `setsid` is what makes the build outlive the hook: without a session of
# its own it is killed with the hook's process group.
WARM_BUILD_GRACE=15
SENTINEL="$repo/target/.session-warm-build"
BUILD_LOG="$repo/target/.session-warm-build.log"
# ---------------------------------------------------------------------------
say "warm build (workspace, all targets)"
mkdir -p "$repo/target"

# Reads the sentinel into `state` (running|ok|failed|empty), `detail` (its
# second field) and `summary` (the whole line, for reporting).
# Guarded on existence rather than redirecting stderr away: a `<` on a
# missing file is reported by the shell BEFORE the command's own
# redirections apply, so `2>/dev/null` on the read does not suppress it.
read_sentinel() {
  state=""
  detail=""
  summary=""
  if [ -f "$SENTINEL" ]; then
    read -r state detail _ < "$SENTINEL" || true
    IFS= read -r summary < "$SENTINEL" || true
  fi
}

# Adopt a live build only if the recorded pid is STILL A BUILD. `kill -0`
# alone answers "does some process hold this pid", which after recycling
# is not the same question — and getting it wrong is the one failure mode
# with no recovery: the hook would report a detached build that does not
# exist, and the session would wait out a compile nobody is running.
read_sentinel
running_pid=""
if [ "$state" = running ] && [ -n "$detail" ] && kill -0 "$detail" 2>/dev/null \
   && tr '\0' ' ' < "/proc/$detail/cmdline" 2>/dev/null | grep -q 'cargo build'; then
  running_pid="$detail"
fi

if [ -n "$running_pid" ]; then
  echo "adopting the build an earlier fire of this hook started (pid ${running_pid})"
else
  : > "$BUILD_LOG"
  # INVALIDATE THE OLD RESULT BEFORE SPAWNING, or the grace loop below
  # races the child and reads the PREVIOUS run's verdict: the child needs
  # a moment to write `running`, and until it does the sentinel still says
  # `ok`/`failed` from last time. A stale `failed` read that way makes a
  # perfectly good build report the last one's failure and exit 1.
  # `starting` is deliberately not a terminal state and not `running`, so
  # neither the grace loop nor the adoption guard above can act on it.
  printf 'starting\n' > "$SENTINEL"
  setsid bash -c '
    cd "$1" || exit 1
    printf "running %s\n" "$$" > "$2"
    start=$SECONDS
    if cargo build --workspace --all-targets >>"$3" 2>&1; then
      printf "ok %ss\n" "$((SECONDS - start))" > "$2"
    else
      rc=$?
      printf "failed rc=%s after %ss\n" "$rc" "$((SECONDS - start))" > "$2"
    fi
  ' _ "$repo" "$SENTINEL" "$BUILD_LOG" </dev/null >/dev/null 2>&1 &
  disown 2>/dev/null || true
fi

# Give a warm tree the chance to finish before we announce anything.
waited=0
while [ "$waited" -lt "$WARM_BUILD_GRACE" ]; do
  read_sentinel
  case "$state" in
    ok|failed) break ;;
  esac
  sleep 1
  waited=$((waited + 1))
done

read_sentinel
case "$state" in
  ok)
    echo "up to date (${detail})"
    ;;
  failed)
    # Finished inside the grace period AND failed: nothing is detached
    # any more, so the fail-loud contract still applies in full.
    echo "WARNING: the warm build FAILED (${summary}). Last lines:" >&2
    tail -20 "$BUILD_LOG" >&2 || true
    exit 1
    ;;
  *)
    {
      echo
      echo "================================================================"
      echo " WARM BUILD STILL RUNNING, DETACHED — the session is yours now."
      echo
      echo "   status:  cat ${SENTINEL}"
      echo "            (starting | running <pid> | ok <n>s"
      echo "             | failed rc=<n> after <n>s)"
      echo "   log:     ${BUILD_LOG}"
      echo
      echo " A cold workspace takes ~2.6 min. Until it finishes, any cargo"
      echo " command you run will BLOCK on the target/ lock — 'Blocking"
      echo " waiting for file lock on build directory', no other output."
      echo " That is the build ahead of you, not a hang; it clears on its"
      echo " own. Reading code costs nothing, so front-load that."
      echo
      echo " NOTHING ELSE WILL TELL YOU IF THIS BUILD FAILED. Check the"
      echo " sentinel before concluding the tree is good."
      echo "================================================================"
    } >&2
    ;;
esac

# ---------------------------------------------------------------------------
# 7. The demo renderers' Python venv (numpy + matplotlib, pinned by
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
# container the tour's kernel lane takes its matplotlib fallback and says
# so loudly, exactly as designed.
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
# ---------------------------------------------------------------------------
if [ "${#DEGRADED[@]}" -gt 0 ]; then
  {
    echo
    echo "================================================================"
    echo " DEGRADED CONTAINER — the compiler and the warm build are fine,"
    echo " but ${#DEGRADED[@]} gate row(s) cannot run in this session:"
    echo
    printf '   * %s\n' "${DEGRADED[@]}"
    echo
    echo " Hosted CI runs the full gate regardless; this only bounds what"
    echo " can be checked BEFORE pushing. If a tool was denied by egress"
    echo " policy, report the blocked host rather than routing around it."
    echo "================================================================"
  } >&2
fi

say "ready"
