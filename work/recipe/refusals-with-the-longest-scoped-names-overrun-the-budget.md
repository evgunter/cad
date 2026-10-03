---
id: refusals-with-the-longest-scoped-names-overrun-the-budget
kind: issue
title: Refusals forwarding the corpus's longest table-scoped names overrun the 75-word budget
status: open
opened: 2026-10-03
---


Found by PR 3886's fix pass, building the ruling of #3906 ("every refusal that forwards a name meets the budget with the corpus's longest scoped names").

**What was measured.** `editor-core/tests/name_words_corpus.rs` (`every_corpus_name_reads_apart_and_forwards_within_the_refusal_budget`) speaks every name of every corpus table through `Speaker::of(doc).within(&evaluation)`, the least detail that tells each name apart in its table (`names::words::least_detail`). The two longest scoped names are 59 words (a `crossing_slots` edge: the part of a seam edge between two seam vertices); the two longest scoped face names are 45 words (the part of a part of an end cap, bordering two side walls). Said with them:

| row | words |
|---|---|
| `ResolveError::Vanished` (cascade, two names) | 135 |
| `ResolveError::NodeGone` | 72 (fits) |
| `SelectRefusal::InBand` | 96 |
| `SelectRefusal::TiedDisagrees` | 81 |
| `SelectRefusal::Unreadable` | 76 |
| `SelectRefusal::PairInBand` | 137 |
| `NodeErrorKind::CrossingUnverified` (its part-local name said by tag, in full) | 124 |
| `HitTestError::Ambiguous` (two hits) | 133 |

The test pins each as a ratchet (`OVER_BUDGET`): a row that grows fails, a row that shrinks must lower its number.

**Why it does not fit.** A scoped name keeps its core at every detail — role, feature, joins, and each opened citation's own role and feature — and the 45–59-word names are ones whose rivals in their table differ only two citations down. `InBand`'s own prose is about 50 words, so any name over about 25 words overruns it alone; a refusal naming two such names cannot fit at all. The designers' measurement (#3906) was an approximation that did not reach the opened citations' features.

**Open.** Either the budget is read per refusal as "the prose plus the name", or long names are said shorter in refusals (a refusal could scope a pair against each other rather than against the table, which #3906 rejected for one-name notices), or the prose of `InBand`, `TiedDisagrees` and `Unreadable` shrinks. Which is a design choice on the ruled gate, so it is Ev's.
