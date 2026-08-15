# LIB-G14 spec — the split-naming walls, executed (binding)

Mandate: implement the RATIFIED G14 disposition (#512;
docs/NAMING-DESIGN.md's split-naming-walls section is the
contract; the survey at cad-work/g14-survey.md has the measured
mechanics and file:line pointers).

## 0. Discipline (absolute)

docs/LIB-PYG1-SPEC.md §0 verbatim and binding
(local-scripts/with-build-slot.sh, foreground one-at-a-time,
kill-your-own-waiter, commit+push per chunk, NO Co-Authored-By,
no model names, merge-main-before-open + re-merge, checks
STARTED, cold clippy CI scope, k-lint discipline, invariant
comments). SHARD any battery row >~15 min; prefer express.
CONCURRENCY FENCE: RESPELL PR-1 (crates/profile + step
vocabulary) and PYG5 (detect/declare + the boolean-refusal
PAYLOAD) are in flight — your territory is names/ (emit_topo,
role, the emitters) + the NamingError type; if a collision
appears, STOP and report.

## 1. Deliverables

1. **Wall B (B1, align-to-ratified)**: upstream_name and
   name_boolean's shared guard propagate Entry::Tied as TIED
   (the name_pattern/name_in_part/graft_names precedent —
   mirror their shape); the refusal narrows to entities whose
   own name genuinely requires a unique upstream. Rows: the
   U-cutter fixture (the survey's measured case) names with the
   tie propagated; a genuinely-unique-requiring case still
   refuses typed.
2. **Wall A (A2, ratified)**: a section line crossing one
   operand face twice mints TIED SectionEdge entries instead of
   refusing. Rows: the L-shaped single-loop extrude (the
   survey's boolean-free repro) now names; the tied chords are
   narrowed to a specific chord via the existing selector layer
   (a select_where row proving reachability); the
   multiple-chords refusal string RETIRES (grep-proven absent).
3. **#380 rider**: NamingError gains Display carrying the
   emitter's payload (fail-loud: refusals name their subject);
   the pncad-py error text now shows it (no tag changes —
   message content only). One row pinning the prose carries the
   payload.
4. **Audit**: cutaway (31) flips to an executed YES row against
   its exact oracle IF its chain now names end-to-end from
   Python (measure; if another wall surfaces, re-partition
   honestly and file it). Keep the audit edit MINIMAL (your row
   + counts — two other in-flight units also touch the page;
   re-derive counts from the table at your FINAL re-merge).
5. Numbered findings.

## 2. Fence

OUT: crates/profile, the step vocabulary, detect/declare,
selector atoms, schema, CI structure. Anything missing: REPORT.

## 3. Acceptance

cargo test -p editor-core -p pncad -p pncad-py green; the
retired refusal string grep-absent; python suite green (delta
stated); cold clippy CI scope; hosted CI green; zero new
[[test]] binaries. Pre-draw fields at dispatch: difficulty M,
task-class STRUCTURAL.

## 4. PR discipline

One PR. Report ≤150 lines to
~/.local/share/cad-work/lib-g14-report.md, per-phase figures.
Open, do NOT merge. Final message: PR number + report path +
≤10-line summary. Forks: report, smallest faithful reading, flag.
