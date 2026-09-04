---
id: document-seam-no-in-session-change-detection
kind: issue
title: The document seam has no in-session change detection - a store edit is invisible until reopen (plus two adjacent edges)
status: open
opened: 2026-08-31
github: 1387
refs: [1376]
---

## From GitHub issue 1387

Opened 2026-08-31; 0 comments.

Filed out of the GAUTH-3 fix pass (PR #1376), which made assemblies authorable from inside the viewer and thereby made this class reachable by ordinary use. **None of it is a defect in the memo, the resolver, or that unit** — each behaviour is correct where it was designed. What changed is who meets it: until now an assembly was opened from disk, evaluated once, and closed; now one is built, edited, and lived in while its part documents sit in the same directory being edited too.

## 1. The head of the class: `Reevaluate` cannot observe ANY store change

An `InstantiatePart` node's memo key hashes its `DocRef` — id and pin — and nothing about the store. The session hands the previous `Evaluation` to the next `evaluate` as its memo (`DocSession::request_eval`), so an instantiate node whose reference has not changed is served from the memo and the resolver is never consulted.

Consequence: with an assembly open, a part document that is **edited, moved, ε-changed, or deleted** leaves the open assembly green. `SessionOp::Reevaluate` — the door whose whole purpose is "ask for the current document to be evaluated again" — cannot see it either, because the document did not change. The picture and the badges stay right about the reference and stale about the world.

The fix is not "hash the file": that would put a filesystem read in a memo key. Candidate shapes, none decided here:

- a store epoch/generation the session bumps on an explicit "re-read the directory" action, folded into the evaluation's memo identity;
- a resolver-level cache with its own invalidation, leaving node keys alone;
- an explicit `SessionOp::RefreshReferences` that drops instantiate results from the carried memo (the smallest thing that would work);
- nothing at all, plus a documented "reopen to re-read" — an honest answer if it is written down where a user meets it.

Demonstrable today with no new machinery: author or open an assembly, edit one part through `Workspace::resave`, `Reevaluate`, observe the tree still green; reopen the same file and observe `PinMismatch`. `crates/viewer/tests/instance_authoring.rs`'s three badge rows reopen the document for exactly this reason, and say so.

## 2. Save-as into a partless directory silently rebinds the seam

`SessionOp::Save` rebinds the resolver to the new file's parent (the directory rule following the file — correct). Saving an assembly into a directory that does not hold its parts therefore moves the store out from under every instance, and they go `FAILED` at the next evaluation: typed, recoverable by saving back, but with no warning at the moment of the act, when the user still has the context to understand it.

A sentence naming this now sits at that op's docs (in #1376). Whether the door should warn, offer to copy the parts, or keep quiet is a design question, not a bug report.

## 3. The chooser has no part-vs-assembly vocabulary

`Add part…` lists every document in the directory, so an assembly can be instantiated inside another assembly. That **works** — nesting is what `InstantiatePart` is for, and A1's scope ladder wants it — but the chooser calls everything a "part" and offers no way to see which entries are assemblies, how deep a pick would nest, or that a pick would pull in a whole tree. Recorded as a chooser-vocabulary question for whoever next touches that listing; nothing is broken.

## Home

GAUTH's closing entry names this issue as its residue; the ground is the viewer's document seam (`crates/viewer/src/session.rs`, the workspace resolver), and GAUTH and GUI are both closed programs, so it lands in `work/issues/`.
