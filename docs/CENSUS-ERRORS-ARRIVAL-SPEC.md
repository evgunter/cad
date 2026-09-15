# CENSUS-ERRORS-ARRIVAL — nothing enumerates `errors.rs`, and a row saying so did not stop the fifth map (spec)

**Program:** CENSUS (`work/census/plan.md`). **Item:**
`work/census/errors-rs-holds-four-python-visible-maps-and-nothing-enumerates-that-file.md`
(the id says `four`; the file holds five — read it as a name). **Class
at the cut:** E. **Track:** no A/B protocol, no ordinal. **One style
review**, carrying the silent-omission obligation — this unit's whole
subject is an instrument's reach, so it carries it by the trigger
`plan.md` §Review posture states. No correctness lane.

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
item whole, including its two closing observations; `work/census/plan.md`
§Charter; **`work/census/log.md`'s standing findings**, and especially
the CENSUS-PY-RAISE-LITERALS entry.

**Read the reader you are reasoning about**: `crates/pncad-py/src/tests.rs`'s
`TopForm`, `ArmShape` and `read_tag_table`, plus
`the_tag_table_reader_recognises_every_form_it_claims`. You cannot
disposition this row without knowing what that grammar admits.

## Why this row is next

It **predicted its own next instance in writing and did not prevent
it.** CENSUS-PY-RAISE-LITERALS added `ValidationRefusal::attribute` —
two Python-visible words — with no pin, no sentence saying a fifth map
had arrived, and this row untouched in the same PR. A style reviewer
found it; no instrument did.

That is the sharpest thing this program has learned: **nothing reads a
row at the moment a lane writes code.** The row is right that the fix
is an instrument, and the unit is worth running for that reason rather
than for the five maps, none of which is unguarded today.

## What I verified, and where it sharpens the row

Measured 2026-09-15. **Re-take it** — this program's specs have been
wrong on five units out of five, and twice the orchestrator's own check
was the weak link.

- **Five maps, and the row's shape claim holds exactly**: `dimension_tag`,
  `measurement_dimension_tag` and `canonical_unit` are **top-level**;
  `ErrorClass::class_name` and `ValidationRefusal::attribute` are
  **inside `impl` blocks**. A reader keyed on top-level items sees three
  and misses two.
- **All five are `pub const fn`** — the row does not say this, and it
  matters: the tag reader's `TagFn` and `OptionTagFn` forms strip
  **`"pub fn "`**, so none of the five is a tag-function *form* to that
  grammar. Relocating one into `tags.rs` verbatim would not work either.
- **`tags.rs` holds zero `pub const fn`**, so the reader has never met
  one — and `TopForm::Const` matches on `"pub const "`, which is a
  prefix of `"pub const fn "`. **Check what the reader actually does
  with such a line.** If it fails loud, say so and move on; if it
  mis-parses or skips, that is a finding about the reader this program
  already owns, and it is filed rather than fixed here.

## The disposition is yours, and I am deliberately not taking it

The row names two shapes and there is a third. **I am not pre-deciding
between them**, and the reason is on the record: three of my last four
specs pre-decided a disposition and two of those were overturned by a
probe the lane ran. The evidence is in the code and the grammar, not in
this page.

- **A second, looser reader over `errors.rs`** — an arrival alarm, not
  a word inventory. It must walk `impl` bodies, which is most of what
  made the tag reader hard, and it is itself a hand-maintained thing
  needing a guard.
- **Reduce the population** — mint `Measurement.dimension` by
  capitalising `dimension_tag`'s word at the boundary, and that map
  stops existing. Changes no word's value. **Note what it does and does
  not do**: it takes the file from five maps to four and does not close
  arrival at all.
- **Make arrival impossible rather than detectable** — a rule that a
  Python-visible word comes from `tags.rs` only. This runs straight into
  the `pub const fn` fact above and into the two inherent methods, which
  cannot move without changing their call shape.

Whichever you take, the acceptance below asks the same two things of it:
**execute the arrival**, and **say what your choice does not close.**

## The trap, named as this unit's specific growth direction

If you build a reader, you are adding a hand-maintained instrument to a
program whose charter is hand-maintained lists, in the file whose row
says exactly that. Both previous instruments in this crate needed a
guard of their own within one unit of landing —
`the_tag_table_reader_recognises_every_form_it_claims` exists because a
reader branch went unexercised, and it was itself a hand census until a
reviewer said so.

**So before you write the guard's own guard, say what would make it go
blind**, and run that. A reader that reports agreement over a population
it stopped seeing is this program's defect, and you would be shipping a
new one.

## Points 5, 6 and 10, addressed to you

**Point 5 — a probe inherits the spec's fence.** Name the unit of
guarding your probe tests (a file, a form, a map, a word) and say
whether that is the unit the defect lives at, before you run it.

**Point 6 — a sweep shaped like the defect finds that defect again.**
Six sweeps have now run on this crate. If you sweep, shape yours
differently from all six and say how first.

**Point 10 — "verified the example, asserted the class."** The
orchestrator checked one instance and carried a row's generalisation
over two others that did not hold. **Any claim you make about "every
map in this file" must be checked over every map**, not over the ones
that prove the point.

## Acceptance

1. A disposition, with its argument, and **what it does not close**,
   said at the site in those terms.
2. **The arrival executed**: add a sixth Python-visible `-> &'static str`
   map to `errors.rs`, in the position your disposition is weakest
   against (an inherent method inside an `impl`, if you built a reader),
   and report what reds. If nothing does, say so plainly — that is the
   finding.
3. If an instrument lands: what makes it go blind, executed, and its
   own guard or a written reason it can have none.
4. The item corrected — including its `four`/five naming, if your work
   changes the count.
5. **No Python-visible word changes value.** `pncad.pyi` and the
   832-test Python suite are the contract.
6. Hosted CI green; `pncad-py` is the wheel, so the python suite runs.

**On `gate ok`**: it may false-red on an API-lag race that is not yours
— every other job green, the gate naming a `k-lint` row `in_progress`.
That is `work/ciw/gate-ok-has-no-expected-job-roster.md`, recorded
twice. Do not chase it, do not touch `ci.yml`, never push an empty
commit; report it.

## What this unit is NOT

Not a re-siting of `BoundaryEdit`, `UnmirroredSelect` or `StlRefusal`
(the item's first closing observation) and not a widening of the tag
reader's grammar to admit an `enum` (its second) — both are recorded on
that row as evidence and both are larger than this unit. Not a change to
any word's text. `crates/pncad-py/*` is LIB's territory and this
program's `keep_out` announces its pncad-py rows there.
