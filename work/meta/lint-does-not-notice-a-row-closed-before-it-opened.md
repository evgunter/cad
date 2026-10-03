---
id: lint-does-not-notice-a-row-closed-before-it-opened
kind: issue
title: work.py lint resolves references and measures territory but never compares closed against opened, so a row closed before it opened passes
status: open
opened: 2026-09-13
priority: P3
cost: E
---

## Finding

Filed by WIRE's `small-batch` lane (PR 2499), on META's slate because
the subject is `scripts/work.py` and the defect crosses every program's
rows.

`work/wire/placement-rs-frame-carries-51-doc-lines-over-a-7-line-struct.md`
carried `opened: 2026-09-13` with `closed: 2026-09-12` — **closed the
day before it opened** — and `python3 scripts/work.py lint` reported
*ok (0 problems)*. It was caught because a reviewer read the
frontmatter, which is the whole reason it was caught at all.

Both halves of the contradiction were ordinary mistakes: the `opened`
was wrong at filing (the row was committed on 2026-09-12 —
`git log --diff-filter=A` on the file says so — and dated a day later),
and the `closed` was written by this PR against the true date. Neither
is interesting on its own. What is interesting is that nothing said so.

## Why lint cannot see it today

`lint` does three kinds of work on these two fields and none of them
compares them:

- the **schema** types both as `date`, and the checker validates the
  SHAPE only — `DATE_RE.match(value)`, i.e. "is this `YYYY-MM-DD`";
- the **status coupling** is checked in both directions — `status:
  closed` requires a `closed:` date, and a `closed:` date requires
  `status: closed`;
- nothing reads the two VALUES together.

So a row can be closed before it opened, closed in the future, or
opened in the future, and pass.

## Why this one is worth an instrument

`work/README.md`'s own framing applies: `scripts/work.py`'s header
says *"a disclosed blind spot is a work order"*, and this blind spot
is not disclosed there — the header's `WHAT LINT CANNOT SEE` section
lists the GitHub facts the tracker deliberately does not resolve, all
of which are unresolvable without calling out. This one is the
opposite: **the parser already holds both values, both are already
typed and already shape-checked, and the comparison is one `<`.** It is
the cheapest possible check, over data in hand, for a class of error
that is otherwise invisible — dates are not read by anything else, so a
wrong one is never noticed by a downstream consumer the way a wrong
`blocked_on` would be.

The second-order cost is what the dates are FOR. `STATUS.md` and the
staleness rules read them; a row whose `opened` is a day late is a row
whose age is wrong everywhere it is quoted, and nothing anywhere would
say so.

## What would settle it

An error (not a warning — there is no inference here, both values are
in the file) when `closed < opened`, with the two dates in the message.
`lint`'s self-test has the shape to extend: the
`("closed needs date", …)` case in the table of planted-defect rows is
the neighbour, and a `("closed before opened", …)` row beside it plants
the defect and asserts the phrase.

Two adjacent questions the taker should answer rather than inherit:
whether a date in the FUTURE is also an error (it is the same class,
and `dt.date.today()` is already imported for `work.py new`), and
whether `opened` should be checked against the file's add-date in git
— which would have caught this one at filing, but couples lint to
history and is a different, heavier instrument.
