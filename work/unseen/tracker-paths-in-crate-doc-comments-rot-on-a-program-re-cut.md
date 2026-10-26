---
id: tracker-paths-in-crate-doc-comments-rot-on-a-program-re-cut
kind: issue
title: A CLASS - a work/<program>/<id> path in a crate doc-comment rots the day the row moves programs, and nothing resolves it: 75 of the 144 distinct paths cited under crates/*/src point at no file
status: open
opened: 2026-09-24
priority: P4
cost: E
---


Found by both reviewers of PR 2603 (two new test docs cited
`work/trim/validate-pcurves-cannot-tell-…` after the row had moved to
`work/pcert/`; `euler_ring.rs` cited `work/topo/two-provenance-…`
after it moved to `work/origin/`), and filed by that PR's fix pass as
the class rather than the instances. On META's slate because the
paths cross every program and the instrument belongs beside
`scripts/work.py`, like `doc-citations-no-gate-checks-rot-silently` —
which this is the third arm of: that row's arms are `file.rs:NNN` in
`work/` and test names in doc comments; this one is a `work/` path in
a crate.

**The shape.** A doc-comment in `crates/*/src` cites a tracker row by
its path, `` `work/<program>/<id>` ``. The id is stable for life
(`work/README.md`, "Ids are stable"), but the DIRECTORY is the owning
program, and rows move between programs by `git mv` — at every
re-homing, every priority-seam cut, and every program close. The path
in the crate does not follow. `scripts/work.py lint` reads only
`work/`; rustdoc's `broken_intra_doc_links` reads only bracketed
links; a backticked path is inert text to both.

**Measured at PR 2603's fix-pass head** (`grep -o 'work/[a-z-]*/[A-Za-z0-9_-]*'`
over `crates/*/src/**/*.rs`): **144 distinct paths cited, 211
citations; 75 of the 144 resolve to no file.** Of the 75, about 60
name a row that exists under another program's directory (the
2026-09-20 TOPO cut alone accounts for `work/topo/…` → `work/origin/`
and `work/probe/`; TRIM's re-cut for `work/trim/…` → `work/pcert/`,
`work/chart/`, `work/iso/`; and so on across `blend`, `bool`,
`props`, `sym`, `view`, `wire`), and about 15 name nothing that
exists anywhere — closed rows whose programs were deleted, or ids
truncated at a line wrap (`work/props/coincidence-zone-priced-`,
`work/census/pncad-py-tests-rs-`). The fix pass repaired the five in
the two files it was editing, as `work/README.md` says a passing lane
should; it did not sweep the rest, which are 40-odd files across
every crate and would be a PR of their own.

**Instrument.** `scripts/work.py lint` (or a sibling mode) resolves
every `work/<p>/<id>` cited under `crates/`, `docs/` and `work/`
itself: a path whose id exists under a DIFFERENT program is a warning
naming the move (mechanically fixable — the id is the key, the
directory is derived); a path whose id exists nowhere is an error, or
a warning if closed programs are too common a cause. The check is a
directory listing plus a regex; the hard part it does NOT have is
knowing when a citation should have been an intra-doc link instead,
which `a-citation-in-a-line-comment-is-not-checked` (BLIND) is
about. Alternatively the convention changes: cite rows by bare id
(`` `validate-pcurves-cannot-tell-…` ``, no directory), which is what
`refs:` and `parent:` already do inside `work/`, and the reader runs
`work.py` to find it. That is a decision about how crates cite the
tracker, so this row is filed rather than fixed.
