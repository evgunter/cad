---
id: document-seam-no-in-session-change-detection
kind: issue
title: The document seam has no in-session change detection - a store edit is invisible until reopen (plus two adjacent edges)
status: parked
opened: 2026-08-31
refs: [1376]
priority: P1
cost: M
blocked_on: [the-store-keeps-every-version-and-history-undoes-without-destroying]
parent: the-store-keeps-every-version-and-history-undoes-without-destroying
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

## Un-parked — the trigger fired (2026-09-04)

`viewer-session-god-module-split` closed on 2026-09-04, so this row's
only blocker is gone and the row is dispatchable. Un-parked here, from
VIEW's PR #1857, rather than by CHROME: on Ev's ruling there, `work.py
lint` now REFUSES a `parked` row whose every blocker is closed, and a
program cannot un-park another program's rows in the PR that closes
their trigger — `work/README.md`'s one-file-one-item rule makes that a
merge conflict by design.

## Evidence: the accept-version door is gated on this (AUTH-15, 2026-09-30)

An instance whose pin no longer holds now offers **Accept updated
version** on its tree row (`SessionOp::AcceptPartVersion`,
`author/accept-part-version`, PR #3591). The offer is read off the
instance's `PinMismatch` failure. Because of §1, a part repaired while
its assembly is open raises no failure, so the offer does not appear
until the assembly is reopened. The viewer README says so beside the
feature. Until this row lands, "reopen to re-read" is the documented
answer.

## 2026-11-22 — weighed: what the store owes a pin

Two designers weighed this row over four rounds (fork-log row 301). The row's premise was written before IDENTITY DI2, which Ev ruled in chat on 2026-09-04 (DOCM, commit 08777903615: "Ruled for option 2"), and was never updated. DI2 already settled what an instance means and picked a door: `Reevaluate` re-mounts the store. What was still open is whether that door is right.

The question underneath is **what the store owes a pin**. Today a folder holds one version per id, so re-saving a part destroys the version every older assembly pins. A pin can then work only as a drift alarm. Meanwhile the session's memo keeps drawing the old version, backed by nothing, and the Accept offer can only be read off a broken instance.

The designers converged on:
- the store keeping every pinned version (ASSEMBLY AQ1);
- a kernel reuse check (`holds`) in place of DI2's "never consulted against the resolver";
- the session holding one read of the folder, with a per-reference standing and a Re-read parts door.

**Build order: retention, then the kernel check, then the session side.** No interim meaning ships. The ratified text this changes is in `[ev]` PR (this row's PR). §2 (save-as into a partless folder) dissolves, because every save keeps its pins in the target folder. §3 moved to `the-part-chooser-calls-every-document-a-part`.

The off-question findings below are fixed by the build or ride with it:
- `SessionOp::Save`'s doc in `session/op.rs` cites "issue #1387" instead of this row;
- the `same_resolver` doc in `evalseam.rs` contradicts DI2's memo half;
- `parts.rs`'s `PartFiles` doc claims the scan follows its resolver's own read, which is false on a memo hit.

Once there is one read, `the-part-chooser-keeps-its-snapshot-across-a-document-swap` closes. (VSEAM orchestrator)

## 2026-11-23 — moved to VERSIONS

Ev ruled on PR 7462 that the store question gets its own program, with one X row: work out how, then do it. This row moved there by `git mv`, id unchanged, and is parked on that row, which carries its design. It closes when the build makes a store change visible in session. (VSEAM orchestrator)
