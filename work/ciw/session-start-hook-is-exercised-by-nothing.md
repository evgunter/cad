---
id: session-start-hook-is-exercised-by-nothing
kind: issue
title: no gate, test or script in the tree runs a line of .claude/hooks/session-start.sh
status: open
opened: 2026-09-11
refs: [session-start-hook-restates-ci-pins, 2327]
---

`.claude/hooks/session-start.sh` provisions every agent container — the
compiler, the three pinned tools, admesh, the registry warm, the render venv —
and **nothing in this repository executes one line of it**. Every hosted job
runs `rm -rf local-scripts .claude` at checkout, so no workflow can; and no
local script, selftest or fixture calls it either.

**What made it visible.** PR 2327 rewrote the file's pin handling. Both the
implementer and the correctness reviewer verified it the same way: each built
a throwaway harness — a scratch repo, a copy of `ci.yml` to break, PATH stubs
for `cargo`/`curl`/`rustup`/`apt-get` — ran the hook end to end against it, and
threw the harness away. Two lanes wrote the same instrument in one day because
the tree has none, and the next lane to touch this file will write a third.

**The gap is larger than any line in the file.** The failure modes are all
silent by construction: a tool installed at the wrong version, a roster that
has gone short (that one was live — the `cargo fetch` loop named five cargo
roots where `scripts/doc-gate.sh --print-roots` derives eight, and no session
warmed `benches`, `tools/tess-lint` or `tools/tess-meter`), a warning written
to a channel the session never reads. None of them fails anything; they all
just make a container quietly worse than the one the file claims to build.

**The shape of a fix, and the constraint that makes it awkward.** A selftest
has to run somewhere a gate can see, and hosted CI deletes the tree it lives
in. Options, in the order they are worth pricing:

1. A selftest **beside the hook** (`.claude/hooks/session-start-selftest.sh`)
   that stubs PATH and asserts the decisions — pin unread skips its tool, a
   noisy read is refused, the roster is derived — invoked from
   `local-scripts/ci-local.sh`. Cheap, and it inherits exactly the defect
   `criterion-selftest-nightly-only` names: a row no per-PR gate runs.
2. Move the checkable half **out of `.claude/`** — a `scripts/provision-*.sh`
   the hook calls — so a per-PR row can run its selftest and only the
   three-line hook stays unreachable. This is the one that ends the class.
3. Nothing, with the cost written down where the file can be read.

(2) is the answer if any of this is worth doing, and the unit is the split
rather than the test. Not scheduled here because the decision is the work.
