---
id: boolean-error-has-no-fieldless-kind
kind: issue
title: topo::BooleanError has no fieldless kind, so editor-core's checks door degrades it to reason: String — the PathErrorKind shape, one crate down
status: closed
opened: 2026-09-01
github: 1491
refs: [1490]
branch: fix/boolean-error-kind
pr: 1806
closed: 2026-09-04
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

Two rows pin it, and a style review replaced the first draft of both.
`topo`'s `each_kind_has_an_arm_and_each_built_arm_projects_to_its_own_kind`
carries an exhaustive match over `BooleanErrorKind` — so a phantom
variant fails to COMPILE, by name, at `error[E0004]`, in the crate that
owns both enums — plus 34 constructed errors whose projected kind is
compared against the variant name `Debug` prints for the error itself,
which catches a mis-projected arm with no expected value written down
twice. `editor-core`'s
`the_separation_door_carries_the_class_of_the_error_it_saw` pins the
door's own constructor: the prose is the kernel's sentence whole and
the kind is the arm the error IS. Verified red: a planted phantom
(E0004, names the variant), a planted `Self::GermFrameCylinderPinch =>
GermFrameUnsupported`, and a kind hardcoded at the door.

**What the first draft got wrong, and it matters more than what it
got right.** The original guard read the two enums and the projection
out of the module's SOURCE TEXT. The review planted two pieces of
ordinary Rust and both broke it: a `/** ... */` doc comment on a
variant read as a variant named `Nothing`, and an unbalanced `{` inside
a `/* */` comment truncated the scan silently. Both failed with a
message telling the author to delete a phantom variant that did not
exist — a plausible false accusation against correct code. That is the
third instance of
`work/issues/source-scanning-censuses-are-a-tripwire-on-ordinary-rust.md`
and the worst of them; the scanner is gone rather than patched, and
both plants are now inert.

**The direction still unguarded.** The compiler sees error → kind
through `kind()`, and now kind → error through the exhaustive visit.
What no guard in the tree closes in general is **pairing**: an arm
projected to the wrong kind type-checks. Here it is closed only for the
34 arms a test cheaply constructs; the 7 whose payload nests another
crate's typed refusal (`Euler`, `CrossingInsertion`, `Join`, `Merge`,
`Revert`, `GraftRecertify`, and the `DeclaredContact` cusp arm) are
unchecked, and nothing reds if a future arm is simply absent from that
list — the row measures what it builds and accuses no one of anything
else. `kind-mirrors-have-no-single-declaration` carries the general
fix and now records the corrected picture: `PathErrorKind` IS guarded
(`pncad-py/src/tags.rs:88`'s `path_error_tag`), `VerbKind::ALL` is the
census precedent, and `AttrKind` is the one with no exhaustive
consumer anywhere (measured: zero `AttrKind::X =>` arms tree-wide).

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
program's `keep_out`; `crates/pncad-py/src/py/checks.rs` and
`crates/pncad-py/src/tests.rs` — `lib`, both one-line threads of the
new field. `crates/topo/src/lib.rs` (the re-export) is unowned;
`crates/editor-core/src/checks.rs` is this program's.
`crates/editor-core/tests/dsc_checks.rs` was touched by the first
draft and is back to `main` byte-for-byte: the claim moved into
`checks.rs`'s own test module, so `tcost`'s fence is no longer
crossed.

**Not taken, and filed rather than left in prose.** The Python surface
still reads the reason string and publishes no boolean-kind tag, so the
defect this item describes is still true one door out —
`boolean-kind-not-published-at-the-python-door`, which names
`path_error_tag` as the written template and both reasons this unit did
not carry it (LIB's fence; 41 FFI names is a vocabulary decision).
