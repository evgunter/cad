# DOOR — the doors whose fix is already written (plan)

**STATUS: OPEN (2026-09-11).** Opened in the tracker cut of 2026-09-11
(`docs/WORK-TRACKS-2026-09.md` addendum 3). Live state is
`work/door/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix: **`door/`** — unit branches `door/<unit>-<slug>`.
Away-channel tag `(DOOR orchestrator)`. A/B ordinal band
**DOOR = 3800–3899**.

## Charter

A row belongs here on one test: **reading it tells you the diff.** Not
"the problem is understood" — the *fix* is written, in the row, and a
lane can land it without deciding anything first.

**The test bites, and it has bitten four times.** One row failed it on
the first day (`viewer-cannot-author-a-part-node` named its files but
not what its new op takes) and went to CHROME; four more left on
2026-09-20 in the design-free sweep below.

## No design decisions (Ev, in chat, 2026-09-20)

Asked of FIX — *"can you kick all the design decisions back to the track
they actually belong to, leaving fix design-free?"* — and applied here
in the same sitting, where it bites harder for a structural reason:

**This program claims no paths, so it can never be the owning track for
a decision.** FIX owns three files and can rule on its own ground; DOOR
owns none, so there is no row here whose surface is its to decide. A row
that needs a decision therefore ALWAYS leaves, to the track that owns
the surface, by `git mv` with a `## Re-homed` section on the item and a
note on that program's log.

**That retires the charter's old M clause.** It used to admit rows whose
fix was written but which added *"a small design call (where a shared
helper's home goes, what a door looks like)"* — which contradicted this
program's own rule, stated two paragraphs later, that *"a row that grows
a design question stops being this program's"*. The rule wins. Both rows
the M clause was written for (`S114`, `patherror-display-renders-float-noise`)
asked the same question — where a shared thing lives in a `geom-core`
file PROPS owns — and both went to PROPS on 2026-09-20.

## Territory — none, and why

Every row sits on a file some live program owns, which is why none of
them were claimed: each is one small thing in someone else's house. The
rule that replaces a fence:

- **One PR is one row.** A lane that finds itself editing a second row's
  file has left this program's posture and should say so rather than
  widen. The mirror class was the one ruled exception and it has closed.
- **Each PR draws its own fence and announces it** to the owning program
  in the PR body, naming the file and the owner — and takes the owner
  from `scripts/work.py territory`, never from a row's own prose. Three
  rows on this slate named DOCM as their owner for a week after DOCM
  closed.
- **A row that grows a design question stops being this program's**,
  immediately and without exception.

## The slate

**The count is not written here, deliberately.** `python3 scripts/work.py
status --program door` derives it; a number in this paragraph would be a
hand-maintained census of a table below it, which is the defect this
program exists to close.

Four rows, every one `E`, every one a written fix with its owner named:

- `node-placer-field-docs-say-body-where-instances-are-accepted` —
  `Node::Transform`'s and `Node::Pattern`'s field docs still say "the
  body placed" / "the body replicated" though both placers accept
  `Instances` and `eval::node_value_kind` now reads a transform's family
  on that premise. Two doc sentences. EDIT's `node.rs`.
- `part-fault-partproduct-degrades-the-product-refusal` — one field and
  one call site, now that `ProductErrorKind` exists with an exhaustive
  `kind()`. Note `PartFault` already carries a typed `cause` on its
  `PartRootFailed` arm, so this arm is the odd one out. EDIT's
  `eval/parts.rs`.
- `the-third-datum-axis-phrase-lives-in-mate-member` — swap one literal
  for `crate::eval::phrase::DATUM_AXIS`, the last copy in the tree. The
  refusal text does not move, so the pinned `contains("datum axis")`
  stays green by construction. MSOLVE's `mate/member.rs`.
- `vectorslot-slots-has-no-reader` — delete an unread `pub fn` and its
  re-export. The row reads "delete it or name the consumer", and that is
  no longer a decision: its own sibling closed by deletion one PR ago on
  the same lines (`vectorslot-all-has-no-reader`, PR 2446). EDIT's
  `node.rs`.

There are no dependencies between them, which is most of what makes the
track worth having.

## Review posture

One review per unit against `docs/prompts/reviewer-style-lane.md`, no
A/B row — the FIX and CHROME posture. **Light by default; full where a
row has real risk of being wrong** (Ev, 2026-09-11, in-chat).

Nothing on the current slate takes a full correctness lane: the four
rows are two doc sentences, one field, one literal and one deletion. The
rows that did take one have closed. **Ordering rule 5 still applies** —
where a fix closes a structural finding, check whether it mints a fresh
instance of the defect it closes; on rows this small the reviewer is the
only one who has ever caught it.

## How the class column is read

`E` / `M` / `H` is a **dispatch estimate** made by reading a row against
the tree, and it is the axis this program's order runs on. It is not a
verdict on the finding and it is not in any header: no field carries it
and `work.py` does not parse it. A lane that finds an estimate wrong
says so in its PR.

- **E** — the fix is written in the row or obvious from it: one or a few
  files, no design question, no ruling, small diff.
- **M** — multi-file, or a census or instrument to build first. **No
  longer "or a small design call"**: that clause is retired, see above.
- **H** — cross-cutting, numeric or algorithmic, gated on a ruling, or
  spanning several programs' territory. An `H` row is not this
  program's at all.

The cut that opened this program is `docs/WORK-TRACKS-2026-09.md`
addendum 3; it is a survey, and this plan supersedes it as the charter.
The opening corrections of 2026-09-11, the mirror class's two PRs and
the five rows they filed are the log's, not this file's — `work/README.md`
says a plan states present state only.
