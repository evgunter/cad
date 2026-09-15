# TINT-2 — a stand-down that nobody can hear

**Unit of S-TINT.** Rows:
`work/tint/loud-stand-down-announcements-are-discarded-by-the-gate` and
`work/tint/loud-skip-marker-is-a-hand-kept-idiom`. They are one
mechanism seen twice and take one lane.

Branch: `tint/2-stand-down-channel`. Read
`docs/prompts/implementer-discipline.md` in full before you start; it
binds you alongside this spec. This spec is deleted at merge
(`docs/DOC-LEDGER.md`); the item files are the record that survives.

## The measured facts — do not re-derive these, they are this session's

1. **`test_utils::vacuity::stood_down` is a `println!`.** Its own rustdoc
   calls it *"the loud stand-down"*.
2. **nextest discards a passing test's stdout.** `--success-output`
   defaults to `never`; the twelve gating `test (…)` jobs pass no such
   flag, set no `NEXTEST_SUCCESS_OUTPUT`, and the repo has no
   `nextest.toml`. Measured under the pinned 0.9.140 with a one-test
   crate: the printed line does not appear.
3. **Exactly one job in the workflow passes `--success-output immediate`**
   — `cargo nextest run -p viewer --features app` — and its comment says
   it is there for the smoke row's adapter. No `stood_down` call site is
   in `crates/viewer/`.
4. **22 `stood_down` call sites in 10 files.** None under `viewer/`.
5. **Ten `#[cfg]`-gated marker rows**, not the title's eight, and
   **eight of the ten hand-keep an enumeration** in their `println!`
   body.

## The crisp finding, which decides the shape

**The marker rows have a working half and a broken half.** The `fn` NAME
reaches nextest's PASS list and IS read — that is the payload that
works. The `println!` BODY, which is the hand-kept enumeration, is
discarded on every gating run. So the eight hand-kept lists are not
merely stale-prone: **they have no readers at all** on the runs that
matter.

**`stood_down` has no working half.** It is called inside a passing
test, so no name reaches the PASS list and the entire payload is
discarded. It is 100% invisible on the gate.

Two of the ten markers have already gone stale exactly as predicted:
`crates/viewer/tests/error_display.rs` names ONE row and gates TWO (its
own rustdoc predicted this in so many words), and
`crates/sweep/tests/blend_margin_payload_interval.rs` names *"the
enclosure arm"*, singular, where three rows are gated.

## What this unit must NOT do

**Do not build a louder printer.** Routing more text into the same
discarded pipe is the shape this row exists to name. Adding
`--success-output` to the workflow is CIW's file and is at best the
cheap partial; it also prints every passing test's output, which is a
log-volume decision this unit does not get to take alone.

**And do not maintain an enumeration nobody can read.** An eight-copy
hand-kept list whose readers number zero cannot be justified by the
cost of keeping it in step. Where the enumeration cannot be made
mechanical, DELETING it is a better answer than re-writing it, and the
marker keeps the half that works — its name.

## The fork this spec does not settle, and must

Two shapes are available and the lane picks one with its reasoning in
the PR:

- **(A) Tally it.** Make a stand-down a fact the suite can floor rather
  than a line it prints — `vacuity::Exposure` already has that shape in
  the same module. Something then goes RED when the tally is wrong,
  which is this program's whole question. The cost is that a floor
  concedes the skip, and `C21` says whether the row should stand down at
  all comes first.
- **(B) Strip the unreadable payload.** The markers keep their names and
  drop their hand-kept row lists; `stood_down` keeps its call sites but
  stops pretending to announce. Cheap, honest, and buys no guard.

**(A) for `stood_down` and (B) for the markers is a legitimate answer**
and is this seat's lean — the marker's name already works, so only its
body is the problem, whereas `stood_down` has nothing that works and so
needs a mechanism or nothing.

## What the spec must state, and the lane must carry into the code

**TINT-1's lesson, learned the expensive way.** Its spec warned in a
whole paragraph against the fix minting a fresh instance of the defect,
and the first implementation did it anyway — hand-typed identifier
strings a rename could silently desync — and only an outside reader
caught it. **Naming a trap does not prevent it.**

So this unit states, in the code and in the PR body, **what its guard
does not enforce**, before anyone asks. If the answer is a tally, say
which wrong tallies go red and which do not. If the answer is deletion,
say plainly that no guard was added and what would have to exist for one.
A sentence claiming more than the mechanism delivers is the defect this
program is named for.

## Fences

- In: `crates/*/tests/**`, `crates/test-utils/**`.
- Out: `.github/workflows/*` (CIW's) — you may READ it as evidence and
  you may not change it. If the chosen fix needs a workflow change, that
  half is filed on CIW's slate and announced, not taken.
- Out: `scripts/**` (S-TCOST's), every `crates/*/src/**`.
- `crates/viewer/src/lib.rs` carries one of the ten markers and is
  `src/` — out of fence. File it, do not edit it.

## Verification

Hosted CI is the record: twelve `test (…)` jobs and five
`k-lint (gate, …)`. **Prove the guard bites** — if a tally lands, plant
a wrong tally and show it red; if deletion lands, there is nothing to
mutate and the PR says so rather than implying a guard.

## Review

One style review by path, no A/B row. Its first question is the one
TINT-1's review had to ask: does this fix mint a fresh instance of what
it closes?
