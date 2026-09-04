---
id: debug-in-prose-at-offset-fit-and-voids
kind: issue
title: two more Display impls render a struct-shaped payload through Debug: geom-brep's SplineError and topo's RevertError
status: open
opened: 2026-09-04
---


Found by FIX's `prose-gate-has-no-mechanical-guard` unit (PR 1809) —
by the census it built, which is the first mechanical sweep of this
class rather than a grep. Reported by the lane per
`docs/prompts/implementer-discipline.md` §6 and placed here by the FIX
orchestrator.

## The two

- **`crates/geom-brep/src/offset_fit.rs:373`** — `OffsetFitError`
  renders a `SplineError`, whose variants are struct-shaped.
- **`crates/topo/src/boolean/voids.rs:191`** — `VoidInsertError`
  renders a `RevertError`, likewise.

## Reachability is NOT established, and that is the first question

`crates/pncad-py/src/errors.rs`'s `reads_as_prose` rejects the
field-brace fingerprint `" { "`, and `py::typed_err` asserts it on
every raise — live under release. So whether either of these is
**cosmetic or a live panic** turns entirely on whether its error
reaches `typed_err`.

The lane did not trace it, and said so. Do that before anything else:
the sibling filing
`work/issues/debug-in-prose-at-blend-and-step-import.md` began as
"cosmetic or live, unknown" and both of its entries turned out to be
**live panics** once someone rendered the variants and walked the raise
path. Do not assume these are the milder case because they were found
by a sweep rather than by a crash.

## Why a sweep found them and three years of reading did not

These are the first two instances found **mechanically**. The three
before them were each found by someone running the door that panicked.
The census that found these judges declared payload types rather than
sampled values, which is why it reaches sites no test exercises.

Caveat worth carrying: that census is under review and its parser layer
has known defects (an or-pattern resolved at its last alternative only,
two item-head shapes misparsed, a cross-crate name collision producing
two false positives in its own allowlist). **These two hits are not
among the falsified ones** — they are ordinary named-field payloads
rendered through `Debug` — but confirm each by reading the declaration
rather than trusting the roster.

## Home

`work/issues/` — `crates/geom-brep/src/offset_fit.rs` is S-CERT's then
PROPS' by SHELL's `keep_out`; `crates/topo/src/boolean/*` is S-BOOL's.
Re-home by header edit, or split if the two programs prefer to take
them separately.
