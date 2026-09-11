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
instruction in `work/code-quality/plan.md`'s *How the numbering works* —
*"Read a citation by the file it names, never by its letter"* — is the
whole disposition.

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

**The collision is wider than the row's instance list, and its two sides
are now cleanly separated by kind.** The row names `C15` and `C17`. At
head `C17` has no schedule row, and **seven** numbers do collide — every
one of them a `C<N>.md` item file scattered across six programs by the
2026-09-11 cut, against a same-numbered observation in the single
narrative file `work/code-quality/process-observations.md`:

| `C<N>` | the schedule row | the process observation |
| --- | --- | --- |
| `C3` | `work/mesh/C3.md` — split `props/quad.rs`'s quadrature engines | C3. Deferrals must land in a register that executes |
| `C6` | `work/docm/C6.md` — collapse the W2f remainder of S4 | C6. Some of these were ratified before they were written |
| `C13` | `work/code-quality/C13.md` — give epsilon a type | C13. Half-fixes read as whole fixes… |
| `C14` | `work/code-quality/C14.md` — STEP writer's Part 21 header | C14. Pins guard the invariant as it was reachable *then* |
| `C15` | `work/instr/C15.md` — the budget gate's face identity | C15. A sweep's result is worth nothing without… |
| `C18` | `work/tint/C18.md` — H12's three enumeration residues | C18. Two of my dispatch briefs were wrong… |
| `C23` | `work/mesh/C23.md` — `RATIONAL_CERT_SPLITS` | C23. The A1 rule has not taken yet |

**Why the cut helps.** Before it, both sides lived under
`work/code-quality/` and "read a citation by the file it names" was a
weak instruction, because the two files sat together. Now the schedule
rows are `C<N>.md` **item files** that `work.py` parses, resolves
references against, and puts on the board, while the observations are
`## C<N>` **headings inside one unparsed narrative file**. The two sides
are no longer symmetric, which is what a prefix ruling wants: the cheap
side to re-letter is the one nothing cites by id and nothing parses.

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
