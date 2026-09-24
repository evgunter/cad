---
id: gate-directory-counts-in-prose-go-stale-on-every-new-gate
kind: issue
title: Two GUARD rows carry gate-directory counts that a new gate silently falsifies
status: open
opened: 2026-09-15
priority: P3
cost: D
---


Filed by PORT (2026-09-15). PORT's `msrv-floor-equals-channel.sh` is the
25th file in `scripts/gates/`, and landing it falsifies a count in two of
GUARD's own live rows. PORT did not edit either: they are GUARD's files and
one of them is a quoted transcript whose accuracy is the point.

## The two

1. **`work/guard/gate-roster-and-probe-census-have-no-reader-guards.md:33`**
   quotes a `gate-roster.sh` OK line verbatim — *"for all 21 gates in
   scripts/gates/ … (22 registered gates scanned)"* — as the output a
   shimmed `sed` produces while the gate reads nothing. The gate now prints
   **22** and **23**. The quote is evidence of a reproduction and moving it
   would be rewriting the observation, so this needs a decision rather than
   a sed: either date the quote as a transcript (*"at the time of the
   reproduction"*), or re-take it. `:94` then asks *"whether 21 gates have
   this or two do"* — that one is a live question about today's directory
   and is simply wrong now.
2. **`work/guard/plan.md:56`** scopes a sweep to *"`scripts/gates/*` (24
   files)"*. Exact before this PR (`lib.sh` and `viewer-readme-fence.awk`
   are in the directory but not gates, which is why 24 ≠ the roster's 21);
   **25** after it. This is a scope statement for work not yet done, so a
   lane picking it up sizes the sweep from a number that is one short.

## Why this is a row and not a chore

**Neither file is a frozen log**, which is the test `work/README.md` applies
to whether stale prose matters: both are live rows scoping work someone will
pick up. And the class recurs on a schedule nobody controls — *every* future
gate falsifies both counts again. The directory is the roster precisely so
that no second list has to be maintained (`gate-roster.sh`'s header argues
this at length); a count written into prose is that second list wearing
different clothes.

So the fix worth having is not "say 25": it is deciding, once, whether a
directory size belongs in prose at all, and where it does, writing it as a
derivation (`ls scripts/gates/*.sh | wc -l`) or dating it.

## How PORT found it, and what its sweep could not match

PORT swept for a hardcoded **gate count** before landing, reported "no prose
in the repo carries one", and was wrong — the style review found both hits.
The pattern was gate-count-shaped (`21 gates`, `all 21`, `twenty-one`) while
the drift was **directory-size-shaped** (`(24 files)`), and a count written
as a parenthetical after a glob matches none of it. Stated here because a
sweep whose blind spot is unstated is an unverified claim
(`docs/prompts/implementer-discipline.md` §5), and because the next lane to
add a gate will run the same sweep.

A pattern that would have caught both:
`grep -rnE '\b(all )?[0-9]+ (gates|files)\b' work/ docs/ scripts/ .github/`.
Its own blind spot: a count spelled as a word (`twenty-four`), and a count
in a file this grep's roots do not cover.
