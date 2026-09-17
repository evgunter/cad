---
id: loud-skip-row-did-not-stop-a-lane-verifying-the-wrong-build
kind: issue
title: The loud-skip row makes the app-feature gap visible to a CI log and not to a lane, and a lane verified an entirely app-gated diff without it
status: open
opened: 2026-09-06
refs: [loud-skip-marker-says-two-modules-and-there-are-six, 1848, 2026]
---

Found by #2026's style review, against #2026's own lane.

## What happened

`crates/viewer/src/lib.rs` gates `app`, `pane`, `drafts`, `forms`,
`gpu` and `widgets` behind `#[cfg(feature = "app")]`; `frame` is
unconditional. #2026's diff touched `app.rs`, `pane/create.rs`,
`pane/view.rs`, `pane/viewport.rs` and `frame.rs` — **every file but
one behind that gate** — and a plain `cargo test -p viewer` compiles
none of them.

`lib.rs` carries a row built for exactly this. `app_lane_skipped_no_app_feature_coverage_here`
is a `#[cfg(all(test, not(feature = "app")))]` test whose entire payload
is a `println!` saying that every `app`-gated module is absent from
this build. Its own doc states the purpose: *"a reader of a
default-feature run meets the absence instead of inferring it."*

**The reader met it and inferred nothing.** The row printed, the run
was green, and the diff was unverified.

## Why this is not the closed item

`loud-skip-marker-says-two-modules-and-there-are-six` (closed, #1848)
was about the marker going STALE — it named two modules when there
were six. That is fixed: the marker names the feature, enumerates
nothing, and cannot go stale when the `cfg` block changes.

This is a different failure. The marker is **accurate and was read past
anyway.** So the fix that item took — make the sentence true — is
orthogonal to whether the sentence does any work.

## Why it did not work, which is the filable part

Three properties, and each is defensible on its own:

1. **It is evidence, not a gate.** By construction: *"This row closes
   no gate and cannot fail."* A row that cannot fail cannot stop
   anything.
2. **It fires on the RUN, not on the DIFF.** It says "this build has no
   app modules". It cannot say "and you have edited four of them",
   because it does not know what was edited. The signal a lane needs is
   the conjunction, and nothing computes the conjunction.
3. **It is one line in a passing run's stdout.** `cargo test`'s output
   for a green run is scrolled past. The row's whole delivery mechanism
   is a channel that is only read when something is red.

The row is doing less work than its doc claims, and the claim is worth
correcting or the mechanism worth changing — **not both by a lane, and
not in a PR that is touching the gated files**, which is how this one
was found.

## What might actually close it

Unranked, and each has a real cost:

- **A gate at the diff, not at the run.** A CI step that fails a PR
  touching an `app`-gated path whose checks did not include a
  `--features app` job. Knows the conjunction; needs a path list, which
  is the hand-kept enumeration #1848 removed, arriving one layer out.
- **Make the default-feature `viewer` job refuse to be the only one.**
  If `cargo test -p viewer` is never a complete answer for this crate,
  the honest thing may be for the crate's dispatch prose to say so and
  for the row to point at the command that IS complete.
- **Weaken the doc.** Say the row is a note in a log and not a
  safeguard, which is what it is, and stop expecting it to catch this.

## Where this belongs

The row is `crates/viewer/src/lib.rs`, VIEW's territory. The conjunction
gate is CI's, and `loud-skip-marker-is-a-hand-kept-idiom`
(`work/tcost/`, open) already owns the eight-copy idiom across four
crates — this is evidence for that item too, but the failure here is
about efficacy rather than duplication, so it is filed rather than
appended.

