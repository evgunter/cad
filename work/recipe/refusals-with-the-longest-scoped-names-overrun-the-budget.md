---
id: refusals-with-the-longest-scoped-names-overrun-the-budget
kind: issue
title: Refusals forwarding the corpus's longest names overrun the 75-word budget
status: review
priority: P2
cost: M
opened: 2026-10-03
pr: 3886
branch: recipe/leaf-role-words
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

## Built (2026-10-06, PR 3886)

- **The gate** (`editor-core/tests/name_words_corpus.rs`): each row said through its production door with the corpus's 90th-percentile name (scoped faces: 21 words; full form: 30), the vanished row's upstream name said by its kind. Names ratcheted on their own: `NAME_WORDS` (scoped faces p50/p99/max 16/34/38, full 19/73/111) and `SAID_DIGEST` over every form of every name; the review's cap/member mutant turns the digest red. The longest names' rows are printed, not gated.
- **At the p90 name:** Vanished 50, NodeGone 39, InBand 59, TiedDisagrees 57, Unreadable 52, PairInBand 77, CrossingUnverified 74, the pick tie 67, a fillet's stranded edge (by its slot) 16. PairInBand alone is over, admitted at 77 in `OVER_BUDGET`: two 21-word names and the band ending the funnel says (28 words) leave 7 words of lead.
- **Resolve rows lead with the reference.** A node's resolve failure says its slot where the speaker's document holds the node (`this fillet's edge 0 is stranded: Extrude … was deleted. Recourse: rebind it`; `Node::reference_slot`, `Speaker::reference`, `resolve::AboutReference`); the properties pane says `this face …` and not the name again; the vanished-upstream clause says the embedded name by its kind; a bare refusal says the name once, in full.
- **Trims.** "the leg of" goes (a leg is its step's only piece); ", on <node>" leaves the name. The flush pair and the pick tie say each face's node where two faces read alike (the gate `two_copies_of_one_body_read_apart_where_a_sentence_names_both`).
- **Prose.** The band rows say the band once, through the funnel's payload, with no predicate id or query-policy explanation, under the levers a query has (`NO_DECLARATION_RECOURSE`: no "declare the coincidence"). The pick tie drops "which this refusal lists in full" and is the status line's sentence too.
- **Found beside it, fixed:** Python's `PairInBand` carries the second face (`SelectRefusal.other`); `standing_verdict` speaks within the landed evaluation and does not repeat its subject; `CrossingUnverified` says its part-local name through the speaker, by tag.
- **Remains:** the holder a bare `Vanished` cannot say is `a-vanished-name-carries-no-holding-node`.

## Built (2026-10-06, PR 3886, fourth fix pass)

- **No row is admitted over the budget.** `PairInBand` reads "select: A and B are flush? undecided: …" (5 words besides the names, was 7), so at the p90 names it renders 75; `OVER_BUDGET` is empty.
- **Python says the in-band rows through the speaker.** `pncad-py`'s `InBand` and `PairInBand` messages are the kernel's sentence spoken from the evaluated document within its evaluation (`SelectRefusal::spoken`): no predicate id, no query-policy text, no `COINCIDENCE_RECOURSE`. So the gate's `SelectRefusal` rows measure the door Python raises through. `PairInBand` carries the two holding nodes (`SelectRefusal.at`, `.other_at`), which tell two copies' faces apart when `name == other`.
- **The ratchet sees a word said once more by a whole kind.** `NAME_WORDS` adds each form's total words beside p50/p99/max (scoped faces 38,230, full 252,376).
- **The slot gate strands an edge above 0**: a corpus fillet's last selected edge of several (`slot_rows`, edge 11); forcing the slot to the first match turns it red.
