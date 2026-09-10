---
id: gate-section-scans-end-on-any-column-zero-hash
kind: issue
title: the two README section scans end at any column-zero # so a Rust attribute or shebang inside a fenced block truncates the section, and the diagnosis blames the anchor
status: closed
opened: 2026-09-08
closed: 2026-09-10
pr: 2282
---

Found by the style review of #2172, which hit it while attacking the
new anchor scan with real markdown. **Pre-existing** — it is shared by
both README readers and predates that unit — but the anchor gives it a
new way to bite, which is why it is filed rather than absorbed.

## The mechanism

`scripts/gates/viewer-vocab-declared-once.sh:501` (`readme_kinds`) and
`:536` (`readme_table`) both end the section on `insec && /^#/` — any
line starting `#` at column zero, on the assumption that such a line is
a markdown heading.

Inside a fenced code block it is not. `#[derive(Debug)]`, `#!/bin/sh`
and `# a comment` all start with `#` at column zero and all are
ordinary content. A fence carrying one **truncates the section at that
line**, and everything after it — including the announcing paragraph
and the ratified bullets — becomes invisible to the scan.

## Why the anchor makes it worse

Before #2172 a truncated section produced a kind count that was wrong
in a way the author could puzzle out. After it, a fence landing *above*
the anchor makes the gate red claiming **"the anchor paragraph is
gone"** — a statement about a sentence that is right there in the file,
two lines below the fence. The author is told to restore something they
never removed.

The review reproduced this: a `#[derive(Debug)]` at column zero inside
a fence, placed before the anchor, reds with the missing-anchor
diagnosis.

## Why it is live rather than theoretical

The scanned region is 149 lines of prose about how a Rust macro
projects a vocabulary. A fenced Rust example carrying an attribute is
exactly the thing someone would add to it, and `crates/viewer/README.md`
already carries fenced Rust elsewhere.

## The fix, and the shape it has to have

Track fence state — a line of three or more backticks or tildes toggles
it — and only treat `^#` as a heading outside a fence. Both readers
need it, so it wants to be one helper rather than two copies, which is
the same argument this file's own `readme_table`/`readme_kinds` split
already makes about the `@` sentinel.

**It owes a self-test case**: a fence containing a column-zero `#`
planted inside the section, expecting the gate to stay GREEN. And per
the negative-control practice #2106 established, that case owes proof
it fails against the unfixed reader.

## Confidence

`sure` on the mechanism and on the reproduction. `likely` that fence
tracking is the right repair rather than, say, requiring a heading to
be followed by a space.

## Closed (2026-09-10)

**The citations above are true of the tree this was filed against**
(`:501`, `:536`); the repair moved both. Today's locations are given
below and were re-derived by finding each subject, never by shifting a
number.

**Fixed, in one helper, and wider than the item asked for.**

`FENCE_AWK` (`scripts/gates/viewer-vocab-declared-once.sh:541-614`,
the helper's argument; `md_fenced` at `:615`) is prepended to BOTH
README readers the way `gate_record_awk` prepends `gate_record_split`
to its callers, so the two cannot answer *"is this line markdown
structure"* differently. `readme_kinds` (`:719`) and `readme_table`
(`:757`) each ask it of EVERY rule they have, not only of the
section-end test at `:726` and `:763`.

**Wider than "only `^#`", deliberately.** A `|` line inside a fence
was being read as a roster row, which is the same defect with the
other sentinel: a worked example of a table row, written in a fence
below the real table, reds with *"row `GHOSTS` says `ghosts` declares
a hand-written list"* about a list nobody wrote. A helper that is
right for headings and wrong for the roster would be exactly the
divergence it exists to prevent.

**The rule is CommonMark's and not a toggle.** Open on three or more
backticks or tildes indented at most three spaces; close only on the
same character, at least as long, with nothing but whitespace after.
A bare toggle lets a ``` line close a `~~~` block and hands the rest
of it back to the heading rule — the repaired defect re-minted by the
cheaper spelling of the repair, so that case is planted.

**Where the real tree actually stood, stated because the item's *"live
rather than theoretical"* heading leaves it ambiguous.** All fourteen
fence lines in `crates/viewer/README.md` sit ABOVE the section
(`:3-262`; the section opens at `:930`), so the defect was **latent, not
firing** — one fenced example inside the section away. The tracker is
nonetheless exercised over all fourteen on every pass, and they balance:
an unclosed one would leave the section heading itself fenced and red
the gate, which is what the closed-fence case plants.

**Both halves of the mechanism were reproduced against the unfixed
reader before any edit**, and every new case owes the same: five
planters at `:1326-1376`, five rows at `:1660-1669`, each run against
the file at `104f1445b` and against this one.

| case | before | after |
|---|---|---|
| fenced `#[derive(Debug)]` above the anchor | RED — *"no line … begins "Three kinds of list stay hand-written""* | GREEN |
| fenced `#!/bin/sh` + `# a comment` below the list | RED — *"carries no "#### The lists that stay hand-written" heading"* | GREEN |
| backtick line inside a tilde fence | RED — same missing-anchor red | GREEN |
| worked table row written inside a fence | RED — *"row `GHOSTS` says `ghosts` declares"* | GREEN |
| closed fence, decoy roster outside the section | RED — same missing-anchor red | GREEN |

**Disclosed rather than fixed, and stated at the helper:** `md_fenced`
is a fence tracker, not a markdown parser. Indented (four-space) code
blocks, HTML blocks and block quotes are not modelled, so a `#` at
column zero inside one of those still ends the section. Four-space
indentation cannot put a `#` at column zero by construction, and
neither of the other two has ever appeared in this section. **Not
scheduled, deliberately**: the residue's failure is the same loud red
this repair just removed, so it announces itself rather than going
quiet, and it has no file because there is nothing waiting to be done
— an item would be a row nobody can act on until someone writes a
block quote into a Rust README.

**Found on the way, and out of fence:** `mawk` 1.3.4 aborts its regex
compiler on an interval followed DIRECTLY by `(`
(`REcompile() - panic: values still on machine stack`), which is
exactly the natural spelling of *"three or more backticks or tildes"*.
`gawk` compiles it. The derivation and the portable spelling are at
`:567-587`; reported in #2282 rather than filed, because whether
`lib.sh`'s conventions block should carry the rule for the other gates
is GATES' call.
