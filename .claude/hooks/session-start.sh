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
# TWO ENTRY POINTS, ONE FILE — AND THEY RUN IN DIFFERENT WORLDS. Claude
# Code on the web provisions a container from two places, and this script
# is meant to be installed in BOTH:
#
#   SETUP SCRIPT — pasted into the environment dialog at claude.ai/code.
#     Runs as root, once per environment cache, BEFORE Claude Code
#     launches and BEFORE THE REPO IS CLONED. Anthropic then snapshots
#     the filesystem and reuses it, so whatever this installs is free for
#     every later session. Re-runs only when the script text changes, the
#     allowed-hosts list changes, or the cache expires (~7 days).
#     Constraints: must EXIT ZERO (a nonzero exit fails the session
#     outright) and finish inside ~5 minutes.
#   SESSIONSTART HOOK — registered in .claude/settings.json. Runs after
#     the clone, with the repo on disk, on every session AND every
#     resume/clear/compact. NOT cached, ever; its cost is paid in full
#     each time.
#
# THE 2026-08-15 BREAKAGE, and the reason for the split below: this file
# assumed the repo. Pasted verbatim into the setup-script box it resolved
# `repo` to `/`, reported the container's default `stable` instead of the
# pinned channel because rust-toolchain.toml was not there to read,
# `cd`-ed into demo roots that did not exist, and finally died on
# `cargo build` with "could not find Cargo.toml in `/`" — exit 101, which
# in setup-script context means THE SESSION DOES NOT START. So the phase
# is DETECTED, not assumed, and each half does only what its world can:
#
#   * REPO-INDEPENDENT (rustup, nextest, maturin, ty, admesh) runs in
#     both. In the setup script it lands in the snapshot and every later
#     session finds it already installed; in the hook it is a no-op when
#     the snapshot already carried it, and the fallback when there is no
#     setup script configured at all.
#   * REPO-DEPENDENT (the toolchain pin, cargo fetch, the warm build, the
#     demo venv) runs ONLY when the repo is actually on disk — §5-§7 are
#     skipped wholesale otherwise.
#
# Neither half is redundant. The setup script cannot warm target/ (no
# repo, and the build is minutes past the 5-minute budget); the hook
# cannot cache anything. Configure both.
#
# REMOTE ONLY, when there is a repo. Local machines are provisioned by
# hand and carry machine-local tuning this script must not second-guess
# (see local-scripts/setup-build-env.sh); the guard below makes the hook
# a no-op outside Claude Code on the web. The setup-script phase is
# exempt from that guard on purpose — see the guard for why.
#
# THE WARM BUILD IS SYNCHRONOUS, deliberately: the session must not open
# onto a half-built toolchain, because the first thing an agent does here
# is compile. It is the expensive step (~9.5 min cold, 0.2 s warm) and,
# because hooks are never snapshotted, it is paid by every fresh session.
# See §6 for why backgrounding it is not the free win it looks like.
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

# WHICH PHASE ARE WE IN? Ask the filesystem, not the environment. The
# question that actually matters is "is the repo here?", and the honest
# test for that is whether rust-toolchain.toml — the file §1 needs — can
# be read. CLAUDE_PROJECT_DIR is the hook's answer; falling back to the
# script's own location covers a hook invoked with it unset, and a
# by-hand run from an arbitrary cwd.
#
# `repo` empty means SETUP-SCRIPT PHASE: no clone yet, §5-§7 skipped.
repo=""
for candidate in "${CLAUDE_PROJECT_DIR:-}" \
                 "$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." 2>/dev/null && pwd)"; do
  if [ -n "$candidate" ] && [ -f "$candidate/rust-toolchain.toml" ]; then
    repo="$candidate"
    break
  fi
done

# The remote-only guard applies to the HOOK, which is the only phase that
# can fire on Evan's laptop. It deliberately does NOT gate the
# setup-script phase: that phase is reached only when no repo is on disk,
# which never happens locally, and gating it on an environment variable
# would mean one unset variable silently provisioning nothing and
# snapshotting the emptiness for a week.
if [ -n "$repo" ]; then
  if [ "${CLAUDE_CODE_REMOTE:-}" != "true" ]; then
    exit 0
  fi
  cd "$repo"
  printf '\n== phase: SessionStart hook (repo at %s)\n' "$repo"
else
  printf '\n== phase: setup script (no repo on disk yet — VM provisioning only)\n'
  printf '   Repo-dependent steps (cargo fetch, warm build, demo venv) are the hook'"'"'s job.\n'
fi

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
# HOOK PHASE: `rustup show active-toolchain` honours rust-toolchain.toml —
# it installs the pinned channel AND its components list on first call,
# and prints in under a second afterwards. Doing it here rather than
# letting the first `cargo` invocation trigger it keeps the download out
# of the middle of an agent's first command, where it interleaves with
# real output.
#
# SETUP-SCRIPT PHASE: rust-toolchain.toml is not on disk, so there is
# nothing to honour — this is exactly how the broken run came to report
# `stable` / cargo 1.94.1 where the repo pins 1.97.0. The channel is
# therefore named by the literal below, which is the ONE place in this
# script that duplicates a fact the repo already states.
#
# THAT DUPLICATE IS CHECKED, NOT TRUSTED. A stale literal here would be
# invisible in the worst way: the snapshot would carry a toolchain nobody
# uses, every session would silently re-download the real one, and the
# setup script would look like it was working. So the hook phase — which
# CAN read rust-toolchain.toml — diffs the two and warns loudly on drift.
# Installing is not the same as selecting: rust-toolchain.toml still
# picks the channel once the repo lands, so this only pre-populates
# ~/.rustup for the snapshot and never overrides the pin.
RUST_TOOLCHAIN=1.97.0
# ---------------------------------------------------------------------------
say "rust toolchain (rust-toolchain.toml)"
if [ -n "$repo" ]; then
  rustup show active-toolchain
  pinned=$(sed -n 's/^[[:space:]]*channel[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' \
             rust-toolchain.toml | head -1)
  if [ -n "$pinned" ] && [ "$pinned" != "$RUST_TOOLCHAIN" ]; then
    echo "WARNING: this script's RUST_TOOLCHAIN literal (${RUST_TOOLCHAIN}) has drifted" >&2
    echo "         from rust-toolchain.toml (${pinned}). This session is fine — the" >&2
    echo "         .toml wins here — but the SETUP SCRIPT is pre-installing the wrong" >&2
    echo "         channel, so every session pays a toolchain download the snapshot" >&2
    echo "         was supposed to have covered. Update the literal in §1." >&2
  fi
elif rustup toolchain install "$RUST_TOOLCHAIN" \
       --profile minimal --component rustfmt --component clippy; then
  echo "pre-installed for the snapshot: ${RUST_TOOLCHAIN} (+rustfmt, clippy)"
else
  warn "the pinned toolchain ${RUST_TOOLCHAIN}" \
    "the snapshot carries no pinned compiler, so the FIRST cargo command of every session in this environment pays the rustup download instead. Nothing is broken; startup is just slower than it should be."
fi

# Name the channel explicitly without a repo: a bare `cargo --version`
# there reports the image's default (1.94.1 as of writing), which reads
# like the pin failed one line after it succeeded.
tc=()
[ -n "$repo" ] || tc=("+${RUST_TOOLCHAIN}")
cargo "${tc[@]}" --version
cargo "${tc[@]}" fmt --version
cargo "${tc[@]}" clippy --version

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
#
# SKIPPED WHOLESALE WITHOUT A REPO. In the setup-script phase there are no
# lockfiles to honour, and the loop's `cd` into each root is what produced
# the "No such file or directory" spray in the 2026-08-15 failure.
# ---------------------------------------------------------------------------
if [ -z "$repo" ]; then
  say "skipping cargo fetch / warm build / demo venv (setup-script phase)"
  echo "The repo is not cloned yet; those are the hook's half of the split."
  if [ "${#DEGRADED[@]}" -gt 0 ]; then
    {
      echo
      echo "================================================================"
      echo " SETUP SCRIPT FINISHED WITH ${#DEGRADED[@]} TOOL(S) MISSING from the snapshot:"
      echo
      printf '   * %s\n' "${DEGRADED[@]}"
      echo
      echo " The SessionStart hook retries every one of them per session, so"
      echo " this is a slower startup rather than a lost gate row. If a tool"
      echo " was denied by egress policy, report the blocked host rather than"
      echo " routing around it."
      echo "================================================================"
    } >&2
  fi
  # EXIT ZERO, ALWAYS. A nonzero exit here does not degrade the session,
  # it prevents the session: "Setup script failed with exit code 101 —
  # edit your environment's setup script and start a new session". Every
  # absence above has already been reported loudly and is recoverable by
  # the hook, so none of them is worth that.
  say "ready (VM provisioned; the hook finishes the job once the repo lands)"
  exit 0
fi

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
# Measured on cloud_default: ~9.5 min cold, ~0.2 s warm.
#
# THE SNAPSHOT DOES NOT COVER THIS, contrary to what this comment used to
# claim. Only the SETUP SCRIPT's filesystem is snapshotted, and the setup
# script runs before the clone — there is no target/ to snapshot and no
# repo to build. Hook output is never cached. So this cost is real, and
# paid in full, by every fresh session in this repo.
#
# WHY IT IS STILL SYNCHRONOUS. Backgrounding it (`nohup cargo build &`,
# or the hook's `{"async": true}` mode) is legal and the docs even
# recommend the shape for long installs — but here it buys less than it
# looks like and costs more:
#
#   * cargo holds an exclusive lock on target/. A background build does
#     not make the agent's first `cargo` command faster, it makes it
#     BLOCK — on "Blocking waiting for file lock on build directory",
#     with no progress output, for as long as the build has left to run.
#     A wedged-looking session is worse than an honest wait;
#   * a backgrounded failure is a silent failure. This script's whole
#     posture is fail-loud, and `nohup` puts the one step that is allowed
#     to fail the hook somewhere nobody reads;
#   * the win is bounded by how long the agent spends NOT compiling at
#     session start — real (reading DESIGN.md, the milestone plan,
#     memories) but a few minutes, against a 9.5-minute build.
#
# The honest fix for cold-start cost is the setup script covering more of
# the toolchain, which is what the split above does. Revisit
# backgrounding only with a sentinel file the agent is told to check.
# ---------------------------------------------------------------------------
say "warm build (workspace, all targets)"
cargo build --workspace --all-targets

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
