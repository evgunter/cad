---
id: vocab-gate-counts-bullets-across-a-whole-prose-section
kind: issue
title: the vocab gate's kind scan spans the whole prose section, so the real constraint is never write a bolded bullet there and the error misdiagnoses it
status: closed
opened: 2026-09-08
closed: 2026-09-08
pr: 2172
branch: view/gate-bullets
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


## Closed (2026-09-08, PR 2172)

**The mechanism holds; the numbers above do not, and are corrected here
rather than repeated.** The scan region is not the section: `readme_kinds`
stopped at the next heading of ANY level, and `#### The lists that stay
hand-written` is one. Re-derived at this branch's base `d02bb0e6b` by
running the reader's own awk over the page, the region is
`crates/viewer/README.md:931-1079` — **149 lines** — and the three
bullets in it are `:1023`, `:1026`, `:1031`, all inside the enumeration
paragraph `:1020-1033`. At the tree this item was written against the
region was `:908-1021` (114 lines), not `:907-1037` (131): `:1037` is
inside the table's trailing prose, which is neither the end of the scan
region nor the end of the `###` section (`:1044`). The `title:` above
carried the wrong figure and now carries none — a count re-derived at
one site and left in a header is this program's own #2103 defect.

**The fix is the anchor the item proposed.** `KIND_ANCHOR='Three kinds
of list stay hand-written'` pins the announcing sentence; the reader
finds it inside the section (the existing scope, kept) and reads the
bullets of the one list under it, closing at the first line that is
neither a bullet, an indented continuation nor a blank. Two departures,
argued at the site: it matches a PREFIX of the line, because a
paragraph's wrap point is an artifact of the fill column and the four
constants this gate already pins are headings and a table header row,
which are whole lines by construction; and the count word is INSIDE the
anchor, which is a hold the gate did not have — the section could say
"Four kinds of list stay hand-written" over three bullets and nothing
read it.

**`@` for the anchor, as `readme_table` does for its heading**, so "the
paragraph is gone", "it announces no list", "it is announced twice" and
"the reader died" stay four answers. No reader was added: the kinds
reader is the same two-stage `awk | sed` pipeline under the same single
guard, no `|| true` anywhere, and the header's stated population of six
readers still holds.

**The case this owed, with the proof it fails unfixed.** Two
`gate_selftest_passes` rows plant a bolded bullet in the section's
PROSE — above the anchor and below the list, two different branches —
and expect GREEN. Against the whole-section scan restored into the file
both fail with the misdiagnosis itself: *"ratifies 3 kinds … and this
pass read 4: "A bolded bullet" …"*. Three new failing cases cover the
anchor reworded, doubled, and separated from its list; the first and
third PASS against the unfixed gate, the second fires with the wrong
message. Three negative controls (the scan reverted, the list's closing
rule deleted, the missing-anchor guard deleted) each turn the suite red.

**The README says so where it matters**: the kinds are the bullets of
the list that sentence announces "and no other", and the prose in that
section may carry bulleted lists like any other prose. No bullets were
added to it to demonstrate that; that is a separate change.

**Residue, none of this program's.** Two citations into this gate in
`work/issues/` were stale at `d02bb0e6b`, before this branch:
`gate-selftest-cannot-observe-the-identity-a-gate-names.md:25` names
`:611-625` and "twelve `gate_selftest_case` rows" where the rows were
`:930-973` and there were twenty (23 now, `:1090-1143`), and
`gate-rust-reader-splits-an-array-type-at-its-semicolon.md:65` names
`:139-153` where the sentence it quotes is the item reader's header,
`:212-218` then and `:256-262` now. Reported in PR 2172's body as §6
findings on another slate rather than edited from a unit branch.
