#!/usr/bin/env bash
# no-ambient-env.sh — no ambient environment in the kernel. ONE home;
# ci.yml's "no ambient environment in the kernel" step and
# local-scripts/ci-local.sh's discipline row both call this file.
#
# NO AMBIENT ENVIRONMENT IN THE KERNEL. A runtime env read inside
# crates/*/src is a back channel: it changes shipped behaviour
# with no rebuild, no flag, and no call site to review. This gate
# exists because one had been live — `mesh::probe_stats::armed()`
# answered true for `NURBS_PROBE`, which switched on a 91-sample
# resampling of every emitted triangle (measured 7.9 s -> 19.8 s
# on the tour's release binary, same binary, same arguments) AND
# put an `assert!` in the tessellation path, so an environment
# variable converted `tessellate`'s typed error contract into a
# panic.
#
# WHEN AN AMBIENT CHANNEL IS NOT THAT (S22, ruled 2026-08-19).
# The indictment above is the rule, and it is not discharged by a
# variable being USEFUL — that argument would have saved
# NURBS_PROBE too. A channel escapes it only when all four hold:
#
#  1. CONTRACT-RATIFIED. The value is a named parameter of the
#     ratified model contract, not an implementation switch. The
#     model is a pure function of (parameter vector, eps); eps is
#     in that signature, and a probe's sampling density is not.
#  2. COMMIT-ONCE, IMMUTABLE. One get_or_init, and no API to
#     change it afterwards — so "shipped behaviour with no call
#     site to review" is bounded to a single decision made before
#     the first predicate runs, not a switch flipped under one.
#  3. REPORTED. The committed value AND ITS PROVENANCE are
#     visible in the run's output, so "no call site to review"
#     does not mean "nobody can tell". A channel that fails this
#     one is indistinguishable from NURBS_PROBE from outside.
#  4. RECONCILED. A more authoritative source either wins or
#     refuses — the ambient value is a BOOTSTRAP, never the last
#     word.
#
# NURBS_PROBE had none of the four. It also failed a fifth thing
# these two do not: it changed the TYPED ERROR CONTRACT, putting
# an assert! in a path whose refusals are values.
#
# The allowlist is the two RATIFIED knobs, both read once and
# documented where they are read:
#  - geom-core tolerance.rs — CAD_TOLERANCE_EPS, the eps matrix
#    this whole workflow is built on (a OnceLock, read at init).
#    Rows 1, 2 and 4 have always held: eps is D4's declared run
#    parameter; the OnceLock is the only thing STRUCTURALLY
#    enforcing one eps per process (which is why S22 kept it
#    rather than threading eps to the predicate funnels); and a
#    loaded document's recorded eps outranks an unread env value,
#    with a disagreement refusing as ToleranceConflict. Row 3 was
#    the one it FAILED, and that gap is the whole content of
#    issues #415 and #497 — a stale value in a shell changed what
#    "coincident" means with no output line saying so. It is
#    closed now: EpsilonSource records which channel committed the
#    value, and Tolerance::report / committed_report render it,
#    exposed as `pncad::tolerance` and printed by the demo runs.
#    So this entry is an INSTANCE OF THE RULE ABOVE, not an
#    exemption from it — the escape-hatch sentence in the error
#    message below means "argue the four rows", nothing weaker.
#  - test-utils fuzz.rs — CAD_FUZZ_SEED / CAD_FUZZ_EFFORT, and
#    test-utils is a dev-only leaf no shipped build can reach.
#    This one is discharged by REACHABILITY before the four rows
#    are reached: there is no shipped behaviour to change.
#  - viewer frame.rs — the GUI shell's PLATFORM PROBES (#1097
#    first-light hardening): PATH + DBUS_SESSION_BUS_ADDRESS for
#    the file-chooser-backend verdict, WSL_DISTRO_NAME/WSL_INTEROP
#    for the WSLg X11 preference. The data flow is the REVERSE of
#    NURBS_PROBE's: the environment is the SUBJECT being observed,
#    not a knob into the model — no read can change what any
#    document evaluates to; they adapt chrome affordances (disable
#    a dialog with the reason attached, prefer the X11 backend
#    where Wayland RAIL is broken). Against the rows: read once at
#    startup and stored, never re-read under a running app
#    (commit-once); the chooser verdict is rendered in the UI
#    itself — the disabled control's tooltip IS the report — and
#    the WSL preference is in the crate README (reported); the
#    dialog's own outcome outranks the portal hint, which is
#    documented as a hint and nothing stronger (reconciled).
#    CONTRACT-RATIFIED holds vacuously: these are not model
#    parameters at all, and the windowing/dialog stack underneath
#    (winit, rfd) already reads WAYLAND_DISPLAY/DISPLAY and the
#    portal environment ambiently on every start — the probes make
#    a dependence that already exists visible instead of adding a
#    new kind. ONE file on purpose: every ambient read the viewer
#    performs lives in frame.rs, so this entry is a single door,
#    not a pattern.
#
# `env!` is deliberately NOT matched: it is compile-time, baked
# into the binary, and cannot be an ambient channel.
#
# Telemetry is the recurring offender, so it has a rule of its
# own: gate it behind a feature at the module boundary from the
# first commit, and arm it by an explicit call — never by the
# environment. Worked example: `mesh::budget`.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

gate() {
  gate_require_crate_sources
  local hits
  hits=$(gate_rust_code "${GATE_SOURCE_FILES[@]}" \
    | gate_grep -P '\benv::vars?(_os)?\s*\(' \
    | gate_grep -vE '^crates/geom-core/src/tolerance\.rs:' \
    | gate_grep -vE '^crates/test-utils/src/fuzz\.rs:' \
    | gate_grep -vE '^crates/viewer/src/frame\.rs:')
  if [ -n "$hits" ]; then
    echo "$hits"
    gate_error "a kernel crate reads the environment at runtime — that is a back channel into shipped code, changing behaviour with no rebuild and no call site to review (NURBS_PROBE was exactly this). Arm it by an explicit call and gate it behind a feature, or ratify this file into the allowlist."
    exit 1
  fi
  gate_ok "no kernel crate reads the environment at runtime"
}

plant() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn armed() -> bool { std::env::var("PLANTED_PROBE").is_ok() }\n' \
    > "$1/crates/planted/src/lib.rs"
}

plant_after_block_comment() {
  mkdir -p "$1/crates/planted/src"
  printf 'pub fn armed() -> bool { /* why */ std::env::var("PLANTED_PROBE").is_ok() }\n' \
    > "$1/crates/planted/src/lib.rs"
}

# THE NEAR MISSES, and the fourth line is the one that matters: the call
# is spelled inside a STRING LITERAL, which is how a doc constant names
# the back channel it forbids. A gate reading literals as code reds on
# its own documentation.
plant_prose_only() {
  mkdir -p "$1/crates/planted/src"
  {
    printf '//! Never call env::var here - arm it by an explicit call.\n'
    printf '/*\n * Nor std::env::var_os inside a block comment.\n */\n'
    printf 'pub const WHY: &str = "env::var(NURBS_PROBE)";\n'
    printf 'pub fn ok(a: f64) -> f64 { a } // nor env::vars() in a trailing one\n'
  } > "$1/crates/planted/src/lib.rs"
}

gate_selftest() {
  local want="a kernel crate reads the environment at runtime"
  gate_selftest_clean
  gate_selftest_case "$want" plant
  gate_selftest_case "$want" plant_after_block_comment
  gate_selftest_passes "prose, a block comment and a string literal naming the call" plant_prose_only
  printf '%s selftest OK: passes a clean fixture and prose/block-comment/string-literal mentions of the call; fires on a read, and on one hidden behind a block comment\n' "$(gate_name)"
}

gate_parse_args "$@"
gate_main
