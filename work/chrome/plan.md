# CHROME — viewer chrome and coverage (plan)

**STATUS: OPEN; slate re-cut 2026-09-15.** The opening slate of nine
units all landed on 2026-09-04 and the program then went dormant for
eleven days. What it holds now is the residue those units filed, five
hand-offs from DOCM, and rows other programs filed onto this slate
while it was asleep. Live state is `work/chrome/log.md`'s tail and the
item files beside this plan, never this file.

Branch prefix (the #396 convention): **`chrome/`** — unit branches
`chrome/<unit>-<slug>`. Sessions whose harness pins a branch drive the
orchestrator half from that branch instead (VIEW's plan sets the
precedent); unit branches are unaffected and keep the `chrome/` prefix.
Away-channel tag `(CHROME orchestrator)`.

## Review posture

**No A/B duals, no row in `docs/MODEL-AB-LOG.md`** (Ev, in-chat,
2026-09-04, reaffirmed 2026-09-15). The band 1600-1699 stays claimed
and empty. The default is a **style review** against
`docs/prompts/reviewer-style-lane.md`; a correctness arm is added only
where a unit's failure mode is a *confident wrong answer* rather than a
refusal, and the dispatch says which it chose and why.

The 2026-09-04 slate is the evidence for this posture, not an
assumption: **on every unit that got a review, the review found
something the unit's own evidence did not** — and two of those were
correctness defects found by a reviewer refusing to accept a claim,
not by a correctness lane. A style lane that verifies claims finds
correctness defects as a side effect of checking whether the words are
true.

## Territory — the VIEW carve-out (2026-09-15)

CHROME and VIEW both claim `crates/viewer/src/*` and both `keep_out`s
name the other, so lint is quiet; but until 2026-09-15 both clauses
were **false**. CHROME's said *"CHROME goes first"* (spent — VIEW went
first). VIEW's said CHROME *"has been dormant since 07:00"* and treated
the wait clause as discharged on that basis (true when written, false
now). The programs no longer take turns; they divide the files.

**CHROME works:** `datums.rs`, `bounds.rs`, `combine.rs::denotes_body`,
`tree.rs::blamed_mates`, and
`crates/viewer/tests/{valid_range,combine_ops,tree_badges}.rs`.

**CHROME cedes**, each because VIEW holds an open row on the same
ground: `scene.rs` and `gpu.rs`
(`scene-mesh-carries-an-identity-index-buffer`, opened 2026-09-15,
rewrites `SceneMesh::build_parts_focused` and swaps `draw_indexed` for
`draw`); `theme.rs` and `pane/features.rs`
(`tone-is-a-value-in-frame-and-a-comment-in-two-panes`); `marks.rs` and
`blend.rs` (`renamed-module-leaves-citations-in-two-other-programs`,
which claims CHROME's `edge-cost-claims-name-a-search-that-is-gone` by
id and line). And the spine CHROME never had a claim on in practice:
`app.rs`, `session.rs`, `session/*`, `pane/*`, `frame.rs`,
`pickindex.rs`, `display.rs`, `props.rs`, `forms.rs`, `sketch.rs`.

**`crates/editor-core` is not DOCM's any more.** DOCM exited 2026-09-14
(`docs/DOC-LEDGER.md`, sweep 14) and its ground divided: `mate.rs` and
`mate/*` to MSOLVE; `doc.rs`, `edit.rs`, `node.rs`, `resolve/*` and
`REFERENCES.md` to EDIT; `crates/pncad-py` is LIB's. A CHROME row
reaching any of them is a **hand-off, not a fence to work around**.

## What the 2026-09-15 audit found

Every open row was re-read against the tree. **Nothing was dead** —
VIEW closed none of CHROME's rows in eleven days. But it moved the
*addresses* of about twelve, and three rows existed only to report that
rot. Two findings are worth carrying forward:

- **A rotted citation is not a cosmetic defect here.** Several rows
  cited line bands that now hold unrelated code, one cited a file that
  no longer exists (`crates/viewer/src/pick.rs`), and one row's premise
  had been falsified outright by work that landed after it was filed —
  it said the add-profile tool authors on world XY only, where the op
  now carries a picked frame node. A lane taking that row on trust
  would have built on a false statement. `docs/prompts/implementer-
  discipline.md` §7 already says why: cite by name, because numbers rot.
- **The counts inside rows rot too, and more quietly.** One row's
  census (*"`ui.weak` is spelled 49 times in `app.rs`"*) is off by an
  order of magnitude after the split — `app.rs` has three. A number a
  row asserts is evidence only as of its filing.

## Unit order

E-first, and each unit names the ground it may touch.

1. **`chrome/citation-repoint`** — repoint the four rotted rows by
   subject and close the two reports. Tracker only, no source. *(done;
   under style review)*
2. **`chrome/datums-substitution-sweep`** — `datums.rs` holds four
   members of one fail-loud class (a NaN or failed conversion floored
   into a plausible number): `inclusive-rule-range-draws-a-line-on-a-
   nan-count` (E, fix written in the row),
   `metres-per-pixel-swallows-a-nan-depth` (HARDER — an `Option<f64>`
   ripple), and two sites the audit found unfiled. No VIEW row touches
   the file. *(dispatched)*
3. **`bounds.rs`, two rows** — `certify-affordance-on-the-bounds-panel`
   is live, and `bounds.rs`'s own module header still says the three
   kernel doors it needs "are missing" when DOCM-9 built them. Fix the
   stale header first; it is E and it currently misleads any taker.
4. **`tree.rs::blamed_mates`** — `band-refusal-still-badges-every-row`.
   `MateFault::Band` carries only a `BandError` and names no mate, so
   every row in a refused cluster keeps its own badge. No test covers
   the Band shape today.
5. **`combine.rs::denotes_body`** — `body-seat-reads-through-the-placer-
   chain`. Signature change (the gate needs the `Doc`) and it re-pins
   two `combine_ops` rows. Land it aware of
   `work/door/node-placer-field-docs-say-body-where-instances-are-
   accepted`, which names this row as its viewer-side member.
6. **`probe-rows-assert-in-one-direction-only`** — tests only, but its
   fourth finding (a stale `derived.bounds` survives a refusal) lands
   in `session.rs`, which is ceded. Take findings 2 and 3 now; finding
   4 waits on a hand-off.

**Not scheduled, and why.** `culling-is-load-bearing-with-no-pixel-
test` and `viewer-expresses-no-gpu-adapter-preference` are both live
and both HARDER; the first would be the first row in the crate to
assert about a rendered image and therefore sets a convention, and the
second has three open decisions and a fault that is by construction not
catchable in Rust. Neither is a lane unit until someone decides the
shape. `viewer-cannot-author-a-part-node` was already adjudicated at
class M by DOOR and rejected: the seat and instance arguments are the
work, not a detail.

## Exit shape

The program does not close on this slate. `crates/viewer/tests/*` is
CHROME's and VIEW's territory both, VIEW's architecture work will keep
landing findings on this ground, and what the carve-out leaves of
`crates/viewer/src` is VIEW's the day CHROME closes. The walk
convention applies when it does; residue re-homes per
`work/README.md`, not into `work/issues/`.
