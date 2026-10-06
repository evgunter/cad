---
id: refusals-with-the-longest-scoped-names-overrun-the-budget
kind: issue
title: Refusals forwarding the corpus's longest names overrun the 75-word budget
status: open
priority: P2
cost: M
opened: 2026-10-03
---


Found by PR 3886's fix passes, building the ruling of #3906 ("every refusal that forwards a name meets the budget with the corpus's longest scoped names").

**What is measured.** `editor-core/tests/name_words_corpus.rs` (`every_corpus_name_reads_apart_and_forwards_within_the_refusal_budget`) says each refusal through the door production says it through, `spoken(doc, evaluation)`, over the corpus's longest names that door says:

- The four select, crossing and hit-test rows forward names their evaluation holds, so each name is said within its table (`Speaker::within`). The longest such face names are 45 words: the part of a part of an end cap, bordering two side walls.
- The two resolve rows forward a name their evaluation does not hold: a vanished name is in no table of the run that refuses it, and a deleted node has none. So the name is said in full, and the longest full forms are 123 words: the part of a seam edge between two seam vertices, each citing its seam's two faces.

| row | words |
|---|---|
| `ResolveError::Vanished` (cascade, two names, in full) | 263 |
| `ResolveError::NodeGone` (in full) | 136 |
| `SelectRefusal::InBand` | 106 |
| `SelectRefusal::TiedDisagrees` | 81 |
| `SelectRefusal::Unreadable` | 76 |
| `SelectRefusal::PairInBand` (two names) | 145 |
| `NodeErrorKind::CrossingUnverified` (as `NodeError`; its part-local name said by tag, in full) | 127 |
| `HitTestError::Ambiguous` (two hits) | 133 |

On main, before the names carried their roles in words, the same rows measured 21–58 words.

The test pins each row as a ratchet (`OVER_BUDGET`): a row that grows fails, and a row that shrinks must lower its number. A second ratchet (`SAID_WORDS`) pins the words of every name the corpus holds, in full and scoped, so a regression in any name's words shows, not only in the longest.

**Why it does not fit.**
- A scoped name keeps its core at every detail: its role, its feature, its joins, and each opened citation's own role and feature. The 45-word names are ones whose rivals in their table differ only two citations down.
- A full-form name says every citation, however deep.
- `InBand`'s own prose is about 60 words, so any name over about 15 words overruns the budget by itself, and a refusal naming two such names cannot fit at all.

## A refusal reads in one pass at a typical name, and a name says only what tells it apart

Measured on PR 3886's build over all 12,490 corpus names: names are long in the middle, not only at the tail. A face said within its table is median 21 words, p90 25, p99 40, max 45; table scoping opens a citation for 187 of 2,618 faces. The length is the fixed core: the modal face (840 of 2,618) is "the side wall over the leg of loop 0 step 2 of Extrude X, cut in at Subtract Y, on Boolean Z", 22 words. With that median name four rows already overrun (in-band ~82, flush pair ~97, pick tie ~85, crossing ~79). And "with the longest scoped names" cannot hold for any two-name refusal under a rule that keeps names distinct: two 45-word names are 90 words with no prose, and names nest without bound. Part of the measurement is also off: the four select rows are never drawn by the viewer (Python writes its own name-free messages for them), the pick tie the viewer draws is `frame::pick_refusal`'s, not the kernel's, and the 263-word vanished row pairs two unrelated names where the second is always embedded in the first.

- **The gate.** Each refusal is measured at the door production says it through, with a payload that can occur and the corpus's 90th-percentile name. A separate ratchet holds the names' own lengths (p50, p99, max). The longest-name tail is reported, not gated. The 75 stands as a prose limit on a typical payload, which is what the refusal standard says.
- **A refusal about a reference leads with the reference.** A node's resolve failure says which slot: "this fillet's edge 2 is stranded: Boolean aa4f… was deleted. Recourse: rebind it" (about 15 words against 136 today); under D10 the `Face`/`Edge` variable, its name or "the edge Fillet 12 rounds". A name that is the sentence's subject is not repeated (the properties pane's "this face is gone: <the same face>"). The "vanished upstream first" clause says the embedded name by kind. Where there is no reference to lead with (the bare refusal as Python or a log prints it), the name is said once, in full: no production door holds a table that still holds a vanished name.
- **A name's words carry only what tells it apart.** "the leg of" goes where the leg is its step's only piece (a single-segment step draws one piece; fillet steps keep run-in, arc, run-out). The ", on <node>" phrase names the node that minted the name, not the node whose output holds it, so two Transform copies of one body already say the same holder; which output an entity is in belongs to the sentence (a pick hit's node, a flush query's two nodes, the node a select row is about), and the sentence says it where it is not already fixed. With both trims: median 15, p90 16, p99 24, max 36.
- **The band refusals' prose** says the band once, drops predicate ids and query-policy explanation, and the pick tie drops "which this refusal lists in full". The forwarded recourse "declare the coincidence" points at a seat D10 retires.

With all four every row is within 75 at the p90 name; a tie between two of the corpus's longest names runs to about 85–90 words, which the name ratchet reports.

Found beside it: Python drops a flush pair's second face (`pncad-py/src/py/select.rs`); `pane/properties.rs` `standing_verdict` speaks `ResolveError` without `.within` and repeats the subject; the `*Resolve` node errors name no slot; `CrossingUnverified` says its part's name by tag outside the speaker; a forwarded `StableName` carries no holding node (`PairInBand`, `Vanished`), so "X and X" for two copies cannot be fixed by words alone.

Weighed by two designers over two reconciliation rounds (fork-log row 69).

## Ruled (Ev, PR 4069, 2026-10-05)

As the section above says ("sounds good!"). PR 3886 builds it in its last fix pass, with the third review's findings.
