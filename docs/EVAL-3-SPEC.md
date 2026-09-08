# EVAL-3 — `emit_blend` cites the kernel types that own its two arguments instead of restating them (spec)

**Program:** EVAL (`work/eval/plan.md`, unit 3). **Item, one unit:**
`work/eval/emit-blend-restates-the-kernels-own-arguments.md`.
**Track:** E — the CIW/CHROME posture: one implementer lane, one style
review with claims to falsify, a fix pass, record-at-merge. No A/B draw.
**Correctness arm:** none needed on its face (prose only, no code path
moves) — but the review's claim 1 below is what proves that, so it runs.
**Branch:** `eval/3-emit-blend-cites`. **Difficulty:** S.

## The claim

**A rule has one home, and a consumer cites it.** `D324` (PR 1943) gave two
arguments their kernel homes; `crates/editor-core/src/names/emit_blend.rs`
still carries its own derivation of both, and one has already drifted:

1. **"A retired key is never reissued."** The kernel's statement is
   `crates/sweep/src/blend/naming.rs:45`–`:49`: the `Retired` guard "cannot
   fire while the arenas reissue no retired key; it holds the invariant
   against that changing". The consumer restates it twice — the module-doc
   paragraph at `emit_blend.rs:40`–`:56` ("unreachable BY CONSTRUCTION, and
   it is worth saying which construction": `slotmap::SlotMap`,
   `new_key_type!`, version bump on removal) and the site comment at
   `:253`–`:258`, which cites "(module docs)" — the emitter's own, not the
   kernel's. Two homes in agreement, and nothing enforces the agreement.
2. **"`Retired` carries no face channel."** The kernel's home is the type:
   `naming.rs:129`–`:133` on `Retired` ("the surgery cannot retire a source
   face — enforced at `surgery`'s one face-destroying door,
   `SourceFaces::kef_minted`"), and the door at `surgery.rs:3657` states the
   rule again in its own voice. The consumer's site comment at
   `emit_blend.rs:263`–`:267` restates it and adds a COVERAGE sentence —
   "Asserted in both directions by `sweep/tests/m6_5_fillet_naming.rs`" —
   while the kernel's coverage sentence (`naming.rs:32`–`:35`) names TWO
   rows, `m6_5_fillet_naming.rs::every_output_entity_is_a_recorded_mint_or_a_survivor`
   and `verbs_arms1_annulus.rs::every_annulus_output_entity_is_a_recorded_mint_or_a_survivor`.
   The two homes already disagree about their own coverage. That is the
   drift the item predicted, and it is silent because it is agreement
   that decays rather than a contradiction.

## What lands

1. The module-doc paragraph at `emit_blend.rs:40`–`:56` becomes a citation:
   the guard is unreachable while `topo::Body`'s arenas reissue no retired
   key, which is the kernel's own statement at `sweep::blend::naming`'s
   module doc — one sentence, a rustdoc link to the item that owns it
   (`sweep::blend::naming` or `Retired`), and the posture sentence
   ("kept because the property lives in another crate's choice of
   container") kept if it survives as an invariant rather than history.
   The `slotmap`/`new_key_type!`/version-bump mechanism is NOT re-derived
   here; if the kernel's sentence does not name the mechanism and the
   implementer judges the mechanism worth naming, the place is the kernel
   sentence, which is `crates/sweep` — BLEND's ground — so that goes in
   the PR body as a note for BLEND, not as an edit.
2. The site comment at `:253`–`:258` cites the same home (no "(module
   docs)" pointing at itself).
3. The site comment at `:263`–`:267` cites `Retired`'s doc for the no-face-
   channel rule and DROPS its coverage sentence: which tests execute a
   kernel invariant is the kernel's sentence to keep, and a consumer
   naming a fixture is the second home that drifted. The comment keeps
   one line of consumer-side meaning: "so a face key here is a real
   survivor".
4. The `wire_blend` cross-reference in the paragraph ("same posture as
   `wire_blend`'s refusal of `naming: None`") is checked against the tree:
   if `wire_blend` still refuses `naming: None`, keep the sentence; if not,
   delete it and say so in the PR body (Q4 — a premise something cites).
5. No code changes. `cargo doc -p editor-core --no-deps` adds no
   NEW broken intra-doc link in the touched files — 58 pre-existing ones
   exist elsewhere in the crate and are not this unit's (the citations are
   links, so a moved home reds
   the doc build rather than rotting silently — that is the enforcement
   the item asked for).

## Sweep

The class is "a consumer re-deriving a kernel argument it could cite". The
reviewer-style prose grep, restricted to the emitters (EVAL's ground):

```
rg -n -i 'by construction|module docs\)|asserted (in|by)|never reissued|slotmap' crates/editor-core/src/names/emit*.rs crates/editor-core/src/names/mod.rs
```

Hit list with per-hit disposition in the PR body; a restatement in another
emitter of a kernel argument is fixed in this unit only if its kernel home
exists and the fix is one citation — otherwise reported with the home
named. State what the pattern cannot match (a restatement using none of
those words, which is the majority).

## Review

One style lane, `docs/prompts/reviewer-style-lane.md` by path. Claims to
falsify:

1. The diff is comments and doc-comments only: `git diff --stat` and a
   read of every hunk; any token outside a comment is a finding.
2. Every citation resolves: `cargo doc -p editor-core --no-deps` shows no
   new intra-doc warning in the touched files against the same command at
   the merge base (58 pre-existing elsewhere are not this unit's),
   and each linked item actually states the rule the consumer relies on —
   read the target, not the link.
3. The coverage sentence is gone from the consumer and the kernel's still
   names both rows (Q4: nothing else cited the consumer's sentence).
4. Q1 over the other emitters: the sweep's hit list is real; run it shaped
   differently.
5. Q2: is any surviving justification in `emit_blend.rs` longer than the
   code it defends? Q8: read `emit_blend.rs` end to end.

## Records at merge

`work/eval/log.md` entry; the item `closed` with `pr:`; this spec deleted
per `docs/DOC-LEDGER.md`.
