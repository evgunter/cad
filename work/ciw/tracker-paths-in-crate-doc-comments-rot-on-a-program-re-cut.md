---
id: tracker-paths-in-crate-doc-comments-rot-on-a-program-re-cut
kind: issue
title: Crate doc comments cite tracker rows by work/<program>/<id> path, which rots on every re-home; cite by bare id instead
status: open
opened: 2026-09-24
priority: P4
cost: E
---

A `work/<program>/<id>` path in a `crates/*/src` comment stops resolving the
day the row moves programs by `git mv` (`work/README.md`: ids are stable,
directories are not). Measured at PR 2603: 75 of 144 distinct cited paths
resolved to no file. Inside `work/`, `refs:` and `parent:` already cite by
bare id.

A lint that resolves the paths would make every program re-cut or merge (a
tracker-only diff that today runs no build: `scripts/ci-filter.py`'s
`AUX_PREFIXES` lists `work/`) red until the same PR edits comments across
`crates/*/src`, which then selects those crates' builds and tests. That turns
a free tracker PR into a full gate run, so no gate.

Owed: the convention, written once where crates' citation style lives
(`work/README.md`'s ids section): a crate cites a row by its bare id, and
`python3 scripts/work.py` finds it. Paths are converted when a lane edits a
file for its own reasons; no sweep PR.

What would close it: the convention sentence landed (it binds future work,
so it waits for Ev).

- 2026-11-22 — **Reformulated by CIW triage** (the earlier body is in git history): The instrument it proposes (`work.py lint` resolving `work/<p>/<id>` under `crates/`)
would red every program re-cut until that tracker PR edits comments across `crates/*/src` —
turning a diff that runs no build (`scripts/ci-filter.py:1725` lists `work/` as aux) into a
full multi-crate gate run. Re-cuts are frequent (TINT, GUARD, TCOST merges 11-20 alone).
Grain: the bare-id citation convention it names as the alternative. Reform:
reform/tracker-paths-in-crate-doc-comments-rot-on-a-program-re-cut.md

## Evidence (2026-11-22, OFFER's kernel certify unit)

`work/props/` no longer exists and 25 citations of it remain outside `work/` (`grep -rn "work/props/" --include=*.rs --include=*.py --include=*.md .`, less `work/` and `memories/`): in `crates/topo`, `crates/mesh`, `crates/sweep`, `crates/geom`, `crates/geom-core`, `crates/geom-brep`, `crates/editor-core/tests`, `crates/pncad-py/tests` and `docs/CURVED-SPIRIC-DESIGN.md`. That unit fixed the four on its own ground (`crates/editor-core/src/range.rs`, `crates/viewer/src/bounds.rs`) and left the rest to this row.
