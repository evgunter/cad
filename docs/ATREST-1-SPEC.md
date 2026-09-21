# ATREST-1 — tier 3 learns to ask which solid

**Binds one implementer lane.** Deleted at merge (`docs/DOC-LEDGER.md`);
`work/atrest/ATREST-1.md` is the record that survives.

Read `docs/prompts/implementer-discipline.md` in full first. It is
binding alongside this spec.

Branch `atrest/1-per-solid`. Program ATREST, prefix `atrest/`.

## The defect, stated once

Tier 3 never asks which **solid** anything belongs to. Two filed P0
admit-holes are that one fact:

- `work/atrest/an-inside-out-part-passes-tier-3-because-only-the-body-total-volume-is-pinned`
  — check 7 reads the BODY's total signed volume (Σ over every face).
  A reverted part beside a larger ordinary solid sums positive and
  certifies.
- `work/atrest/tier-3-does-not-check-shell-roles-per-solid`
  — no tier asks which solid a shell belongs to, so a solid holding an
  `Outer`, a `Void`, and a second `Outer` inside that void — the
  boolean's hollow-operand subtraction — validates. Two material
  components filed under one solid.

Read both files before you start. This is the door every other program
reads as proof, so it is wrong in a direction that matters: it ADMITS.

## What the structure already gives you

`Solid { shells: Vec<ShellKey> }` and `Shell { faces, solid }`, with
the back-pointers validated against each other in tier 1. So a solid's
face set is reachable and total: every shell has an owning solid.

`crates/topo/src/props.rs` already carries the restriction idiom you
need: `classify_shells` is `classify_shells_of` over every shell, and
its rustdoc states why a restricted door exists at all — *"a caller
asking about one solid's shells is asking a question that does not
involve any other solid's, and it should not pay another solid's
refusal for it."* That sentence is this unit's argument, already
written, for a door one level down.

## Settled design — these are decisions, not options

**D-A. Check 7's subject becomes the SOLID, with a proved
short-circuit.** Do not add a per-solid check beside a body-total
check: that is two implementations of one underlying logic, the class
`work/README.md` puts at P1, and it would leave the body-total read as
a weaker duplicate of the per-solid one.

The short-circuit: when the body holds exactly one solid, the body's
face set and that solid's face set are the same set, so the existing
walk already answers the per-solid question and nothing new runs — no
extra quadrature, no change to what `validate_geometric_certificate`
returns on the overwhelmingly common body. **This is an equivalence,
not a special case, and it only lands if you can cite the tier-1 check
that makes it true** (every shell has an owning solid; every solid's
shell list and every shell's back-pointer agree). Cite it by name in
the code. If that guarantee turns out to be weaker than stated, say so
and run the per-solid walk unconditionally — do not paper over it.

**D-B. One uncomputable vocabulary.** A solid whose volume sign cannot
be decided refuses `ValidationError::VolumeUncomputable`, check 7's
existing posture, with the solid named. Do **not** mint a second
"could not decide" variant. Refusing rather than passing is the
established direction at check 7 and it is the only honest one here: a
program whose subject is admit-holes does not close one by opening
another.

**D-C. Shell roles are a new check with a new typed variant.** The
claim: **each solid has exactly one shell that classifies `Outer`, and
every other shell of that solid classifies `Void`.** The refusal names
the solid and the shells that made it fire.

**The `and inside it` half is NOT this unit.** SHELL-5's row describes
the full check as *"exactly one `Outer`, every other shell `Void` and
inside it"*. Nesting one shell inside another is a containment claim
tier 3 has no walk for at rest — it is the same family as check 9's
gap. **File it as its own row on `work/atrest/` in this PR, at the
moment you disclose it**, and have the check's rustdoc point at that
file. `work/README.md`: disclosing a residue is not scheduling it.

**D-D. Do not couple either new read to the REPORTING target.** This
is the binding constraint of the unit.

`classify_shells_of` reads at the reporting level — its own comment
says so: *"a shell role is a claim about a volume, and its faces run
their whole schedules."* Calling it from tier 3 would mint a fresh
instance of exactly the defect
`work/atrest/tier3-prime-still-couples-plus-v-to-the-reporting-target`
exists to close, inside the program that filed it. Both of this unit's
questions are **sign** questions — is this solid's volume positive, is
this shell's positive or negative — and a sign is what
`props::sign_certified` decides, stopping at the round where the
enclosure's sign stops being in doubt.

So: give `sign_certified` a face-restricted sibling, in the shape
`classify_shells_of` already has in the same file — the existing
`sign_certified` becomes that sibling over every face, exactly as
`classify_shells` is `classify_shells_of` over every shell. One
helper, two callers: the solid's faces for D-A, a shell's faces for
D-C. `crates/topo/src/props.rs` is in no open program's territory.

If you find a reason this cannot be a sign-level read, **stop and
report it** rather than reaching for the reporting door. That would be
a finding about D-D, which is a design decision, and design decisions
are the orchestrator's.

## What you owe

1. Both checks, per D-A through D-D, with the tier-3 rustdoc's check
   list and its **not-yet-checked list** updated to state what is now
   checked and what the `inside it` residue still is.
2. Rows in `crates/topo/src/tier3_tests.rs` that **go red without the
   change**: a multi-solid body with one inverted solid and a positive
   total (this is the fixture the first item describes, and
   `crates/sweep/tests/review_d2_adv_probes.rs` knows where multi-solid
   bodies come from), and a solid carrying two `Outer` shells (the
   hollow-operand subtraction,
   `work/bool/subtract-of-a-hollow-operand-files-the-island-under-one-solid`).
   **Write assertions a bug could break** — name the runtime value
   that would make each false.
3. A row proving the **false-refusal direction is closed**: an
   ordinary multi-solid body, and an ordinary solid with a genuine
   cavity (one `Outer`, one `Void`), still certify. This program is
   wrong in both directions and a new check is the cheapest way to add
   to the second one.
4. **The sweep, per discipline §5.** Your pattern is *reads the body
   where it means a solid*. Grep for it — every tier-3 read that folds
   over `body.faces` or `body.shells` and states a per-component
   claim — and put the hit list and its disposition in the PR body,
   one line per hit: fixed, or not-this-unit and why. Say what the
   pattern could not match.
5. **File what you find outside the fence** (discipline §6), on the
   owning program's slate, in this PR. `python3 scripts/work.py
   territory --files -` says who owns a path.
6. `python3 scripts/work.py lint` green, and the tracker rows' state
   updated on this branch.

## Verification

Hosted CI is the verification of record: twelve `test (…)` jobs and
five `k-lint (gate, …)` jobs on a code-tier run. Read the **run's**
jobs API at the step level, never job-name green. Do not end a turn
with background work live; poll a CI wait in the foreground.

Private paths, per `memories/agent-lane-operations.md`: your own
`CARGO_TARGET_DIR` **outside** the worktree, and a private scratch
directory — never the session scratchpad, which every lane of this
session shares.

## Review

Outside protocol v7 — opus implementer, opus reviewer, **full** review
(correctness claims alongside `docs/prompts/reviewer-style-lane.md`).
No A/B draw, no ordinal, no row. The triage reason is recorded in
`work/atrest/log.md` and on `work/atrest/ATREST-1.md`: the design calls
are settled here rather than in the lane, so what remains is execution
and evidence — but it lands a new refusal on the door every program
trusts, which is more than reading the diff can settle.
