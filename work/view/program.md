---
id: view
kind: program
title: VIEW — viewer architecture
status: active
opened: 2026-09-03
area: gui
prefix: view/
tag: (VIEW orchestrator)
ab_band: 1900-1999
paths: [crates/viewer/src/*, crates/viewer/tests/*, crates/viewer/README.md]
keep_out: [CHROME's slate LANDED 2026-09-04 and that program has been dormant since 07:00 — the wait clause is discharged and this program's paths now cover crates/viewer/tests/* by Ev's word (in-chat, 2026-09-04); crates/viewer/tests/* is also S-TCOST's, S-TINT's and Track W's by declaration — S-TCOST split on 2026-09-11 and the integrity half is S-TINT's, which is the half a VIEW lane adding assertions actually touches, so test-MECHANISM changes are announced to both, the rulings this program's builds consume (layer-3 identity and free-move commit) are DOCM's, crates/editor-core is DOCM's and is read-only here EXCEPT for one narrow amendment Ev authorised in-chat 2026-09-04 — EditError's user-facing Display wording (the `edit: ` prefix and the {:?}-quoted payloads), because the layer that raises it has no reason to know the viewer renders it verbatim to a person and VIEW cannot fix the sentence from its own side; no EditError variant, no edit semantics, the authored-step to canonical-segment map door straddles TWO other programs' globs and not one — the authored `step` coordinate is ProfileProgram::step_args in crates/editor-core/src/program.rs (DOCM) and the canonical `segment` is crates/profile's canonicalization (S-BOOL) — so it is two announces, and where the map lives is a question neither this program nor either owner can answer alone, the GUI-3 §5 seam is ratified design and 6a is RULED (Ev, #1843) — a further revision is an [ev] PR, PERF cedes per-frame rendering and hover-picking here, RE-SCOPED 2026-09-17: four successor programs vnews and vgeom and vseam and vdoc were opened on this program's ground and 86 of its 94 live rows moved to them or to seven live programs - this program's paths are unchanged and still cover crates/viewer/src/* and crates/viewer/tests/* and crates/viewer/README.md, so every path the four claim is claimed here too and this clause is the other half of the record their keep_outs already carry; what stays here is the eight rows the re-scope did not move - the six in review whose lanes are in flight and would conflict on a rename, startup-notices-need-holding-to-badge, and a-dead-seam-worker-reads-as-an-ordinary-idle-state whose PR 2762 is parked on a ruling; each review row's successor is named in that program's plan under Inbound and arrives when its PR merges; this program does not dispatch new units - its remaining act is the exit walk]
priority: P1
---

The viewer's structural questions, gated by one conversation: the
split of `session.rs` and `app.rs` into modules, ratified into
`crates/viewer/README.md`, then the picking vocabulary, status-line
ownership, per-segment focus and the off-thread pick index. Class D
with one D→H build. CHROME's slate has landed, so the opening
condition is spent. Charter and order: `work/view/plan.md`; narrative
in `work/view/log.md`.
