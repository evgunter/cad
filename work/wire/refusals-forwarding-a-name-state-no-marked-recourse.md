---
id: refusals-forwarding-a-name-state-no-marked-recourse
kind: issue
title: Six refusals that forward a name state their recourse in words the refusal standard does not read as one
status: open
opened: 2026-10-03
priority: P3
cost: E
---


Found by PR 3886's second fix pass. Its corpus gate (`editor-core/tests/name_words_corpus.rs`, `every_corpus_name_reads_apart_and_forwards_within_the_refusal_budget`) holds every refusal that forwards a name to `test_utils::refusal::problems`. Six of the eight rows state no recourse the standard reads (`recourse_markers`: a `Recourse:` label, "there is no way through", or a `BARE_RECOURSES` phrase). Each states one in its own words:

| row | where | its recourse, unmarked |
|---|---|---|
| `ResolveError::Vanished` | `resolve/mod.rs`, `Say for ResolveError` and `Diagnosis` | none stated in the cascade arm, which points at the upstream name's own failure |
| `ResolveError::NodeGone` | same | "the repair is an explicit rebind" |
| `SelectRefusal::TiedDisagrees` | `names/geompred.rs`, `Say for SelectRefusal` | "disambiguate the name, or ask something all its candidates answer alike" |
| `SelectRefusal::Unreadable` | same | "ask about one of its faces, edges, or vertices instead" (the `WholeBody` arm) |
| `NodeErrorKind::CrossingUnverified` | `eval/mod.rs`, `Say for NodeErrorKind` | none stated; "the crossing does not re-verify against this version of the part" |
| `HitTestError::Ambiguous` | `resolve/hit.rs`, `Say for HitTestError` | "aim away from the shared edge, or choose one of the tied faces" |

The gate admits these six by row (`UNMARKED_RECOURSE`) and fails once one of them marks its recourse, so the admission cannot outlive the fix.

**The fix.** Write each recourse as one `Recourse:` clause, and write one for the two rows that state none. Then empty `UNMARKED_RECOURSE`.

Ground: `eval/mod.rs` and `names/geompred.rs` are WIRE's. `resolve/hit.rs` is DOCTAIL's, and `resolve/mod.rs` is RECIPE's. All six rows are one change, filed here with the majority.
