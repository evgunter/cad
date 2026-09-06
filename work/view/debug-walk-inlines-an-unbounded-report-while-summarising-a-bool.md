---
id: debug-walk-inlines-an-unbounded-report-while-summarising-a-bool
kind: issue
title: the new session dump inlines a whole ChecksReport and drops resolver, which is the summarise-or-print taste the derive was rejected for lacking
status: open
opened: 2026-09-06
refs: [2093]
---



Found by the style review of #2093.

The argument that rejected the derive is *"a derive cannot summarise,
which is the one thing the existing walk's taste does (`states`,
`gesture`)"*. The walk that shipped applies that taste unevenly, and
the three places it does not are all reachable from one `{:?}` on a
session.

## `checks` is inlined and is unbounded

`crates/viewer/src/session.rs:444` renders `LandedRun::checks` in
full, and `DocSession`'s walk reaches it now (`session.rs:1971`
renders `derived`, `:329` renders `landed`, `:444` renders `checks`).
`ChecksReport` (`crates/editor-core/src/checks.rs:475-483`) is
`{ findings: Vec<CheckFinding>, skipped: Vec<CheckId> }` — one entry
per finding, no bound. `at_rest` beside it carries a `String` of the
gate's whole rendered refusal
(`session.rs:463-476`, `AtRestBadge::Refused { message }`).

The old dump printed neither: it carried `landed_generation` and
nothing else about the run. So the change moves an unbounded rendering
into a dump that previously had none — and `ChecksReport` has a
`Display` that summarises exactly this (`checks.rs:486-491`,
*"checks: {} finding(s)"*), i.e. the summary the walk's own taste calls
for exists and is not used. The PR's behaviour-change paragraph names
"the four verdicts" without naming that one of them is a vector.

## `resolver` is dropped where the same walk renders presence

The walk renders `gesture` as `gesture.is_some()`, `scratch` as
`scratch.is_some()`, `body` as `body.is_some()` — three fields it
will not print, carried as the one bit that matters. `resolver`
(`session.rs:203`) is the same shape and is dropped entirely, though
its own field doc says the bit is behavioural: *"A session over an
in-memory document carries no resolver, and its instantiate nodes
refuse typed."* `DocSession::resolve_dir` (`:736-738`) already
computes a cheaper-than-presence summary.

Whether to carry it is a judgement. What is odd is that the walk makes
the opposite judgement three times in the same function body and the
doc above it gives no reason for the fourth.

## `Gesture` is passed over because it derives, which is the hazard

The sibling check says *"`Derived`'s other siblings need nothing:
`Gesture` (`session.rs:162`) derives `Debug`"*. `Gesture`
(`session.rs:161-170`) holds `base: Doc<ProfileProgram>` — a whole
recipe DAG — so its derive is precisely the *"prints the entire recipe
DAG at every `{:?}`"* outcome the same PR gives as the reason not to
derive on `Derived` and `LandedRun`. The session's walk never reaches
it (it renders `gesture.is_some()`), so nothing is broken today; the
sibling check reads the derive as the answer where the PR's own
argument reads it as the problem.

## `finish()` now says two things at once

`Derived`'s walk ends in `finish()` (`session.rs:331`) on the new
local rule *"no `_` arms above it"*, while rendering `scratch` as a
`bool` in place of a `Doc`. `std`'s meaning of the pair is about
whether the dump shows all the fields, so a reader who knows `std` and
not this crate's README reads `Derived { … scratch: false … }` as a
complete rendering of a document-shaped field. The distinction the PR
draws — summarised-and-rendered versus not-carried — is real and is
the good part of the change; `finish`/`finish_non_exhaustive` is a
two-valued marker being asked to carry it.
