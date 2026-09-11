---
id: gate-selftest-cannot-observe-the-identity-a-gate-names
kind: issue
title: gate_selftest_case matches one substring of a gate's output, so a gate whose product is a file:line and an identifier can ship a wrong identifier with every case green
status: open
opened: 2026-09-07
---

Found by the correctness review of #2106, and found **by accident**,
which is the part worth keeping: the reviewer broke the gate's name
extraction while building a candidate fix, and `--selftest` stayed
**green** while the gate on the real tree printed
`declares `const pub`` three times. The harness never noticed that
every diagnosis had become garbage.

## The gap

`scripts/gates/lib.sh:1798-1829`'s `gate_selftest_case` matches one
substring of the gate's output and asserts a `gate_error` framing. That
is enough for a gate whose product is a verdict. It is not enough for a
gate whose whole product is **a `file:line` and an identifier** — the
identifier is exactly what a reader acts on, and nothing in the harness
can assert it.

`scripts/gates/viewer-vocab-declared-once.sh:611-625` — `:1548-1617`
on this tree, see the correction below — is the worked
example: twelve `gate_selftest_case` rows, every one matching on a
name-independent fragment of the message, so twelve green cases are
compatible with every name in every diagnosis being wrong.

## Two shapes of fix, and they are not the same size

- **A convention**: at least one case per gate must match a fragment
  containing the subject the gate names. One line per gate, no harness
  change, and nothing enforces it.
- **An affordance**: a `want` that can assert the hit LINE rather than a
  substring of the message, so a case says *"this gate, on this
  fixture, names this file, this line and this identifier"*. That is a
  harness change and it is what makes the convention checkable.

`scripts/gates/lib.sh` is GATES' file, so this is a report rather than
a change. The gate that found it will carry the one-line convention for
itself; the affordance is the durable half and is not that gate's to
build.

## Correction, 2026-09-08: the worked example was stale on arrival

Re-derived by the `view/gate-bullets` lane and verified here against
`origin/main`. **Three figures in this file were wrong when it was
written**, and the reason matters more than the figures.

| this file said | at `origin/main` |
|---|---|
| `gate_selftest_case` is at `lib.sh:1421-1445` | `:1798-1829` — a THIRD stale citation in this row, corrected 2026-09-10 by the `view/module-kinds` lane while it was in the file; `lib.sh` was not in that branch's diff, so this one had simply never been re-read |
| the cases are at `viewer-vocab-declared-once.sh:611-625` | `:930-973` at that tree (`:611` is inside the hit-diagnosis loop); **`:1548-1617`, 27 rows** after #2282 and `view/module-kinds` moved and grew them (that PR's first re-derivation said `:1528-1597`, which opens on a blank line and holds 23 of the 27) — a citation into a self-test's case list moves whenever anyone adds a case, and re-deriving it as a span rather than by finding its first and last member is how the same row goes stale a third time |
| twelve `gate_selftest_case` rows | **20** |
| *"every one matching on a name-independent fragment"* | false — **4** assert a `const` identifier by name (`declares \`const ALL\``, and its siblings), and on a broader reading of "identifier" the lane counts 11 |

**Why it was wrong: this was filed against a tree a lane was actively
changing.** The reviewer's observation was true of #2106 at REVIEW
time. #2106's fix pass then took that very finding as its MINOR-8 and
added the name-bearing cases — so the example was falsified by the same
PR it was reported against, before this file reached `main`. Nothing
re-derived it after that PR landed.

**The rule this earns**: a §6 report filed against a tree a lane is
still changing owes a re-derivation after that lane lands. A report is
a claim about a tree, and naming the tree it is true of is part of
making it.

## What survives, and it is the whole point

**The mechanism claim is untouched.** `scripts/gates/lib.sh`'s
`gate_selftest_case` still matches ONE SUBSTRING of a gate's output and
asserts a `gate_error` framing; it still cannot assert the `file:line`
and identifier a gate's diagnosis is made of. That a particular gate's
author has since written name-bearing fragments by hand is exactly the
convention-not-affordance half this file already names — it holds
because someone remembered, and the harness cannot ask for it.

The demonstration also stands unchanged: a broken name extraction left
a full self-test green while the gate printed `declares \`const pub\``
three times on the real tree. That was observed, not inferred.

