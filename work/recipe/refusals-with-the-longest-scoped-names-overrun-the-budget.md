---
id: refusals-with-the-longest-scoped-names-overrun-the-budget
kind: issue
title: Refusals forwarding the corpus's longest names overrun the 75-word budget
status: open
priority: P2
cost: M
needs_ev: true
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
- `InBand`'s own prose is about 60 words, so any name over about 25 words overruns the budget by itself, and a refusal naming two such names cannot fit at all.

**Open: a design choice on the ruled gate, so it is Ev's.** Three options:
1. Read the budget per refusal as "the prose plus the names".
2. Say long names shorter in refusals. For example, a refusal could scope a pair against each other rather than against the table, which #3906 rejected for one-name notices. A name the evaluation no longer holds could be scoped against the table that last held it.
3. Shrink the prose of `InBand`, `TiedDisagrees` and `Unreadable`.
