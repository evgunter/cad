---
id: blend-mod-recourse-docs-accumulate
kind: issue
title: blend: mod.rs's recourse constants and BlendError carry 200+ lines of justification prose
status: open
opened: 2026-09-08
---


## Finding

`crates/sweep/src/blend/mod.rs` is 15 recourse constants and one error
enum, and most of its bulk is prose arguing for the code rather than
stating what a reader must uphold.

The fifteen `*_RECOURSE` constants (`crates/sweep/src/blend/mod.rs:543`
through `:789`) carry 201 lines of doc comment above 55 lines of
sentence. The worst rows:

| constant | doc lines | sentence lines |
| --- | --- | --- |
| `FILLET3_ASSEMBLY_RECOURSE` (`:711`) | 40 | 7 |
| `FILLET3_SEAM_VERTEX_RECOURSE` (`:666`) | 24 | 5 |
| `FILLET3_GEOMETRY_RECOURSE` (`:743`) | 20 | 7 |
| `FILLET3_CORNER_RECOURSE` (`:634`) | 19 | 8 |
| `FILLET3_RING_RECOURSE` (`:768`) | 18 | 2 |

`BlendError` itself (`crates/sweep/src/blend/mod.rs:816-1176`) is 361
lines for a closed enum of refusals, most of it prose on which fixture
reaches which arm and which review found which wording wrong.

What the prose is doing is legible per paragraph and not per file.
Three recurring kinds do not belong in a file everyone reads:

- **Which fixture reaches this arm.** `FILLET3_SPINE_RECOURSE`
  (`:583`) spends 15 lines on the fact that no followability fixture
  reaches its sentence, and names the row that measures it —
  a `docs`-shaped argument attached to a string constant. The
  NAME of the enforcing row is the part something would go wrong
  without; the argument that produced it is in the PR.
- **The review that changed the wording.** "Endorsing 'every edge of
  the corner' named a door that cannot serve the caller who was just
  refused" (`:591-593`) is an argument with a
  position nobody holds any more.
- **Provenance tags.** "which is the A3-2 defect" (`:593`), "README
  A3-2" (`:696`), "Fix pass F6" (`:1328`) — PR and milestone
  archaeology, which `docs/prompts/implementer-discipline.md` §4
  already forbids in comments and which nothing removes once written.

`memories/cad-working-style.md`'s code-comment rule is the standard:
"keep a comment only if something would go wrong without it. The
obligation a caller must uphold, why a match is exhaustive, why an API
is private, why a panic path is absent, what a refusal means, a hazard
invisible from the code — those stay. The incident that produced a
rule, a dated timing, a count of call sites, an argument with a
position nobody holds any more: git history has them."

This is an accumulation, not a single author's habit: each paragraph
was added by the unit that learned the thing, and no unit's fence has
ever covered the file as a whole. That is why it needs its own item.

## Fix shape

A prose pass over `crates/sweep/src/blend/mod.rs`, keeping per doc:

- the invariant the sentence stands on (what makes the recourse TRUE
  of the variant that appends it, which is the D2 contract);
- the NAME of the row that holds it there, without the argument;
- any hazard a reader could not see from the code.

Everything else goes; the argument stays in the PR that made it, and
`crates/sweep/README.md` is where a durable design statement belongs.
Expect the file to lose 150–200 lines with no contract change. The
message-pinning suites assert against rendered SENTENCES, not doc
comments, so the pass is checkable by the existing gates.
