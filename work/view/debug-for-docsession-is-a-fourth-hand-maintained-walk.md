---
id: debug-for-docsession-is-a-fourth-hand-maintained-walk
kind: issue
title: Debug for DocSession lists fields by hand and is non-exhaustive, so a field added to Derived is silently absent
refs: [session-clearing-walk-is-hand-maintained-three-times, 1885]
status: closed
opened: 2026-09-05
closed: 2026-09-06
branch: view/debug-walk
pr: 2093
---


Found by the #1885 style review (S3), one screen below the walk that
PR collapsed. Pre-existing.

## The duplication

`impl Debug for DocSession` (`crates/viewer/src/session.rs:1663-1674`)
names its fields by hand — `generation`, `landed_generation`,
`selection`, `hover`, `states`, `gesture`, `path` — and ends in
`finish_non_exhaustive()`. So a field added to `Derived` (the value
`Open` and `NewDocument` now reset by construction) is silently absent
from every debug rendering, exactly the way it used to be silently
absent from a clearing walk: nothing red, nothing missing at compile
time, and the omission is only visible to someone who reads a dump and
wonders what is not in it.

Neither `Derived` nor `LandedRun` derives `Debug`, which is what keeps
this impl hand-written. `Selection`, `Hovered`, `Generation` and
`PathBuf` all implement it already; the blockers are the `Box<dyn
EvalService>` and the document values, which is what
`finish_non_exhaustive` is standing in for.

## What resolving it looks like

Derive `Debug` on `Derived` and `LandedRun` (or write one impl for
each, once) and let `DocSession`'s render name the block rather than
its members, so the members travel with the declaration. Whether the
outer impl stays non-exhaustive is a separate question: it is honest
about `eval` and the documents, and it should stay non-exhaustive for
those and no longer for the fields a value now carries.

**Sweep before calling it fixed.** `impl Debug for DocSession` is the
only hand-written `Debug` under `crates/viewer/src/` today (grep
`impl.*Debug for`, one hit; `finish_non_exhaustive`, one hit), so the
sweep is currently trivial — but the pattern that matters is a
hand-listed field census of any kind, not the `Debug` trait, and that
grep does not find `Display` impls, serialisers, or panel inventories
that enumerate the same fields. Re-run both greps at the fix.
## Closed

**The mechanism: an exhaustive destructure at each value, and `_` for
what the walk will not carry.** `Debug` for `DocSession`, `Derived`,
`LandedRun` and — the sibling — `PickCache` each open with
`let Self { … } = self;` naming every field, so a field added to any of
the four raises **E0027**, pattern-does-not-mention-field, in its own
walk. `DocSession` renders `Derived` as ONE field, so `Derived`'s
members travel with their declaration instead of being listed a second
time, which is what this file asked for. The argument has one home,
`crates/viewer/README.md`'s *The dump is held to the same declaration*;
the four walks state their own `_` arms and point at it.

`PickCache::forget` takes the same destructure — see *The sibling and
the in-fence instance*, below.

**`Derived::none`'s error is E0063, not E0027**, and the first version
of this walk's prose said otherwise. It is a struct literal, so its
error is missing-field-in-initializer; the property is the same at both
sites and the error class is not, which matters because a reader
looking for one and finding the other concludes the mechanism is not
there. Corrected in both walks and in the README.

**What `finish_non_exhaustive` means now: exactly the `_` arms above
it, one reason each.** It was standing in for "some fields, we are not
saying which"; it now stands for a list the compiler holds complete.
`Derived` has no `_` arm, so it `finish`es and the marker is gone
there. `LandedRun` keeps it for `evaluation` and `doc`, the result DAG
and the recipe DAG it answers. `DocSession` keeps it for four: `tol`
(`Tol(())`, a ZST with no content), `eval` (a `dyn` service
implementing no `Debug`), `requested_doc` (a whole recipe DAG) and
`display` (not derived from the document, as large as its hidden and
moved sets, and reachable through `DocSession::display`). `PickCache`
keeps it for `seam`.

The first version of that justification said `tol`, `display` and
`resolver` were *"values the session owns rather than knows — each with
its own `Debug`, dumped by asking it"*. Three things were wrong: `tol`
is a ZST and its `Debug` says nothing; **there is no accessor for
`resolver`** (`DocSession::resolve_dir` hands back `Option<&Path>`, the
directory, not the resolver); and `resolver` should not have been a `_`
arm at all — see the next paragraph. The reason is now per field.

**`resolver` is rendered, as its presence.** The walk carries
`gesture`, `scratch` and `body` as `is_some()` — the one bit that
matters about a value it will not print — and dropped `resolver`, whose
own field doc says the bit is behavioural: a session over an in-memory
document carries no resolver and its instantiate nodes refuse typed.
Making the opposite judgement three times in one function body and the
fourth silently was the defect this file is about, one level up.

**`checks` is summarised, not inlined.** `ChecksReport` is
`{ findings: Vec<CheckFinding>, skipped: Vec<CheckId> }` with no bound,
and the first version of this walk printed it whole into a dump that
had previously carried `landed_generation` and nothing else about the
run. It is now its two counts. `at_rest` is printed: it is one badge
carrying at most one `String`, which is the same size as `path`. **The
rule the walks follow, stated because it was being applied unevenly:
a `Vec`, a document or a DAG is summarised; a single value is
printed.**

## Rejected

**Deriving `Debug` on `Derived` and `LandedRun`** — this file's own
first suggestion. It compiles: `Doc`, `Evaluation`, `ProductError`,
`ChecksReport` and `AtRestBadge` all derive `Debug` today, so the
"blockers" named above are not blockers, and the style review of #2093
proved it mechanically by replacing both walks with `#[derive(Debug)]`
and compiling. **The reason to reject it is DAG size**, not a general
principle that a derive cannot summarise: `Evaluation` is the result
DAG and `Doc` is the recipe DAG, and a derive prints them in full at
every `{:?}` on a session with no way to say otherwise. The earlier
form of this argument — *"a derive cannot summarise"* — was true in the
abstract and was not honoured by the walk that shipped beside it, which
inlined `ChecksReport`. Both were fixed rather than either being kept.

**A test over the rendering.** The property is a compile error and a
test cannot assert one without a `trybuild`-shaped harness this repo
does not have. Verified instead by adding a witness field to each value
and reading the compiler — E0027 at every walk — then reverting. The
style review re-ran that experiment independently and got the same, and
went further: deleting both `session.rs` walks and running
`cargo check --workspace --all-targets` gives zero errors, so nothing
in the workspace requires either impl and the rendering change below is
genuinely unobserved.

**Moving `DocSession`'s walk to its declaration.** It sits 1,779 lines
below `DocSession` at the end of the file, and the other three sit
beside theirs. Declined here because moving it is a pure move inside a
PR whose warrant is a mechanism change, and this program spent
2026-09-06 learning not to mix those; filed as
`four-debug-walks-are-spelled-and-placed-two-ways`.

## The rendering changed

`selection` and `hover` are now inside a `derived` block;
`landed_generation` is now `derived.landed`, a block carrying the
generation and the verdicts beside it; `derived.scratch`,
`derived.bounds` and `resolver` are rendered for the first time —
`scratch` and `bounds` are the two fields this file's defect had
ALREADY eaten. `PickCache`'s rendering is unchanged.

**Nothing in the tree renders a `DocSession` or a `PickCache`** — no
`{:?}` over either in `src`, `tests` or `examples`, and the review's
delete-and-compile confirms it — so no assertion moved.

## The sibling and the in-fence instance

`PickCache` (`crates/viewer/src/pickcache.rs`) is the same `Debug`
shape one seam away and took the same destructure, with `seam: _` as
the arm the marker names. Its rendering is unchanged.

**`PickCache::forget` is an instance of this file's own stated class,
eleven lines below that walk, and the first sweep missed it.** This
file defines the class as *a hand-listed field census of any kind, not
the `Debug` trait*, and then swept the trait. `forget` cleared four of
`PickCache`'s five fields by hand; it now destructures, with `seam: _`
as the arm that must stay. It is the instance with a live consequence
rather than a readability one: its own doc argues that leaving
`attempted` set is what lets a late build install "an index of a
document nobody is looking at, over a scene of a third one, with
nothing running and nothing said."

Two further in-fence instances stay filed rather than taken —
`impl PartialEq for Camera` and `DisplayState::clear` — on
`field-censuses-inside-view-survived-the-debug-sweep`, which also
carries the five struct-shaped `Display` impls this pass turned up.

`Derived`'s other siblings: `Gesture` (`crates/viewer/src/session.rs`)
derives `Debug` and no walk here reaches it — `DocSession` renders
`gesture.is_some()`. That derive is not an endorsement: `Gesture` holds
`base: Doc<ProfileProgram>`, so anything that ever dumps one prints a
recipe DAG, which is the outcome the paragraph above gives as the
reason not to derive. Named, not fixed, because nothing renders it.

**This file's sweep claim is stale and was true when written.** It says
`impl Debug for DocSession` is the only hand-written `Debug` under
`crates/viewer/src/` (`impl.*Debug for`, one hit; `finish_non_exhaustive`,
one hit). Re-run at the fix, both give **two**: `PickCache`'s impl
landed at `83fcb9540` (2026-09-06), a day after this file was opened.

## Sweep, and what the pattern could not match

**Rule: `grep -rnE "impl[^=]*\bDebug\b for" crates/`, plus
`grep -rn "debug_struct|debug_tuple|finish_non_exhaustive" crates/`.**
Inside `crates/viewer/`: two hits, both fixed. Outside it: **eight
rows, nine concrete impls** — `crates/geom/src/curves/nurbs.rs:221` is
inside `nurbs_curve!`, invoked twice (`:1492` for `CurveWindow2`,
`:1493` for `CurveWindow3`).

Seven of the eight rows are the class, and each ends in `finish()`
rather than `finish_non_exhaustive()` — a completeness claim over a
hand-listed census, strictly worse than what this file fixed. **The
eighth is not the shape**: `crates/topo/src/param_source.rs:84` is a
`write!` over the newtype `pub struct ParamSource(Arc<[u8]>)`, with no
`debug_struct`, no `finish()` and no census to fall behind. The first
version of this list said "seven" over eight rows and put
`param_source.rs` under a sentence claiming every one ends in
`finish()`.

Filed, on the VIEW orchestrator's direction, as
`work/issues/hand-listed-debug-censuses-in-geom-core-geom-and-topo.md`
— §6 makes reporting the filing act outside the fence and gives the
file to the party with the whole board, and this program established
on 2026-09-06 that a §6 report alone is not a durable artifact.

### The blind spot, re-derived

The first version of this paragraph said *"every `impl Display` in
`crates/viewer/src/` (20 of them) is a `match` over an enum's
variants"*. **That was an unverified negative and it is false** — §5
makes the blind-spot sentence part of the receipt, which is the worst
place for one. What the grep actually gives:

**Rule: `grep -rnE "impl[^=]*\bDisplay\b for" crates/viewer/src`.**
**36** impls across 19 files. Resolving each subject against its
declaration in the crate: **31 are over enums** — `match`es over
variants, exhaustive by construction unless they wildcard, and a
wildcarding one is
`refusal-rank-wildcards-the-display-fault-payload`'s subject. **Five
are over structs and read fields by hand**, which is this file's class
in another hat:

| site | value | reads | of |
|---|---|---|---|
| `crates/viewer/src/prefs.rs:304` | `StoreError` | `doing`, `because` | 2 |
| `crates/viewer/src/frame.rs:320` | `Message` | `text` | 2 |
| `crates/viewer/src/frame.rs:706` | `Withdrawal<'_>` | `kind`, `withdrawn` | 2 |
| `crates/viewer/src/frame.rs:1847` | `Disagreement` | `from_gpu`, `from_ray` | 2 |
| `crates/viewer/src/blend.rs:128` | `BlendTarget` | `node`, `body` | 2 |

None is missing a field today. Four render every field they have, and
`Message`'s omission of `subject` is deliberate — the subject routes
the message, it is not part of its text. What none of them has is a
compile-time tie, so a third field added to any is silently unrendered;
`Disagreement`'s own doc argues that both of the two it renders are
load-bearing, which is the sentence a third field would falsify. Not
taken here: they are a `Display` question with no defect today, and
they go on
`field-censuses-inside-view-survived-the-debug-sweep` with the other
in-fence instances.

**What both patterns still cannot match.** They find the `Debug` and
`Display` traits, not the class. A hand-listed field census wearing any
other hat — a serialiser, a panel that inventories fields, a `PartialEq`
written out field by field, a clearing walk — is invisible to both.
Three such were found only by reading rather than grepping
(`PickCache::forget`, `impl PartialEq for Camera`,
`DisplayState::clear`), which is the measure of the blind spot: the
greps in this file are the wrong instrument for the class this file
defines, and nothing here claims otherwise any more.

## Citation receipt

**Rule: every coordinate of the form `path:N`, `path:N-M`, or a bare
`:N` / `:N-M` inheriting the path named immediately before it,
appearing in a line this branch ADDS to any Markdown file** — whole
file for the two files this branch creates, added lines only for the
rest. Stated separately from its result, and re-run after the last edit
on the branch with every hit read by `sed -n Np`.

**Two blind spots, stated because a rule without one is the same
unverified negative as a sweep without one.**

- It matches a coordinate written as a coordinate, so a line number
  reaching prose in words — `work/view/log.md`'s *"the entry at line
  1833 of this log"*, the coordinate the sibling-check paragraph turns
  on — is outside it, and so is any citation by symbol name.
- It covers lines this branch adds and NOT the pre-existing bodies of
  the seven items the style review filed, whose `session.rs` and
  `pickcache.rs` coordinates this pass's own edits have moved. They are
  left as filed, on the same reasoning as the body of this file: they
  describe a state of the code that no longer exists, and repointing
  them would aim a historical sentence at present code. They resolve at
  `origin/review/2093-style-findings`, which is where a reader who
  needs them should look.

The count and its result are in the PR body, which is where they can be
re-run against the branch's final head.
