---
id: engine-directories-are-leaves-guard
kind: unit
title: "[topo.homes]'s guard: a source_walk row that no file outside boolean/ or splitting/ names them, neither names the other, and neither declares pub(crate), over a committed shrink-only allowlist of today's violations"
status: closed
opened: 2026-11-03
priority: P1
cost: M
branch: onedoor/engine-leaves-guard
closed: 2026-11-03
pr: 6458
---


Unit 1 of `[topo.homes]` (`crates/topo/README.md`, ratified on PR 6426, 2026-11-03).

Build one source-walk row, `engine_directories_are_leaves`, in `topo`'s existing source walker. It checks, over production and test code alike:
- **(a)** no file outside `boolean/` names `crate::boolean::`, except `lib.rs`'s re-exports, and the same for `splitting/`;
- **(b)** neither engine directory names the other;
- **(c)** no item inside an engine directory is declared `pub(crate)`.

It reads a committed allowlist of today's violations, one line per (file, kind). Two rules keep the allowlist honest:
- it may only shrink, so an entry that no longer matches is a red too, and a move that clears a site must drop its line;
- its failure message points at `[topo.homes]` and its signature test.

Then delete the placement arguments that the clause and the guard now replace. Each module doc keeps a one-line citation of `[topo.homes]`. The arguments to delete:
- the four module-doc placement paragraphs (`sector_shape`, `sector_face`, `chord_join`, `face_normal`);
- `sector_shape`'s rejected-`geom-brep` paragraph;
- the three `lib.rs` placement comments.

The allowlist is the move list for units 2 onwards: `crossing-doors-leave-topo-for-geom-brep`, `topo-shared-cores-hosted-in-one-half`, `S173`, `chart-window-walk-written-twice`, and the `boolean/` library moves. Moving no code is in scope here. Review tier: single style review.

## Closed (PR 6458, 2026-11-03)

The guard is the per-PR gate `scripts/gates/engine-leaves.sh`, with the counter `engine_leaves.py` and the shrink-only allowlist `engine-leaves.counts`. It counts engine items by name, however they are reached, and refuses outright any construct that binds a new name to an engine item or to `crate`.

Allowlist at merge, the move list (files / sites):
- names-boolean: 71 / 1594
- names-splitting: 19 / 271
- boolean-names-splitting: 24 / 342
- splitting-names-boolean: 10 / 100
- crate-visible: 48 / 780
- refused-binding: 11 / 25

61 ambiguous names are listed.

The review history: a style review, a re-review and an adversarial check each found the scanner satisfiable without a move. The scanner stopped there, with its blind spots stated. Its exact successor is `engine-leaves-by-visibility`.
