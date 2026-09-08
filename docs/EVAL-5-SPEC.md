# EVAL-5 — two public `Verb` types, one stated convention (spec)

**Program:** EVAL (`work/eval/plan.md`, unit 5). **Item:**
`work/eval/two-public-verb-types-verbs-and-profile.md`.
**Track:** E, docs-only — one implementer lane, one style review, no A/B
draw, no correctness arm. **Branch:** `eval/5-two-verbs`. **Difficulty:** S.

## The claim

**`verbs::Verb<T>` (the kernel verb vocabulary) and `profile::Verb` (which
verb a sketch `Step` names) are both public, the collision is known at one
site (`crates/verbs/src/verb.rs:357` cites `profile::Verb::ALL` as its
census precedent) and reconciled nowhere.** No behaviour is at stake; the
cost is every reader's `use` line and every sentence that says "the verb"
without a crate. Readers of `profile::Verb` outside `profile` today:
`editor-core`'s `eval/mod.rs` (`verb_tag` and its census), `program.rs`,
`persist/check.rs`, and `viewer/tests/error_display.rs` — all by qualified
path already.

## What lands

The item offers a rename on the profile side or a stated convention. This
unit lands the **convention**, because `crates/profile` is S-BOOL's glob
and a rename there is theirs to sequence; the rename is recorded, not
done:

1. `crates/verbs/src/lib.rs`'s module doc gains one paragraph, at the
   point where it says what the vocabulary is: **`Verb` here is the
   KERNEL's verb — an operation on a body. `profile::Verb` is the sketch
   program's — which transition a `Step` takes — and the two never meet
   in one signature; every reader spells the crate.** Two sentences, no
   history.
2. `verbs/src/verb.rs:357`'s precedent citation says which `Verb` it
   means in the same breath (it does; make it read as the convention's
   first application rather than an aside).
3. `crates/profile`'s `Verb` doc (`src/path/program.rs`, the
   `transition_table!` output) gets the mirror sentence — ONE line,
   pointing at `verbs::Verb` as the kernel's — by announced seam to
   S-BOOL (the orchestrator's log line and this PR). If S-BOOL would
   rather rename (`StepVerb` is the natural spelling), that is their
   PR; this unit files nothing for it beyond the sentence, because a
   convention that holds needs no rename.
4. `rg -n '\bVerb\b' crates/*/src/**/*.rs docs/*.md` over prose: any
   sentence that says "the verb"/"a `Verb`" and could mean either gains
   the crate. Hit list with dispositions in the PR body; EVAL's files
   fixed, others reported.

## Review

One style lane, `docs/prompts/reviewer-style-lane.md` by path. Claims: the
diff is prose only; the two sentences agree with each other and with the
tree (Q5); the prose sweep's hit list is real (run it shaped differently:
`rg -n 'verb' -i` over the module docs of the two crates and
`editor-core/src/eval/mod.rs`'s content-key section). Q7: would you have
renamed instead? Say so; it goes to S-BOOL with the seam note.

## Records at merge

`work/eval/log.md` entry with the S-BOOL seam note; the item `closed` with
`pr:`; this spec deleted per `docs/DOC-LEDGER.md`.
