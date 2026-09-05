---
id: one-line-one-subject-loses-a-mixed-frames-expiry
kind: issue
title: One line holds one subject, so a mixed frame's rank-2 join loses the finer expiry
status: open
opened: 2026-09-05
---


## What this is

`frame::frame_status`'s rank 2 joins a frame's notices into ONE
`Message`, and a `Message` carries one `Subject`. When the notices
disagree about their subject, `frame::joined_subject` falls back to
`Subject::Document` — which has no `Expire` issuer, so the joined line
is swept only by an act the document accepts and the finer expiry is
lost.

Concretely: a frame carrying a picking disagreement (`Subject::Cursor`,
which `frame::cursor_status` would retire on the next cursor move) and
a tool notice (`Subject::Document`) produces a line that survives every
cursor move until the next accepted edit.

## Why it is not reachable yet, and when it becomes so

Every notice a frame produces today is document-provoked, so the
fallback arm has never fired in production. `status-line-writers-bypass-
the-ranking` routes `crates/viewer/src/pane/viewport.rs`'s
disagreement into the frame's notices, and from that unit onward a
frame that carries a disagreement *and* a tool notice hits it.

The arm itself is asserted —
`frame_policy::a_joined_line_keeps_a_shared_subject_and_falls_back_when_they_differ`
pins both the fallback and the fact that the fallback survives a cursor
move — so the behaviour is stated, not silent. What is unresolved is
whether it is the behaviour anyone wants.

## The fork

1. **Keep the fallback.** A conservative line that says too much for
   too long is cheaper than one that deletes a clause a reader had not
   read. Cheapest, and the status quo.
2. **The line holds one message PER SUBJECT** rather than one message.
   Expiry then reaches exactly its own clause and nothing else, and the
   join happens at paint. It is the shape the vocabulary wants — but it
   contradicts rank 1's ratified *"a refusal wins, alone"*, which is a
   statement about the whole line, so it is a design change and not a
   refactor.
3. **Rank 2 refuses to join across subjects** and shows only the
   highest-ranked subject's notices. Needs a rank among subjects, which
   nothing has argued for.
