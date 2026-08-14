#!/usr/bin/env bash
# SessionStart hook — provision a Claude Code on the web container so the
# gate is runnable the moment a session opens.
#
# WHAT "RUNNABLE" MEANS HERE: the rows of .github/workflows/ci.yml that a
# session actually reruns by hand — `cargo fmt --check`, `cargo clippy
# ... -D warnings`, `cargo nextest run`, `cargo test --doc`, the tripwire
# scripts, `crates/pncad-py/run-python-tests.sh`, and the admesh
# watertight check. Everything those need that a bare container lacks is
# installed below.
#
# REMOTE ONLY. Local machines are provisioned by hand and carry
# machine-local tuning this script must not second-guess (see
# local-scripts/setup-build-env.sh); the guard below makes this a no-op
# outside Claude Code on the web.
#
# SYNCHRONOUS, deliberately: the session must not open onto a half-built
# toolchain, because the first thing an agent does here is compile. The
# container state is snapshotted once the hook finishes, so the cold cost
# below is paid by the first session on a given cache and reused after.
#
# IDEMPOTENT: every step is a no-op when its output is already present, so
# a resume/clear/compact re-fire costs seconds.
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
say "cargo-nextest ${NEXTEST_VERSION}"
if cargo nextest --version 2>/dev/null | grep -qw "${NEXTEST_VERSION}"; then
  echo "already installed: $(cargo nextest --version)"
else
  # --retry: ~10 hosted jobs per run taught us that an unretried fetch
  # turns one bad minute on a CDN edge into a whole-gate failure
  # (.github/actions/install-nextest carries the same reasoning).
  curl --proto '=https' --tlsv1.2 -LsSf --retry 3 --retry-delay 2 \
    "https://get.nexte.st/${NEXTEST_VERSION}/linux" \
    | tar zxf - -C "${CARGO_HOME:-$HOME/.cargo}/bin"
  cargo nextest --version
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
else
  curl --proto '=https' --tlsv1.2 -LsSf --retry 3 --retry-delay 2 \
    "https://github.com/PyO3/maturin/releases/download/v${MATURIN_VERSION}/maturin-x86_64-unknown-linux-musl.tar.gz" \
    | tar zxf - -C "${CARGO_HOME:-$HOME/.cargo}/bin" maturin
  maturin --version
fi

say "ty ${TY_VERSION} (static stub checker, python 3.12 venv)"
if [ -x "$VENV/bin/ty" ] && "$VENV/bin/ty" --version 2>/dev/null | grep -qw "${TY_VERSION}"; then
  echo "already installed: $("$VENV/bin/ty" --version)"
elif [ -x "$PY" ]; then
  "$PY" -m venv --clear "$VENV"
  "$VENV/bin/python" -m pip install -q --disable-pip-version-check "ty==${TY_VERSION}"
  "$VENV/bin/ty" --version
else
  echo "WARNING: no ${PY}; test_ty.py will skip (it says so loudly)." >&2
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
  if ! (DEBIAN_FRONTEND=noninteractive $sudo apt-get update -qq \
        && DEBIAN_FRONTEND=noninteractive $sudo apt-get install -y -qq admesh); then
    echo "WARNING: admesh install failed — scripts/check_admesh.sh will be" >&2
    echo "         unavailable; every other gate row is unaffected." >&2
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
# This is the expensive step (minutes cold, seconds warm) and it is the
# main thing the post-hook container snapshot is worth caching.
# ---------------------------------------------------------------------------
say "warm build (workspace, all targets)"
cargo build --workspace --all-targets

say "ready"
