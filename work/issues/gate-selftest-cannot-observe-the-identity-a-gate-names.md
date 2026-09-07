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

`scripts/gates/lib.sh:1421-1445`'s `gate_selftest_case` matches one
substring of the gate's output and asserts a `gate_error` framing. That
is enough for a gate whose product is a verdict. It is not enough for a
gate whose whole product is **a `file:line` and an identifier** — the
identifier is exactly what a reader acts on, and nothing in the harness
can assert it.

`scripts/gates/viewer-vocab-declared-once.sh:611-625` is the worked
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

