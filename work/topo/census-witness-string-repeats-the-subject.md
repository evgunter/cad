---
id: census-witness-string-repeats-the-subject
kind: issue
title: two census witness STRINGS repeat the subject in the slot documented for the witnessing position
status: open
opened: 2026-09-11
priority: P4
cost: E
---


## The defect

`ValidationError::UndeclaredContact`'s `witness` field is documented
"a debug rendering of the witnessing **position**"
(`crates/topo/src/validate.rs:1101`), and
`ContactContradicted`'s "a debug rendering of the witnessing **site**"
(`validate.rs:1123`). Both `Display` arms put it after the preposition
*at*: "undeclared contact {contact} **at** {witness}"
(`validate.rs:1888`) and "…is contradicted **at** {witness} by…"
(`validate.rs:1895`).

Two sites fill that locative slot with a repeat of the subject the
same sentence has already named:

- `crates/topo/src/census.rs:1677` — `witness: format!("{fa:?}~{fb:?}")`
  beside `contact: CensusContact::ConformalPatch { finding }`, whose
  `finding.pair` is `(fa, fb)`. The message renders the pair twice and
  supplies no position at all.
- `crates/topo/src/census.rs:2830` — `witness: format!("{:?}~{:?}", c.face_a, c.face_b)`
  beside `declaration: DeclaredContact { a: c.face_a, b: c.face_b, .. }`.
  Verbatim: the same two keys, the same `{:?}` formatter, two clauses
  apart.

The contrast that shows what the field is for is `census.rs:732`:
`contact: CensusContact::VertexVertex { a: ka, b: kb }` carries the
pair typed and `witness: witness(pa)` carries the coordinate. The
other seven `ValidationError` string witnesses in the file do the
same.

This is not the `{:?}`-where-`Display`-belongs class
(`tier-3-prime-findings-render-through-debug`, closed): that is about
the formatter, this is about the content. Nor is it a correctness gap
— both variants carry the pair in a typed field, so a consumer
resolves from the field and never parses the prose. It costs a reader
the locus, which is what makes the other findings actionable.

## Why it was not fixed where it was found

Neither site has a position to hand.

- At `1677` the arm's evidence is `ChartOverlap::PositiveArea`, a
  region verdict the chart-region predicate returns without a point.
  Supplying a coordinate means changing what that predicate hands
  back, or running a witness search beside it — a kernel change in
  `crates/topo/src/census.rs`, which is CURVED's territory; FIX edits
  that file only by the recorded seam in `work/fix/program.md`, and a
  predicate change is well past it.
- At `2830` the duplicate could be dropped cheaply, but `witness` is
  `String`, not `Option<String>`, so "this arm has no position" has no
  spelling. Whether the field should become optional, or the two arms
  should carry a typed locus, is a decision about
  `ValidationError`'s shape.

## Provenance

Found by `pair-subject-witness-strings-unswept` while establishing
that the ORDER question that item was opened on does not arise. The
order is correct at both sites; the content is what is wrong.

## Re-homed to TOPO, 2026-09-20

(FIX orchestrator) Ev, in chat, 2026-09-20: *"can you kick all the design decisions back to
the track they actually belong to, leaving fix design-free?"* FIX is the
program for rows whose fix is already written; a row whose blocking
question is a DESIGN decision belongs to the track that owns the surface
the decision is about.

**The decision this row is blocked on:** `ValidationError`'s shape — whether `witness` becomes `Option<String>`, or
the two arms carry a typed locus.

**Why TOPO.** `crates/topo/src/validate.rs`, where `UndeclaredContact` and
`ContactContradicted` are declared and documented, is **owned by topo**. The
decision is about that type; TOPO's slate already carries its neighbours
(`three-refusal-variants-nest-a-certification-error`,
`censussubject-eq-answers-false-for-a-new-variant-against-itself`).

**The two defective sites are REACH's, not CURVED's.** Both are in
`crates/topo/src/census.rs`, which territory says is **owned by reach** —
`work/fix/program.md`'s `keep_out` called that file CURVED's, which was true
when the clause was written and is not true now. CURVED's `paths` no longer
name it. Whoever takes this row crosses REACH at the two `format!` sites and
announces there; the chart-region half (site `1677` has no position to supply
because `ChartOverlap::PositiveArea` returns no point) reaches CHART, which
owns `crates/topo/src/chart_region.rs`.

Nothing about the finding is changed by the move: same id, same
evidence, still `open`, and no part of its question is answered for
you except where this note says Ev answered it.
