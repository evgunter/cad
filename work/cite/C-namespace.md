---
id: C-namespace
kind: ruling
title: The C-namespace collision — process observations and schedule rows share the C<N> prefix
status: closed
opened: 2026-08-20
closed: 2026-09-11
---

## Ruling (Ev, 2026-09-11): leave the ambiguity, no prefix

**No side takes a new prefix and nothing is re-lettered.** Ev, asked with
the seven live pairs in front of him: *"i'd be plenty happy to just leave
the ambiguity as all are temporary items anyway where the context is
clear… i don't care which side you re-letter if you do choose to
re-letter."* The option to re-letter was offered and declined.

The reason it is safe to decline, recorded so the next reader does not
re-raise it: **the collision set is bounded and shrinking.** The
observations are a closed historical list — `C1`–`C27`, from finished
scans, which does not grow — and each of the seven colliding schedule
rows is an open work item that is deleted when it closes. So the
ambiguity has an end date that arrives on its own, and a prefix rule
would govern only rows that may never be allocated. The standing
instruction the register carried — *"Read a citation by the file it
names, never by its letter"* — is the whole disposition. Its home,
`work/code-quality/plan.md`, left the tree at sweep 11 and resolves
through `docs/DOC-LEDGER.md`.

## Question (as asked)

After the merge renumbered the second scan's observations to
**C18–C25**, the schedule's Track C rows still occupy **C15** and
**C17** for different things. One sentence giving Track C's rows a
distinct prefix closes it permanently.

## Gates

Every row numbered `C<N>` — Track K's `C15`, Track R's `C3` and `C23`,
Track N's `C24`, Track U's `C13` and `C14`, Track V's `C6`, Track W's
`C18` — and every prose citation of a `C<N>`, which today resolves by
where it points rather than by its letter (`plan.md`, *How the numbering
works*). Ids are stable for life, so the sentence decides the prefix new
rows take, not a renumbering.

## Re-derived at the CITE cut (2026-09-11, orchestrator)

The question is unchanged and still Ev's. Two things about the *gates*
above moved, and both make the ruling easier rather than harder.

**The collision is wider than the row's instance list, and the
2026-09-11 sweep then dissolved half of it.** The row names `C15` and
`C17`. Re-derived at the opening of this program there were **seven**
live pairs. Re-derived again after sweep 11 there are **none**, because
one side of every pair left the tree:

`work/code-quality/` was swept on 2026-09-11 and its
`process-observations.md` — the sole home of the `C1`–`C27`
observations — went to the archive with it, recoverable at the SHA
`docs/DOC-LEDGER.md` names. The ledger states the disposition in terms
that answer this row directly: **"The register's numbering scheme is
retired, not relocated."** Two of the seven colliding schedule rows
(`C13`, `C14`) went the same way.

So what is left is five `C<N>` item files, each now the only live
holder of its number:

| `C<N>` | the schedule row |
| --- | --- |
| `C3` | `work/mesh/C3.md` — split `props/quad.rs`'s quadrature engines |
| `C6` | `work/docm/C6.md` — collapse the W2f remainder of S4 |
| `C15` | `work/instr/C15.md` — the budget gate's face identity |
| `C18` | `work/tint/C18.md` — H12's three enumeration residues |
| `C23` | `work/mesh/C23.md` — `RATIONAL_CERT_SPLITS` |

**This does not change the ruling; it is why the ruling was right.** Ev
declined a prefix on the grounds that the items are temporary and the
context is clear. Within a day the other side of the collision was
retired by an unrelated sweep, which is the bounded-and-shrinking
argument arriving faster than anyone predicted. A prefix would have
governed a namespace that no longer has two occupants.

A citation to a `C<N>` observation in older prose still resolves — to
the archive, through `docs/DOC-LEDGER.md`, which is what the ledger is
for.

Stated for the ruling, not deciding it: ids are stable for life, so the
sentence picks the prefix **new** rows take on one side, and the seven
above keep their numbers and are disambiguated by prose.

## Re-homed to CITE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

CITE collects the rows about the project's own text and harness rather
than its kernel: citations that rot, numbers that were reissued, and the
paperwork a lane runs on. This row is one of them.

Its class at the cut was **E** — one sentence from Ev picks a prefix;
ids stable, no renumbering, no code. The class is a dispatch estimate
made by reading the row against the tree on 2026-09-11, not a verdict on
the finding, and a lane that finds it wrong says so in its PR. The id,
the `track:` letter where the row carries one, and the body above are
unchanged by the move.
