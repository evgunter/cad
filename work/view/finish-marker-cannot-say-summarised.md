---
id: finish-marker-cannot-say-summarised
kind: issue
title: finish and finish_non_exhaustive are a two-valued marker being asked to carry a three-valued distinction: printed, summarised, not carried
status: closed
opened: 2026-09-06
refs: [2093, debug-walk-inlines-an-unbounded-report-while-summarising-a-bool]
closed: 2026-09-08
branch: view/summarised
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
(`crates/viewer/src/session.rs:329`) on the local rule *no `_` arms*
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

## Closed — the distinction moves to the FIELD, and the marker is left alone (2026-09-08, Ev's ruling)

None of the three candidates above. The tree already held the right
mechanism in one of the four walks: `LandedRun` summarises `checks`
through a `format_args!` that renders as something obviously a
summary, while the other summarised fields rendered as `is_some()`, a
`len()` or a mapped generation — and `false` is indistinguishable from
a real `bool` field. So every summarised field now renders through a
`format_args!` naming what it stands for, and `std`'s markers are
untouched: `Derived` still `finish`es, the other three still
`finish_non_exhaustive`. The marker answers *are all fields shown?*;
the question a reader has at a summarised field is *is this value the
whole field?*, and that is answerable only at the field. Asking a
two-valued marker to carry a three-valued distinction is the category
error this item was circling.

| field | was | is |
|---|---|---|
| `Derived::scratch` | `scratch: false` | `scratch: Some(<Doc>)` |
| `LandedRun::body` | `body: true` | `body: Some(<Body>)` |
| `DocSession::gesture` | `gesture: true` | `gesture: Some(<Gesture>)` |
| `DocSession::resolver` | `resolver: false` | `resolver: Some(<DirResolver>)` |
| `PickCache::index` | `index: Some(Generation(1))` | `index: Some(<PickIndex for Generation(1)>)` |
| `DocSession::states` | `states: 1` | unchanged — a count reads as a count |
| `LandedRun::checks` | `checks: 0 finding(s), 0 skipped` | unchanged — this is the precedent |

An absent field still renders `None`, which IS the whole field. No key
was renamed: `states` still dumps `history`, and whether that should
be fixed is not this item's.

**Nothing in the tree rendered these dumps**, proved by deleting all
four impls: the workspace builds `--all-targets` under both of
`viewer`'s feature configurations and its one doctest is unaffected,
so no `{:?}`, no `#[derive(Debug)]` over these types and no `T: Debug`
bound reached them. `crates/viewer/tests/debug_dumps.rs` is now the
only reader, and holds the seven summarised fields to their spellings.
`crates/viewer/README.md`'s "A carried field may be summarised"
paragraph carries the rule, the sweep that produces its list, and the
reason a future field can have no mechanical guard.

### The citation sweep this change owed

The change moves lines in `session.rs`, `pickcache.rs` and
`crates/viewer/README.md`, so every citation into those three files in
a live `work/view/` row was re-derived by re-finding its SUBJECT at
head — never shifted by a delta. **The enumeration rule**: over every
`work/view/*.md` except `log.md` whose `status:` was not `closed` at
`92b2c303d`, match `(session\.rs|pickcache\.rs|viewer/README\.md):[0-9]+`;
then `sed -n Np` each hit at the merge base and at head and read
whether the named subject is there. **57 tokens in 18 rows.** A second
line named only as a continuation (`session.rs:1213,1590`, `the
require_kind at :1200`, ``LandedRun`'s is at `:421-448` ``) is not a
token and travels with the citation it hangs off; three of those were
re-derived too. What the rule cannot see, and this sweep's blind spot:
a citation written any other way — prose, a path with no line, a range
inside a code block, or a file this change does not move.

**21 tokens in 4 rows are records, not pointers, and are left as
written**: `stale-file-citations-after-the-split`'s Was/Now tables
(re-pointed against `d799235e`, and dated),
`sweep-blind-spots-the-precheck-sweep-could-not-see`'s hits (the file
says in bold that its sweep is accurate as of #1846's merge base and
nothing re-runs it),
`the-citation-receipts-summary-numbers-are-not-re-derivable`'s (read
on `bc44531e1`), and
`viewer-preview-names-a-verb-by-its-variant-identifier`'s, which
quotes pre-split paths AS pre-split paths. Re-pointing a dated receipt
falsifies the record it is. A closed row is the same case: this pass
touched none.

**The other 36 tokens, in 14 rows, were read line by line.** 29 were
re-pointed and 7 are still true as written. Of the 29, **three were
correct at `92b2c303d` and this change falsified them** — this row's
own `session.rs:325` (`.finish()`),
`viewerapp-document-derived-state-has-no-boundary`'s `:1583-1586`
(`clear_for_new_document`) and
`a-module-named-for-its-spine-type-is-unfalsifiable`'s
`README.md:787-797` (the ring held open on purpose) — and **the other
26 were already wrong at that merge base**, most by hundreds of lines
(`session.rs:613-655` for `DocSession::land`, which is `:894-988`;
`:1319` for `clear_for_new_document`, which is `:1591`). The rows
re-pointed, besides the three above:
`adjacent-same-typed-arguments-are-the-same-swap`,
`display-clear-drops-free-move-placements-silently-while-prune-reports-them`,
`evalservice-coalescing-rule-is-prose-no-implementor-is-held-to`,
`four-debug-walks-are-spelled-and-placed-two-ways`,
`free-move-drag-dissolved-by-open`,
`gesture-drags-have-no-cancel-door`,
`outstanding-and-progress-are-two-three-state-enums-one-hop-apart`,
`readme-and-type-docs-restate-one-argument-for-landing-and-landedrun`,
`revolve-tool-unreachable-no-axisinplane-form`,
`two-hand-written-copies-of-the-g1-gesture-machine`,
`ui-thread-work-after-the-index-seam`.

Citations into these files from OUTSIDE `work/view/` were read too —
`work/chrome/` (5), `work/docm/` (3), `work/fix/` (4) and
`docs/DOCM-IDENTITY-DESIGN.md` (4) — 16 tokens, of which eight name a
line past `session.rs`'s last and the other eight resolve to something
that is not their subject. Every one was already stale at
`92b2c303d`, so this change falsified none of them and none is a §6
report of this lane's making; they are the class
`stale-file-citations-after-the-split` already carries.
