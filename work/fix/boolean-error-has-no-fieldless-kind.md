---
id: boolean-error-has-no-fieldless-kind
kind: issue
title: topo::BooleanError has no fieldless kind, so editor-core's checks door degrades it to reason: String — the PathErrorKind shape, one crate down
status: review
opened: 2026-09-01
github: 1491
refs: [1490]
branch: fix/boolean-error-kind
pr: 1806
---

## From GitHub issue 1491

Opened 2026-09-01; 0 comments.

(SMELL-UV orchestrator) Filed as the durable home for a deviation PR 1490's sweep disclosed and could not carry — the fix lives in `crates/topo`, Track Q's fence (S-BOOL's per the register).

**The defect.** `editor-core/src/checks.rs` (~:275-282) carries a boolean refusal as `reason: String` because `topo::BooleanError` is `#[derive(Debug)]` only — no `PartialEq`, no fieldless kind — so a typed refusal degrades to prose at the checks door, and any consumer wanting the class back must substring-match. This is the exact shape §D row `D39` closed for `profile::PathError` in PR 1490: a fieldless kind enum (`PathErrorKind`, 28 variants, projected by an exhaustive `kind()`), carried beside the rendered prose, letting tests and doors match on type. The site's own comment discloses the degradation.

**The fix shape, established by precedent.** A `BooleanErrorKind` + exhaustive `BooleanError::kind()` in `topo` (the `Attr`/`AttrKind` and now `PathError`/`PathErrorKind` convention), then the checks door carries the kind beside the prose. PR 1490's review notes the one-way residue of a hand-mirrored kind enum (a phantom kind variant reds only in a downstream exhaustive match) and that a `transition_table!`-style single declaration closes that direction — worth deciding once for this and the `NodeErrorKind` sibling (SMELL-UV's §D row for that is being minted).

Not taken by SMELL-UV: `crates/topo` is outside both its fences. Per the partition rule, filing is the handoff.

## Home

`work/bool/` — the issue names the fix as living in `crates/topo`, Track Q's fence, which S-BOOL's charter claims (SMELL track Q's topo rows).

## Closed

**What landed.** `topo::BooleanErrorKind` — 41 fieldless variants, one
per `BooleanError` arm — plus `BooleanError::kind()`, an exhaustive
projection, both in `crates/topo/src/boolean/mod.rs` and re-exported
from `topo`'s root. `CheckEvidence::SeparationUnavailable`
(`crates/editor-core/src/checks.rs`) now carries
`kind: topo::BooleanErrorKind` beside `reason: String`, built from the
same error (`source.kind()` / `source.to_string()`), so the class
rides into a `Clone + PartialEq` finding record the error itself
cannot enter. The site's disclosure comment is replaced by the
invariant: prose is what a reader reads, kind is what a consumer
matches, and neither half is a substring hunt through the other.

Two rows pin it. `topo`'s
`the_kind_enum_names_exactly_the_error_arms` reads both declarations
and the projection out of the module's own source and asserts the
three name lists agree — verified red on a planted phantom kind
variant and on a planted mis-paired arm. `editor-core`'s
`separation_unavailable_carries_the_kernel_class_beside_its_prose`
pins that the finding forwards the kernel's own sentence whole, that
the class is recovered by pattern, and that two findings with
byte-identical prose and different classes compare unequal.

**The direction still unguarded.** The compiler sees the error→kind
direction only. The source-scan row closes the kind→error (phantom)
and the mis-pairing directions FOR THIS PAIR, and its blind spot is
stated at the row: it is a text scan of one file, so a macro-expanded
variant or an arm whose formatting the patterns misread is invisible
to it. `PathErrorKind` and `AttrKind` have no equivalent row at all.
The single-declaration form is filed as
`kind-mirrors-have-no-single-declaration`.

**The sweep** (typed error degraded to `String` at a consumer door),
one line each:

- `editor-core/src/checks.rs:282,291` `SeparationUnavailable` —
  FIXED, this unit.
- `editor-core/src/checks.rs:476,727` `ChecksError::Product` from
  `product::ProductError` — same file, same door, needs a
  `ProductErrorKind` in `product.rs`, which is off this unit's fence;
  filed as `checks-product-refusal-degrades-to-string`.
- `viewer/src/sketch.rs:633,909` `PreviewError::Geometry { rendered }`
  from a `PathError<f64>` — a live instance with the kind ALREADY
  minted (`PathErrorKind`, PR 1490) and not carried. CHROME/VIEW's
  fence; reported to the orchestrator, not filed here.
- `viewer/src/tree.rs:75` `RowStatus::Failed { message }` from
  `NodeError` — the `NodeErrorKind` sibling this item names; two
  fences (the kind belongs in `editor-core`, the door is CHROME's).
  Reported.
- `editor-core/src/eval/parts.rs:92,110,117` `PartFault`'s
  `Unresolved`/`PartRootFailed`/`PartProduct` messages — DOCM's
  (`eval/parts.rs` is its glob). `PartRootFailed` already carries a
  typed `cause` for the chaining case; the `message` arm is the
  residue. Reported.
- `viewer/src/session.rs:1719` `AtRestBadge::Refused { message }` from
  the at-rest assembly gate's typed refusal — CHROME/VIEW's. Reported.
- `editor-core/src/clearance.rs:614` `WitnessUnverified { what }` —
  NOT this class: the prose describes what an `f64` rebuild found, not
  a typed error rendered.
- `pncad/src/workspace.rs:78,149`, `viewer/src/docio.rs:108,113`,
  `viewer/src/prefs.rs`, `editor-core/src/persist/mod.rs:216,251,272`
  — foreign errors (`std::io`, `serde_json`, the OS entropy source),
  not typed in-tree refusals. Out of the class.
- `editor-core/src/parse.rs:95,109,237`,
  `viewer/src/session.rs:1052` — `text: String` is source text, not a
  rendered error. Not the class.

**What the sweep could not match.** The patterns were the field shape
(`^\s+(reason|message|rendered|because|detail|text|err|error|what|prose|display|msg):\s*String,`)
and the construction shape (`: <expr>.to_string()`), so: a String
payload under a field name outside that list; a degradation that goes
through `format!`, `Into`/`From`, a helper function, or a `Box<str>` /
`Cow` / `Arc<str>` rather than `.to_string()`; a whole error rendered
into a struct rather than an enum arm; and — the one a rename would
hide — the reverse spelling, a site described by yesterday's field
name. The reverse grep for the retired spelling was run
(`SeparationUnavailable`, whole tree, `.rs` and `.py`) and found only
the tag table, the Python accessor and the one test.

**And the blind spot neither grep sees**: a consumer recovering the
class by matching a FRAGMENT of the rendered message. A `contains(`
sweep over the tree found no production consumer doing that to a
boolean refusal — the hits are test assertions on message content
(`topo/src/boolean/ops.rs:2584`, `mesh/tests/r2_bool_door.rs:93`,
`mesh/tests/fitted_refusals.rs:286`), which is a different thing. A
consumer that reconstructed the class from a fragment SPELLED
differently from any of these patterns would still be invisible.

**Fences crossed** (`work.py territory --base origin/main`):
`crates/topo/src/boolean/mod.rs` — `bool` (S-BOOL), announced in this
program's `keep_out`; `crates/editor-core/tests/dsc_checks.rs` —
`tcost`; `crates/pncad-py/src/py/checks.rs` and
`crates/pncad-py/src/tests.rs` — `lib`, both one-line threads of the
new field. `crates/topo/src/lib.rs` (the re-export) is unowned;
`crates/editor-core/src/checks.rs` is this program's.

**Not taken.** The Python surface reads the reason string and does not
publish a boolean-kind tag. Minting 41 FFI names is a new published
vocabulary and the phantom-tag hazard `PathErrorKind`'s doc names; it
waits for a caller that wants it.
