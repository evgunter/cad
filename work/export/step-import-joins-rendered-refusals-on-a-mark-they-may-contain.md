---
id: step-import-joins-rendered-refusals-on-a-mark-they-may-contain
kind: issue
title: two StepImportError arms join rendered refusals flat on a "; " the elements may carry
status: open
opened: 2026-09-16
priority: P3
cost: E
---


Found by the sweep of
`work/view/startup-notices-join-on-a-mark-a-prefs-notice-contains`
(VIEW, #2710), whose population was every flat join of already-rendered
sentences on a mark the elements are free to write themselves. Two hits
landed here.

## The two sites

`impl Display for StepImportError` (`crates/step-import/src/error.rs`):

- the `EdgeUncertified` arm renders `"{candidate}: {refusal}"` per
  attempt in a `for` loop and writes a bare `"; "` between them —
  `refusal` is a `topo::EulerOpError`'s own `Display`.
- the `TierInvalid` arm builds `format!("{e:?} — {e}")` per validation
  verdict and `verdicts.join("; ")`. The `{e:?}` half is a `Debug`
  dump, which std renders with `", "` between fields and which nothing
  bounds.

A refusal whose own sentence writes a `"; "` makes the list read as one
item more than it has, and the `TierInvalid` arm states its count in
the same sentence, so a reader who checks the count against the pieces
is told the message is inconsistent with itself.

## Latent today, and held by nothing

No `EulerOpError` arm writes a `"; "` at the moment — checked by
scanning the string literals in `Display for EulerOpError`
(`crates/topo/src/euler.rs`). That is a property of those sentences and
not of any type: a new arm, or a widened error set, re-opens it with
nothing going red. **The scan has a blind spot**: a `"; "` split across
a multi-line string-literal continuation (`"…;\` / `` ` b…"``) is two
literals and neither contains the pair, so it would not have been seen.

The same shape is being fixed on VIEW's side twice — once by narrowing
the element type (`display::AdmissionFault`, #2693) and once by moving
the elements up a level so a door-held boundary mark applies (the
startup line). `crates/viewer/README.md`'s *"The line is composed at
two levels and they are two marks"* section is the written-up version
of both, if either shape suits here.

## Home

EXCH's: `crates/step-import/src/error.rs`.
