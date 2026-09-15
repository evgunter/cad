# CENSUS-TAG-REACH — a refusal word minted at a call site, where the gate cannot see it (spec)

**Program:** CENSUS (`work/census/plan.md`). **Item:**
`work/census/pncad-py-eval-err-variants-outside-the-tag-inventory.md`.
**Class at the cut:** E. **Track:** no A/B protocol, no ordinal. **One
style review**, carrying the silent-omission obligation (`plan.md`
§Review posture); no correctness lane.

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
item whole; `work/census/plan.md` §Charter's trap paragraph; and
**`work/census/log.md`'s section "Two units in, and what the program has
learned about itself"** — its four numbered points are about the unit you
are running, not about history.

## The row has decayed, and the correction changes what this unit is

Measured 2026-09-15. **Re-take all of it** — the program is two for two
on specs whose premises did not survive the lane.

The item, written 2026-09-04, says eight `eval_err` call sites mint
**four** words as string literals, and that the fourth,
`measure_unavailable`, *"is named in exactly one place in the
repository"* — the call site that mints it, `value.rs:721`. That is the
row's sharpest point and the reason it was filed.

**It is closed.** `crates/pncad-py/src/py/value.rs` now routes
`ValuePayload::MeasureUnavailable` through
`super::measure::measure_unavailable_at_err`
(`crates/pncad-py/src/py/measure.rs`), a typed door whose `variant`
field reads `measure_unavailable_at_tag(reason)` — a **`tags.rs`**
function, which is exactly what the inventory lexes. The word now
appears 19 times across the tree and is pinned in `src/tests.rs`.
`value.rs:721` is today `Datum::__repr__`.

Nobody closed the row when the code closed the instance, and the row
still reads as though a Python-visible word sits ungated. **That is the
decay direction that inflates a board** — a slate row that looks like a
leak repair and is actually a design question.

**What actually remains**, measured: **seven** literal sites in
`value.rs` minting **three** words —

| word | sites |
| --- | --- |
| `wrong_kind` | `:884`, `:920`, `:995`, `:1034`, `:1047` |
| `empty_boolean` | `:878` |
| `unknown_node` | `:1171` |

All three are covered by accident (pinned in `src/tests.rs`, named in
`pncad.pyi`). **There is no uncovered word today**, so this unit is a
decision about the gate's REACH and not a leak repair. It is still worth
running: the next literal added at an `eval_err` call site is public
Python vocabulary that no inventory looks at, and that is the row's
standing lesson.

## The disposition — mine, and it is not either of the row's two options

The item offers two closes: widen the reader to lex the literal sites
into their own inventory row, or rule a call-site literal deliberately
out of scope and pin the uncovered word. **Take neither.**

`value.rs:1164` — inside the same function as two of the literals —
already passes `NODE_NOT_EVALUATED`, a `pub const` imported from
`crate::tags`. And the gate already reaches consts: `src/tests.rs`
carries `TAG_CONSTS`, *"the committed inventory of `src/tags.rs`'s
`pub const` tag words"*, the reader parses
`pub const NAME: &str = "value";` alongside `pub fn NAME(..) ->
&'static str {`, and a new one reds as `NEW `pub const` tag word`.

**So the third option is to move the word, not the reader**: each of the
three becomes a `pub const` in `tags.rs` (or a `pub fn` where a kernel
value keys it), imported and passed at the call site exactly as
`NODE_NOT_EVALUATED` is. Why this over the row's two:

- **It adds no reader.** Widening the lexer to scan `src/py/` means a
  new source-text reader, and a reader needs a guard of its own — this
  program's standing trap, which both previous units sprang. Option 1
  buys the defect it is meant to close.
- **It does not document a second channel as permanent.** Option 2
  leaves call-site literals outside the inventory forever, by decision.
- **The machinery exists, gates today, and has a working example in the
  same function.** Nothing is invented.

If you find this wrong — if a word genuinely cannot be a const because
its value is a function of something — say so with the evidence and
propose the shape that fits. The call is mine and the correction is
yours if I have it wrong.

## The trap, named as a prediction rather than a warning

`plan.md` §Charter says the fix must not mint a fresh instance of the
defect it closes, and this program's log now records that **naming it
has not once prevented it**: unit 1 hand-spelled a shared predicate one
line after calling its other half; unit 2 keyed a suppression list so it
could grow silently, three lines below a doc explaining why it must not.
Both were caught only by a reader who did not write the fix.

**This unit's specific growth direction**: `tags.rs` is a hand-written
file of words, and you are adding to it. The question to answer by
execution, not by reasoning, is *what stops the NEXT refusal word from
being minted at a call site instead of here?* Today the answer is
"nothing, and the inventory cannot see it" — which is the row. If your
change leaves that answer unchanged for the next word, you have moved
three literals and closed no class.

**Probe it**: add a fresh literal-variant `eval_err` call site, run the
suite, and report what reds. If nothing does, say so plainly — that is a
finding about the fix, and it is better said by you than by the reviewer.

## Acceptance

1. The three words are sourced from `tags.rs` at all seven sites; the
   hit list is in the PR body, one line per site.
2. The inventory (`TAG_INVENTORY` / `TAG_CONSTS`) carries them, re-derived
   by the guard rather than hand-added — show the guard reding before and
   passing after.
3. The arrival probe above, executed, with its result reported whichever
   way it goes.
4. `crates/pncad-py/src/tests.rs`'s stated scope (*"the shared Rust-source
   lexer: `src/tags.rs` is READ by the tag-table guard"*) still says
   something true afterwards; correct it if the change moves it.
5. The item corrected: `measure_unavailable`'s closure, the 7/3 count,
   and the class estimate if you find E wrong.
6. No Python-visible word changes value. This is about where a word is
   declared, not what it says. If a word's text would change, stop and
   report — `pncad.pyi` and the python suite are the contract.
7. Hosted CI green. `pncad-py` is the wheel, so the python suite runs.

**A note on `gate ok`**: it may false-red on an API-lag race that is not
yours — the tell is every other job green and the gate naming a `k-lint`
row as `in_progress`. That is
`work/ciw/gate-ok-has-no-expected-job-roster.md`, twice recorded. Do not
chase it, do not touch `ci.yml`, do not push an empty commit; report it.

## What this unit is NOT

Not a change to any refusal's text or to `pncad.pyi`'s surface. Not a
widening of the tag-table reader. Not a sweep of `eval_err`'s message
strings — the `message` argument is prose for a human and is not the
variant. `crates/pncad-py/*` is LIB's territory and this program's
`keep_out` announces its pncad-py rows there; say so in the PR body.
