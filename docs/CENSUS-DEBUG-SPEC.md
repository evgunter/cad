# CENSUS-DEBUG — a hand-listed `Debug` under a completeness claim (spec)

**Program:** CENSUS (`work/census/plan.md`). **Item:**
`work/census/hand-listed-debug-censuses-in-geom-core-geom-and-topo.md`.
**Class at the cut:** M. **Track:** no A/B protocol, no ordinal.
**One style review**, carrying the silent-omission obligation
(`plan.md` §Review posture); no correctness lane — this is not one of
the H rows.

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
item whole; `work/census/plan.md` §Charter's trap paragraph;
`work/census/log.md`'s CENSUS-INERT-DENY entries — the first unit's
record, including what it got wrong, and the paragraph headed *what this
unit teaches the next one*, which is about YOUR instrument; PR 2093's
diff inside `crates/viewer/` (the proven pattern).

## The finding

A `Debug` impl that names its value's fields by hand has no compile-time
tie to the declaration: a field added to the struct is silently absent
from every dump. These are the worse version, because each ends in
`finish()` rather than `finish_non_exhaustive()` — **the rendering claims
to show every field while listing them by hand.**

#2093 closed this inside `crates/viewer/` by destructuring `Self`
exhaustively in the impl, so a new field is an `E0027` unbound-pattern
error, and binding a field the dump will not carry to `_` — a decision a
reader can see and the compiler still forces.

## The premise correction, which IS this row's thesis

Measured 2026-09-15. **Take it as a hypothesis and re-take it** — the
first unit's spec was wrong in three places and the lane found all three.

The item, written 2026-09-06, says *"eight rows, nine concrete impls,
seven of the eight rows in the class."* Running its own enumeration rule
today — `grep -rnE "impl[^=]*\bDebug\b for" crates/`, minus
`crates/viewer/` — gives **14**, and four of the six new ones are
squarely in the class:

| site | value | shape |
| --- | --- | --- |
| `crates/mesh/src/memo.rs` | `PatchMemo` | `debug_struct` … `finish()`, 4 fields by hand |
| `crates/editor-core/src/resolve/pick.rs` | `PickMemo` | `debug_struct` … `finish()`, 6 fields by hand |
| `crates/editor-core/src/names/table.rs` | `NameTable` | `debug_struct` … `finish()`, 2 fields by hand |
| `crates/geom-core/src/sym/memo.rs` | `DriveMemo` | `debug_struct` … `finish()`, 2 fields by hand |

The other two are **not** the class and are listed so you do not fix
them: `topo/src/props.rs`'s `SignCertificate` uses `write!`, and
`editor-core/src/names/role.rs`'s `NameRef` delegates to `self.0.name`.
`topo/src/param_source.rs` is the one the item already corrected #2093
about, on the same grounds.

**So the class grew by four in nine days and nothing observed it.** That
is the row's own thesis arriving as evidence about the row, and it is the
argument for whatever you leave behind: the item's hit list was accurate
when written and is now short by a third, because a hand-written list of
hand-written lists decays at the same rate as its subject.

## What to do

**Half A — the destructure.** Every impl in the class destructures `Self`
exhaustively, so a new field is `E0027`. A field the dump deliberately
will not carry binds to `_`. Do not change what any dump prints unless
you find a field genuinely missing — if you do, that is a finding, and
say so rather than quietly widening the output.

Each of these is a borrow-carrying token whose doc explains that a
derived `Debug` would follow the reference and dump a whole knot vector
or control net. **That reason is good and is not at issue** — it is why
they are written out, and the same reason applied to the four walks
#2093 rewrote. What is missing is only the destructure that makes the
hand-listing safe.

**Half B — `PartialEq`, and it is IN SCOPE.** The item names it unswept:
`hull.rs` (four), `curves/nurbs.rs`, `surfaces/nurbs.rs`, written field
by field beside each `Debug` on the same address-equality argument.

The scope call is mine and I am taking it, because the item states the
reason itself: **a `Debug` that misses a field misleads a reader; an
`Eq` that misses one answers wrong.** These are the same impls on the
same types in the same files, and the repair is the same destructure.
Landing the `Debug` half and leaving the `Eq` half hand-listed is the
half-fix `docs/prompts/reviewer-style-lane.md` names under the
class-not-instance rule.

**Bounded:** `PartialEq` for the types whose `Debug` you touch. There are
19 such impls outside `viewer` tree-wide; the ones beyond your types are
**not yours to sweep** — check them, and file what you find rather than
widening this unit.

**Half C — the instrument, and read this before you build one.**

The first unit left a source-text reader with a hand-maintained tally,
and that was right *there* because its population was not compiler-known:
no type, module or manifest enumerates the occurrences of an attribute.
**Your population is different and the difference decides the design.**

- **The per-field question is compiler-known.** Once an impl destructures
  exhaustively, a new field is `E0027` at build time. That is a stronger
  guard than any census, it needs no reader, and it cannot go blind.
  **Half A is itself the instrument for this half** — do not add a
  census that re-asks a question the compiler already answers.
- **The arrival question is not.** *"Did a new hand-listed `Debug` impl
  land without a destructure?"* is a property of source text, and the
  four impls above are the proof that nothing asks it.

So the only guard worth building is the arrival guard, and it is a
reader. **Reaching for the first unit's shape without noticing this
split would be the charter's trap running the other way** — a
hand-maintained reader standing in for a check the language performs.

If you build the arrival reader: it goes through `test_utils::source`
and owes its line in `crates/test-utils/tests/reader_census.rs`; it
needs a row that reds when it stops SEEING, not only when it disagrees;
and it must say what it cannot see. Check first whether #2093 left
anything for `viewer` you should extend rather than duplicate — a second
reader asking one question is this program's class.

**You may also conclude the arrival guard is not worth its cost**, and
that is a real answer if you argue it at the site: say what would detect
the next arrival instead, and file the gap. What is NOT acceptable is
shipping the twelve destructures with nothing said about arrival, which
leaves the row's own decay mechanism running.

## Fences to announce

CENSUS claims no paths. Run `python3 scripts/work.py territory --files -`
over your diff and name every crossing in the PR body. Known:
`crates/geom-core/src/spline/*` and `crates/geom/src/*` are PROPS'
(the item is announced there); `crates/topo/*`, `crates/mesh/*`,
`crates/editor-core/*` and `crates/geom-core/src/sym/*` reach other
programs — `sym/memo.rs` is SYM's and is new to this row.
`crates/test-utils/*` is TCOST's and TINT's if a reader lands.

## Acceptance

1. Every in-class impl destructures exhaustively; the hit list is in the
   PR body, one line per site, as a receipt. Re-take the enumeration —
   it moved once already.
2. The `PartialEq` siblings of those same types likewise; the ones beyond
   them triaged and filed, not swept.
3. Mutation-verified: add a field to one of these types and show `E0027`
   red, then restore. Both directions, shown.
4. The arrival question answered — a guard with its blind spots at the
   site, or a reasoned refusal plus a filed row.
5. No dump's output changes silently. If one does, say which and why.
6. The item corrected: the count, and the four impls it does not list.
7. Hosted CI green — twelve `test (…)`, five `k-lint (gate, …)`.
   `editor-core`, `geom-core`, `geom`, `topo` and `mesh` are all in the
   wheel's closure, so the python suite runs.

**A note on `gate ok`.** It may false-red on an API-lag race that is not
yours: the tell is every other job green and the gate naming a `k-lint`
row as `in_progress`. That is
`work/ciw/gate-ok-has-no-expected-job-roster.md`, twice recorded. Do not
chase it, do not fix `ci.yml`, and do not push an empty commit — report
it and let the orchestrator handle it.

## What this unit is NOT

Not a conversion of these impls to `derive(Debug)` — the borrow-carrying
argument stands. Not a change to what any dump prints. Not a sweep of
`PartialEq` beyond the types in Half A. Not a `Display` or prose-rendering
change: `NameRef`'s delegating `Debug` is the prose-census rows' subject,
not yours — if you notice more of that shape, add it to
`work/census/prose-census-cannot-see-a-bypassed-prose-renderer.md` as
evidence rather than acting on it.
