#!/usr/bin/env bash
# setup-build-env.sh — apply this machine's local cargo build configuration.
#
#   local-scripts/setup-build-env.sh [--print]
#
# EVERY NUMBER BELOW IS A DATED READING ON ONE DEVELOPER BOX, AND NOTHING
# RE-TAKES ANY OF THEM. There is no register for them and there cannot
# usefully be one: the subject is a machine nobody else runs, the readings
# are wall clock and disk on that machine, and this repo reports timings
# rather than gating them (docs/PERF-PLAN.md). They are here as the dated
# REASON a knob is set or was reverted — enough to stop the reverted ones
# being re-adopted from first principles — and not as facts about the box
# you are reading this on. Re-take before arguing from one; a number that
# has moved is information, not a regression to undo.
#
# WHAT IT SETS, AND WHAT IT DELIBERATELY DOES NOT
#
# Sets exactly one thing: `debug = "line-tables-only"` on dev+test.
# Measured 2026-08-11 on this box: target/ 4.7 GB -> 1.5 GB (-68%). On a
# 10 GB-RAM machine carrying ~10 lanes that is page cache and disk. It buys
# no compile time; it is a SIZE knob and is kept for that alone.
#
# Two other knobs were applied on 2026-08-11 and then REVERTED the same day
# on measurement. Both are recorded here so they are not re-adopted from
# first principles by someone reading #174 and reasoning forward:
#
#   mold (via `-C link-arg=-B<mold libexec>`) — NO measurable wall-clock on
#     this workspace. Control run: baseline 189 s vs mold+thin-debug 186 s,
#     cold `cargo build --workspace --all-targets`. That does NOT contradict
#     #174's -38%: that was measured across 261 test binaries, and after
#     #179 plus the step-import collapse (#387) this workspace has 14, so
#     the per-binary link constant mold attacks is now a small share of the
#     build. Dropped rather than kept-because-harmless: it is a
#     machine-specific dependency at a machine-specific path (Ubuntu 20.04
#     has no mold package, and gcc 9.4 cannot even spell `-fuse-ld=mold` —
#     it needs mold's `libexec/mold/ld` shim), and an unused dependency in
#     the build path is a liability, not a free option. Revisit ONLY if the
#     test-target count grows back toward triple digits.
#
#   sccache (`rustc-wrapper` + `incremental = false`) — a NET LOSS here, and
#     the most instructive of the three. It is genuinely good at what it
#     does: a new lane's cold build went 156 s -> 96 s, because the ~225
#     dependency crates are identical across lanes and content-addressed
#     caching serves them. But sccache CANNOT coexist with incremental
#     compilation — it hard-refuses ("incremental compilation is
#     prohibited"), so enabling it forces CARGO_INCREMENTAL=0 machine-wide.
#     Measured cost of that, edit-one-crate-then-rebuild:
#
#       edit             sccache/no-incr    incremental/no-sccache
#       geom-core        91 s, 89 s         18 s, 19 s
#       topo             74 s, 88 s         10 s, 12 s
#
#     5-7x slower on the loop agents run all day, to save ~60 s on an
#     operation that happens once per LANE. Wrong term optimized. If a
#     future need makes cold-lane cost dominate (mass lane churn, say),
#     sccache is the right tool for exactly that and nothing else.
#
# WHY ~/.cargo/config.toml AND NOT THE REPO'S .cargo/config.toml:
#   * The repo file is checked out on CI runners too, where machine-local
#     paths and tools do not exist.
#   * The setting MUST apply to every cargo invocation on this machine. A
#     profile/flag difference is part of cargo's fingerprint, so if only
#     slot-wrapped builds carried it, every unwrapped `cargo check` an agent
#     ran in its own shell would invalidate the whole target/ dir and the
#     next wrapped build would invalidate it back. That thrash would cost
#     far more than the knob saves, which is why this is machine-wide config
#     rather than an export inside with-build-slot.sh.
#   * `local-scripts/gate.sh` unsets RUSTFLAGS to protect its warm target dir.
#     Config-file settings are not env, so the gate sees them CONSISTENTLY
#     on every run — no re-fingerprinting, which is what that unset is for.
#
# DETERMINISM (D9): `debug=` controls DWARF emission only. `debug-assertions`
# and `overflow-checks` are untouched, so fail-loud postconditions keep their
# teeth and backtraces keep file:line. What is given up is variable
# inspection under a debugger.
#
# COST: changing this knob re-fingerprints every lane, so each lane pays ONE
# cold rebuild the next time it builds. After that it is warm forever.
set -euo pipefail

CONFIG="$HOME/.cargo/config.toml"

want_config() {
  # QUOTED heredoc, deliberately: an unquoted one performs command
  # substitution on backticks, and the prose below quotes shell commands.
  # That is not hypothetical — the first draft of this script had a
  # backticked `cargo build --workspace --all-targets` in a COMMENT and ran
  # it (2026-08-11).
  cat <<'EOF'
# Machine-local build configuration — written by local-scripts/setup-build-env.sh
# in the cad repo. See that script for the full rationale, including the two
# knobs (mold, sccache) that were measured and deliberately NOT adopted.
# NOT repo-committed on purpose — CI runners must not see machine-local
# settings.

[profile.dev]
# Full DWARF across the workspace's test binaries is the bulk of target/
# size: measured 4.7 GB -> 1.5 GB on this box. line-tables-only keeps
# backtraces with file:line and loses only debugger variable inspection.
# debug-assertions and overflow-checks are NOT affected. This is a SIZE
# knob — it buys no measurable compile time here.
debug = "line-tables-only"

[profile.test]
debug = "line-tables-only"

# Deliberately ABSENT (measured 2026-08-11, both reverted):
#   rustc-wrapper = "sccache" + incremental = false
#     -> 5-7x slower edit-rebuild loop (91s vs 18s); sccache and incremental
#        are mutually exclusive, and the edit loop dominates by frequency.
#   [target.x86_64-unknown-linux-gnu] rustflags = mold
#     -> no measurable wall-clock at 14 test targets (189s vs 186s control).
EOF
}

if [ "${1:-}" = "--print" ]; then
  want_config
  exit 0
fi

mkdir -p "$(dirname "$CONFIG")"
if [ -e "$CONFIG" ] && ! grep -q 'written by local-scripts/setup-build-env.sh' "$CONFIG"; then
  echo "ERROR: $CONFIG exists and was not written by this script — refusing to" >&2
  echo "       clobber it. Merge the block from \`$0 --print\` by hand." >&2
  exit 1
fi
want_config > "$CONFIG"
echo "config: wrote $CONFIG"
echo
echo "Every lane pays ONE cold rebuild on its next build, then stays warm."
