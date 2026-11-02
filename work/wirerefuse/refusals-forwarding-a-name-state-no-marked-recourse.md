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

## Moved to WIRETAIL (2026-10-17)

WIRE's priority-seam cut moved this row, keeping its id and body: WIRE keeps its P0–P2 rows and the E rows that ride with them.

Signed (WIRE orchestrator).

## Triage (2026-10-17, WIRE orchestrator)

Shrunk: `UNMARKED_RECOURSE` now lists four rows (`ResolveError::Vanished`, `SelectRefusal::TiedDisagrees`, `SelectRefusal::Unreadable`, `NodeErrorKind::CrossingUnverified`); `NodeGone` and `HitTestError::Ambiguous` are already fixed.

## Re-checked (2026-10-26, tracker sweep)

Shrunk again: `UNMARKED_RECOURSE`
(`crates/editor-core/tests/name_words_corpus.rs:74`) lists two rows,
`SelectRefusal::TiedDisagrees` and `NodeErrorKind::CrossingUnverified`.
`ResolveError::Vanished` and `SelectRefusal::Unreadable` are no longer
listed, so the gate reads a marked recourse on both.

## Fixed (2026-11-02, wire-refuse1, branch `wirerefuse/marked-recourse`)

- `SelectRefusal::TiedDisagrees` (`names/geompred.rs`, `Say for
  SelectRefusal`): its recourse now opens with `sentence::Recourse`.
- `NodeErrorKind::CrossingUnverified` (`eval/mod.rs`, `Say for
  NodeErrorKind`): states one, "update the reference to a version of
  the part that names it" (`DocEdit::UpdateReference`); the clause "so
  the crossing does not re-verify" went, and its `OVER_BUDGET` ceiling
  moved 80 → 87 (the recourse's words). Its `FILED_NO_RECOURSE`
  admission in `refusal_concision_chains.rs` (filed under
  `work/wire/wire-refusals-short-of-the-shape-guard.md`) is gone too.
- `UNMARKED_RECOURSE` and the gate's admission branch are deleted, so
  any row of the corpus gate that states no marked recourse now fails.

Sweep (pattern: a refusal literal whose recourse follows `;`, `—` or
`. ` as an imperative, or ends `instead`, with no `Recourse` on the
line) found and fixed four more: `SelectRefusal::NotAPlane`
(`names/geompred.rs`), `ParseError::RatioPartNotInteger` (`parse.rs`),
`RangeRefusal::SlotIsDefined` (`range.rs`) and `NodePickError::NotABody`
(`resolve/pick.rs`). Blind spot: a recourse joined by `,`/`:` or
spread over a wrapped literal line the regex does not see, and rows
that state none at all (those are `FILED_NO_RECOURSE`'s).
