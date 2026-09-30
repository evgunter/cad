---
id: tools-doc-prose-cites-thirteen-dead-work-meter-paths
kind: issue
title: tools/ doc prose cites thirteen work/meter/ paths that resolve nowhere, two of them in the list whose own sentence claims its pointers followed
status: open
opened: 2026-09-16
priority: P4
cost: E
---



## Was

found by unit 0's style review, on the fix that re-pointed the sites
naming the census file METER's unit 10 folded away. Unit 0's sweep was
shaped by the census FILENAME, so it found the tracker copies and could
not see this: the same defect, in live code doc, keyed on the tracker
DIRECTORY instead.

## Finding

METER closed and its directory left the tracker whole
(`docs/DOC-LEDGER.md` sweep 10; recoverable at
`2723839067e80198bec2889d041e490000f02275`). **Thirteen citations in
`tools/` still name a `work/meter/<row>` path.** None is a `keep_out`
clause and none is a dated record: each is live prose telling a reader
to go and read a row, and the reader arrives nowhere.

| file | citations | rows named |
| --- | --- | --- |
| `tools/tess-lint/tests/baseline_census.rs` | 5 | `C15` (x2), `D201`, `baseline-sizing-census-second-copy`, `tess-lint-ungated-columns-fold-silently` |
| `tools/k-lint/tests/predicate_roster.rs` | 5 | `k-lint-eps-coupled-criterion-unwritten` (x3), `k-lint-roster-wants-a-kernel-side-vocabulary`, `k-lint-last-round-is-eps-coupled-but-unrostered` |
| `tools/k-lint/src/lib.rs` | 1 | `k-lint-eps-coupled-criterion-unwritten` |
| `tools/tess-meter/src/lib.rs` | 1 | `tess-meter-sampled-retune-figure-unreproducible` |
| `tools/README.md` | 1 | `tess-lint-zero-certificate-two-meanings` |

A fourteenth occurrence, `baseline_census.rs`'s *"re-homed from
`work/code-quality/` to `work/meter/` in the tracker-wide cut"*, names
the directory as a FORMER home and is a record, not a pointer. It is
excluded from the thirteen on the same reasoning that excludes
`work/instr/plan.md`'s *"has just deleted …"*.

**Two of the thirteen are inside the doctrine's own worked example.**
`baseline_census.rs:84-85` are the `work/meter/C15.md` and
`work/meter/D201.md` entries of the four-item list at `:78-92`, and the
paragraph immediately under it (`:94-101`) is the sentence this whole
class is adjudicated by: *"The FIGURES in that list are frozen; the
PATHS are not. The first two rows were re-homed from
`work/code-quality/` to `work/meter/` … and the pointers here followed
them."* They followed once and have not followed since. The file that
states the rule is the file that breaks it, in the list the rule is
about.

**One is inside an assertion string.** `predicate_roster.rs:146` puts
the path in the message a failing test prints, so the defect is not
merely read — it is handed to whoever the test reds on, at the moment
they are least able to check it.

## The repair is not uniform, which is why this is a row and not a sed

Three destinations, and they have to be resolved against the tracker
one citation at a time:

- **Nine** name rows now open on `work/instr/`.
- **`k-lint-roster-wants-a-kernel-side-vocabulary`** moved again: it is
  on `work/props/` as of 2026-09-16, and was still on `work/instr/` at
  this row's own merge base (`f8fbf916e`). A pattern rewrite keyed on
  the program name gets this one wrong.
- **`D201`** exists nowhere. It closed with METER and is recoverable
  only at the ledger's sweep SHA, so its citation needs the reading
  stated or the SHA named, not a path.

## Not fixed here, deliberately

`predicate_roster.rs` and `k-lint/src/lib.rs` are unit 12's live
ground and `baseline_census.rs` is unit 4's. One-file-one-item makes a
second editor a merge conflict by design, so this is filed at the
moment of disclosure rather than scheduled into unit 0's fix pass.

**A form is already settled and does not need re-deciding.** METER's
own closed row `sweep-deleting-work-meter-dangles-six-refs-on-instr-rows`
answered the `refs:` FRONTMATTER half of this at the sweep, adopting
GATES' shape: replace the dying id with its closing PR number and carry
one prose clause naming the sweep. That row's scope was the header
field `scripts/work.py` lints, so it never looked at prose, and prose
is where all thirteen of these are. The same two-part form applies.

## Sweep

`grep -rn "work/meter/" tools/` at `f8fbf916e`, then every named row
resolved with `find work -name '<id>.md'` against `origin/main` rather
than against the merge base — which is how the `work/props/` move
above was caught. **Re-run after merging `origin/main`** (which is the
merge that brought the `work/props/` rename into this tree): the
thirteen and their five files are unchanged, and the `work/props/`
destination is now the local tree's answer rather than the remote's.

**What the pattern could not match**: a citation that names a row by
id without its `work/<program>/` prefix; one that paraphrases the row
instead of naming it; a `work/<program>/` prefix for a program that
has not closed yet, which is the same defect with its fuse unlit
(`tools/` cites live programs too, and those citations rot the day
those programs close); and a dead path under a directory this grep did
not walk.

## The same class outside `tools/`, and where it belongs

`work/issues/dead-work-citations-from-shipped-code-and-docs` is the
class row — no single owner, spanning crates — and it already carries
the `work/docm/` family. Running its question over `crates/*/src` and
`crates/*/tests`, which **nobody had run**, adds five citations in
three further dead families; the evidence is appended there rather
than restated here. Two things that row should hear from this one: its
proposed gate is scoped to `crates/**/*.rs` and `crates/*/README.md`,
which would have caught **none** of the thirteen above, and
`tools/` is excluded from `Cargo.toml`'s workspace, so a gate written
as a workspace test does not see it either.

## Six of the thirteen closed by INSTR unit 12 (2026-09-16)

Unit 12's sweep reached the same class from `tools/k-lint`'s side and
repaired its own six in the PR that found them — the five in
`tools/k-lint/tests/predicate_roster.rs` (including the one **inside an
assertion string**) and the one in `tools/k-lint/src/lib.rs`. Re-counted
on that branch after the merge:

| file | citations left |
|---|---|
| `tools/tess-lint/tests/baseline_census.rs` | 5 |
| `tools/README.md` | 1 |
| `tools/tess-meter/src/lib.rs` | 1 |

**Seven live, across three files.** The eighth occurrence in
`baseline_census.rs` — *"in the tracker-wide cut of 2026-09-06"* — names
the directory as a FORMER home and is a record, not a pointer; it is
excluded here for the same reason this row excludes `work/instr/plan.md`.

**The argument unit 12 contributed, and it is the one that decides the
repair.** `docs/DOC-LEDGER.md`'s *A note on inbound references* settles
citations to **deleted** files: recover them at the SHA the ledger
names. These are not that. Every one of the seven names a row that
**moved and is open today**, so the ledger's recovery recipe yields a
*superseded snapshot* of a live row — worse than a dangling pointer,
because it resolves and lies. The repair is to re-point at where the
row lives now, and it is not uniform: the rows landed on `work/instr/`
and one on `work/props/`, and `D201` exists nowhere at all.

**`baseline_census.rs`'s two are the sharp case**, and unit 12 first
mis-filed them as historical prose before adopting this row's reading:
`work/meter/C15.md` and `work/meter/D201.md` sit inside the worked
example of the doctrine paragraph whose own sentence says *"the PATHS
are not [frozen] … and the pointers here followed them"*. They are live
pointers in the doctrine's own demonstration that pointers follow.

## One of the seven is gone (INSTR unit 1, 2026-09-16)

`baseline_census.rs`'s `work/meter/baseline-sizing-census-second-copy`
citation went with the paragraph that carried it: that paragraph said
`docs/TESS-BUDGET.md` *"still carries four of the sizing figures
asserted below in present-tense prose"*, which unit 1's fix made
false, so it was rewritten and the pointer had nothing left to point
at. **That is not the repair this row asks for** — the citation was
deleted as collateral, not re-pointed — and the other four in that
file (`work/meter/C15.md` x2, `work/meter/D201.md`,
`work/meter/tess-lint-ungated-columns-fold-silently`) are untouched,
including the two in the doctrine's own worked example. From this PR
on the count above reads `baseline_census.rs` 4, **six live across
three files**.
