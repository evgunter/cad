---
id: was-the-status-route-supposed-to-fire-for-an-absent-chooser
kind: ruling
title: was the dialog status route supposed to fire for an absent chooser, or did the add_enabled gate quietly take its job
status: closed
opened: 2026-09-09
closed: 2026-09-09
pr: 2275
refs: [ranked-and-unranked-verdicts-are-one-type, 2272, 2278]
---



Raised by #2272's style review as its blind spot #5, and **not
resolved there.** #2272 needed only to know that `deliver_status` has
no wasm caller, which it established. This question is what the same
evidence raises and does not answer, and it is a design question about
intent rather than a defect anyone can read off the tree.

## The question

`frame::dialog_status` has a `Show` arm for `(chose: false, usable:
false)` — an empty-handed dialog under a confidently absent backend —
that carries `frame::NO_CHOOSER_BACKEND` to the status line. **No call
can reach it.** One copy of `self.chooser` both gates the button
(`add_enabled(chooser.usable(), …)`, `crates/viewer/src/app.rs:1230`
and `:1254`) and feeds `dialog_status` inside the click body, so a
click implies `usable()` and every reachable verdict at both sites is
`Keep`. The same const string is already on the disabled button's
hover text (`app.rs:1231`, `:1255`).

So the crate says the sentence twice, on two routes, and only one of
them can fire. Which was the design?

- **(a) The hover text is the answer and the status arm is belt to its
  braces.** Then the arm is correct as it stands, and
  `app.rs:1056-1073`'s paragraph — which argues exactly this — is the
  record. Nothing to do beyond what #2272 wrote there.
- **(b) The status route was meant to fire, and `add_enabled` took its
  job without the decision being made.** #1125's posture is *"a door
  that cannot open says so"*, and the disabling arrived to satisfy it;
  whether it was also meant to silence the status line is not written
  anywhere. If (b), the repair is not in `deliver_status` — it is in
  deciding which of the two surfaces states a missing backend.

## Why it is a ruling and not an issue

Nothing in the tree distinguishes (a) from (b). Both are consistent
with every line of code, every test and every doc comment now in the
crate; the difference is what #1125 intended, and the argument at
`app.rs:1075-1082` is a lane's reconstruction of it rather than a
citation. A lane picking one and writing it down would be manufacturing
a ratification — which is the failure mode `work/README.md` gives
`kind: ruling` for.

## What must not happen meanwhile

`app.rs:1056-1073` says the arm is latent rather than dead and gives
the reason it cannot be guarded. That paragraph is true under both (a)
and (b) and is not an answer to this question. **A reader must not take
it for one**, and #2272's PR body says so.

## A third reading, added by the orchestrator: the arm is in the wrong CHANNEL

(a) and (b) both take for granted that a status-line sentence was a
coherent thing to want here. Checked against the crate's own ratified
rule, it may not have been — and if so the question has an answer that
does not depend on recovering anyone's intent.

**`crates/viewer/README.md`'s provenance rule** (Ev, 2026-09-06):

> A `frame::Badge` is a **read of held state a reader consults**; a
> `frame::Message` on the status line is the **outcome of something
> that just happened.** … The channel is decided by PROVENANCE — what
> caused the sentence to exist — and not by what it is about.

**`self.chooser` is held state by construction.** It is written exactly
once, in `ViewerApp`'s constructor (`crates/viewer/src/app.rs:688`,
`chooser: frame::chooser_backend()`), and its own field doc says so
(`:446-448`: "once at startup"). There is no `self.chooser =` anywhere
else in `crates/viewer/src` or `crates/viewer/tests`. "This machine has
no file chooser" is therefore not the outcome of anything that just
happened — it is a fact about the environment, decided before the first
frame and true for the whole run.

**But `dialog_status` returns it on the outcome channel.** Its `Show`
arm builds a `Message` with `Subject::Document`
(`crates/viewer/src/frame.rs:1757-1760`), and a `Message` is by the rule
above an outcome. So the arm is not merely unreachable; **it is a
Message carrying what the rule classifies as a badge.**

The arm's own comment says as much, against itself:

> The document the user asked for is the subject: they aimed Open or
> Save at it and this is what came back

That sentence describes a user who **aimed** — which is exactly what
`add_enabled(chooser.usable(), …)` prevents. The comment was written for
an event the gate makes impossible, which is the clearest evidence in
the file that the two halves were written under different pictures of
what kind of fact this is.

**So the hover text is not a lesser substitute for the status line — it
is closer to the right shape.** A disabled control with its reason on
hover is a read of held state a reader consults, which is what the rule
asks for. What is wrong-shaped is the arm.

### (c), and the orchestrator's recommendation

**(c) The status arm is misclassified, and that is the finding.** Under
the ratified rule an absent backend is badge-shaped, so no correct
version of this sentence belongs on the status line. Then:

- (b) is not the repair it looks like — routing this to the status line
  would put a badge-shaped fact on the outcome channel deliberately,
  against the rule;
- (a) is right about the *outcome* (hover text is the answer) and wrong
  about the *reason* (`app.rs:1056-1073` defends the arm as "belt to
  that disabling's braces", i.e. as a redundant Message, rather than
  noticing it is the wrong channel);
- the repair is to **delete `dialog_status`'s `Show` arm** — making the
  function total on `Keep` and probably making it disappear — and, if
  the hover text is judged too weak a surface for a whole-run
  environmental fact, to give it the badge the crate already has
  machinery for.

I recommend (c). It needs no reconstruction of #1125's intent: it
follows from a rule that is already ratified plus a fact about the field
that anyone can check. **What it needs from Ev is only whether hover
text is an adequate badge**, or whether "no file chooser on this
machine" deserves a real one.

`ranked-and-unranked-verdicts-are-one-type` is the neighbouring row: a
`Message` that cannot be ranked because nothing raises it is the same
shape as a verdict whose door is carried only by prose.

## Closed — Ev ruled (c), 2026-09-09

> **"(c) is right!"** — PR 2275, 2026-09-09.

So: **an absent chooser is badge-shaped by the provenance rule, and
`dialog_status`'s `Show` arm is a `Message` carrying it on the outcome
channel.** The arm is misclassified, not merely unreachable, and no
correct version of that sentence belongs on the status line. (a) had the
outcome right and the reason wrong; (b) would have put a badge-shaped
fact on the outcome channel deliberately.

**The surface stays the hover text.** The ruling was put with a
conditional attached — *if* hover text is judged too weak for a
whole-run environmental fact, give it a real badge — and Ev ratified (c)
without asking for one. (c)'s own argument is that a disabled control
with its reason on hover **is** a read of held state a reader consults,
which is what the rule asks for. So no badge is built here; if one is
ever wanted it is a new row, not a residue of this one.

**What the build is**, carried by its own unit rather than this row:
delete the `Show` arm, which makes `dialog_status` total on `Keep` and
probably deletes the function; that removes both `deliver_status` call
sites, which likely deletes `deliver_status` and with it the
`#[cfg(not(target_family = "wasm"))]` #2272 put on it, and the paragraph
at `crates/viewer/src/app.rs:1056-1073` that defends the arm. Whatever
`frame::deliver` retains as callers decides whether that goes too.
`frame::NO_CHOOSER_BACKEND` stays — the hover text is its reader.
`tests/frame_policy.rs`'s `an_empty_dialog_is_loud_only_under_a_confidently_absent_backend`
loses its subject and goes with it, so the test count moves against the
merge base for a stated reason.

**The build landed at #2278, and every step above held.** `dialog_status`
went whole, both call sites with it, `deliver_status` and #2272's `cfg`
with them; `frame::deliver` kept its one caller (`pane/viewport.rs:48`),
`NO_CHOOSER_BACKEND` kept its two (`on_disabled_hover_text` on Open… and
on Save As…), and the `frame_policy.rs` row went — `app.rs` 2,022 →
1,961 lines and `#[test]` under `crates/viewer` **539 → 538**, the one
deletion and nothing else. That row lost its subject only in PART: its
`dialog_status` calls had none left, but its three assertions on the
remedy string's CONTENT still had one, and nothing replaced them — the
honest statement of what that costs is
`hover-route-for-an-absent-chooser-has-no-test`. The `Keep` no-op the
deletion rests on was proved before anything was cut, three ways.

**This row's own `app.rs:1056-1073` was right** and is left as written:
on `origin/main` that span is exactly the two paragraphs the row
describes — *latent, not dead* at `:1056-1063` and *nothing holds
that reading mechanically* at `:1065-1073`. The whole item and its doc
ran `:1046-1099`, which is a different span for a different subject.
