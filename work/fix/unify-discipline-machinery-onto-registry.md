---
id: unify-discipline-machinery-onto-registry
kind: issue
title: checks/disciplines - unify the shared machinery onto the registry (planned)
status: open
opened: 2026-08-25
github: 981
refs: [978, 979]
---

## From GitHub issue 981

Opened 2026-08-25; 0 comments.

Recording Ev's ask (2026-08-25, the checks-registry conversation, on reading PR #978's diff): the registry unit deliberately unified none of the existing discipline machinery (DISCIPLINES-DESIGN DS8's no-speculative-refactor rule — sharing lands where a second consumer makes it real), and the unification is now **planned**, not merely possible. Not scheduled by this issue.

The plan, in order:

1. **The finding/menu sink** — the document layer's one rendering door: `refusal_menu`'s discipline refusals (`FlushFinding` / `NodeErrorKind::UndeclaredContact`, `crates/editor-core/src/eval/wire.rs`) and `checks::CheckFinding` rendering through shared machinery. The seam DS8 already names as first.
2. **The DS1 discipline scaffolding** — the shared five-part-pattern plumbing (ladder walk, verify-table shape, detector-as-verifier, refusal menu) that today is hand-built three times (profile tangency #101, carrier equality, declared contact C4). It materializes **with the parameter-coincidence unit** (the first new discipline — a template with no second instance is speculation; the second instance is what proves the template), after which the flush/Rest detect-declare pieces (`names/flush.rs`) migrate onto it.

Explicitly NOT planned, by DS1's own rule: moving the three disciplines' predicates and verify tables out of their geometry homes. The registry unifies the machinery *around* the predicates; each stratum's mathematics stays where its geometry lives (the C4 tables in `topo`, the joint classifier in `profile`).

Pointers: `docs/DISCIPLINES-DESIGN.md` DS1/DS8/DS9, PR #978, #979 (the void-birth-marking plan, independent).

## Home

The registry and the finding sink live in `crates/editor-core/src/checks.rs` and `eval/wire.rs`, which no open program's territory covers, so it lands unowned under `work/issues/`.
