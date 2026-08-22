#!/usr/bin/env bash
# Shared rustdoc gate — the SINGLE implementation, called by BOTH
# .github/workflows/ci.yml (its `fmt` job, since #852 folded the old
# standalone `doc` job into it) and local-scripts/ci-local.sh,
# the ci-filter.py arrangement applied to a second gate. Both halves
# run `--selftest` first, the way every gate in scripts/gates/ does.
#
# WHY THIS GATE EXISTS (#465). `cargo check` and `cargo clippy -- -D
# warnings` are both SILENT about broken documentation: the relevant
# lints (rustdoc::broken_intra_doc_links and friends) are RUSTDOC lints,
# emitted only when rustdoc actually builds the docs. `cargo test --doc`
# is not a substitute — it executes doc EXAMPLES and says nothing about
# whether the prose renders. `missing_docs` is caught, because it is a
# rustc lint that clippy sees, which makes the coverage look better than
# it is.
#
# The failure that surfaced this (PR #463): reflowing a doc comment put
# `> 0` at the start of a line, markdown read the leading `>` as a
# blockquote, the enclosing code span terminated early, and `knots[span]`
# became an unresolved intra-doc link. The gate was green. In a codebase
# where the invariant argument lives in the doc comments, prose that
# quietly stops rendering is a real loss.
#
# --document-private-items is deliberate: much of the load-bearing prose
# sits on private functions (span_offset, span_indices,
# frame_from_unit_aim), and without the flag those are never rendered and
# never checked.
#
# WHY private_intra_doc_links IS ALLOWED. That lint fires when a public
# doc comment links to a private item, warning that the link "resolves
# only because you passed --document-private-items". Here that condition
# is not an accident, it is the configuration: this gate ALWAYS passes
# the flag, so those links always resolve in the docs this repo actually
# builds. Leaving the lint on would mean 82 warnings whose only remedy is
# to stop linking public prose to the private functions it is about —
# exactly backwards for a codebase whose private helpers carry the
# argument. Whether to reinstate it (and render two doc sets, public and
# private) is banked as its own question: issue #519.
#
# COVERAGE: EVERY CARGO MANIFEST IN THE TREE, IN TWO PASSES. `cargo doc
# --workspace` sees workspace MEMBERS, and the root manifest excludes
# `demos`, `tools` and `interval-transcendentals` — five more cargo
# roots whose prose that pass cannot reach. So the gate runs the
# workspace pass and then one `--no-deps` pass per manifest the
# workspace pass did not cover. The two sets are complementary BY
# CONSTRUCTION: `cargo metadata` answers which manifests `--workspace`
# just documented, `find` answers which exist, and every manifest lands
# in exactly one pass.
#
# THE ROOT LIST IS DERIVED AND MUST STAY DERIVED. A literal list here
# would be the second hand-written roster in this repo, and
# scripts/gates/gate-roster.sh exists because the first one drifted: a
# root added to `workspace.exclude` and forgotten here would be a tree
# nothing documents, reading as covered. Deriving it from MEMBERSHIP
# rather than from parsing `exclude` is the stronger of the two
# available derivations — it is a statement about what the workspace
# pass actually covered, so a root that is outside the workspace for
# some reason other than `exclude` (a nested `[workspace]` table nobody
# listed) is covered too, and a renamed or restructured field cannot
# make the gate go quiet.
#
# The excluded roots went uncovered, and the hole was patched one root
# at a time: #709 moved ~1,050 lines of prose from
# `crates/mesh/src/budget.rs` into `tools/tess-meter` — prose that went
# from covered to uncovered BY MOVING — and answered it with a `cargo
# doc` step hand-copied into that crate's `k-lint` row; #738 copied the
# step again for `tools/tess-lint`. Both copies are gone: this gate
# covers those two roots along with the other three, so a third copy is
# never the answer to a fourth root.
#
# FEATURES, AND WHY THE TWO PASSES DIFFER.
#
# --all-features on the WORKSPACE pass, UNLIKE the clippy job. Clippy
# avoids it because the `interval` feature is a second build graph whose
# test targets would double that job's compile time for no extra
# coverage, and the interval job owns its own clippy pass. Neither
# reason survives here: rustdoc builds no test targets, and there is no
# second doc job. What the flag buys is real — under default features
# alone, every doc link into `#[cfg(feature = "probe")]` or
# `#[cfg(feature = "interval")]` code resolves to nothing, so rustdoc
# reported 12 CORRECT links as broken while the prose on those items
# went unchecked entirely. Documenting the full feature set is also what
# docs.rs does by default.
#
# DEFAULT features on the roots outside the workspace, which is the
# posture the two deleted hand-copied steps already had, and a ruling
# this repo has already made once: ci.yml's `interval backend crate` job
# is "deliberately the crate's DEFAULT feature set: without
# `oracle-inari` there is no inari/gmp-mpfr-sys and no C toolchain in
# the graph". That crate's ONLY feature is that test-only oracle (its
# manifest: `src/` must never reference `inari` or this feature), so
# --all-features there would document not one extra line while dragging
# the LGPL C build back into a doc job — the exact thing the exclusion
# exists to prevent.
#
# TARGETS, AND THE HALF-COVERED PACKAGE. `cargo doc`'s default target
# selection documents a package's library and SKIPS a binary that shares
# its name — silently, with no warning and a zero exit. `tools/k-lint`
# is exactly that arrangement, and a broken intra-doc link planted in
# its `src/main.rs` passed this gate green. So every pass here runs a
# second `--bins` invocation; see doc_pass below for why that spelling
# and not `--lib --bins`. What is still uncovered by construction is
# `tests/`, `examples/` and `benches/`: rustdoc builds no test targets
# at all, so an intra-doc link written under `tests/` is inert on every
# tier. That is a decision this gate cannot take on its own — it is
# banked as D113, with the census at S135.
#
# THE RESIDUAL, stated because it is real: `demos/tour`'s `probe` and
# `budget` features gate the tour's OWN `src/probe.rs` and its dispatch,
# not only the kernel features they forward, so that file's prose is
# documented by nothing. It is the one place where the rule above costs
# coverage rather than only cost.
#
# THE SELF-TEST, AND THE ONE THING NOT WATCHING IT. Every gate in
# scripts/gates/ runs `--selftest` first, on the standing rule that a
# guard never shown to fire is not a guard; this one had none, so
# dropping `-D warnings` from RUSTDOCFLAGS, `--document-private-items`
# from the invocation, or the second pass below would each have left it
# green over a broken tree with nothing saying so. The fixture lives
# here rather than in scripts/gates/ because the local half runs THAT
# WHOLE DIRECTORY in a loop inside its `discipline` row, which is greps
# plus one `cargo metadata` — moving this script there would put a full
# `cargo doc --workspace --all-features` inside that row, which
# ci-local.sh already runs as a row of its own.
#
# THE TRADE THAT BUYS, WRITTEN DOWN RATHER THAN LEFT TO BE
# REDISCOVERED: `gate-roster.sh` proves that both halves call each
# gate's `--selftest` AND the gate — and its roster is the DIRECTORY, so
# it says nothing about this script. Delete `--selftest` from either
# half's rustdoc row and no gate reds. That is the price of the siting
# above, and it is paid knowingly.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/gates/lib.sh"

GATE_SCAN_NOUN="cargo root"
RUSTDOC_LINTS="-D warnings -A rustdoc::private_intra_doc_links"

# Physical path of a file, without depending on `realpath`: cargo
# reports canonical manifest paths and a `--root` under /tmp can be a
# symlink, so both sides of the membership comparison are normalised the
# same way.
abs_path() {
  (cd "$(dirname "$1")" && printf '%s/%s\n' "$(pwd -P)" "$(basename "$1")")
}

# Every manifest in the tree. `target/` is pruned because a build
# directory holds vendored manifests that are not this repo's prose, and
# `.git/` because a packed tree is not source.
tree_manifests() {
  find . \( -name target -o -name .git \) -prune -o -name Cargo.toml -print \
    | sort
}

# The manifests `cargo doc --workspace` just covered. `--no-deps` makes
# cargo report the workspace members and nothing else, which is exactly
# the set that pass documented.
workspace_manifests() {
  local meta
  if ! meta=$(cargo metadata --no-deps --format-version 1 2>&1); then
    printf '%s\n' "$meta" >&2
    gate_error "$(gate_name): cargo metadata failed in $PWD, so the gate cannot tell which manifests the workspace pass covered — it would document the members twice and the roots outside the workspace not at all"
    exit 1
  fi
  printf '%s' "$meta" | grep -o '"manifest_path":"[^"]*"' | cut -d'"' -f4
}

# One pass, its own diagnosis, and TWO cargo invocations — because
# `cargo doc`'s default target selection SILENTLY SKIPS a binary whose
# name matches its package's library. `tools/k-lint` is the live case:
# lib.rs and main.rs, one crate name, and a planted broken link in
# main.rs left this gate green until `--bins` was added. The second
# invocation is a no-op warning ("target filter `bins` specified, but no
# targets matched") for a package with no bin, which is what makes it
# safe to run unconditionally; `--lib --bins` is NOT the shorter
# spelling, because `--lib` is a hard error on a bin-only package like
# `demos/tour`.
doc_pass() {
  local why=$1; shift
  local rc=0
  RUSTDOCFLAGS="$RUSTDOC_LINTS" cargo doc --no-deps --document-private-items "$@" || rc=1
  RUSTDOCFLAGS="$RUSTDOC_LINTS" cargo doc --no-deps --document-private-items --bins "$@" || rc=1
  [ "$rc" -eq 0 ] || gate_error "$(gate_name): rustdoc rejected $why"
  return "$rc"
}

gate() {
  local rc=0 m members n=0
  local -a roots=()
  if [ ! -f Cargo.toml ]; then
    gate_error "$(gate_name): no Cargo.toml under $PWD — the gate documented nothing, which is not a pass"
    exit 1
  fi

  # PASS 1 — the workspace.
  doc_pass "the workspace pass — a doc comment above has stopped rendering (a link to a renamed, deleted, or test-only item is the usual cause), and clippy is blind to every one of these lints" \
    --workspace --all-features || rc=1
  n=1

  # PASS 2 — every manifest the workspace pass did not cover, one
  # `--no-deps` pass each, so a package in a nested workspace is
  # documented exactly once however those workspaces are arranged.
  members=$(workspace_manifests)
  while IFS= read -r m; do
    # THE WORKSPACE'S OWN MANIFEST IS NOT A MEMBER OF IT. This one is
    # virtual (no root package), so `cargo metadata`'s package list
    # cannot contain it and the membership test below reads it as an
    # uncovered root — whereupon this loop documents every member a
    # SECOND time under DEFAULT features, which is exactly the misread
    # the --all-features paragraph above exists to prevent: twelve
    # correct links into `#[cfg(feature = ...)]` code report as broken.
    # It is the manifest pass 1 was invoked on; it is covered.
    [ "$m" = ./Cargo.toml ] && continue
    case "$members" in
      *"$(abs_path "$m")"*) continue ;;
    esac
    roots+=("$m")
  done < <(tree_manifests)

  for m in ${roots[@]+"${roots[@]}"}; do
    n=$((n + 1))
    doc_pass "$m — this cargo root is outside the kernel workspace, so the workspace pass never reads its prose and this pass is the only thing that does" \
      --manifest-path "$m" || rc=1
  done

  GATE_SCAN_FILES=$n
  [ "$rc" -eq 0 ] || exit 1
  gate_ok "every cargo manifest in the tree renders, library and binary targets alike: the workspace under --all-features, plus ${#roots[@]} root(s) outside it under default features"
}

# THE FIXTURE. Dependency-free by design — the cases are about this
# script's control flow and its flags, not about the kernel, and a
# fixture that built the kernel would cost minutes to prove seconds of
# logic. Two packages, because the gate has two passes: a workspace
# MEMBER and a root the workspace EXCLUDES. `edition = "2021"` rather
# than the repo's 2024: the fixture is built by whatever toolchain is
# default outside this tree, since rust-toolchain.toml does not reach a
# /tmp directory. Both packages carry a lib AND a same-named bin, which
# is the arrangement `cargo doc`'s default target selection skips half
# of — the fixture has to contain it or the `--bins` invocation above is
# unguarded.
gate_plant_clean() {
  local r=$1
  mkdir -p "$r/crates/clean/src" "$r/outside/src"
  {
    printf '[workspace]\nresolver = "2"\n'
    printf 'members = ["crates/clean"]\n'
    printf 'exclude = ["outside"]\n'
  } > "$r/Cargo.toml"
  {
    printf '[package]\nname = "clean"\nversion = "0.0.0"\nedition = "2021"\n'
  } > "$r/crates/clean/Cargo.toml"
  {
    printf '//! A workspace member whose prose links to [`identity`].\n'
    printf 'pub fn identity(x: f64) -> f64 { x }\n'
  } > "$r/crates/clean/src/lib.rs"
  printf '//! The member bin, linking to [`main`].\nfn main() {}\n' \
    > "$r/crates/clean/src/main.rs"
  {
    printf '[workspace]\n\n'
    printf '[package]\nname = "outside"\nversion = "0.0.0"\nedition = "2021"\n'
  } > "$r/outside/Cargo.toml"
  {
    printf '//! A root outside the workspace, linking to [`identity`].\n'
    printf 'pub fn identity(x: f64) -> f64 { x }\n'
  } > "$r/outside/src/lib.rs"
  printf '//! The outside bin, linking to [`main`].\nfn main() {}\n' \
    > "$r/outside/src/main.rs"
}

# The shape #740 and #744 arrived in, and the shape #755 arrived in: a
# link to an item that is not there.
plant_broken_link_in_member() {
  printf '\n/// Links to [`no_such_item`].\npub fn documented() {}\n' \
    >> "$1/crates/clean/src/lib.rs"
}

# THE CASE THE SECOND PASS EXISTS FOR, and the one it can silently fail
# open on: the same break, in a root `--workspace` cannot see. A gate
# that lost its excluded-root loop passes every other case here.
plant_broken_link_in_excluded_root() {
  printf '\n/// Links to [`no_such_item`].\npub fn documented() {}\n' \
    >> "$1/outside/src/lib.rs"
}

# THE SAME-NAMED BINARY, in each pass. `cargo doc` documents the lib and
# skips the bin that shares its name, so without the `--bins`
# invocation every one of these files is prose no rustdoc lint has ever
# read — which is how `tools/k-lint/src/main.rs` passed a planted break.
plant_broken_link_in_member_bin() {
  printf '\n/// Links to [`no_such_item`].\npub fn documented() {}\n' \
    >> "$1/crates/clean/src/main.rs"
}

plant_broken_link_in_excluded_root_bin() {
  printf '\n/// Links to [`no_such_item`].\npub fn documented() {}\n' \
    >> "$1/outside/src/main.rs"
}

# --document-private-items, pinned. Without the flag rustdoc never
# renders a private item and so never resolves its links, and this
# fixture goes green — which is precisely how the flag could be dropped
# from the invocation above with no test noticing.
plant_broken_link_on_private_item() {
  printf '\n/// Links to [`no_such_item`].\nfn private_helper() {}\n' \
    >> "$1/crates/clean/src/lib.rs"
}

# THE NEAR MISS, and it pins the `-A rustdoc::private_intra_doc_links`
# decision argued in the header: public prose linking to a private
# sibling is the house style here, not a defect, and a widened gate that
# reds on it would be telling this codebase to stop explaining itself.
plant_public_link_to_private_item() {
  {
    printf '\n/// Explained by [`private_helper`].\npub fn documented() {}\n'
    printf 'fn private_helper() {}\n'
  } >> "$1/crates/clean/src/lib.rs"
}

gate_selftest() {
  local want="rustdoc rejected"
  gate_selftest_clean
  gate_selftest_case "$want" plant_broken_link_in_member
  gate_selftest_case "$want" plant_broken_link_in_excluded_root
  gate_selftest_case "$want" plant_broken_link_in_member_bin
  gate_selftest_case "$want" plant_broken_link_in_excluded_root_bin
  gate_selftest_case "$want" plant_broken_link_on_private_item
  gate_selftest_passes "public prose linking to a private sibling" \
    plant_public_link_to_private_item
  printf '%s selftest OK: passes a clean two-root fixture and a public link to a private sibling; fires on a broken link in a workspace member and in a root outside the workspace, in each of their same-named binaries, and on a private item\n' \
    "$(gate_name)"
}

gate_parse_args "$@"
gate_main
