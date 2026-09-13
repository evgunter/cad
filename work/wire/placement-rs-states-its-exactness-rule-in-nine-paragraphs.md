---
id: placement-rs-states-its-exactness-rule-in-nine-paragraphs
kind: issue
title: placement.rs asserts exact / by bits / D9-deterministic in nine separate doc paragraphs, none the authority for any other, and is 60% prose
status: open
opened: 2026-09-11
refs: [2375]
pr: 2475
---


## Finding

From the full review of PR 2375 (S6 `likely`, S8 `unsure`, S9 `unsure`),
which read `crates/editor-core/src/placement.rs` end to end under Q8 —
392 lines. Three findings, one file, filed together because they are the
same accumulation seen three ways.

**S6 — nine spellings of one rule.** After PR 2375 the file states
"this is exact / by bits / D9-deterministic" at `:103-112`, `:126-128`,
`:165-169`, `:227-229`, `:234-236`, `:245-252`, `:263-266`, `:271-272`
and `:296-300` — nine doc paragraphs, nine different sentences, **none
of which is the authority for any other**. That is the shape
`docs/prompts/reviewer-style-lane.md` Q2 names: a comment existing to
reconcile two spellings of one rule is evidence the rule needs one home.
PR 2375 added two of the nine, which is how the file got here — no
single unit added an unreasonable paragraph.

The reviewer's taste finding, recorded as such: one paragraph on `Frame`
and cross-references from the methods.

**S8 — `#[must_use]` is inconsistent inside the file.** None of `Frame`'s
methods carry it (`:187`, `:198`, `:204`, `:216`, `:230`, `:237`, `:253`,
`:267`, `:273`) while `AxisRefusal::kind` and `::carried` at `:34` and
`:41` in the same file do, and `Mat3::map` / `Affine3::map` — which the
new bodies now call — do. Pure-query methods on a `Copy` struct are the
canonical case. The reviewer flags the **inconsistency**, not the rule,
and does not know whether the project has ruled on it; a taker should
find out before changing anything.

**S9 — the ratio.** The file is 392 lines and about 60% prose.
`rotate_then_translate` (`:100-159`) is 38 doc lines over a 21-line body,
four titled paragraphs deep, arguing its bit-agreement with the transform
node. **Nothing in it is wrong** — that is the point of the finding. It
is the accumulation Q8 exists to surface, and the reviewer notes that
PR 2375's new test module is the first thing in the file that would catch
any of the nine claims going false.

## What this row is not

It is not "delete the prose". Every paragraph is individually defensible
and several are load-bearing arguments about bit-exactness that this
project genuinely wants written down. The row is about **where** the rule
lives, not whether it is stated: one home, cross-referenced, so the tenth
unit does not add a tenth sentence.

## Taken (2026-09-12) — WIRE's `placement-prose` lane

**S6 — the one home is `Frame`'s own type doc, a `# Exactness` section.**
Four claims, stated once each: coordinates are carried and never
recomputed (so a read-back at any scalar is exact and the identity at
`f64`); where a door does arithmetic the claim is D9-deterministic and
explicitly *not* exact; a bit-exact identity is a fast path and only a
bit-exact one may be; a placement and a modeled transform agree bit for
bit. Method docs that used to state one of those now carry a one-clause
cross-reference and their own semantics only — see the honest census in
the fix pass below; it is six, not eight. Two of the nine
were folded rather than moved: `rotate_then_translate`'s
normalize-here argument and the "decided direction normalizes by the
same expression" paragraph are one hazard — *removing this normalization
costs the bit agreement* — and are now one paragraph at the site, where
something would go wrong without it.

The section also says **where each claim is guarded**, which no site did
before — see the fix pass below for the shape that took.

**S8 — the project has NOT ruled on `#[must_use]`.** Nothing in `docs/`,
no `crates/*/README.md`, nothing in the tracker states a convention;
`clippy::must_use_candidate` is not in the workspace lint table
(`Cargo.toml`), so no lint decides it either. What exists is practice:
232 `#[must_use]` sites on `pub fn`, 4 on non-`pub` ones
(`geom-brep/src/ssi/march.rs` ×2, `geom-core/src/real.rs`,
`sweep/src/blend/battery.rs`), and reviewers flagging its absence as a
finding (`work/m10/the-span-identity-is-not-a-theorem-of-the-floats.md`,
where a missing one on `Real::register_equal` was the defect).

So, per this row's own instruction, the file was made internally
consistent without minting a project-wide rule: **every `Frame` method
carries `#[must_use]`** — eleven of them, the private `linear_f64` and
`affine_f64` included, since they are the same pure-query shape as the
public `linear`/`affine` and a rule that stopped at the `pub` keyword
would leave the same inconsistency one level down. The one exception is
`rotate_then_translate`, which returns `Result` and is already
`#[must_use]` by its type; annotating it would fire
`clippy::double_must_use`.

**S9 — the ratio did NOT improve, and this row should not be read as
closing it.** (Counts superseded by the fix pass below.) The one home
states in one place what nine sites stated in fragments, and adds the
three guard citations, so it is longer than the sum of what it replaced.
Nothing else in the file passes the deletion test in
`memories/cad-working-style.md` — the module header is present design,
`AxisRefusal`'s paragraph says why the type is narrow, the fixtures'
`-0.0` note is load-bearing by its own terms. If the ratio is still the
complaint, the remaining answer is fewer doors, not less prose: see
`frame-linear-generic-door-has-no-consumers`.

**Prose has no assertion.** Nothing in this change is testable and no
test covers it; `cargo fmt`, `cargo clippy --workspace --all-targets -D
warnings`, the rustdoc gate's lint set and `-p editor-core --lib
placement` are what ran, and they say the links resolve and the
attributes compile, not that the rule is stated once.

**Filed while here**:
`work/wire/from-affines-identity-fast-path-cannot-change-its-answer.md`
— re-homing the paragraph over `Frame::from_affine`'s identity branch
showed the branch cannot change the function's answer.

## Fix pass (2026-09-12, PR 2475's review — MERGEABLE, no MAJOR)

The review found that the fix had **minted two fresh instances of the
defect it was closing**, plus one false sentence. All three repaired.

**The false sentence.** *"Every claim above is guarded"* named four rows
and none of them guarded `Mat3::determinant`'s fixed-evaluation-order
claim: `editor-core` had no determinant test at all, and the only one
anywhere is `geom-core`'s `determinant_of_identity_is_one`, which
asserts `1.0` and cannot tell one evaluation order from another. The
guard turned out to be cheap, so it is **written** rather than excused:
`determinant_is_mat3s_association_and_not_just_its_value` pins
`c0 · (c1 × c2)` with the dot summed left to right, on a frame whose
three summands are `1.0`, `1e16`, `-1e16` — where the two groupings of
one addition chain give `0.0` and `1.0`. The row asserts the groupings
DISAGREE before asserting which one `Frame::determinant` answers, so it
cannot pass vacuously, and it reddens on a reassociation inside
`Vec3::dot`/`Vec3::cross` as well as on a `Frame::determinant` that
stops delegating (verified: mutating the body to the other grouping
fails it, leaving the other four rows green). No PROPS row is owed —
`Mat3::determinant`'s order now has a guard, one crate away.

**Instance 1 — a hand-written census of four test-function names.** The
section listed its guards in backticks, which are not intra-doc links
(a test fn is not a link target), so the rustdoc gate could not see a
rename and the PR's claim that the gate proves the links resolve was
true of the `[`Frame::…`]` links and false of those four. The list is
**deleted**: the home now says each claim is guarded by a row in this
module's `tests` and that each row names the claim it keeps, and each
of the four rows carries that one-line doc. Nothing names a test from
outside, so nothing rots. The one citation that cannot be removed —
`r1_the_placement_frame_matches_the_transform_node_bit_for_bit`, in
another file because the claim needs a whole document — now carries the
*"unguardable, and here is why"* sentence at the site per
`reviewer-style-lane.md` Q6, and the class is filed on `meta`'s slate.

**Instance 2 — the `#[must_use]` rule lived only in this row.** Written
in the tracker, enforced by nothing (`clippy::must_use_candidate` is off
workspace-wide and no test can read an attribute), so the twelfth method
would have arrived without it. The rule is now a comment on `impl Frame`
stating the obligation a new method upholds, and naming its own single
exception. That comment IS the mechanism, and it says so.

The `rotate_then_translate` exclusion is restated as what it is: a
**choice, not a constraint**. The review reproduced
`clippy::double_must_use` and found clippy's own help offers *"either add
some descriptive message or remove the attribute"*, so
`#[must_use = "…"]` would be accepted. Declined — a message on a `Result`
that is already `#[must_use]` by type buys a reader nothing and costs a
second place to keep in step.

**The sibling instance is NOT fixed here, deliberately.**
`work/wire/the-third-tag-vocabulary-macro-owes-a-unification-trigger.md`
records the same inconsistency one file over (`Target::kind()` has the
attribute, `ArcData::mode()` does not). It is already a scheduled row
with its own subject — a macro unification trigger — and reaching into
it from a prose unit would be a second surface decision nobody asked
for. That row now carries a pointer to the rule this PR wrote, so its
taker has the precedent rather than re-deriving it.

**The honest census (NOTE-3).** "One home" is one home plus survivors,
and the earlier "eight method docs" was wrong. After this pass **six**
sites cite the home (`rotate_then_translate`'s first paragraph,
`linear`, `affine`, `compose`, `is_identity_bits`, and the test module
header) and **three** still state part of the rule, each for a reason:

- `rotate_then_translate`'s *"normalizing the axis here is not
  redundant"* paragraph — a HAZARD, not a restatement. It is the one
  place a future editor could delete the normalization, and the home
  cannot stop them from three screens away.
- `from_affine`'s *"a bit-exact identity map returns `Frame::IDENTITY`"*
  — that is the function's OUTPUT, not the exactness rule. It reads like
  the rule, which is exactly why the cross-reference was removed rather
  than added: the branch that makes it look like a fast path is inert,
  and this module now has a row proving it
  (`from-affines-identity-fast-path-cannot-change-its-answer`).
- `sample()`'s *"the `-0.0` is LOAD-BEARING"* — a fixture's obligation
  to whoever edits it next, which no type can carry.

Nine → one home + three, not nine → one.

**Counts, split so the growth is readable.** Against `main`, whole file
430 -> 569 lines. Outside `mod tests`: 293 -> 336 (comment 130 -> 162,
code 140 -> 151 — the eleven `#[must_use]` attributes). Of that +32
comment lines, 9 are the `#[must_use]` rule block and 23 are prose.
Inside `mod tests`: 137 -> 233, all of it the two new rows and their
docs. **The S9 ratio finding is still not closed**, and the qualifier
above stands: the remaining answer is fewer doors, not less prose.
