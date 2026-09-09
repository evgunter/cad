---
id: was-the-status-route-supposed-to-fire-for-an-absent-chooser
kind: ruling
title: was the dialog status route supposed to fire for an absent chooser, or did the add_enabled gate quietly take its job
status: open
opened: 2026-09-09
needs_ev: true
refs: [ranked-and-unranked-verdicts-are-one-type, 2272]
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
