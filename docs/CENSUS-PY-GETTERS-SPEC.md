# CENSUS-PY-GETTERS — a discriminant word minted by a getter, outside the inventory (spec)

**Program:** CENSUS (`work/census/plan.md`). **Item:**
`work/census/py-discriminant-getters-under-src-py-are-outside-every-inventory.md`.
**Class at the cut:** M. **Track:** no A/B protocol, no ordinal. **One
style review**, carrying the silent-omission obligation; no correctness
lane.

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
item whole; `work/census/plan.md` §Charter and §Order's `pncad-py`
block paragraph; and **`work/census/log.md`'s two standing-findings
sections** — "Two units in…" and CENSUS-TAG-REACH's points 5, 6 and 7.
Points 5 and 6 are addressed to you by name below.

**Read the code CENSUS-TAG-REACH landed** (merged `6b432a15`):
`crates/pncad-py/src/errors.rs`'s `ErrorClass`/`EvalReason`,
`crates/pncad-py/src/tags.rs`'s `eval_reason_tag`, and
`crates/pncad-py/src/tests.rs`'s `TAG_INVENTORY` and `TAG_CONSTS`. Your
fix shape is that unit's, one door over — which is why these five rows
run as a block.

## The population, verified

Measured 2026-09-15 and **independently re-measured by me at spec
time**. This is the first row in this program whose numbers survived
that check, so treat it as solid and re-take it anyway:

- **30** functions returning `&'static str` under `crates/pncad-py/src/py/`.
- **Six** mint their words as literals and reach a Python caller:
  `assembly.rs`'s `entity_kind_tag` (4), `mate.rs`'s `primitive_tag`
  (4), `ClassAdmission::variant` (3), the subgroup `variant` (7), the
  gauge-edit `variant` (4), `refactor.rs`'s `variant` (1) — **23
  distinct words**, every one named in `pncad.pyi`.
- `doc.rs`'s `_binds_every_kernel_window` is correctly **excluded**: a
  never-called compile-time tripwire, its two words not Python-visible.
  The style review that opened the row counted them; the row says so.

## The sharpening — read this before you plan the fix

The row says these words are "outside every inventory", which is true
and is not the whole shape. **All six functions are exhaustive matches
over a kernel enum with zero wildcard arms** — I checked every one. So
`E0004` already holds each map to its kernel type: **a new variant
stops the build today.**

What they are outside is the **inventory**, which pins a word's TEXT.
So the live defect is narrower and sharper than "unguarded":

- **a RENAME is what nothing catches.** Change `revolute` to `hinge`
  and every test passes; the word is public Python vocabulary and no
  committed table holds it.
- **seven of the 23 have a second spelling in `src/tags.rs`** —
  `face`, `edge`, `vertex`, `empty`, `join`, `split`,
  `no_at_rest_record` — minted by a different map for a neighbouring
  concept, held equal by nothing. That is this program's Q1 shape and
  it is the substance of the unit.

**So the mechanical half is a relocation**, and CENSUS-TAG-REACH proved
it: a map that lives in `tags.rs` is lexed by the reader, pinned in
`TAG_INVENTORY`, and reds on an addition or a rename. Moving the six
maps buys the text pin without touching a single word's value. Do that
half first and keep it separate in the history from the half below.

## The call you owe, which is the design half

For each of the seven second spellings: **is it one concept or two?**

- **One concept, two maps** → it has one home and the other site reads
  it. Say which is the home and why.
- **Two concepts that share a word** → they are allowed to, and what is
  missing is anything saying so. Pin them equal where they must agree,
  or state at both sites that the collision is deliberate.

Do not answer this uniformly for all seven — `face`/`edge`/`vertex`
(an entity kind versus an entity id) and `no_at_rest_record` (a class
admission versus a mint refusal) are plainly different situations. The
row has the pairs; verify each against the tree before deciding, and
say your reasoning per pair in the PR body.

**This is the unit's judgement and it is yours, not mine.** I am not
pre-deciding it because the seven are not alike and the evidence is in
the code rather than in the row.

## Also sweep: the prose restatement

`mate.rs`'s subgroup getter carries *"The primitive's stable tag:
`frame_coincidence`, `coaxial`, `planar_rest`, `clocking`"* in its doc
— a **hand-listed copy of a vocabulary that has a map**, four lines
above the delegation to that map. Nothing holds the prose to the
function. Check every one of the six for the same shape and disposition
each: delete the restatement, or point it at the map.

## Points 5 and 6, addressed to you

**Point 5 — a probe inherits the spec's fence.** CENSUS-TAG-REACH's
lane ran the probe its spec asked for, and the probe passed while the
defect stood one level out, because the spec had named the wrong unit
of guarding. **Before you run any probe, write down what that probe
cannot see.** If your probe is "rename a word and watch the inventory
red", say what a rename-probe is blind to (a word added, a word
deleted, a second spelling drifting apart, a map moved wholesale) and
whether any of those is the real risk here.

**Point 6 — a sweep shaped like the defect you already found finds that
defect again.** This row exists because a `-> &'static str` sweep found
what a literal-beside-a-key sweep could not. **Your sweep must be
shaped differently from both**, and you must say how before you run it.
Shapes neither has covered, from the previous unit's own report: a word
in tuple position; a word passed as a `&'static str` argument to a
helper; a `&'static str` struct field on a `#[pyclass]` (which is
`datum-kind-vocabulary-is-hand-spelled-and-uncensused`, a sibling row —
do not fix it, but if your sweep finds more of that shape, add the
evidence there).

**Point 7 — re-read every row you file or touch against the tree before
you push.** All three previous units filed rows that overclaimed, and
they go to other programs' slates.

## Acceptance

1. The six maps live where the inventory lexes them; `TAG_INVENTORY`
   carries their words, re-derived by the guard rather than hand-added.
   Show the guard reding before and passing after.
2. **A rename probe, executed**: change one word, show the inventory
   red, restore. With its stated blind spot from Point 5.
3. Each of the seven second spellings dispositioned, with the reasoning
   per pair.
4. The prose restatements dispositioned.
5. **No Python-visible word changes value.** `pncad.pyi` and the 832-test
   Python suite are the contract. If a word's text would change, stop
   and report.
6. Your differently-shaped sweep, its shape stated, and what it could
   not match.
7. The item corrected if anything in it fails re-measurement.
8. Hosted CI green. `pncad-py` is the wheel, so the python suite runs.

**On `gate ok`**: it may false-red on an API-lag race that is not yours
— every other job green, the gate naming a `k-lint` row as
`in_progress`. That is
`work/ciw/gate-ok-has-no-expected-job-roster.md`, recorded twice. Do not
chase it, do not touch `ci.yml`, never push an empty commit; report it.

## What this unit is NOT

Not a change to any word's text or to `pncad.pyi`'s surface. Not a
re-architecture of the six maps — they are already exhaustive over their
kernel enums and that half is sound. Not the `Datum.kind` struct-field
shape, which is its own row. `crates/pncad-py/*` is LIB's territory and
this program's `keep_out` announces its pncad-py rows there; say so in
the PR body.
