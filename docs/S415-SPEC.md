# S415 — the three boundary residues (spec)

**Unit:** `work/port/S415.md`. **Program:** PORT. **Branch:**
`port/s415-boundary-residues`. **Deleted at merge** per
`docs/DOC-LEDGER.md`; the item file is the record that survives.

Read `docs/prompts/implementer-discipline.md` in full first. It binds
you, and two of its sections carry most of the weight here: §5 (a sweep
states what its pattern could not match) and §6 (file what you find
outside the fence, in this PR, on the owner's slate).

## Territory — none of this is PORT's

`crates/step-import/*`, `crates/step-export/*` and `crates/stl/*` are
**EXCH's**; `crates/pncad-py/*` is **LIB's**. PORT claims no paths and
announces every unit to its owner. Say so in the PR body and name both
programs. Either may take the row instead — that is a good outcome.

## The row's premises are wrong in two places

`work/port/S415.md` was written on 2026-09-01 and the orchestrator
re-read it against the tree on 2026-09-15. **Check both corrections
yourself before building on them** — they are the dispatcher's belief,
not a finding, and a brief whose premise is wrong produces a detailed
and plausible report about something that is not there.

1. **Residue 3 says "no kernel enum exists behind either" of the two
   literal tags. That is false for `no_minted_id`.** `tags.rs`'s
   `declare_error_tag` maps `DeclareError::NoMintedId` to exactly the
   string `"no_minted_id"`, and `tests.rs`'s `declare_error_tags_are_stable`
   pins it. It is true for `name_serialize`, which is a serde failure
   with no kernel arm at all.
2. **Residue 2 says "three formats' genuinely separate rules". It is
   two formats, not three.** `step-import`'s `parse.rs` and
   `step-export`'s `lib.rs` are both **Part 21's basic alphabet** — one
   rule, two crates. Only `stl`'s is independent. The row counted
   crates; the question is about formats.

Correct the row's body in this PR, with the evidence. A dispatch
estimate is correctable by the lane that finds it wrong
(`work/port/plan.md`), and so is a finding's premise.

## Residue 1 — the two scaffold-strut mint sites hold one hazard to two standards

`crates/step-import/src/assemble.rs`, `plant` and
`insert_selfloop_tied`.

Both mint a temporary scaffold strut through `mev_line`. A **zero-length
strut cannot certify**, and the two sites treat that the same way only
by luck:

- `plant` scans for an anchor whose position differs from the target
  **bitwise on all three components**, skips any that coincides, and
  when no anchor survives raises `StepImportError::Topology` naming the
  case. Fail-loud, with an arm of its own.
- `insert_selfloop_tied` builds its strut endpoint as
  `Point3::new(p.x + 1.0, p.y, p.z)` and checks nothing. At
  `|p.x| ≳ 2^53` the spacing of `f64` exceeds 1.0, so `p.x + 1.0` **is**
  `p.x`, the strut is zero-length, and the site walks into precisely the
  state its sibling refuses twenty lines away.

**The asymmetry is the finding and it stands whatever the reachability
is.** Do not spend the unit arguing about whether a STEP file can carry
a coordinate near 2^53: the reader takes untrusted input, `plant` already
treats a comparably absurd case as worth an explicit refusal, and
fail-loud is the house posture (CLAUDE.md). Reachability decides how the
site reports, not whether it checks.

**Left to you**, with your reasoning in the PR:

- Whether the guard is a post-condition on the minted offset (assert the
  strut is non-degenerate, refuse if not) or a pre-condition on `p`.
  Prefer whichever makes the two sites read as one rule, since that is
  the finding.
- Whether the two sites should share the rule outright rather than
  state it twice. If they should, say where it lives.
- **A second half the row did not name, and your call whether it is in
  scope:** `1.0` is an absolute offset in a format whose coordinates
  carry no unit contract here. It is degenerate at the top of the range
  and enormous at the bottom — a 1.0 strut in a model measured in
  microns. If you judge it out of scope, **file it** (§6) rather than
  mentioning it; EXCH owns the ground.

**A test that can go red** (`docs/prompts/reviewer-style-lane.md` Q3):
a row that pins today's behaviour at an ordinary coordinate proves
nothing. The row that matters drives a start vertex with `|x|` past the
spacing threshold and asserts the site refuses rather than minting a
degenerate strut.

## Residue 2 — the Part 21 basic alphabet is one rule in two crates

Three admissibility sites over the band `0x20..=0x7E`, in two notations:

| site | notation | format |
| --- | --- | --- |
| `crates/step-import/src/parse.rs` | `(0x20..=0x7E).contains(&c)` | Part 21 |
| `crates/step-export/src/lib.rs` | `' '..='~'` match arm | Part 21 |
| `crates/stl/src/options.rs` | `(' '..='~').contains(c)` | STL name rule |

Plus the band restated in prose at roughly six more sites across the
three crates.

**The first two are one rule.** They are the same paragraph of the same
standard, read and written by the same project, and if Part 21's
alphabet is ever read differently they must move together. **The third
is not**, and the row's steelman is right about it: STL's name rule
coincides with Part 21's band today and is an independent spec. A
shared constant across all three would couple two rules that are only
accidentally equal, and would silently widen STL the day STEP widened.

**The shape of the answer, and the honest obstacle.** `step-import` and
`step-export` share no crate but `geom-core`, `geom` and `topo` — the
kernel. A Part 21 *text-format* constant does not belong in a kernel
geometry crate (D-layering: the kernel is geometry and topology, the
format is the boundary), and a new crate or a dependency edge between
the two STEP crates is a heavy answer for one range. **So the outcome
may legitimately be that the duplication stays.** If it does, what is
owed is that it stops being silent:

- Each Part 21 site **names the other** in the disclosed-copy vocabulary
  the project already uses, so the next reader finds the pair.
- The STL site says it is **coincidentally equal and deliberately not
  shared**, so the next reader does not "fix" it into a shared constant.
  That comment is not archaeology under §4 — it states an invariant
  (these two rules are independent), which is exactly what a comment is
  for.
- **Both bounds are pinned by a test at each site.**
  `crates/stl/tests/export.rs` already does this for STL and says so;
  the two STEP sites should have the same, and its row is the model.

If you judge a shared home worth its cost after all, take it and argue
for it — the above is the orchestrator's reading, not a constraint.

## Residue 3 — two hand-minted tags against a stated through-`crate::tags` rule

`crates/pncad-py/src/py/doc.rs` mints two `variant` strings as literals
through `boundary_edit_err`: `"no_minted_id"` in `Doc::insert` and
`"name_serialize"` in `name_text`. `Doc.new`'s prose states the rule
that tags come through `crate::tags`.

They are **not the same case**, which is what the row missed:

- **`name_serialize` has no kernel arm.** Nothing in `tags.rs` produces
  it and no enum stands behind it; it reports a `serde_json`
  serialization failure at the boundary. This one is a genuine
  exception to the stated rule, and the question the row asks is the
  right one: **should the rule's statement carry the exception?** Answer
  it in the PR. A boundary-minted tag for a refusal the kernel has no
  arm for is defensible; a rule that says "always" while two sites say
  otherwise is not.
- **`no_minted_id` collides with a kernel-derived tag.**
  `declare_error_tag(&DeclareError::NoMintedId)` yields the identical
  string, and both paths reach Python as `ErrorClass::Edit` carrying
  that `variant`. So two different refusals, from two different doors,
  publish one tag — and they are **not interchangeable to the caller**,
  because `boundary_edit_err` passes `EditPayload::NONE` while
  `declare_err` carries the arm's fields. A Python caller matching on
  `variant == "no_minted_id"` cannot tell which it has, and reading the
  payload to find out is the thing the tag exists to avoid.

  **Verify this before you act on it** — confirm both paths really do
  land in the same class with the same variant, and check whether
  anything (a census, a `.pyi` claim, a Python test) already asserts the
  tag namespace is injective. If nothing does, that absence is itself
  worth reporting.

  Whether the collision is a defect or an intended synonym is **your
  call**: they arguably mean the same thing. What is not defensible is
  that neither site knows the other exists. If you rule it an intended
  synonym, the two sites say so and the payload difference is either
  removed or documented.

**Sweep obligation (§5).** These are two instances of "a tag minted by
hand where a tag function exists". Grep the shape across
`crates/pncad-py/src/py/` — string literals reaching a `variant` field —
not just these two symbols, and put the hit list and its disposition in
the PR body, one line per hit: fixed, or not-this-unit and why. State
what your pattern could not match.

## Scope

Three unrelated residues in one unit because they are all small and all
on this program's boundary ground. **They are independent**: if one
grows, land the other two and file the third as its own row on the
owner's slate rather than letting the unit sprawl. Say in the PR which
of the three you closed.

## Verification

Hosted CI is the verification of record. Push, mark ready for review,
poll the run's jobs API in the foreground until it concludes, and report
in the same turn — do not end your turn on a pending run. A green
code-tier run means twelve `test (…)` jobs and five `k-lint (gate, …)`
jobs; if you see fewer, find out what narrowed it. The python suite runs
here because `pncad-py`'s dependency closure is seeded.

Do not write a `CI-Config:` trailer — nothing reads one.

## Review

**Style review** (`docs/prompts/reviewer-style-lane.md`), which is
PORT's default. This unit does not get the correctness arm; that is
reserved on this slate for `load-path-stringifies-structured-refusals`.
Write the PR so a style reviewer can find the three decisions you made
and the reasoning behind each.
