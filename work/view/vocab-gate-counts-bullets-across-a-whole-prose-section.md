---
id: vocab-gate-counts-bullets-across-a-whole-prose-section
kind: issue
title: the vocab gate's kind scan spans 131 lines of prose, so the real constraint is never write a bolded bullet there and the error misdiagnoses it
status: open
opened: 2026-09-08
---

Found by the style review of #2143, whose author had to write a README
rewrite with **no bulleted list anywhere** to keep this gate green —
and who reported that as a workaround rather than swallowing it. The
gate is `scripts/gates/viewer-vocab-declared-once.sh`, landed by VIEW
at #2106, so this is VIEW's own file and its own item.

## The mechanism

`readme_kinds()` (`scripts/gates/viewer-vocab-declared-once.sh:423-434`)
reads from the `### Closed vocabularies are declared once` heading to
**the next heading of any level**, and collects every line matching
`^- \*\*`. `:539-540` reds when that count is not `KIND_COUNT=3`
(`:206`).

At head that region is `crates/viewer/README.md:907-1037` — **131 lines
of ordinary expository prose**, not a list of ratified kinds. The three
kinds it means to count sit under one specific, quotable sentence:
*"Three kinds of list stay hand-written, and each is a different answer
rather than an exception:"* (`crates/viewer/README.md:962`).

## Why it is a defect and not a constraint to live with

The constraint the gate actually imposes is **"never write a bolded
bullet anywhere in a 131-line prose section"**. Nobody designed that,
nobody would write it down, and it is not what the gate's own header
says it does.

Worse, the diagnosis is wrong. An author who adds a bolded bullet while
writing about something else is told they added a **ratified kind** —
which is a claim about a design ratification they did not make. This
program's standing rule is that a misdiagnosis pointing at the wrong
repair is the same class the reader-failure marker exists for
(#2106's own demonstration bought that fix one layer in); this is the
same shape at the README half.

## The fix, which is cheap

Anchor the bullet scan to the paragraph rather than the section: begin
collecting at the *"Three kinds of list stay hand-written"* sentence
and stop at the first line that is neither a bullet nor a bullet's
continuation. The gate already knows how to scope by an exact string —
`readme_table()` anchors on `#### The lists that stay hand-written` —
so this is the same technique applied one level finer.

Whatever the fix, it owes a **self-test case that plants a bolded
bullet in the section's prose and expects the gate to stay GREEN**,
because that is the case no current fixture covers and the one that
would have caught this.

## Confidence

`sure` on the mechanism — the awk is quoted above and the region was
read. `likely` that anchoring to the paragraph is the right repair
rather than, say, requiring the bullets to be contiguous.

