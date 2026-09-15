# CENSUS-PY-RAISE-LITERALS — nine words minted at a raise site, in three shapes (spec)

**Program:** CENSUS (`work/census/plan.md`). **Item:**
`work/census/py-reason-and-variant-literals-outside-any-enum.md`.
**Class at the cut:** M. **Track:** no A/B protocol, no ordinal. **One
style review**, carrying the silent-omission obligation — this unit
lands and changes instruments, so it carries it by the trigger
`plan.md` §Review posture now states. No correctness lane.

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
item whole; `work/census/plan.md` §Charter, §Order's `pncad-py` block,
§Review posture; **`work/census/log.md`'s standing findings** — all
nine, but 5, 6 and 7 are addressed to you by name below.

**Read the two units that landed this shape** — `6b432a15`
(CENSUS-TAG-REACH: `ErrorClass::Evaluation` carries `EvalReason`) and
`7a187c74` (CENSUS-PY-GETTERS: seven maps relocated into `tags.rs`).
Between them they proved every form you will need, and one thing they
proved is a NEGATIVE that governs this spec.

## The row is good; verify it anyway

Two fix passes have corrected this row already and it is precise. I
re-checked its two sharpest claims against the tree by a different
route than its own — `select_refusal_tag` does return `"unclassified"`
at its wildcard arm while `py/flush.rs` hand-spells the same word for
the same attribute of the same class, and `run_validator` does take its
door word as a positional `&str` at four call sites.

**Re-take the rest.** This program's specs have been wrong on four
units out of four, twice on a criterion and once because the
orchestrator's check was shaped like the claim it checked. The row's
nine words in four files are a hypothesis until you have run your own
measurement, and the numbers here are mine, not yours.

## The three shapes, which want three different answers

The row's real finding is that *"outside any enum"* is true of six words
and **false of the three that matter most**. Do not treat these as one
sweep.

**Shape A — a second mint of a word the door's own inventoried map
already mints.** `unclassified` (`py/flush.rs`, `SelectRefusal`),
`wireframe` (`py/value.rs`, `StepImportError`), `not_utf8`
(`py/mesh.rs`, `stl_err`'s seventh arm, in tuple position). Each writes
the same attribute of the same class as a map that is already in
`TAG_INVENTORY`. This is the worst of the three: the word is inventoried
AND hand-copied, so the inventory is green while a second spelling
drifts.

`unclassified` is the hard one and CENSUS-TAG-REACH already established
why: `select_refusal_tag` takes a `&SelectRefusal` and `flush.rs` holds
an unknown `ContactClass`, so it **cannot** call that map. Do not
propose that it can.

**Shape B — a word with no home anywhere.** `name_serialize`
(`py/doc.rs`, `EditError`) and `mass_properties_failed`
(`py/value.rs`, `ValidationError`). Each needs a home minted.

**Shape C — a whole vocabulary passed as arguments.** `validate`,
`validate_closed`, `validate_geometric`, `validate_pseudomanifold` —
four words on `ValidationError.door`, spelled at four `run_validator`
call sites. This is the shape CENSUS-TAG-REACH closed for `reason`.

## The negative that governs your disposition

CENSUS-TAG-REACH's spec ruled that each word become a `pub const` in
`tags.rs`, and required the lane to test by execution what would stop
the next word. **Against that disposition nothing red**: a fresh raise
site minting a word that had never existed passed 85 Rust and 832
Python tests. A const puts a word *in the inventory*; it does not stop
the next literal at the next call site.

So for each shape, answer the sharper question: **can this door's
attribute carry a TYPE**, the way `ErrorClass::Evaluation` carries
`EvalReason`, so that a raise cannot be written without naming a
variant? Where it can, that is the answer. Where it genuinely cannot,
a `pub const` in `tags.rs` is the inventoried fallback — **and you say
plainly that it pins the text and does not close the class**, rather
than letting the const read as a fix.

Shape C looks most like the proven case and Shape A's three look least
like it. I am not pre-deciding any of them: the doors differ, and the
evidence is in the code.

## Points 5, 6 and 7, addressed to you

**Point 5 — a probe inherits the spec's fence.** CENSUS-TAG-REACH's
lane ran the probe its spec asked for; the probe passed while the
defect stood one level out, because the spec had named `eval_err` when
the door was `typed_err`. **Before you run a probe, write down what it
cannot see, and name the unit of guarding you are testing** — a
function, a door, a class, or the crate. Then ask whether that is the
unit the defect lives at.

**Point 6 — a sweep shaped like the defect you already found finds that
defect again.** This row came from a key-adjacent literal sweep;
CENSUS-PY-GETTERS came from a `-> &'static str` sweep; its fix pass ran
a word-keyed sweep and an `&'static str`-binding sweep. **Shape yours
differently from all four and say how before you run it.** One shape
none of them covered: a word reaching an attribute through a local, a
helper, or a `format!`.

**Point 7 — re-read every row you file or touch against the tree before
pushing.** Four units, four sets of filed rows, and only the last
caught its own overclaim before a reviewer did.

## Acceptance

1. Every one of the nine words dispositioned, by shape, with the
   reasoning per shape in the PR body.
2. For each disposition: does it close the class, or only pin the text?
   Say which, at the site, in those terms.
3. **A probe per shape, executed**, with its stated blind spot and its
   named unit of guarding: add a fresh literal at a fresh raise site of
   that shape and report what reds. If nothing does, say so — that is
   the finding, and it is better from you than from the reviewer.
4. The inventory guard red-then-green for anything that lands in
   `tags.rs`.
5. **No Python-visible word changes value.** `pncad.pyi` and the
   832-test Python suite are the contract. If a word's text would
   change, stop and report.
6. Your differently-shaped sweep, its shape stated, and what it could
   not match.
7. The item corrected if anything fails re-measurement.
8. Hosted CI green; `pncad-py` is the wheel, so the python suite runs.

**On `gate ok`**: it may false-red on an API-lag race that is not yours
— every other job green, the gate naming a `k-lint` row `in_progress`.
That is `work/ciw/gate-ok-has-no-expected-job-roster.md`, recorded
twice. Do not chase it, do not touch `ci.yml`, never push an empty
commit; report it.

## What this unit is NOT

Not a change to any word's text or to `pncad.pyi`'s surface. Not a
widening of the tag-table reader. Not the 54 unread collisions
(`sixty-one-tag-words-…`) nor the `errors.rs` enumeration gap — both
are their own rows on this slate. `crates/pncad-py/*` is LIB's
territory and this program's `keep_out` announces its pncad-py rows
there; say so in the PR body.
