---
id: finish-marker-cannot-say-summarised
kind: issue
title: finish and finish_non_exhaustive are a two-valued marker being asked to carry a three-valued distinction: printed, summarised, not carried
status: open
opened: 2026-09-06
refs: [2093, debug-walk-inlines-an-unbounded-report-while-summarising-a-bool]
---

Residue of #2093, disclosed by the style review of that PR
(`debug-walk-inlines-an-unbounded-report-while-summarising-a-bool`,
closed there) and not fixed by it.

The four exhaustive `Debug` walks draw a three-way distinction over a
value's fields:

- **printed** — the field's own `Debug`, in full (`selection`, `hover`,
  `bounds`, `fault`, `at_rest`, `path`, `generation`);
- **summarised** — carried as a length, a presence or a count, because
  the field itself is a document, a DAG, an unbounded `Vec` or an
  index (`states`, `gesture`, `scratch`, `resolver`, `body`, `checks`,
  `index`);
- **not carried** — the `_` arms.

`std`'s pair says only whether the dump shows all the *fields*, so it
collapses the first two. `Derived`'s walk ends in `finish()`
(`crates/viewer/src/session.rs:325`) on the local rule *no `_` arms*
while rendering `scratch: false` in place of a whole
`Doc<ProfileProgram>`; a reader who knows `std` and not
`crates/viewer/README.md` reads that as a complete rendering of a
document-shaped field. The same reading applies to `body` and
`resolver` under `finish_non_exhaustive`, where the marker is true for
a reason unrelated to them.

Nothing is wrong today: no code reads either dump, and the README's
table says which fields are summarised. What is missing is a spelling
that says it at the site. Candidates, none obviously right: a key that
names the summary (`scratch_present`), a one-field wrapper whose
`Debug` prints an elision (`scratch: Some(<Doc>)`), or accepting that
the marker cannot carry it and putting the three-way split only in the
README, which is what #2093 did.

This wants deciding once for all four walks rather than per walk, which
is why it is a row and not a follow-up edit.
