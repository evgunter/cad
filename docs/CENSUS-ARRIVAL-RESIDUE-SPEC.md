# CENSUS-ARRIVAL-RESIDUE — the arrival alarm's four disclosed residues, three repaired and one measured (spec)

Binds one unit. Item:
`work/census/the-errors-arrival-blind-spot-list-claimed-exclusivity-and-was-short.md`.
Branch: `census/arrival-residue`. Deleted post-merge with a
`docs/DOC-LEDGER.md` entry citing the merge SHA.

Read the item first. It is the residue of CENSUS-ERRORS-ARRIVAL
(#2691, merged `17a68a0fa`), whose instrument you are about to work
inside: `ERRORS_MINTING_ITEMS` and `read_minting_items` in
`crates/pncad-py/src/tests.rs`.

## Why this row is next

`work/README.md`: *"disclosing a residue is not scheduling it."* Three
of the four residues are disclosed at the site, which is where a
reader of the code needs them, and **none is scheduled anywhere**. The
unit that wrote them closed with its spec deleted. The context is at
its warmest it will ever be — four probes have been run against this
reader in the last two hours and each is described — and almost all of
this row's value is that those probes exist. A lane arriving in a month
re-runs four experiments to get back to where the row starts.

## What I verified, and where it sharpens the row

Each of these I ran against `origin/main` at `370bd6f41`. Do not take
them on trust — the standing record of this program is that the
orchestrator's premises decay, and three of my last six specs carried
an error a lane found.

1. **The roster is TEN items and 53 literals**, not nine. The row said
   "nine" in two places and I corrected it in the same change that
   opens this unit. The tenth is `is_bare_camel_token`, one char
   literal, which appeared when unit 6's fix pass closed the
   char-literal hole. Re-derive it; do not copy my number.

2. **Every `held_by` name resolves.** Ten rows name nine distinct
   tests; `grep -c "fn <name>"` under `crates/pncad-py/src/` is 1 for
   each. So residue 4 is a **risk and not a defect today**, and any
   claim you make that it is live has to be executed.

3. **`impl_spans` is local to `crates/pncad-py/src/tests.rs`.** It is
   not in `test_utils::source`, so the blast radius of residue 3 is one
   file and no sibling census changes behaviour. I checked both
   siblings.

4. **Residue 3's repair has a worked precedent, and it is this
   program's own — for the second time.**
   `crates/test-utils/tests/hand_written_impl_census.rs` does not key on
   column 0: it scans freely with `code[from..].find("impl")` and
   guards each hit with `test_utils::source::{boundary_before,
   boundary_after}`, taking an `impl Trait` return position out through
   `item_body`'s `Declaration` arm and broken text through
   `Unterminated`. All three are already `pub` in that module and
   `tests.rs` already depends on it. **Unit 6 had this available and
   wrote a narrower line-start rule anyway** — exactly as it had
   dropped that census's `(path, trait, self type)` key and walked into
   the collision that census had already solved. That pattern, not the
   nesting, is the thing to read the repair against.

Where this sharpens the row: residue 3 reads as "teach the walk to
nest", which sounds like new machinery. It is not. It is taking a
mechanism that is already in the tree, already shared, already tested,
and already this program's.

## The disposition is yours on residue 1, and I am not taking it

Residues 2, 3 and 4 are work. **Residue 1 — a word that reaches Python
from `errors.rs` without being a literal there — is a design call I am
deliberately leaving open**, because the last two specs that pre-decided
one were overturned by a probe the lane ran, and the one that declined
(unit 6) got a disposition better than any bullet I could have written.

What I ask instead is a **measurement that decides it**: how many words
reach Python from this file today without being a literal in it? A map
forwarding `crate::tags`', a word built from a kernel `Display`. If the
answer is zero, "disclosed and not closed" may well be right and the
row already says so; if it is not zero, those words are Python-visible
vocabulary no instrument reads and the disclosure is a fig leaf. Take
the measurement first and let it choose.

If you close it, say what the closing instrument's own population is
and how a reader knows it is complete. If you leave it, say so at the
site in a sentence that names the count you measured and the date.

## The trap, named as this unit's specific growth direction

The standing trap is that **the fix mints a fresh instance of the
defect it closes**. It has sprung on all six units of this program.

This unit's version is unusually sharp and you should expect it:
**the row's subject IS a blind-spot list that claimed exclusivity and
was short.** It is the fourth consecutive one. The near-certain failure
mode is that you repair the reader, write a fresh exclusivity claim for
it, and the fifth is short too.

So, before you write any such claim:

- **State your list, then state the probe you will run against each
  entry, then state that probe's own blind spot — in that order, and
  in writing at the site, before running anything.** Unit 3's probe
  inherited the spec's fence and proved nothing; unit 4's verification
  used the same lowercase-anchored shape as the defect it was checking.
- **Make the probe differently shaped from the thing it checks.** If
  your reader walks source text, do not check it with a source-text
  walk of the same shape.
- A claim you have not executed is a claim you should not write. If a
  case is hard to execute, disclose it as unexecuted rather than
  asserting it.

## The silent-omission obligation

This unit changes a census, a source-text reader and an inventory, so
it carries the obligation, and the style reviewer will be told to look
for exactly this:

**An instrument that stops seeing part of its population reports
agreement over what it can still read, and an invariant row goes
green.** That is not a missing test; it is a passing test that means
nothing. Every change you make to `read_minting_items`, `impl_spans` or
the roster must be checked in the direction of *what does this reader
no longer see*, not only *what does it now see*. Mutation-prove it:
break the reader deliberately and show the guard reds.

The scan sets to watch are the reader's own. Widening the `impl` walk
can only add sites — but the guard on the walk can narrow, and the
comparison can go vacuous, which is how unit 6's quiet-file guard came
to drive only half of what its doc claimed.

## Acceptance

1. Residues 2, 3 and 4 each end in one of: a repair with a test that
   executes the case, or a written refusal at the site saying why not,
   with its own execution. No residue ends in a sentence alone.
2. Residue 1 ends in a **measured** count and a disposition chosen by
   it.
3. Every new or changed guard is mutation-proved in both directions —
   break it and show it reds; break something else and show it does
   not.
4. The roster's counts are re-derived, not copied, and each is stated
   with the command that re-derives it and the SHA it was taken at.
   This program has had a stale count born stale four times inside one
   unit (standing finding 12); do not make it five.
5. `crates/pncad-py/src/tests.rs` is 7795 lines and its size is an open
   row on this slate. **If this unit grows it, say by how much in the
   PR body and update that row in the same diff.** Unit 6 grew it 997
   lines and left the row saying 6317; a style review caught that and
   no instrument did. Read the row before you push.
6. Hosted CI green, `python3 scripts/work.py lint` clean.

## What this unit is NOT

- It is not a re-opening of unit 6's disposition. Literal-keying stands.
- It is not the rest of the crate. `payload-attribute-names-…` and
  `the-field-brace-fingerprint-…` are their own rows on this slate.
- It is not the file split. That is
  `pncad-py-tests-rs-is-six-thousand-lines-…`, and it is the file
  owner's call, not this unit's — but see acceptance 5.
- It is not a widening of `test_utils::source`'s grammar. If you find
  you want one, file it rather than taking it.

Territory: `crates/pncad-py/*` is LIB's fence and this program's
`keep_out` announces its pncad-py rows there.
`crates/test-utils/*` is TCOST's and TINT's; read from it, and if you
must change it, say so in the PR body and announce it.
