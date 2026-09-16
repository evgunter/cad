---
id: tools-cite-moved-work-meter-rows-not-deleted-ones
kind: issue
title: five tools/ and docs/ citations name work/meter/ rows that MOVED and are still open, which the ledger's deleted-file note does not cover
status: open
opened: 2026-09-16
---



## What

`docs/DOC-LEDGER.md`'s *A note on inbound references* (`:72`) rules
that citations of deleted files are **not broken**: *"the filename plus
this ledger's recovery recipe resolves any of them. No file was deleted
that a live pointer depends on for its content."* METER's sweep leans
on it explicitly — *"prose citations of `work/meter/…` and of the walk
survive across the tree unrewritten, which is what A note on inbound
references above is for"* (`:1924`).

## Finding

**That note covers rows METER CLOSED. Five of the surviving citations
name rows that were MOVED and are open today**, and for those the note's
argument does not hold: the recovery recipe resolves the path to a
snapshot at the sweep SHA, not to the live row the reader is being sent
to read. A reader who follows one gets a superseded copy of an open
item and no indication that it moved.

Citations at `89c8766`, from
`grep -rn "work/meter/" --include='*.rs' --include='*.md' . | grep -v '^./work/'`:

| site | names | lives at | status |
| --- | --- | --- | --- |
| `tools/tess-meter/src/lib.rs:1241` | `tess-meter-sampled-retune-figure-unreproducible` | `work/instr/` | open |
| `tools/README.md:218` | `tess-lint-zero-certificate-two-meanings` | `work/instr/` | open |
| `tools/tess-lint/tests/baseline_census.rs:21` | `baseline-sizing-census-second-copy` | `work/instr/` | open |
| `tools/tess-lint/tests/baseline_census.rs:147` | `C15.md` | `work/instr/C15.md` | open |
| `tools/tess-lint/tests/baseline_census.rs:248` | `tess-lint-ungated-columns-fold-silently` | `work/instr/` | open |

**Three further hits are correctly left alone and are listed so a
sweeper does not re-fix them.** `baseline_census.rs:84` and `:85` name
`C15.md` and `D201.md` inside a historical list of what pointed where
when the fold happened — prose about the past, which the ledger's note
is exactly right about. `baseline_census.rs:96` names the directory
`work/meter/` as the place a cut moved things OUT of, which is a
statement about the move itself. `docs/k-report-data/README.md:35` is
a sixth live-pointer case and is carried by
`k-report-m11-reading-is-stale-and-unwatched`, filed with it, because
the sentence around it is stale for a second reason.

**The repair is one line per site**, and the cheap half of the check is
that every target still exists: `work/instr/<name>.md` for all five.

**Confidence:** sure of the five paths and of where each row lives —
each was resolved against `work/instr/`'s current listing. Sure the
ledger's note is about deleted files, from its own wording. Unsure
whether the distinction was considered and dismissed at METER's sweep:
`sweep-deleting-work-meter-dangles-six-refs-on-instr-rows` closed at
that SHA and its body is only recoverable at
`2723839067e80198bec2889d041e490000f02275`, which this row did not read.

**Sweep and its blind spot.** The pattern is the literal prefix
`work/meter/` over `*.rs`, `*.md`, `*.sh`, `*.py`, `*.yml` and `*.toml`
outside `work/` and outside `docs/DOC-LEDGER.md`. It cannot match a
citation that names a METER row by BARE ID with no directory — the
common spelling in item bodies — nor one that names the row by its
title. Neither of those is a dangling PATH, which is what this row is
about, but a sweeper who wants every stale pointer to a moved row will
need a second pattern over the ids themselves.

## Was

Found by INSTR unit 12, whose own item is one of the moved rows: the
unit repaired the three `work/meter/` citations inside its fence
(`tools/k-lint/src/lib.rs`, `tools/k-lint/tests/predicate_roster.rs`,
six occurrences of three ids) and filed the rest.
