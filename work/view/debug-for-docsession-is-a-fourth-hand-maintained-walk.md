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
what the dump will not carry.** `Debug` for `DocSession`
(`crates/viewer/src/session.rs:1952-1974`), for `Derived` (`:314-333`)
and for `LandedRun` (`:429-448`) each open with `let Self { … } = self;`
naming every field. A field added to any of the three is an
unbound-pattern error (E0027) in the walk, so it cannot be silently
absent from a dump; `DocSession` renders `Derived` as ONE field, so
`Derived`'s members travel with their declaration instead of being
listed a second time, which is what this file asked for.

A field the dump summarises is still bound and still rendered
(`scratch` as its presence, `body` as its presence, `states` as
`history.len()`, `gesture` as its presence). A field the dump will not
carry at all is bound to `_` rather than left out of the pattern — the
omission becomes a decision a reader can see, and the compiler still
forces the author to make it.

**What `finish_non_exhaustive` means now: exactly the `_` arms above
it, and nothing else.** It was standing in for "some fields, we are not
saying which"; it now stands for a list the compiler holds complete.

- `Derived` has no `_` arm — all five fields are rendered — so it
  `finish`es. The marker was a leftover there and is gone.
- `LandedRun` keeps it for `evaluation` and `doc`: the result DAG and
  the document it answers are the run's data, and printing them is the
  dump nobody can read. The five verdicts about that pair are rendered.
- `DocSession` keeps it for five: `eval` (a service, nothing to print),
  `requested_doc` (a whole document), and `tol`, `display`, `resolver`
  — values the session OWNS rather than knows, each with its own
  `Debug`, dumped by asking it rather than by inlining it here.

**Rejected: deriving `Debug` on `Derived` and `LandedRun`**, which is
what this file's *What resolving it looks like* proposed as its first
option. It compiles — `Doc`, `Evaluation`, `ProductError`,
`ChecksReport` and `AtRestBadge` all derive `Debug` today, so the
"blockers" this file names are not blockers at all — and that is the
problem: the derive would print the entire recipe DAG and the entire
result DAG at every `{:?}` on a session. The session holds big values
and the existing walk's taste is to summarise them (`states`,
`gesture`); a derive cannot summarise, so the un-derivable members
would need `#[derive]`-defeating wrappers to get back what the walk
already does plainly. The destructure buys the same compile error with
none of that.

**Rejected: a test over the rendering.** The property is a compile
error, and a test cannot assert one without a `trybuild`-shaped
harness this repo does not have. Verified instead by adding a witness
field to each of the four values and reading the compiler: E0027 at
`Derived`, `LandedRun`, `DocSession` and `PickCache`'s walks, four for
four, then reverted. **Nothing in the tree renders a `DocSession` or a
`PickCache` at all** — no `{:?}` over either, in `src`, `tests` or
`examples` — so no assertion moved, and the rendering change below is
unobserved by any row.

**The rendering did change, and that is a behaviour change.**
`selection` and `hover` are now inside a `derived` block instead of at
the top level; `landed_generation` is now `derived.landed`, a block
carrying the generation and the four verdicts beside it rather than the
generation alone; and `derived.scratch` and `derived.bounds` are
rendered for the first time — they are the two fields this file's
defect had ALREADY eaten, present in `Derived` and absent from every
dump.

**The body's own citation does not resolve, and is left as filed.**
`crates/viewer/src/session.rs:1663-1674` lands in `add_blend`'s body at
this branch's merge base, not on the walk; at that merge base the impl
was at `session.rs:1879-1891`, re-derived by reading the lines rather
than by shifting the number. The body is left as written because it
describes a defect that no longer exists and repointing it would aim a
historical sentence at present code; the live coordinates are the ones
above.

## The sibling, taken

`PickCache` (`crates/viewer/src/pickcache.rs:170-186`) is the same
shape one seam away — five fields listed by hand, `finish_non_exhaustive`
standing in for a `Box<dyn IndexService>` — and it took the same
destructure, with `seam: _` as the arm the marker now names. Its
rendering is unchanged.

**This file's sweep claim is stale and was true when written.** It says
`impl Debug for DocSession` is the only hand-written `Debug` under
`crates/viewer/src/` (`impl.*Debug for`, one hit; `finish_non_exhaustive`,
one hit). Re-run at the fix, both greps give **two** hits: `PickCache`'s
impl landed at `83fcb9540` (2026-09-06), a day after this file was
opened. A sweep is accurate as of its merge base and this one was not
re-run until now.

`Derived`'s other siblings need nothing: `Gesture` (`session.rs:162`)
derives `Debug`, and no other value in `session.rs` writes one.

## Sweep, and what the pattern could not match

Swept for `debug_struct|debug_tuple|finish_non_exhaustive` and for
`impl .*Debug for` across `crates/`. Inside `crates/viewer/`: two hits,
both fixed above. **Seven hits outside VIEW's fence**, every one the
same shape — a hand-listed field census in a `Debug` impl that ends in
`finish()`, so a new field is silently absent AND the rendering claims
to be complete:

- `crates/geom-core/src/spline/knots.rs:340` (`Span`)
- `crates/geom-core/src/spline/hull.rs:262` (`SplineCoeffs`), `:331`
  (`RationalCoeffs`), `:367` (`CoeffWindow`), `:396` (`RationalWindow`)
- `crates/geom/src/curves/nurbs.rs:221` (macro-generated, both windows)
- `crates/geom/src/surfaces/nurbs.rs:164` (`SurfaceWindow`)
- `crates/topo/src/param_source.rs:84` (`ParamSource`, a newtype — the
  weakest of the set)

Reported rather than filed: they are outside this program's fence, and
§6 of the implementer discipline puts a cross-fence finding in the
report and the PR body for the party with the whole board to place.

**What the pattern could not match.** It finds the `Debug` trait, not
the class. A hand-listed field census wearing any other hat — a
`Display` impl over a struct, a serialiser, a panel that inventories
fields, an equality written out field by field — is invisible to it.
Two adjacent things this pass looked at and did not sweep: every
`impl Display` in `crates/viewer/src/` (20 of them) is a `match` over an
enum's variants, which is exhaustive by construction unless it
wildcards, and a wildcarding one is
`refusal-rank-wildcards-the-display-fault-payload`'s subject, not this
file's; and the `PartialEq` impls beside the hull/window `Debug`s above
are hand-written field-by-field comparisons with the same silent-absence
hazard, unswept because they are in the same out-of-fence files.
