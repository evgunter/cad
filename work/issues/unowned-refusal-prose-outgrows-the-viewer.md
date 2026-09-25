---
id: unowned-refusal-prose-outgrows-the-viewer
kind: issue
title: topo/geom-brep: the refusals over 50 words on ground no program owns (Ev's concision request)
status: open
opened: 2026-09-22
refs: [error-and-check-text-overflows-its-region]
---


## What

Refusal `Display` arms on ground no program owns that the viewer shows
and that run past 50 words (literal words, before payload):

| words | site | arm |
|---|---|---|
| 54 | `geom-brep/src/certify.rs` `CertifyError::NotSecondOrderSeparated` | plus `COINCIDENCE_RECOURSE` |

The five `PcurveMintError` arms this row first listed
(`SingularChartJoint` 89, `LoopDiscontinuity` 75, `OuterSpansPeriod`
70, `LoopNotClosed` 56, `LoopWraps` 50) were rewritten at the source by
the CHROME `concision-chains` pass, with the whole enum: no arm now
opens with "pcurve minting:", and every arm is under 50 literal words.
The row stays open on the certify arm and on the key dumps below.

**Arena keys in two forwarded refusals (CHROME concision-chains,
2026-09-23).** `topo::ShellClassifyError` and `topo::MassPropsError`
(`topo/src/props.rs`, no owner) still name a shell or face by arena
key ("shell ShellKey(3v1)'s signed volume is definitely zero …"). They
reach the viewer through `NodeErrorKind::Shell(ShellError::Roles)` and
through the checks window's `CheckEvidence::Unsupported`, where the
key names nothing the person holding the mouse can find. The file sat
inside hunks two open PRs were reworking (#3049, #2861), so the pass
left them; `editor-core/tests/refusal_concision_chains.rs` lists the
two rows (`Shell/Roles`, `Check/Unsupported`) in `KERNEL_KEYED` with a
comment pointing here, and taking them out of that list is the check
that this is done.

**Stage prefixes in the same two, and in the certifier (CHROME
concision fix pass, 2026-09-23).** The feature tree's guard now reads a
stage prefix by its SHAPE (one or two lowercase words and a colon,
`test_utils::refusal::stage_prefixes`) rather than from a list, and it
found three on screen in files open PRs are reworking:

- `shell classification:` on every `ShellClassifyError` arm
  (`crates/topo/src/props.rs:1809-1818`), reached as above;
- `mass properties:` on every `MassPropsError` arm
  (`crates/topo/src/props.rs:218-240`), reached through
  `ShellClassifyError::Props` in the checks window;
- `certification:` on every `CertifyError` arm
  (`crates/geom-brep/src/certify.rs:389-488`), reached through
  `NodeErrorKind::Transform(TransformError::Certify)`.

`props.rs` is in #3049 and #2861, `certify.rs` in #2861. The guard's
`FILED` list admits exactly these labels on exactly the rows named
(`Shell/Roles`, `Check/Unsupported`, `Transform/Certify`); removing
those entries is the check that this is done.

`work.py territory` names no owner for `topo/src/pcurves.rs` or
`geom-brep/src/certify.rs`. `topo::BooleanError` (also unowned) was
rewritten whole by the filing PR and is held to the budget by its test.

## The standard

The standard these arms are held to is stated once, in
`work/chrome/error-and-check-text-overflows-its-region.md` (section
"The standard a refusal is rewritten to"), with its word budget and the
test that enforces it.

## How the census was taken, and what it could not see

A static pass over every `impl Display for` block in `crates/*/src`,
split into match arms, counting the words of each arm's string
literals (a `{…}` placeholder counts as one word; a named recourse
constant such as `COINCIDENCE_RECOURSE` is NOT expanded, so a
constant-carrying arm is longer on screen than its count). A realistic
payload adds to every count: a nested refusal (`{e}`, `{source}`)
renders its own arm inside this one. Arms at or above 50 literal words
are listed. The pass does not prove each arm is reachable from the
viewer — most reach it through `NodeErrorKind`'s forwarding arms
(feature tree fault line, status line) or through the checks window —
and a `Display` written outside `impl Display` (a helper returning a
`String`) is not seen.
