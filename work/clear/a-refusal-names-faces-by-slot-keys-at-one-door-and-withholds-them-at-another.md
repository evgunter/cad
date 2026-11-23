---
id: a-refusal-names-faces-by-slot-keys-at-one-door-and-withholds-them-at-another
kind: issue
title: An engine refusal names its faces by raw slot keys to the editor's user where the shell op withholds them as keys the user never saw
status: open
opened: 2026-11-22
priority: P3
---


## Finding

From the review of `embed/first-finding` (PR 7319, 2026-11-22). Two
doors say one refusal by two policies:

- `EngineRefusal::say` (`crates/topo/src/clearance.rs`) names each face
  of a pair as `face FaceKey(3v1) of {side}`, and the editor's
  assertion report prints that sentence to the document's user
  (`ClearanceRefusal`'s `Say`, `crates/editor-core/src/clearance.rs`).
  Since PR 7319 a budget, a sliver and a declined witness carry their
  two faces too, so three more sentences print slot keys.
- `cavity_refusal_said` (`crates/topo/src/shell.rs`) says the same
  refusals with their faces unnamed, by design: "the payload names
  them, and they are keys the user never saw".

A slot key is an arena address, not a name the user gave or can look
up. The shell's text also now covers one more case than it says: "the
search spent its budget with pairs of walls still undecided" is said
of a depth refusal that ended the ask at its first finding, where no
search was spent.

## What is to decide

One policy for naming a face in a refusal a user reads: a stable name
where the door has one (the editor has `StableName`), the side alone
where it has none, the key only in the payload. Not changed in PR
7319, which kept each door's words.
