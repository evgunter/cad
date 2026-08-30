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
# COVERAGE: EVERY CARGO MANIFEST THE REPOSITORY TRACKS, IN TWO PASSES.
# `cargo doc --workspace` sees workspace MEMBERS, and the root manifest
# excludes `benches`, `demos`, `tools` and `interval-transcendentals` —
# seven more cargo roots (`benches`, `demos/tour`, `demos/wild`,
# `tools/k-lint`, `tools/tess-lint`, `tools/tess-meter`,
# `interval-transcendentals`) whose prose that pass cannot reach. So the
# gate runs the workspace pass and then one `--no-deps` pass per
# manifest the workspace pass did not cover. The two sets are
# complementary BY CONSTRUCTION: `cargo metadata` answers which
# manifests `--workspace` just documented, `git ls-files` answers which
# exist, and every manifest lands in exactly one pass. The seven is not
# load-bearing — the list is derived, and the success line reports the
# count the run actually saw — but a number written here that disagrees
# with the run is the drift this gate is about, so it is spelled out
# rather than summarised.
#
# A THIRD PASS re-documents a DERIVED SUBSET of those same roots under
# the one feature selection the two above cannot express; see the
# `not(feature)` section below. It adds no manifest to the coverage set.
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
# DERIVED FROM THE REPOSITORY, NOT FROM THE FILESYSTEM. The other half
# of the derivation — which manifests EXIST — is `git ls-files`, the
# same instrument scripts/gates/ uses to decide what is in scope. A
# `find` over the checkout answers a different question, and answers it
# wrong: a working checkout grows directories that are not this repo's
# source, and this one grows a whole checkout per agent lane under
# `.claude/worktrees/`. Measured while writing this: 23 tracked
# manifests, 115 on disk. Every one of those 92 is outside the outer
# `cargo metadata`, so pass 2 would `cargo doc --manifest-path
# .claude/worktrees/agent-X/Cargo.toml` under DEFAULT features — which
# is the exact misread the `Cargo.toml` skip in the loop below exists
# to prevent, wearing a path prefix, ninety-two times over. Pruning the
# offending directory names is the weaker answer: the prune list has to
# grow every time the checkout learns to grow a new directory, and the
# gate is wrong-and-quiet in the interval. `git ls-files` cannot be
# defeated by a directory someone adds later, because an untracked tree
# is never source. Hosted CI never saw this — clean checkout, and the
# job `rm -rf`s `.claude` — but the local half runs where the worktrees
# live, which is the half that matters for a gate a developer is meant
# to run before pushing.
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
# FEATURES: --all-features EVERYWHERE, WITH ONE NAMED EXCEPTION.
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
# THE EXCEPTION IS ONE ROOT, AND IT IS NAMED. `interval-transcendentals`
# is documented under DEFAULT features, a ruling this repo has already
# made once: ci.yml's `interval backend crate` job is "deliberately the
# crate's DEFAULT feature set: without `oracle-inari` there is no
# inari/gmp-mpfr-sys and no C toolchain in the graph". That crate's ONLY
# feature is that test-only oracle (its manifest: `src/` must never
# reference `inari` or this feature), so --all-features there would
# document not one extra line of `src/` while dragging the LGPL C build
# back into a doc job — the exact thing the exclusion exists to prevent.
#
# EVERY OTHER ROOT GETS --all-features, and the exception list is the
# way round it is for a reason. The LGPL argument above justifies
# exactly one root; applying it to all six was one root's argument
# stretched over five others, and it cost coverage. Of the six, only two
# declare features at all, and the second is `demos/tour`, whose `probe`
# and `budget` gate the tour's OWN `src/probe.rs` and its dispatch — not
# only the kernel features they forward. Under default features that
# file's prose was documented by NOTHING. So the default here is the
# same flag pass 1 uses and docs.rs uses, and the exception carries its
# reason at the constant. A further root added tomorrow gets the
# covering treatment without anyone remembering to ask; the one root
# that must not is spelled out where a reader trips over it.
#
# WHAT --all-features CANNOT SEE: THE `not(feature)` HALF, AND PASS 3.
#
# Features are additive, so `--all-features` compiles the `feature = F`
# half of every paired module and NEVER the `not(feature = F)` half.
# Those halves are ordinary source carrying ordinary prose, and until
# pass 3 existed the only instrument that would report their doc errors
# was the one instrument that never compiled them: three live rustdoc
# errors sat in `crates/mesh/src/budget.rs`'s `mod inert` with every
# gate green. Widening the FLAG cannot reach them — there is no feature
# selection that compiles both halves of a `cfg`/`not(cfg)` pair — so
# the coverage has to come from a second SELECTION, which is a second
# pass.
#
# PASS 3 IS `--no-default-features`, OVER THE ROOTS THAT NEED IT, AND
# THE ROOT LIST IS DERIVED. `inert_half_roots` greps the tracked Rust
# sources for the `cfg(not(feature` shape and reports the nearest
# tracked manifest above each hit — the same `git ls-files` oracle the
# rest of this gate derives from, for the same reason: a hand-written
# list of "crates with a paired module" is a roster, and this file
# already argues at length about what rosters do. A root with no such
# module is not documented a second time, which is what keeps the cost
# proportional to the shape rather than to the workspace.
#
# WHY `--no-default-features` AND NOT "ALL FEATURES BUT F". The precise
# complement — one pass per (root, F) pair, at every feature except F —
# is what a reader reaches for first, and it is both dearer (six passes
# here rather than five roots' worth) and no more honest, because the
# lint it would save is allowed below anyway. `--no-default-features` is
# also the maximal selection: it compiles the `not(F)` half of EVERY
# paired module in the root at once, including one whose F a crate later
# moves into its own `default` list, where "default features" would
# quietly stop compiling that half again.
#
# AND `rustdoc::broken_intra_doc_links` IS ALLOWED IN PASS 3 — ONLY
# THERE, AND THIS IS THE COST OF THE WIDENING RATHER THAN A TIDINESS
# FLAG. With F off, every link into F-gated code is unresolvable BY
# CONSTRUCTION: measured on this tree the moment the pass was written,
# 14 CORRECT links went red — 12 in `geom-core` (`Interval`,
# `DualInterval`, `Probe`, `start_recording`) and 2 in `topo`'s boolean
# prose (`SweepStrategy::Idealized`) — which is the SAME false-positive
# population the FEATURES paragraph above adopted --all-features to
# kill, arriving from the other side. Reds on prose that is correct
# under the gate's own primary selection are how a gate gets routed
# around, so the lint that cannot tell "this link is broken" from "this
# link's target is in the other half" is off in this pass.
#
# THE BLIND SPOT THAT LEAVES, NAMED: a genuinely broken intra-doc link
# written INSIDE a `not(feature)` half is still reported by nothing. The
# lint is on in pass 1 and pass 2, so it covers the whole tree under
# --all-features and only this one shape escapes it. What pass 3 does
# catch there is every OTHER rustdoc lint — the live instance was three
# `rustdoc::redundant_explicit_links`, and malformed markdown, bare
# URLs, invalid HTML and unportable syntax are the rest — plus the thing
# no lint states: the half now COMPILES in a doc build, so prose on an
# item that stopped existing is a hard error rather than a silence.
#
# THE SIBLING AXIS PASS 3 DOES NOT COVER, swept and named rather than
# left to be rediscovered: `#[cfg(not(debug_assertions))]`. `cargo doc`
# runs the dev profile, so `debug_assertions` is ON in every pass here
# and that half is compiled out of all three — the same shape as the
# feature axis, one profile over, and no feature selection reaches it.
# Sixteen sites carry it (`geom-core/src/spline/knots.rs`,
# `topo/src/review_d18.rs`, `topo/src/review_m1_pr2/
# release_corruption.rs`), every one a statement or expression block
# rather than a documented item, so the axis is LATENT here rather than
# live: there is no prose behind it today to be unread. Covering it
# means a `--release` doc pass, which is a cost decision and not a
# spelling one, and it is not taken on this gate's own authority.
#
# THE NAMED DEFAULT-FEATURES ROOT NEEDS NO EXCEPTION HERE.
# `--no-default-features` cannot turn `oracle-inari` on, so pass 3 is
# already inside the constraint that exception exists to enforce; if
# that root ever grows a paired module it is documented here exactly as
# the exception wants it, with no second rule to keep in step.
#
# TARGETS, AND THE HALF-COVERED PACKAGE. `cargo doc`'s default target
# selection documents a package's library and SKIPS a binary that shares
# its name — silently, with no warning and a zero exit. `tools/k-lint`
# is exactly that arrangement, and a broken intra-doc link planted in
# its `src/main.rs` passed this gate green. So every pass here runs a
# second `--bins --examples` invocation; see doc_pass below for why that
# spelling and not `--lib --bins`.
#
# `--examples` IS THE SAME ONE-FLAG FIX AS `--bins`, and it is here for
# the same reason. The claim that `examples/` is uncovered BY
# CONSTRUCTION was wrong: "rustdoc builds no test targets at all" holds
# for `tests/` and `benches/`, which have no `cargo doc` target filter,
# but `cargo doc --examples` exists. Five Rust example targets carry
# ~50 doc-comment lines no rustdoc lint had ever read
# (`crates/step-export/` ×2, `crates/step-import/`, `crates/stl/`,
# `interval-transcendentals/`); four of the five are in workspace
# members and are now covered by pass 1.
#
# WHAT IS STILL UNCOVERED, AND WHY, stated rather than left to be
# rediscovered:
#
#   * `tests/` and `benches/`. No `cargo doc` filter selects them —
#     rustdoc genuinely builds no test target — so an intra-doc link
#     written under `tests/` is inert on every tier. That is a decision
#     this gate cannot take on its own: banked as D113, census at S135.
#   * `interval-transcendentals/examples/bench.rs` (18 doc lines).
#     Its `[[example]]` declares `required-features = ["oracle-inari"]`,
#     and that root is the one documented under DEFAULT features, so
#     `--examples` selects nothing there and cargo says so only as a
#     no-op warning. This is the SAME SILENT-SKIP SHAPE as the k-lint
#     binary above — a target filter that matches nothing exits zero —
#     and it is accepted here because the alternative is the LGPL C
#     toolchain in a doc job, which is the whole reason that root is
#     excepted. Covering it means an `oracle-inari` doc pass with the
#     inari graph, which is a cost decision, not a spelling one.
#   * `build.rs`, and any future `[[bin]]` or `[[example]]` behind
#     `required-features` that the root's feature set does not turn on.
#     Cargo has no target filter for a build script at all; the
#     `required-features` case is the bullet above, generalised. A new
#     one arrives silently — nothing here will say so — which is the
#     price of the exception paragraph above and is named as such.
#
# THE OUTSIDE ROOTS ARE ALSO THE CACHE'S SCOPE, WHICH IS WHY
# `--print-roots` EXISTS. Hosted CI's `fmt` job restores one
# `Swatinem/rust-cache` entry, and until #921 that entry covered `./target`
# and nothing else — so the workspace pass ran against a warm dependency
# graph while the six roots above compiled from nothing on EVERY run,
# each into its own target directory that no cache carried. Measured on
# the runner (2 vCPU, run 32583370980, warm cache with one kernel doc
# comment changed — the case a real PR is): this script's real pass went
# from 67 s to 33 s once the cache was told about all seven roots, and
# the whole job from 136 s to 99 s, which is 3 billed minutes to 2. The
# cache entry grows 155 MB to 245 MB compressed, paid only when the key
# rotates. Written up as F6 in docs/CI-MINUTES-2026-08.md.
#
# THE LIST IS PRINTED, NOT COPIED. A `workspaces:` roster hand-written
# into ci.yml would be the second hand-kept root list in this repo, and
# the paragraph above is about why the first one must not exist: a root
# added to `workspace.exclude` and forgotten there would silently fall
# out of the cache, and nothing would say so — the same shape as a root
# falling out of COVERAGE, one currency down. So the workflow asks THIS
# script, which already derives the set, and gets `.` plus one directory
# per root. `cargo metadata --no-deps` needs no registry index (checked:
# it answers under an empty CARGO_HOME), so the step can run BEFORE
# rust-cache restores `~/.cargo` — which it must, since its output is
# that action's input.
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
# AND THE SITING COSTS NOTHING, because the watching does not have to
# move with the file. `gate-roster.sh` proves that both halves call each
# gate's `--selftest` AND the gate; its roster is the DIRECTORY, so it
# used to say nothing about this script and deleting `--selftest` from
# either half's rustdoc row red nothing. That is not a trade anyone had
# to accept: `check-ci-mirror-parity.py`'s TIER_BLIND already implements
# "this NAMED PATH must be invoked by the hosted half and by the local
# half", and gate-roster.sh now carries the same shape for gates sited
# outside its directory — see OUTLIER_GATES there. This script is its
# one entry, checked exactly as a member of the directory is.
set -euo pipefail
# shellcheck source=scripts/gates/lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/gates/lib.sh"

GATE_SCAN_NOUN="cargo root"
RUSTDOC_LINTS="-D warnings -A rustdoc::private_intra_doc_links"
# Pass 3's lints: the same set, less the one lint that pass cannot
# judge. See the `not(feature)` section in the header for the measured
# false-positive population this drops and the blind spot it leaves.
RUSTDOC_LINTS_INERT="$RUSTDOC_LINTS -A rustdoc::broken_intra_doc_links"

# Physical path of a file, without depending on `realpath`: cargo
# reports canonical manifest paths and a `--root` under /tmp can be a
# symlink, so both sides of the membership comparison are normalised the
# same way.
abs_path() {
  (cd "$(dirname "$1")" && printf '%s/%s\n' "$(pwd -P)" "$(basename "$1")")
}

# Every manifest the REPOSITORY tracks under $PWD, newline-delimited,
# repo-relative. `git ls-files` and not `find`: see the derivation
# paragraph in the header for why a checkout is the wrong oracle for
# what this repo's source is. Run from $PWD, so it answers about the
# tree the gate was pointed at and nothing above it.
#
# A FAILED READER IS NOT AN EMPTY TREE. Without this branch a `git` that
# cannot answer — no repository, no git — would hand back an empty list,
# pass 2 would run zero times, and the gate would report a green
# "0 root(s) outside it" that is indistinguishable from a repo with no
# excluded roots. That is the shape S157 is about, so it is diagnosed
# and fatal.
tree_manifests() {
  local out
  if ! out=$(git ls-files -- '*Cargo.toml' 2>&1); then
    printf '%s\n' "$out" >&2
    gate_error "$(gate_name): git ls-files failed in $PWD, so the gate cannot tell which manifests this repository tracks — an empty list would silently skip every cargo root outside the workspace and still report green"
    return 1
  fi
  if [ -z "$out" ]; then
    gate_error "$(gate_name): git tracks no Cargo.toml under $PWD, yet the gate is standing on one — the manifests are untracked, or this is not the tree the repository thinks it is, and either way pass 2 would scan nothing and call it green"
    return 1
  fi
  printf '%s' "$out" | sort
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
  # MATERIALISED, THEN CHECKED. Through `gate_grep`, a matcher that
  # cannot search ends this reader loudly instead of as a bare failed
  # assignment; and because `gate_grep` folds "no match" to an empty
  # result, the emptiness is diagnosed here — a metadata reply naming no
  # manifest at all would otherwise hand the caller an empty member list
  # and pass 2 would cover nothing while reading as covered.
  local manifests
  manifests=$(printf '%s' "$meta" | gate_grep -o '"manifest_path":"[^"]*"' | cut -d'"' -f4) || return 1
  if [ -z "$manifests" ]; then
    gate_error "$(gate_name): cargo metadata succeeded but named no manifest_path — the member list is unreadable, so the outside-roots derivation cannot run"
    return 1
  fi
  printf '%s\n' "$manifests"
}

# One pass, its own diagnosis, and TWO cargo invocations — because
# `cargo doc`'s default target selection SILENTLY SKIPS a binary whose
# name matches its package's library, and selects no example at all.
# `tools/k-lint` is the live case for the first: lib.rs and main.rs, one
# crate name, and a planted broken link in main.rs left this gate green
# until `--bins` was added. The second invocation is a no-op warning
# ("target filters `bins`, `examples` specified, but no targets matched")
# for a package with neither, which is what makes it safe to run
# unconditionally; `--lib --bins` is NOT the shorter spelling, because
# `--lib` is a hard error on a bin-only package like `demos/tour`, while
# `--bins --examples` is not an error on any of them.
#
# THE LINT SET IS A PARAMETER, not a global read from in here, because
# pass 3 runs a different one and a pass that silently inherited the
# wrong set would be a gate reporting on lints it did not run.
doc_pass_with() {
  local lints=$1 why=$2; shift 2
  local rc=0
  RUSTDOCFLAGS="$lints" cargo doc --no-deps --document-private-items "$@" || rc=1
  RUSTDOCFLAGS="$lints" cargo doc --no-deps --document-private-items --bins --examples "$@" || rc=1
  [ "$rc" -eq 0 ] || gate_error "$(gate_name): rustdoc rejected $why"
  return "$rc"
}

doc_pass() {
  local why=$1; shift
  doc_pass_with "$RUSTDOC_LINTS" "$why" "$@"
}

# THE ONE ROOT DOCUMENTED UNDER DEFAULT FEATURES, named here with its
# reason rather than derived, because there is nothing to derive it
# from: no manifest field says "this crate's optional feature drags an
# LGPL C toolchain into the graph". Matched on the repo-relative
# manifest path `git ls-files` reports. Renaming or moving the crate
# does not make the gate go quiet — it makes the root fall into the
# --all-features default, where the inari build is loud rather than
# silent. See the FEATURES paragraph in the header.
DEFAULT_FEATURES_ROOT=interval-transcendentals/Cargo.toml

# THE ROOTS OUTSIDE THE WORKSPACE, one repo-relative manifest path per
# line — the derivation the header argues for, in ONE place because it
# now has TWO readers. `gate` documents these; `--print-roots` hands the
# same set to the hosted job's cache (see the OUTSIDE ROOTS ARE THE
# CACHE'S SCOPE paragraph in the header).
outside_roots() {
  local m members manifests
  # `|| return 1` ON BOTH READERS, EXPLICITLY, AND NOT LEFT TO `set -e`.
  # Errexit does NOT survive this nesting: a function running inside a
  # command substitution whose value is assigned goes on past a failed
  # assignment of its own, so `members=$(workspace_manifests)` alone
  # would print the reader's diagnosis, carry on to the loop, and hand
  # the caller an EMPTY list — pass 2 running zero times and the gate
  # reporting green over a tree it never read, which is the exact shape
  # the two readers were written to prevent. The self-test below caught
  # it (`gate_selftest_without_tool git`) the first time this derivation
  # moved into a function; the fix is to say so rather than to depend on
  # a shell option that holds only at one nesting depth.
  members=$(workspace_manifests) || return 1
  # BOTH LISTS CAPTURED, NOT PIPED OR PROCESS-SUBSTITUTED. Each reader
  # can fail, and a failed reader must kill this derivation rather than
  # hand back an empty list: inside `< <(…)` a non-zero return is
  # invisible, so the loop would run zero times and the gate would report
  # green over a tree it never read. The reader writes its own
  # `gate_error` to STDERR before returning, which is why lib.sh puts
  # diagnoses there and not on stdout — this list IS stdout.
  manifests=$(tree_manifests) || return 1
  while IFS= read -r m; do
    # THE WORKSPACE'S OWN MANIFEST IS NOT A MEMBER OF IT. This one is
    # virtual (no root package), so `cargo metadata`'s package list
    # cannot contain it and the membership test below reads it as an
    # uncovered root — whereupon the caller documents every member a
    # SECOND time, one package at a time, for no coverage at all. It is
    # the manifest pass 1 was invoked on; it is covered.
    [ "$m" = Cargo.toml ] && continue
    # NEWLINE-DELIMITED ON BOTH SIDES, and the delimiters are added HERE
    # rather than stored on `$members`: a command substitution strips
    # trailing newlines, so a list built with one would leave its LAST
    # entry unterminated and that member alone would be documented a
    # second time. A bare substring match is the other wrong answer — it
    # reads one manifest path as present because it is a tail of
    # another's.
    case "
$members
" in
      *"
$(abs_path "$m")
"*) continue ;;
    esac
    printf '%s\n' "$m"
  done <<<"$manifests"
}

# THE ROOTS WITH A `not(feature)` HALF, one repo-relative manifest path
# per line — pass 3's subject, derived from the tree exactly as the
# outside-roots list is. See the `not(feature)` section in the header.
#
# EVERY TRACKED `.rs`, WITH NO PATH FILTER. Excluding `tests/` and
# `benches/` — rustdoc builds no test target, so a paired module in one
# is prose this pass could not read anyway — was written and then
# removed: on this tree it changed the derived set by nothing, and it
# bought a blind spot, because a package whose `src/` holds an inline
# `tests/` directory would have been filtered out of a pass its own
# library needs. The filter's only prize was a compilation it might
# skip; the price was coverage decided by a path spelling.
inert_half_roots() {
  local manifests rs f d cand
  manifests=$(tree_manifests) || return 1
  if ! rs=$(git ls-files -- '*.rs' 2>&1); then
    printf '%s\n' "$rs" >&2
    gate_error "$(gate_name): git ls-files failed in $PWD, so the gate cannot tell which sources carry a not(feature) half — an empty answer would skip pass 3 over every one of them and still report green"
    return 1
  fi
  # NO SOURCES IS NOT A FINDING. A cargo root can be all manifest and
  # generated code, and pass 3 having nothing to do is a legitimate
  # green — unlike `tree_manifests`, where an empty answer means the
  # reader failed. So this returns empty rather than diagnosing.
  [ -n "$rs" ] || return 0
  local -a files=()
  while IFS= read -r f; do
    [ -n "$f" ] || continue
    files+=("$f")
  done <<<"$rs"
  local hits
  # `gate_grep`, so a matcher that cannot search ends the gate instead
  # of handing back the empty list that means "no paired module here".
  hits=$(gate_grep -l 'cfg(not(feature' -- ${files[@]+"${files[@]}"}) || return 1
  [ -n "$hits" ] || return 0
  # ACCUMULATED IN A VARIABLE, NOT PRINTED INTO `| sort -u`. A pipeline
  # stage is a subshell, so the `return 1` below could not fail this
  # function from inside one — the caller would get a short list and a
  # zero status, which is the shape every other reader in this file is
  # written to avoid.
  local owners=
  while IFS= read -r f; do
    [ -n "$f" ] || continue
    # The nearest TRACKED manifest above the hit owns it. Walking up
    # rather than splitting the path at `crates/<x>/`: a root can sit at
    # any depth (`demos/tour`, `interval-transcendentals`), and a nested
    # package under another root must resolve to itself.
    d=$(dirname "$f")
    while :; do
      if [ "$d" = "." ]; then cand=Cargo.toml; else cand="$d/Cargo.toml"; fi
      case "
$manifests
" in
        *"
$cand
"*) owners="$owners$cand
"; break ;;
      esac
      if [ "$d" = "." ]; then
        gate_error "$(gate_name): $f carries a not(feature) half and no tracked Cargo.toml stands above it, so pass 3 has no root to document it under — the source is outside every manifest this repository tracks, and skipping it quietly is the blind spot this pass exists to close"
        return 1
      fi
      d=$(dirname "$d")
    done
  done <<<"$hits"
  printf '%s' "$owners" | sort -u
}

# --print-roots. The DIRECTORY of every cargo root this gate documents:
# `.` for the workspace pass, then one line per root outside it. It is
# the same derivation `gate` runs, deliberately — see the header.
#
# PASS 3 ADDS NOTHING HERE, and that is a property rather than an
# oversight: its roots are a SUBSET of these, and it builds into the
# same target directory each of them already owns. Its artifacts are
# cached by this list because they are already inside it.
print_roots() {
  local m outside
  # CAPTURED INTO A VARIABLE, AND THE FAILURE CHECKED HERE TOO. Written
  # as `done <<<"$(outside_roots)"` a reader that could not answer would
  # print its diagnosis, hand back nothing, and this would print a bare
  # `.` — which the hosted job reads as "one cargo root", caches one, and
  # is quietly back to where it started with every gate green.
  outside=$(outside_roots) || exit 1
  printf '.\n'
  while IFS= read -r m; do
    [ -n "$m" ] || continue
    printf '%s\n' "$(dirname "$m")"
  done <<<"$outside"
}

gate() {
  local rc=0 m outside n=0
  local -a roots=()
  if [ ! -f Cargo.toml ]; then
    gate_error "$(gate_name): no Cargo.toml under $PWD — the gate documented nothing, which is not a pass"
    exit 1
  fi

  if [ "$PRINT_ROOTS" = true ]; then
    print_roots
    return 0
  fi

  # PASS 1 — the workspace.
  #
  # `--skip-viewer-toolkit` is the ONE thing that changes here, and it
  # changes the FEATURES of one member rather than dropping it: `viewer`
  # comes out of the --all-features pass and goes back in under DEFAULT
  # features, so its renderer-free modules keep their rustdoc gate and
  # only the `app`-gated ones (which drag ~140 eframe/wgpu crates in)
  # are skipped. Evan's viewer-CI-posture ruling, docs/GUI-LOG.md
  # 2026-08-27; the caller decides, this script only obeys, and the
  # hosted caller passes the flag off the change filter's seed-keyed
  # RUN_VIEWER_TOOLKIT.
  if [ "$SKIP_VIEWER_TOOLKIT" = true ]; then
    doc_pass "the workspace pass (viewer at default features) — a doc comment above has stopped rendering (a link to a renamed, deleted, or test-only item is the usual cause), and clippy is blind to every one of these lints" \
      --workspace --all-features --exclude viewer || rc=1
    doc_pass "the viewer pass at DEFAULT features — its renderer-free modules are gated on every run; only the app-feature modules are skipped" \
      -p viewer || rc=1
  else
    doc_pass "the workspace pass — a doc comment above has stopped rendering (a link to a renamed, deleted, or test-only item is the usual cause), and clippy is blind to every one of these lints" \
      --workspace --all-features || rc=1
  fi
  n=1

  # PASS 2 — every manifest the workspace pass did not cover, one
  # `--no-deps` pass each, so a package in a nested workspace is
  # documented exactly once however those workspaces are arranged.
  #
  # CAPTURED INTO A VARIABLE AND CHECKED, NOT SUBSTITUTED INTO THE
  # REDIRECTION. `outside_roots` reads two tools that can fail, each of
  # which diagnoses and returns non-zero; inside `<<<"$(…)"` that status
  # is invisible, so pass 2 would run zero times over a tree nothing read
  # and the gate would report green for the wrong reason.
  outside=$(outside_roots) || exit 1
  while IFS= read -r m; do
    [ -n "$m" ] || continue
    roots+=("$m")
  done <<<"$outside"

  local -a feat
  local defaulted=0
  for m in ${roots[@]+"${roots[@]}"}; do
    n=$((n + 1))
    feat=(--all-features)
    if [ "$m" = "$DEFAULT_FEATURES_ROOT" ]; then
      feat=()
      defaulted=$((defaulted + 1))
    fi
    doc_pass "$m — this cargo root is outside the kernel workspace, so the workspace pass never reads its prose and this pass is the only thing that does" \
      --manifest-path "$m" ${feat[@]+"${feat[@]}"} || rc=1
  done

  # PASS 3 — the `not(feature)` halves the two passes above compile out.
  #
  # ITS ROOTS ARE A SUBSET OF THE TWO PASSES', so `n` is NOT advanced
  # here: the count this gate reports is manifests covered, and a root
  # documented twice under two selections is still one manifest. A pass
  # 3 that inflated it would be reporting coverage it did not add.
  #
  # CAPTURED AND CHECKED, for the reason pass 2's list is.
  local inert
  inert=$(inert_half_roots) || exit 1
  local -a inert_roots=()
  while IFS= read -r m; do
    [ -n "$m" ] || continue
    inert_roots+=("$m")
  done <<<"$inert"
  for m in ${inert_roots[@]+"${inert_roots[@]}"}; do
    doc_pass_with "$RUSTDOC_LINTS_INERT" \
      "$m at --no-default-features — this root has a #[cfg(not(feature = …))] half, which every pass above compiles OUT, so its prose is read here or by nothing" \
      --manifest-path "$m" --no-default-features || rc=1
  done

  GATE_SCAN_FILES=$n
  [ "$rc" -eq 0 ] || exit 1
  gate_ok "every cargo manifest this repository tracks renders — library, binary and example targets alike: the workspace pass plus ${#roots[@]} root(s) outside it, all under --all-features except $defaulted named exception(s), and ${#inert_roots[@]} of them re-read at --no-default-features for the not(feature) halves --all-features compiles out"
}

# THE FIXTURE. Dependency-free by design — the cases are about this
# script's control flow and its flags, not about the kernel, and a
# fixture that built the kernel would cost minutes to prove seconds of
# logic. THREE packages, one per distinct treatment the gate has: a
# workspace MEMBER (pass 1), a root the workspace EXCLUDES (pass 2 under
# --all-features), and the root named by DEFAULT_FEATURES_ROOT, planted
# at that exact path because the gate matches on it (pass 2 under
# default features). `edition = "2021"` rather than the repo's 2024: the
# fixture is built by whatever toolchain is default outside this tree,
# since rust-toolchain.toml does not reach a /tmp directory.
#
# Every package carries a lib, a SAME-NAMED bin and an EXAMPLE — the two
# arrangements `cargo doc`'s default target selection skips silently.
# The fixture has to contain both or the `--bins --examples` invocation
# above is unguarded.
#
# AND IT IS A GIT REPOSITORY, because the gate derives its root list
# from `git ls-files`. `git add` and no commit: `ls-files` reads the
# INDEX, so an added path is tracked and that is the whole requirement.
# The planters below append to files planted here, so they need no
# second `add`; a planter that writes a NEW path is writing something
# UNTRACKED, which is a case of its own.
gate_plant_clean() {
  local r=$1
  mkdir -p "$r/crates/clean/src" "$r/crates/clean/examples" \
    "$r/outside/src" "$r/outside/examples" \
    "$r/interval-transcendentals/src"
  {
    printf '[workspace]\nresolver = "2"\n'
    printf 'members = ["crates/clean"]\n'
    printf 'exclude = ["outside", "interval-transcendentals"]\n'
  } > "$r/Cargo.toml"
  {
    printf '[package]\nname = "clean"\nversion = "0.0.0"\nedition = "2021"\n\n'
    # A feature on the MEMBER too, so the `not(feature)` planters below
    # have a declared one to negate on both sides of the workspace
    # boundary. Undeclared, `cfg(not(feature = "probe"))` is a
    # `check-cfg` warning and the case would red for the wrong reason.
    printf '[features]\nprobe = []\n'
  } > "$r/crates/clean/Cargo.toml"
  {
    printf '//! A workspace member whose prose links to [`identity`].\n'
    printf 'pub fn identity(x: f64) -> f64 { x }\n'
  } > "$r/crates/clean/src/lib.rs"
  printf '//! The member bin, linking to [`main`].\nfn main() {}\n' \
    > "$r/crates/clean/src/main.rs"
  printf '//! The member example, linking to [`main`].\nfn main() {}\n' \
    > "$r/crates/clean/examples/demo.rs"
  {
    printf '[workspace]\n\n'
    printf '[package]\nname = "outside"\nversion = "0.0.0"\nedition = "2021"\n\n'
    # A feature gating this root's OWN code, which is `demos/tour`'s
    # arrangement: under default features the item behind it is never
    # compiled and its prose is read by nothing.
    printf '[features]\nprobe = []\n'
  } > "$r/outside/Cargo.toml"
  {
    printf '//! A root outside the workspace, linking to [`identity`].\n'
    printf 'pub fn identity(x: f64) -> f64 { x }\n'
  } > "$r/outside/src/lib.rs"
  printf '//! The outside bin, linking to [`main`].\nfn main() {}\n' \
    > "$r/outside/src/main.rs"
  printf '//! The outside example, linking to [`main`].\nfn main() {}\n' \
    > "$r/outside/examples/demo.rs"
  {
    printf '[workspace]\n\n'
    printf '[package]\nname = "interval-transcendentals"\nversion = "0.0.0"\nedition = "2021"\n\n'
    printf '[features]\noracle-inari = []\n'
  } > "$r/interval-transcendentals/Cargo.toml"
  {
    printf '//! The feature-excepted root, linking to [`identity`].\n'
    printf 'pub fn identity(x: f64) -> f64 { x }\n'
  } > "$r/interval-transcendentals/src/lib.rs"
  git -C "$r" init -q >/dev/null 2>&1
  git -C "$r" add -A >/dev/null 2>&1
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

# THE EXAMPLE, in each pass, and the reason `--examples` is not a
# tidiness flag: default target selection selects no example AT ALL, so
# before it was added the ~50 doc-comment lines under the repo's five
# `examples/` trees were read by nothing, silently and with a zero exit.
plant_broken_link_in_member_example() {
  printf '\n/// Links to [`no_such_item`].\npub fn documented() {}\n' \
    >> "$1/crates/clean/examples/demo.rs"
}

plant_broken_link_in_excluded_root_example() {
  printf '\n/// Links to [`no_such_item`].\npub fn documented() {}\n' \
    >> "$1/outside/examples/demo.rs"
}

# --all-features ON A ROOT OUTSIDE THE WORKSPACE. `demos/tour`'s live
# arrangement: a feature gating the root's OWN module. Under default
# features this item is not compiled and its prose is documented by
# nothing, which is what the gate used to do to every excluded root.
plant_broken_link_behind_a_feature_in_excluded_root() {
  {
    printf '\n#[cfg(feature = "probe")]\n'
    printf '/// Links to [`no_such_item`].\npub fn documented() {}\n'
  } >> "$1/outside/src/lib.rs"
}

# THE NAMED EXCEPTION, and the case that keeps it honest in the other
# direction: the same break behind the excepted root's own feature must
# NOT fire, because that root is documented under DEFAULT features on
# purpose. Delete DEFAULT_FEATURES_ROOT's branch and this reds — which
# is the LGPL C toolchain arriving in a doc job, caught here rather than
# on a runner.
plant_broken_link_behind_the_excepted_feature() {
  {
    printf '\n#[cfg(feature = "oracle-inari")]\n'
    printf '/// Links to [`no_such_item`].\npub fn documented() {}\n'
  } >> "$1/interval-transcendentals/src/lib.rs"
}

# THE `not(feature)` HALF — PASS 3's WHOLE SUBJECT, in each pass. This
# is `crates/mesh/src/budget.rs`'s live arrangement: a paired module
# whose disarmed half is the one every shipped build compiles, and which
# `--all-features` therefore never reads. The planted error is a
# REDUNDANT EXPLICIT LINK rather than a broken one on purpose — a broken
# link is the lint pass 3 allows, so a case built on one would pass with
# pass 3 deleted and prove nothing.
#
# THE `pub use` IS PART OF THE SHAPE, not decoration: it is how the
# paired module reaches its callers under either feature, and it is what
# puts the module's items in scope for a bare label — so it is also what
# makes the explicit target redundant. Drop it and the planted link is
# merely unresolved, i.e. the allowed lint, and the case would stop
# testing anything.
plant_doc_error_behind_not_a_feature() {
  {
    printf '\n/// The disarmed half.\n'
    printf '///\n'
    printf '/// [`stub`][off::stub] folds away.\n'
    printf '#[cfg(not(feature = "probe"))]\n'
    printf 'mod off {\n'
    printf '    /// A stub.\n'
    printf '    pub fn stub() {}\n'
    printf '}\n'
    printf '\n#[cfg(not(feature = "probe"))]\npub use off::*;\n'
  } >> "$1"
}

plant_doc_error_behind_not_a_feature_in_member() {
  plant_doc_error_behind_not_a_feature "$1/crates/clean/src/lib.rs"
}

plant_doc_error_behind_not_a_feature_in_excluded_root() {
  plant_doc_error_behind_not_a_feature "$1/outside/src/lib.rs"
}

# THE COST OF PASS 3, PINNED IN THE DIRECTION THAT MATTERS: prose in the
# `not(feature)` half linking to an item the FEATURE half defines. That
# link is correct — it resolves in the docs --all-features builds — and
# it cannot resolve in a pass whose whole point is that the feature is
# off. Turn `rustdoc::broken_intra_doc_links` back on in pass 3 and this
# reds, which is the 14-correct-links measurement in the header
# arriving as a test rather than as a paragraph.
plant_link_from_the_disarmed_half_into_the_gated_one() {
  {
    printf '\n#[cfg(feature = "probe")]\n'
    printf '/// The armed half.\npub fn armed() {}\n'
    printf '\n#[cfg(not(feature = "probe"))]\n'
    printf '/// The disarmed half; [`armed`] is what this replaces.\n'
    printf 'pub fn disarmed() {}\n'
  } >> "$1/crates/clean/src/lib.rs"
}

# THE DERIVATION ITSELF. A whole second checkout under `.claude/`, the
# way this repo's agent worktrees arrive — 92 of them on disk against 23
# tracked, when this case was written — carrying a break in a root the
# outer `cargo metadata` has never heard of. UNTRACKED, so `git
# ls-files` does not report it and the gate must PASS. Swap the
# derivation back to `find` and this case reds, which is the point: it
# is the only thing standing between a developer's checkout and a pass 2
# that documents every lane's worktree under the wrong feature set.
plant_untracked_worktree_with_broken_link() {
  mkdir -p "$1/.claude/worktrees/agent-x/src"
  {
    printf '[workspace]\n\n'
    printf '[package]\nname = "lane"\nversion = "0.0.0"\nedition = "2021"\n'
  } > "$1/.claude/worktrees/agent-x/Cargo.toml"
  {
    printf '//! A lane worktree.\n'
    printf '/// Links to [`no_such_item`].\npub fn documented() {}\n'
  } > "$1/.claude/worktrees/agent-x/src/lib.rs"
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

# THE PRINTER, against the fixture's three roots. `.` for the workspace
# pass, then the two roots outside it in manifest order — and the
# workspace MEMBER must NOT appear: it is covered by pass 1, and
# `crates/clean/target` is a directory cargo never writes, so caching it
# would be caching nothing while reading as a covered root.
gate_selftest_prints_roots() {
  local tmp out want
  tmp=$(mktemp -d)
  gate_plant_clean "$tmp"
  if ! out=$("$0" --root "$tmp" --print-roots 2>&1); then
    rm -rf "$tmp"
    printf 'SELFTEST FAILED: --print-roots exited non-zero on a clean fixture\n%s\n' "$out" >&2
    exit 1
  fi
  rm -rf "$tmp"
  # SORTED, because `tree_manifests` sorts: the printed order is the
  # manifest order, not the order this fixture plants them in.
  want='.
interval-transcendentals
outside'
  if [ "$out" != "$want" ]; then
    printf 'SELFTEST FAILED: --print-roots did not print the derived root set — the hosted cache scopes itself to this list, so a short one is six cargo roots compiling from nothing with every gate green.\nwanted:\n%s\ngot:\n%s\n' \
      "$want" "$out" >&2
    exit 1
  fi
}

gate_selftest() {
  local want="rustdoc rejected"
  gate_selftest_clean
  gate_selftest_case "$want" plant_broken_link_in_member
  gate_selftest_case "$want" plant_broken_link_in_excluded_root
  gate_selftest_case "$want" plant_broken_link_in_member_bin
  gate_selftest_case "$want" plant_broken_link_in_excluded_root_bin
  gate_selftest_case "$want" plant_broken_link_in_member_example
  gate_selftest_case "$want" plant_broken_link_in_excluded_root_example
  gate_selftest_case "$want" plant_broken_link_behind_a_feature_in_excluded_root
  gate_selftest_case "$want" plant_broken_link_on_private_item
  # PASS 3, IN EACH PASS'S ROOTS. Delete the pass, or its derivation,
  # and both go green — which is the state this gate shipped in while
  # three rustdoc errors sat in a `not(feature)` module.
  gate_selftest_case "$want" plant_doc_error_behind_not_a_feature_in_member
  gate_selftest_case "$want" plant_doc_error_behind_not_a_feature_in_excluded_root
  gate_selftest_passes "public prose linking to a private sibling" \
    plant_public_link_to_private_item
  gate_selftest_passes "a link from the disarmed half into the feature-gated one" \
    plant_link_from_the_disarmed_half_into_the_gated_one
  gate_selftest_passes "prose behind the named default-features root's own feature" \
    plant_broken_link_behind_the_excepted_feature
  gate_selftest_passes "a cargo root the repository does not track (an agent worktree under .claude/)" \
    plant_untracked_worktree_with_broken_link
  # THE TWO READERS. Both decide whether pass 2 covers anything, and
  # neither had ever been shown to fail — S157's class, in the gate
  # whose subject is that a guard never shown to fire is not a guard.
  # Each stub also breaks the pass ABOVE the reader, so the wanted text
  # is the reader's own diagnosis and not merely a non-zero exit.
  gate_selftest_without_tool cargo "cargo metadata failed"
  gate_selftest_without_tool git "git ls-files failed"
  # --print-roots, ON THE SAME DERIVATION. The hosted cache's scope is
  # whatever this prints, so a mode that printed a bare `.` — a reader
  # that failed quietly, a `dirname` that lost the outside roots — would
  # put the `fmt` job back to compiling six cargo roots from nothing on
  # every run, with every gate still green. Both directions are checked:
  # the exact set on a clean fixture, and a diagnosis rather than a short
  # list when a reader cannot answer.
  gate_selftest_prints_roots
  GATE_SELFTEST_ARGS=(--print-roots)
  gate_selftest_without_tool cargo "cargo metadata failed"
  gate_selftest_without_tool git "git ls-files failed"
  GATE_SELFTEST_ARGS=()
  printf '%s selftest OK: passes a clean three-root fixture, a public link to a private sibling, a link from a not(feature) half into the gated one, prose behind the excepted root'"'"'s feature, and an untracked worktree checkout; fires on a broken link in a workspace member and in a root outside the workspace — in each of their same-named binaries and examples — on a private item, on an excluded root'"'"'s feature-gated prose, on a doc error inside a not(feature) half in each pass'"'"'s roots, and when either cargo or git cannot answer; prints the derived root set under --print-roots, and diagnoses rather than shortening it when a reader fails\n' \
    "$(gate_name)"
}

# --print-roots pulled out of argv before lib.sh's parser sees it, the
# way scripts/gates/probe-suite-census.sh adds its modes: `gate_parse_args`
# knows `--selftest` and `--root` and rejects anything else.
PRINT_ROOTS=false
SKIP_VIEWER_TOOLKIT=false
gate_args=()
for a in "$@"; do
  case "$a" in
    --print-roots) PRINT_ROOTS=true ;;
    --skip-viewer-toolkit) SKIP_VIEWER_TOOLKIT=true ;;
    *) gate_args+=("$a") ;;
  esac
done
gate_parse_args ${gate_args[@]+"${gate_args[@]}"}
gate_main
