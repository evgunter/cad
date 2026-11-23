---
id: is-a-row-a-joins-census-mints-among-the-values-coincidences
kind: issue
title: A record a join's census mints cites a row that is not among the value's coincidences: is it one of them?
status: open
opened: 2026-11-23
priority: P1
cost: M
---


Left undecided by PR 7124 (`embed/records`, 2026-11-23) on the EMBED
orchestrator's word; both reviewers of that PR stated the two answers.

**What happens today.** A `⊔`'s gate runs the census between its
arrivals (`finish_assembled`, `census::record_at_rest`). A touch it
records cites `Backing::Decided(k)`, where `k` numbers the census's own
rows after the caller's `first_row` (`Assembled::numbering_from`, one
caller: `crates/editor-core/src/eval/wire/distribute.rs`, which numbers
them after its pieces' rows so they name none of the pieces'). The rows
come back beside the body (`Censused::rows`). They are not appended to
the value's `coincidences`, so at the value such a record cites a row
number nothing publishes. `Backing::Decided`'s own doc says it names
"row `.0` of the coincidences of the operation whose result carries
the record".

**Yes, it is one of them.** The value's `coincidences` gains the
census's rows. A minted record's citation resolves at the value the
way a declared one's does. Every reader of `coincidences` (the
listing, the Python surface, a taker downstream) sees rows no author
wrote and no Boolean decided.

**No.** `coincidences` stays the rows the node's own evaluation
decided. A minted record's citation names a row of the door's census,
which only the door's caller holds, and `Backing::Decided`'s doc is
false for it: it wants a third backing, or the citation resolved
before the value is built.

**The hazard one reviewer found**, checked on this head: a reader that
indexes `coincidences` by a record's `Decided(k)` without a bounds
check would panic or read the wrong row on a census record. Every
reader that indexes by `Decided(k)` today:

- `topo::boolean::ops` (`cite_rows`): the Boolean's own records against
  its own decisions; out of range is `BooleanError::JoinDesync`.
- `topo::boolean::ops`, the vertex classes' margins:
  `coincidences.get(k)`, and a miss is skipped.
- `editor-core` `eval/wire.rs`, the union fold, twice, and
  `eval/wire/distribute.rs`, once: each maps a Boolean step's own
  records onto that step's own rows and is `unreachable!` on a miss.

None of them is handed a census record's `Decided`: a `⊔`'s records
enter a later Boolean as that operand's own and leave it as
`Backing::Carried`, and the fold starts from
`records().carried_from(0)`. So no reader resolves a census record
against `coincidences` today, rightly or wrongly. The three
`unreachable!`s were left as they are: a typed refusal there is a new
`NodeErrorKind`, which is this question's answer to shape.

**A citation that names nothing is constructible** (PR 7124's review,
2026-11-23), and belongs to this question: who holds the rows a
`Decided(k)` indexes. `crates/topo/README.md` says of a record's
citation: "A record cannot be built without one (D1 (ii))". What the
type admits: `Cites` is never empty, and `Cites::decided(0)` builds
for any caller, whether or not a row 0 exists and whatever it decided;
`Built::recording(...)` then hands the record to a gate. The
reviewers' probes cite `Decided(0)` on bodies with no row at all. No
gate reads the citation (a record is certified by the census's reading
of it, never by what it cites), so nothing refuses it and nothing is
misled by it today; a reader that resolved it would be. Not built
here.
