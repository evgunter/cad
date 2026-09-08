---
id: emitters-carry-pr-and-unit-archaeology-in-comments
kind: issue
title: Two names emitters carry PR/unit archaeology in comments where the invariant should stand alone
status: open
opened: 2026-09-08
---


## Finding

Surfaced by EVAL-3's sweep of the names emitters (the pattern
`reissue|version|kef|Retired|unreachable|cannot fire|the surgery|slot`,
run to find the blind spot of the spec's own grep). Two comments in
EVAL's ground carry history where `docs/prompts/implementer-discipline.md`
§4 wants the invariant alone:

- `crates/editor-core/src/names/emit_topo.rs:1090-1096` — the
  fusion-partner pass-down argues its kept-key order and then cites
  "review R9 — the PR 4 Vanished-diagnosis item covers the retired
  partner". The order rule (kept-key identity wins; `zip_seam` keeps
  the outer cycle's vertex) is the invariant; the review and PR
  pointers are archaeology, and the item they name is not a file
  anything can follow.
- `crates/editor-core/src/names/emit.rs:785-788` — a test fixture's
  comment explains a refusal sample by what "LIB-G14 retired" (unit
  tag plus a description of the row's previous state). The invariant
  is one sentence: the sample payload names a refusal that exists in
  the emitter, so a grep for it finds its subject.

Neither is EVAL-3's class (a consumer re-deriving a kernel argument it
could cite), so neither was edited there; both are one-comment
rewrites. Citations accurate as of 2026-09-08.
