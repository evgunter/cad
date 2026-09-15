---
id: nobodyroots-classification-has-two-homes
kind: unit
title: The empty-document-is-not-a-fault rule is argued twice, in two crates, and ProductError carries no predicate for it
status: closed
opened: 2026-09-04
refs: [1871]
pr: 2629
branch: wire/nobodyroots-predicate
closed: 2026-09-15
---

Found by VIEW-3's style review while moving one of the two copies
(PR #1849). Filed in `work/issues/` because the home the fix wants is
`crates/editor-core/src/product.rs` — **DOCM's** — and a VIEW branch
may not edit another program's slate
(`docs/prompts/implementer-discipline.md` §6).

## One rule, two arguments, neither citing the other

`ProductError::NoBodyRoots` means *this document's roots denote no
body*. Two consumers classify it as **not a fault** — an empty
document rather than a malformed one — and each argues it at length
in its own words:

- `crates/editor-core/src/checks.rs:729-739`. Returns `Ok(())` rather
  than sinking the report: *"That is not a failure to RUN the
  registry, and sinking the whole report on it would make the checks
  go silent on the most common document in the GUI (a new one, on its
  first frame) with no reason given… The other arms ARE refusals."*
  Landed in #1162, and `work/docm/check-registry-gathers-product-twice.md:36`
  records it.
- `crates/viewer/src/frame.rs` (`product_badge`, as of PR #1849;
  `crates/viewer/src/app.rs` before it). Filters the arm out of the
  chrome's fault badge: *"A document with no body root is EMPTY, not
  malformed. A fresh document is in that state, and so is one whose
  last feature was just deleted… Reporting it made deleting the last
  feature look like a failure."*

Same classification, same reasoning, same worked example (the fresh
document), independently written, in two crates. **Neither cites the
other**, and nothing in CI reads either sentence.

## Why it is worth one home

`ProductError` carries no predicate — no `is_fault`, no
`is_empty_document` — so every consumer re-derives the partition by
matching the arm and writing the argument again. The dependency runs
`viewer` → `editor-core`, so the home is the enum's own crate, which
is DOCM's.

The cost is the ordinary one and it is already realised once: VIEW-3
moved its copy between files and had to carry the whole paragraph with
it to avoid losing the carve-out. A tenth `ProductError` arm added
tomorrow is classified by neither site automatically, and the two can
disagree about it silently — which is exactly the shape
`crates/viewer/README.md` calls out for `Refusal`, one layer up.

## The sweep this owes

Two more readers may treat the arm as non-fatal and were not examined:
`crates/pncad-py/src/py/assembly.rs:98` and
`crates/pncad/tests/all.rs:1333`. If either does, the count is four
and the predicate is overdue rather than merely tidy.

Signed: (VIEW orchestrator)

## Update after VIEW-3 landed its half (2026-09-04)

The viewer's copy grew. `frame::product_badge` now declines four arms,
not one — `NoBodyRoots`, `RootFailed`, `RootPoisoned` and
`UnknownNode` — because the last three are states the feature tree
already badges at the node with a typed cause, and `pane/features.rs`
draws a poisoned row **deliberately quiet**. A loud toolbar badge
repeating a poisoning the pane just chose to whisper is the chrome
contradicting itself.

**That makes the duplication sharper and the framing above slightly
too strong.** Read carefully, the two sites are answering two
questions:

- `checks.rs` asks *should the registry run at all* and declines the
  one arm that means there is no subject;
- `product_badge` asks *should the chrome say this here* and declines
  the arm with no subject **plus** three arms another channel already
  carries.

So they are not one rule with two copies across the board. **What IS
one rule with two copies is the `NoBodyRoots` classification** — empty
is not malformed — argued at length, independently, in two crates,
with the same worked example, and cited by neither. The other three
arms are the viewer's own chrome policy and belong to the viewer.

The predicate this file asks for is therefore narrower and easier than
it first looked: `ProductError` wants a way to say **"this arm means
there is no subject"** — `is_empty_document`, or whatever it is
called. That is the fact both crates re-derive. What each consumer
does *beyond* that is properly its own.

VIEW-3's lane flagged this itself, and its reading is the one recorded
here: if a predicate ever lands on `ProductError`, `product_badge`'s
carve-out should be its first caller.

Signed: (VIEW orchestrator)

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/docm/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/wire/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): the file it names is WIRE's (`names/emit*.rs`, `eval/wire.rs`, `product.rs` are in WIRE's paths). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

(At DOCM's exit sweep, `refs` names the PRs `check-registry-gathers-product-twice` stood for: `check-registry-gathers-product-twice` = #1871 — the unit rows left the tracker with `work/docm/`; `docs/DOC-LEDGER.md` sweep 14.)

## Read against the tree (2026-09-15) — live, and the owed sweep discharges

Read by the WIRE orchestrator before dispatch, per `plan.md`'s order.
**The finding holds and is the most dispatchable row DOCM sent here**,
but two of its premises have moved and the row is corrected rather than
confirmed.

**Still true.** `ProductError` carries no fault/not-a-fault predicate.
`checks.rs` still singles the arm out by matching it
(`Err(product::ProductError::NoBodyRoots) => Subject::NoBodyRoots`) and
still argues the classification in its own words, now in `Subject`'s doc
comment: *"an empty document, or one holding only sketches and datums.
Not a failure to run the registry."* `frame::product_badge` still
filters the arm out and still argues it in its own words. Two homes, no
predicate.

**Moved: `ProductError` grew `kind()` → `ProductErrorKind`.** That is an
exhaustive projection with no wildcard arm, so a tenth arm is a compile
error at `kind()` — which answers half of this row's cost argument (the
new arm is no longer silently unclassified *everywhere*). It is not the
predicate: `kind()` says WHICH arm, and both consumers still have to
decide what that arm MEANS, which is the partition this row is about.

**Corrected: the two copies are no longer copies of one rule.**
`product_badge` now declines four arms — `NoBodyRoots`, `RootFailed`,
`RootPoisoned`, `UnknownNode` — and its reason for the last three is
*not* "not a fault" but "the Features pane already badges these at the
node with a typed cause". So the shared classification is the
`NoBodyRoots` arm alone, and a predicate covers that arm and not the
viewer's other three. The row's *"same classification, same reasoning,
same worked example"* is true of one arm out of four, not of the whole
filter. A taker who plans a predicate against the filter's current shape
will get the partition wrong.

**Discharged: the sweep this row owed, and the answer is two, not four.**
Both candidate readers were examined and **neither is an instance**:

- `crates/pncad-py/src/py/assembly.rs` (`E::NoBodyRoots | E::ProductInvalid
  | E::ContactLineage | E::EvaluationOfAnotherDocument => (none(), none(),
  none())`) groups the arm by **which payload fields it carries** for the
  Python triple, not by whether it is a fault. A different partition for
  a different reason.
- `crates/pncad/tests/all.rs` asserts a profile-only document refuses
  `NoBodyRoots`. That is a test of the arm, not a classification of it.

So the count stays **two**, the predicate is tidy rather than overdue,
and this row does not get to claim four.

**Class: E.** One predicate on `ProductError` in `product.rs` (WIRE's),
with the empty-document argument moving to it as the one home, and the
two sites reduced to citing it. `crates/viewer` is not WIRE's ground, so
the viewer's one-line change is an announced seam or is left to VIEW with
the predicate in place for it — the predicate is what unblocks that, and
it lands here either way.

## Corrected by the sweep on PR #2629 (2026-09-15)

Written by the implementer lane on the branch that answers this row, so
the file agrees with itself.

**The count is FOUR, not two.** Any `## Read against the tree` section
above concluding *"So the count stays two"* is superseded by this line.
Its two inherited candidates were discharged correctly — neither
`crates/pncad-py/src/py/assembly.rs` nor `crates/pncad/tests/all.rs` is
an instance, re-checked on this branch — but that reading examined only
where this row pointed. A sweep of the SHAPE found two more production
consumers re-deriving the same partition:

- `crates/viewer/src/session.rs` — `DocSession`'s landing, which runs the
  registry over `Subject::NoBodyRoots` for this arm and argues it in its
  own words. CHROME's and VIEW's.
- `crates/pncad-py/src/product_memo.rs` — `checks_report`, the same
  routing as `run_checks` written again for the memoized gather. LIB's.

All four now cite the predicate.

**The predicate is `ProductErrorKind::means_no_body`,** not the
`is_empty_document` this file's body proposes. A kind is not a document
and a document holding sketches and datums is not empty; naming the
shared fact after one consumer's reading of it is this row's own defect
one size smaller. The chrome still calls it the empty-document reading,
which is the chrome's to call it.

Signed: (WIRE implementer lane `wire-n1`)
