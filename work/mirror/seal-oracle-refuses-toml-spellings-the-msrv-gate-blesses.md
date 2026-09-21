---
id: seal-oracle-refuses-toml-spellings-the-msrv-gate-blesses
kind: issue
title: seal-oracle.sh refuses rust-toolchain.toml spellings the MSRV gate certifies as valid
status: open
opened: 2026-09-15
priority: P4
cost: E
---


Filed by PORT (2026-09-15) from `msrv-floor-is-declared-and-never-compiled`'s
style review, which found the divergence the new gate creates.

`local-scripts/seal-oracle.sh`'s `toolchain_pin` reads the pinned compiler
out of `rust-toolchain.toml` with

```
sed -n '/^\[toolchain\]/,/^\[/{s/^channel[[:space:]]*=[[:space:]]*"\([^"]*\)".*$/\1/p;}'
```

which requires a **double-quoted** value on a `channel` key at **column 0**
inside a literal `[toolchain]` header. Its own header declares that gap and
argues for it: an indented or single-quoted `channel` is legal TOML and is
**REFUSED, not read**, because "a wrong compiler silently reported as the
project's is the outcome this shape exists to make impossible". That
reasoning is sound and this row does not ask for it to be abandoned.

**What changed is that a second reader of the same field now exists and is
more permissive.** `scripts/gates/msrv-floor-equals-channel.sh` parses the
file with `tomllib`, so it accepts every spelling cargo and rustup accept —
and its self-test plants two of them as **must-pass** fixtures, because
proving the parse-not-grep claim is what those fixtures are for:

- `toolchain.channel = "1.97.0"` as a bare dotted key at document root
- `channel = '1.97.0'` as a single-quoted literal string

So a `rust-toolchain.toml` written either way passes the gate — CI stays
green, the pin is unambiguous, the workspace builds — and then
`seal-oracle.sh` refuses to run until a developer reads its message and
sets `TOOLCHAIN=`. That is a refusal rather than a misread, which is why
this is a row and not an urgent one, but the two readers now disagree about
what a well-formed pin looks like and nothing says so at either site.

## Three ways out, for whoever takes it

1. **Widen the sed** to accept both quote characters and leading
   whitespace. One expression plus the count logic and the message text
   (which says "quoted `channel` key(s) at column 0" and would stop being
   true). `local-scripts/` is pruned by every hosted job, so this has no
   hosted coverage and the script has no `--selftest` — that is the real
   cost of the change, not the sed.
2. **Parse the TOML** in `seal-oracle.sh` as the gate does, which retires
   the gap rather than widening it. The script is bash and this would be
   its first python dependency.
3. **Leave the sed and record the agreement**: state at both sites that
   this repository writes its pin as a double-quoted `channel` at column 0
   and that the permissive reader is deliberate. Cheapest, and it makes
   the divergence a decision rather than an accident — but nothing then
   enforces the convention, so it decays the way an unchecked convention
   does.

PORT took none of them: it is CIW's file, the choice between 1 and 2 is a
judgement about local-only tooling PORT does not own, and 3 is a convention
this repository has not agreed. What PORT did do in its own file is name
the divergence in the gate's header and point at this row, so a reader of
either script reaches the other.

## Citations

- `local-scripts/seal-oracle.sh`, `toolchain_pin` and the KNOWN GAP
  paragraph above it (`:60` and `:72` at the time of filing).
- `scripts/gates/msrv-floor-equals-channel.sh`, `plant_dotted_spelling` and
  `plant_literal_strings`, and the header paragraph that names this row.
