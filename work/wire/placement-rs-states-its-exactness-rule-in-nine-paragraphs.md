---
id: placement-rs-states-its-exactness-rule-in-nine-paragraphs
kind: issue
title: placement.rs asserts exact / by bits / D9-deterministic in nine separate doc paragraphs, none the authority for any other, and is 60% prose
status: open
opened: 2026-09-11
refs: [2375]
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
bit. Eight method docs that used to state one of those now carry a
one-clause cross-reference and their own semantics only. Two of the nine
were folded rather than moved: `rotate_then_translate`'s
normalize-here argument and the "decided direction normalizes by the
same expression" paragraph are one hazard — *removing this normalization
costs the bit agreement* — and are now one paragraph at the site, where
something would go wrong without it.

The section also names **where each claim is guarded**, which no site
did before: `affine_at_f64_carries_the_stored_bits` and
`compose_is_the_affine_product_to_the_last_multiply_add` in this
module's tests, and — because the bit agreement needs a whole document —
`r1_the_placement_frame_matches_the_transform_node_bit_for_bit` in this
crate's `asm2a_instantiate` suite. That last one already existed and
nothing in `placement.rs` pointed at it.

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
closing it.** `placement.rs` went 430 -> 454 lines: doc lines 157 -> 170,
code lines 242 -> 253 (the eleven `#[must_use]` attributes). The one home
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
