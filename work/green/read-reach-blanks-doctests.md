---
id: read-reach-blanks-doctests
kind: issue
title: The read reach blanks doc comments, so a doctest that reads outside its crate is no read edge
status: open
opened: 2026-11-02
priority: P3
cost: E
---


`scripts/ci-filter.py`'s `_file_destinations` (the read reach, ~line 905)
reads each `.rs` file through `_rust_code_view` (~line 697), which blanks
every `//` comment, `///` and `//!` doc lines included. A doctest is code
in a doc comment, and rustdoc compiles it with `CARGO_MANIFEST_DIR` set, so
a doctest that opens `env!("CARGO_MANIFEST_DIR")/../../docs/x` or another
member's file creates a read edge the reach never sees. Its crate then does
not join the closure when the file it reads changes: the selection fails
open for that doctest. No such doctest exists today (found by the sweep in
PR 6324, which reads `///` fenced and indented lines as code for the script
readers but left the reach alone as outside its scope).

Fix shape: give the reach the doc-comment code lines too (fenced, or
indented ≥4 spaces after the prefix), or fail closed when an anchor token
appears in any doc-comment line.
