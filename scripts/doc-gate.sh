#!/usr/bin/env bash
# Shared rustdoc gate — the SINGLE implementation, called by BOTH
# .github/workflows/ci.yml (its `doc` job) and local-scripts/ci-local.sh,
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
# THE RATCHET. This fails CLOSED: the gate runs over the WHOLE workspace
# and every crate is subject to it, except the crates named in
# KNOWN_DIRTY below. A NEW crate is therefore gated from its first
# commit. The list only ever SHRINKS — each cleanup lands one crate's
# fixes and deletes its line, and a crate already off the list cannot
# regress. Do not add to it.

set -euo pipefail

# Crates with a rustdoc-warning backlog, with their current counts, to be
# emptied one crate per change (#465). Cheapest first.
KNOWN_DIRTY=(
  editor-core  #  8
  profile      #  9
  geom-core    # 14
  topo         # 14
)

exclude=()
for crate in "${KNOWN_DIRTY[@]}"; do
  exclude+=(--exclude "$crate")
done

if [ "${#KNOWN_DIRTY[@]}" -gt 0 ]; then
  echo "doc gate: ${#KNOWN_DIRTY[@]} crate(s) still excluded (#465): ${KNOWN_DIRTY[*]}"
else
  echo "doc gate: no exclusions left — #465 is done; delete KNOWN_DIRTY and the --exclude plumbing."
fi

# -D warnings on the rustdoc lints proper; the private-link lint is
# allowed for the reason argued above. No --all-features: the `interval`
# feature is a different build graph and its own cache, exactly as the
# clippy job reasons.
RUSTDOCFLAGS="-D warnings -A rustdoc::private_intra_doc_links" \
  exec cargo doc --workspace "${exclude[@]}" --no-deps --document-private-items
