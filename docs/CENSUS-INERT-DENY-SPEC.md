# CENSUS-INERT-DENY — `#[serde(deny_unknown_fields)]` where there is no field to deny (spec)

**Program:** CENSUS (`work/census/plan.md`). **Item:**
`work/census/inert-deny-unknown-fields-on-unit-enums.md`. **Class at
the cut:** M. **Track:** no A/B protocol, no ordinal drawn. **One style
review**, carrying the silent-omission obligation `plan.md` §Review
posture states, and no correctness lane — this is not one of the two
hardest rows.

This is the program's first unit and it is ordered first on purpose:
the population is decided mechanically, there is no design call in the
sweep itself, and it is where the lane builds the habit the charter
turns on — **leave an instrument behind, and do not let the instrument
become a fresh instance of the defect.**

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
item file whole; `work/census/plan.md` (§Charter's trap paragraph and
§Review posture); `crates/test-utils/tests/reader_census.rs`'s header
and `crates/test-utils/src/source.rs`'s header (the ratified home for
anything that reads Rust as text, and the ledger that forces a
disposition on a new reader); `scripts/gates/kernel-serde-free.sh` and
`scripts/gates/viewer-vocab-declared-once.sh` (two precedents for a
source-text gate, and what each does about its own reader).

## The finding

An externally-tagged enum rejects an unknown VARIANT unconditionally;
`#[serde(deny_unknown_fields)]` governs unknown **named fields**. Where
a declaration has no named field anywhere, the attribute compiles,
reads as a guarantee, and does nothing. BLEND-5's review established
this by execution — removing it from `RimSupport` left every row of the
v18 break suite green.

The cost is not the attribute. It is what the tree has come to believe
about it: the v18 ledger entry credited `deny_unknown_fields` with the
serde-death that justifies a schema break, when the operative machinery
is the version door (`SchemaTooOld` / `UnknownSchema`, before serde is
reached) and the enum's own unconditional unknown-variant refusal.
BLEND-5's fix pass corrected that one entry. **This unit is the sweep
obligation for the class, and the class has two halves** — the
attributes, and the prose that reasons from them.

## Three premise corrections — take these as a hypothesis, not a result

Measured on this unit's merge base (2026-09-15). The item's own numbers
are from 2026-08-30 and its `file:line` citations have rotted;
`docs/prompts/implementer-discipline.md` §7 is why the names below carry
line numbers that may go stale in turn. **Re-take every count before you
build on it** and say in the PR what you measured and what your pattern
could not match.

1. **"73 sites" is grep hits, not attributes.** 73 lines match
   `deny_unknown_fields` under `crates/`; **59 are the attribute** and
   **13 are prose** — module docs, doc-comments and test comments that
   reason about it. The fourteenth non-attribute is `doc.rs`'s
   multi-line `#[serde(bound(…))]`, which the naive walk mis-parses and
   which you should read by hand.

2. **The item's rule is "unit-vs-struct" and that rule is too coarse.**
   `deny_unknown_fields` needs a *named* field to deny, so it is inert
   on a **unit enum**, a **tuple-variant-only enum**, a **tuple struct**
   and a **unit struct** alike. Under the item's rule I count 16 inert
   sites; under the named-field rule, **22**. The six the coarse rule
   misses are real: `Attr`, `WireTarget`, `WireMeasureExpr`,
   `DocParamValue`, `PartSelect`, `MetaValue`, plus the tuple structs
   `ParamName` and `RecipeNodeId`. **Establish the rule by execution
   before you sweep on it** — BLEND-5's method, on one site of each of
   the four shapes: delete the attribute, run the crate's suite, and
   record that nothing flipped.

3. **"every tag enum in `role.rs` is unit-only" is false today.**
   `role.rs` carries 11 attribute sites, 8 enums and 3 structs; two of
   the eight enums carry data (`Qualifier`, `RoleSeg` — whose variants
   are tuple variants, so they land on the *inert* side of the sharper
   rule but not for the reason the item gives). Six of the eight are
   unit-only. Correct the item file in this PR, per §How the class
   column is read: a lane that finds an estimate or a premise wrong says
   so and fixes the row.

## What to do

**Half A — the attributes.** Every inert site is either removed, or
kept with **one sentence at the site** saying it is a habit-guard held
against a future named-field variant. Removal is the default; a keep is
a decision a reader can see. Do not leave a site un-dispositioned, and
do not decide the two halves of the population by different rules.

Governing sites are not yours. Do not touch a site where the attribute
does work, and do not "tidy" one whose declaration might grow a named
field later — that is the keep case, spelled at the site.

**Half B — the prose.** The 13 prose sites are the actual cost of this
defect and are the part a later reader will trust. For each: is the
sentence still true given what the attribute does at the declaration it
names? Three dispositions, and say which at the site —

- **true and load-bearing** — leave it;
- **rotted while the code stayed right** — fix the sentence;
- **reasoning from an inert attribute** — that is the v18 shape, and it
  is the reason this row exists. Rewrite it to name the machinery that
  actually refuses (the version door, the unconditional unknown-variant
  refusal), not the attribute.

One of the 13 already says the attribute is inert
(`crates/editor-core/tests/blend5_rim_support_wire.rs`, the retired-
spelling row) — that is BLEND-5's own residue, and it tells you the
vocabulary a corrected sentence uses.

**Half C — the instrument.** A sweep that stops here closes today's
instances and nothing else; the next inert attribute arrives silently.
Leave a guard that **reds when `#[serde(deny_unknown_fields)]` sits on a
declaration with no named field anywhere.**

Three things bind how you build it:

- **It reads Rust as text, so it goes through `test_utils::source`** —
  `code_only`, `item_body` and `rust_sources` are the views you need —
  and it **owes its line in `crates/test-utils/tests/reader_census.rs`**.
  That census exists precisely so a new reader cannot arrive silently;
  a hand-rolled lexer here is the wrong answer and the census header
  says so in its own words.
- **The reader needs its own guard.** This is the charter's trap in its
  first instance: a guard that reads source text can go quietly blind —
  PR 2501 measured one reporting agreement over a set missing exactly
  the variant it existed to catch, because an attribute in front of a
  name made the walk read an empty name. Your guard must have a row that
  goes red when the reader stops SEEING, not only when it disagrees:
  assert the population it found is the size you measured, so a walk
  that silently narrows to zero fails instead of passing.
- **Say what it cannot see.** Macro-generated declarations, `cfg`-gated
  ones, and anything outside the trees the walk covers. A blind spot
  stated is a negative result; a blind spot unstated is a claim.

Whether the guard lands as a `scripts/gates/*.sh` row or as a test
beside the reader census is yours to choose — take the one whose
precedent you can point at, and say which and why.

## Fences to announce

This program claims no paths. Name each crossing in the PR body:

- `crates/editor-core/src/*` and `crates/editor-core/tests/*` — the bulk
  of both halves.
- `crates/viewer/src/prefs.rs` — one prose site, CHROME's and VIEW's.
- `crates/test-utils/*` — TCOST's and TINT's, if Half C lands a reader
  census line there. Announce it; do not route around it.
- `scripts/gates/*` — GUARD's, if the guard lands as a gate.

## Acceptance

1. Every one of the 59 attribute sites is dispositioned: untouched
   (governs), removed, or kept with its one-sentence reason. The PR body
   carries the hit list — one line per site — as a receipt, not a claim.
2. Every one of the 13 prose sites is dispositioned, with which of the
   three verdicts it got.
3. The guard exists, reds on a planted inert attribute, and **reds on a
   blinded reader** — both directions executed and shown, red then green.
4. Its blind spots are stated at the guard, not only in the PR body.
5. The reader census carries its line if a reader landed.
6. The item file is corrected: the three premise corrections above, and
   the class estimate if you find M wrong.
7. Hosted CI green — twelve `test (…)` jobs and five `k-lint (gate, …)`.
   `crates/editor-core` is in the wheel's dependency closure, so the
   python suite runs; expect it and read it.

## What this unit is NOT

Not a serde audit. Not a change to any wire format, any schema version,
or any refusal's text. If a site's disposition would change what a
document accepts or rejects, stop and report it — that is a finding
about the version door, not a member of this sweep, and it gets its own
file on the slate of whoever owns it.
