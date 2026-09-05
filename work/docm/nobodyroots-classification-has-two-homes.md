---
id: nobodyroots-classification-has-two-homes
kind: issue
title: "The empty-document-is-not-a-fault rule is argued twice, in two crates, and ProductError carries no predicate for it"
status: open
opened: 2026-09-04
refs: [check-registry-gathers-product-twice]
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
