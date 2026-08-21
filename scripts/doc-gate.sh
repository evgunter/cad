#!/usr/bin/env bash
# Shared rustdoc gate — the SINGLE implementation, called by BOTH
# .github/workflows/ci.yml (its `fmt` job, since #852 folded the old
# standalone `doc` job into it) and local-scripts/ci-local.sh,
# the ci-filter.py arrangement applied to a second gate.
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
# COVERAGE. The gate has no exclusions OF ITS OWN: every crate in the
# workspace is subject to it, a new crate from its first commit. It
# landed with a shrinking KNOWN_DIRTY exclusion list (#465 chunk 0)
# because 75 warnings had accumulated unseen; that list is now empty and
# the machinery is gone with it. If a cleanup ever needs staging again,
# stage it in the CHANGE, not by re-adding an escape hatch here.
#
# WHAT IT DOES NOT COVER, stated because "workspace-wide" reads as
# "everything" and is not. `cargo doc --workspace` sees WORKSPACE
# MEMBERS, and the root manifest excludes `demos/` and `tools/`
# (Cargo.toml's `workspace.exclude`). Those crates are separate cargo
# roots with their own fmt+clippy+test rows in ci.yml's `k-lint` job,
# and **only two of those five rows run `cargo doc`** — so the prose in
# `tools/k-lint`, `demos/tour` and `demos/wild` is outside this gate
# entirely, and `tools/tess-meter` and `tools/tess-lint` are covered
# only by a step hand-copied into their own rows, not by anything here.
# That is a real hole by this gate's own argument: #709 moved ~1,050
# lines of prose from `crates/mesh/src/budget.rs` into
# `tools/tess-meter`, which is precisely the prose-goes-dark case
# above, and it went from covered to uncovered by moving. #709 added a
# `cargo doc` step to the tess-meter row as a stopgap and #738 copied
# it to the tess-lint row for the same reason; **a row is owed that
# does the same for every excluded root**, and it is unscheduled.

set -euo pipefail

# -D warnings on the rustdoc lints proper; the private-link lint is
# allowed for the reason argued above.
#
# --all-features, UNLIKE the clippy job. Clippy avoids it because the
# `interval` feature is a second build graph whose test targets would
# double that job's compile time for no extra coverage, and the interval
# job owns its own clippy pass. Neither reason survives here: rustdoc
# builds no test targets, and there is no second doc job. What the flag
# buys is real — under default features alone, every doc link into
# `#[cfg(feature = "probe")]` or `#[cfg(feature = "interval")]` code
# resolves to nothing, so rustdoc reported 12 CORRECT links as broken
# while the prose on those items went unchecked entirely. Documenting
# the full feature set is also what docs.rs does by default.
RUSTDOCFLAGS="-D warnings -A rustdoc::private_intra_doc_links" \
  exec cargo doc --workspace --all-features --no-deps --document-private-items
