---
id: cut-prefix-three-unpinned-spellings
kind: issue
title: The budget cut-line prefix has three independent spellings across two cargo roots and a shell script, pinned in no direction
status: closed
opened: 2026-09-03
closed: 2026-09-08
refs: [D204, cut-line-commit-names-no-baseline-change, cut-regex-unanchored-admits-a-line-the-lint-refuses]
branch: meter/cut-prefix-pin
---

## Was

unrowed. Raised by the `D204` lane's cross-root-constant sweep, which
was looking for the shape `D204` closes — a constant shared across a
cargo-root boundary with no pin in either direction — and found this
one beside it.

## Finding

The budget sweep's cut line is one string with **three independent
spellings**, and nothing holds any two of them to each other:

- `tools/tess-lint/src/lib.rs:478` — `pub const CUT_PREFIX: &str = "# tess-budget-cut:"`, the reader's half (`:518` strips it).
- `scripts/tess_budget_cut.sh:60` — `CUT_RE='^# tess-budget-cut: [0-9a-f]{7,40}(-dirty)? [0-9]{4}-[0-9]{2}-[0-9]{2}'`, the validator.
- `scripts/tess_budget_cut.sh:103,104` — the writer's own `echo "# tess-budget-cut: $commit $date"` and the strip `grep -v '^# tess-budget-cut:'`.

`D204` closed the same shape for `CHART_TAGS` by pinning it from the
meter's side, and `EXPECTED_HEADER` and `GROWTH_TOLERANCE` were pinned
the same way before it. This one is worse than those in one respect and
better in another: worse because the writer and the validator are *in
the same file* and still do not share a spelling, so a change to one is
not even a cross-root problem; better because a drift reds loudly at
parse rather than silently, since `tess-lint` refuses a sweep whose
first line it cannot strip.

**What makes it a finding rather than a tidy-up**: the regex is the only
one of the three that constrains the *shape* of what follows the prefix
(commit, optional `-dirty`, date), and the reader does not check that
shape at all. So the two halves disagree about what a cut line is, not
only about how to spell its prefix.

**Fence.** `scripts/tess_budget_cut.sh` is on retired Track J's unowned
ground (`plan.md`, *What this partition leaves out*), and
`tools/tess-lint/` is Track K's. A row landing on this draws the fence
first, in the same PR that mints it.

## Claimed by METER (2026-09-06)

Moved from `work/code-quality/` to `work/meter/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, `track:` letter and body unchanged. Unlettered; `tess-lint`'s `CUT_PREFIX` is METER's and `scripts/tess_budget_cut.sh` is CIW's — the pin's direction is drawn with CIW in the PR that lands it.

## Closed

`tools/tess-lint/tests/cut_line_pin.rs` is the gate the script's own
header said did not exist. It `include_str!`s
`scripts/tess_budget_cut.sh` — the path is load-bearing, the crate
stops compiling if the script moves — and holds three claims.

**The three spellings, each located on its own.** The validator's
anchor (`CUT_RE='^…`), the writer's `echo`, and the re-stamp strip
(`grep -v '^…'`) are each found THROUGH `CUT_PREFIX` rather than
counted, so each drifts into its own failure; a sweep over the bare
stem `tess-budget-cut` then refuses any fourth spelling nobody has
written yet. Proved red by breaking each in turn in a scratch copy of
the script (M1 anchor renamed, M2 `echo` loses the space, M3 strip
drops the colon; M5 swaps the writer's two fields, which the
`echo`-rendering test catches where the prefix pin cannot).

**The shape, which was the finding.** Since this was filed,
`split_cut` grew a shape check of its own — so the two halves now both
constrain what follows the prefix, and the question became whether
they constrain the SAME thing. They did not: this crate admitted
uppercase hex, an object name longer than 40 characters, and any
whitespace after the prefix, none of which `CUT_RE` matches and none
of which `tess_budget_cut.sh` can emit. That asymmetry is not
symmetric in cost — a line this crate reads as a cut but the script
does not recognise as a stamp is re-stamped by the script's first arm,
walking the record forward past the rows it describes, which is the
inversion that arm exists to prevent. `split_cut` is tightened to the
script's language (lowercase hex, `7..=40`, exactly one space) and the
two are held together by a truth table run against the script's own
regex, extracted from its text and executed by `grep -E`. Each
tightening proved red by reverting it (R1 uppercase, R2 the cap, R3
the whitespace) and each direction of the table proved red by drifting
the regex (M4 floor raised, M6 uppercase admitted).

**Routed by `CC1` and `CC5`** (`tools/README.md`). `CC1` puts the
check at the reading boundary and `split_cut` is that boundary for
the provenance line; `CC5`'s test — an admission refuses what the
producer could not have written, never what the instrument exists to
measure — is what makes each bound admissible, and its voice is the
harness voice because what a malformed cut breaks is the reading, not
a measurement. **The clauses are stated over CROSS-COLUMN readings
and a cut line is not a column**; what transfers is `CC5`'s test,
which `CC5` itself defers to `tess_lint::Report` for. Cited for the
test and the site they give, not as a claim that the page's subject
covers this. That the page's scope is narrower than the rule it
carries is worth Ev's eye while the page is under ratification
(`tools-readme-is-unratified-and-owes-a-design-row`).

**The CIW seam.** `scripts/tess_budget_cut.sh` was read and never
edited; `scripts/tess_budget_sweep.sh` was read for the call site.
What a CIW-side change needs to know: the pin lives entirely on this
side and reds in `tools/tess-lint`'s suite, which is NOT in the
workspace and is not what a `scripts/`-only change would think to
run — CI runs it, and the failure names the script path and line. Any
edit to the prefix, to `CUT_RE`, or to the `echo`'s field list reds
loudly and immediately. A LOOSENING of `CUT_RE` alone does not red,
by design: it only makes the script decline to refuse a re-stamp.

**The adjacent item is left, not ridden.**
`cut-line-commit-names-no-baseline-change` is the same seam and asks
that the reader state what the stamped commit MEANS. Its substance is
what the verdict PRINTS — `main.rs`'s report voice — which no pin here
reaches; and its own text says the two are fixed by different edits.
A doc-only rider would half-close it and leave the printed half
looking done. It stays open, untouched.

**Residue:** `cut-regex-unanchored-admits-a-line-the-lint-refuses` —
`CUT_RE` has no end anchor, so it admits a cut line with junk after
the date that this crate refuses, and the script's already-stamped arm
then blocks the repair path. Its own file; the truth table's
`TRAILING_TEXT` row pins the disagreement meanwhile and reds when the
regex is anchored.
