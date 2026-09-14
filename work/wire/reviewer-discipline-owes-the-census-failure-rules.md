---
id: reviewer-discipline-owes-the-census-failure-rules
kind: ruling
title: Should the census/guard failure rules eight WIRE units paid for become standing reviewer discipline?
status: closed
opened: 2026-09-14
closed: 2026-09-14
---

## The question

Five rules emerged from WIRE's 2026-09-13/14 block, each from a round
that **broke something** rather than from an argument. They are recorded
in `work/wire/plan.md` as this program's working rules. **Should they be
promoted into `docs/prompts/reviewer-style-lane.md`**, where they would
bind every reviewer lane by path?

`CLAUDE.md` says text that binds future work rather than describing a
change waits for Ev, and `docs/prompts/` is named explicitly. So this is
Ev's, not the orchestrator's — hence an `[ev]` PR rather than a merge.

## Why it is being asked now rather than banked

Three of the five were **discovered twice**, in different units, by
different lanes, because the first discovery lived only in a program log
that the second lane had no reason to read. That is the cost of leaving
them here: `reviewer-style-lane.md` is read by every reviewer by path,
and `work/wire/plan.md` is read by WIRE.

## What is proposed, and the evidence for each

**1. Prefer a bijection; a floor is what you write when you have proved
no bijection exists — and then you write why, at the site.**
PR 2480's census guarded two rules with a hand-written roster of four
door names and a floor of twelve. Measured 18 against the floor; one door
contributing two calls was not in the roster at all, so renaming it took
the scan to 16 and nothing reddened. The door set was **derivable from
the region the row already carved**. Replacing the floor with an equality
closed it at the root rather than patching it.

**2. A set equality is not automatically safe.** `assert_eq!(a, b)`
passes when both sides are empty; if both derive from one scan, a dead
scan reads as a pass; and — the shape that cost the most — **one member
can silently leave both sides at once**. In PR 2517 `built` was populated
by iterating `declared`, so re-spelling one field's type path dropped the
carrier from both and the equality passed at the lower count. Every
equality owes a non-emptiness assertion on its **own** derived set, with
a message naming that failure mode.

**3. Assert the rule, not a proxy for it.** PR 2480's census asserted
*"this is not a string literal"* while its message claimed *"the phrase
comes from `family` or `phrase`"*. A reviewer hoisted a const to a call
site and walked straight past it — **and the same commit had performed
that exact refactor three times.** A message that states one rule over an
assertion that checks another is one refactor from useless, and the
refactor may already be in the diff.

**4. A census that finds its sites by the spelling it is normalising can
only ever find the ones that already comply.** PR 2517's census walked
one enum for one field name and missed an eighth and ninth site — both
of which the unit's own `rg` had already printed. Corollary, and the
cheaper half: **triage the sweep to the end.**

**5. Making a thing unspellable moves the forgery to whatever it is
computed from.** PR 2517 replaced a defeated textual guard with a token
whose field is private to its door — total from outside the crate, eight
attack shapes and eight compile errors. Inside the crate the *key* the
token is read off was still the caller's, and a compiled attack produced
a truthful-looking refusal about a forged key with a byte-identical
success path, **which the census passed**. The move is complete only when
that input is not the caller's to choose either; a guard that looks total
because its own type is airtight is the most expensive partial guard,
because it retires the guard covering the rest.

## Two smaller ones, offered separately because they are narrower

- **A bounded walk is guardable exactly when something outside the crate
  can write a step of it.** This replaced three copies of an
  *"unguardable, and here is why"* survey with one checkable property —
  and two of the three were **false** (PR 2518, correcting text PR 2474
  had already merged).
- **An "unguardable" note is a claim about a call graph.** Q6's discharge
  is available only when the guard genuinely cannot be built, and twice
  the evidence against it was in the same file's own tests.

## What the orchestrator recommends

Take **2, 3 and 4** into `reviewer-style-lane.md` — they are about how a
guard fails, they are short, and each was discovered independently more
than once. Rule 2 belongs beside **Q3** (*can this test fail*), rules 3
and 4 beside **Q1**.

Hold **1 and 5**: they are design guidance for an implementer rather than
questions for a reviewer, so if they land anywhere it is
`implementer-discipline.md`, and rule 5 in particular is one session old
and has been tested exactly once.

**Counterargument, stated because it is real**: `reviewer-style-lane.md`
opens by saying its questions are *"not a checklist"*, and five more
imperatives make it more checklist-shaped. If that cost is not worth
paying, the honest alternative is to leave all five in `plan.md` and
accept that the next program rediscovers them — which is what happened
twice inside this one.


## Declined by Ev, 2026-09-14 (PR 2555)

**"i lean declining these changes. they seem like they're adding more
words to a rule that's already there; i believe the problem was there but
i don't know if stating the rule in more words would've helped"**

Accepted, and the test in that second clause is the one I failed to
apply to my own proposal: *would stating the rule have helped?* The
evidence says no. `reviewer-style-lane.md` already tells a reviewer that
naming the trap in a PR body does not prevent it and that **only a reader
who did not write the fix has ever caught it** — and all five of these
were caught by a reader, not by a rule. The lane that wrote the
proxy-assert had read the brief. The file also opens by insisting its
questions are *"not a checklist"*, a cost I named in the PR and then
proposed against anyway.

The five rules stay in `work/wire/plan.md`, where they cost nothing and
bind nobody.

## And Ev corrected the framing, which produced the better result

> *"a scan that died reads as a pass" — this sounds like it could be a
> bug? if it "died" in the sense of encountering an error condition, it
> shouldn't be returning an empty set*

Right, and the sentence hid two different things. A scan that hits an
**error** and returns empty is a bug in the scan; guarding it downstream
treats a symptom. What these censuses actually met is the other case: the
scan ran correctly and **honestly found nothing**, because its needle had
stopped matching — a sentinel renamed, a file moved, a type path
re-spelled.

But the question points past my rule. **If a scan over a file that must
contain its markers finds none, that is not an honest zero either — its
premise is broken and it should refuse at the scan**, which is this
project's own fail-loud rule applied one level earlier than I was
applying it. That is a code change, not a paragraph, and it is checkable
where the prose was not.

Carried to `work/wire/a-source-census-scan-that-matches-nothing-should-refuse-at-the-scan.md`.
