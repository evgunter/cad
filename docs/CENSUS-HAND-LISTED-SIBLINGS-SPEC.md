# CENSUS-HAND-LISTED-SIBLINGS — six hand-listed walks, four suppressed and two the census cannot see (spec)

Binds one unit. Item:
`work/census/hand-listed-partialeq-siblings-outside-the-census-debug-fence.md`.
Branch: `census/hand-listed-siblings`. Deleted post-merge with a
`docs/DOC-LEDGER.md` entry citing the merge SHA.

Read the item first. It is the *checked rest* CENSUS-DEBUG filed rather
than swept, and it is the first unit in three that does not touch
`crates/pncad-py/src/tests.rs` — deliberately, because that file's size
row is this program's largest debt and whether CENSUS should keep
working in it is an open question with its owner.

## Why this row is next

**Its routing decays and nothing else on the slate does.** Six sites in
five crates, and which program owns each is a fact with a shelf life:
every program that opens or closes moves an owner, and the row already
records two files claimed by no open program. The repair is one
destructure per site; the cost of waiting is that the answer to *whose
is this* has to be re-derived from scratch.

## What I verified, and the one thing that was wrong

Re-derived against `origin/main` at `8da78cd08`. **Do not take these on
trust** — the standing record of this program is that the
orchestrator's premises decay, and a lane has found an error in most of
them.

1. **Five of the six citations are exact.** `expr.rs:519`,
   `mate/coset.rs:147`, `program.rs:1276`, `props.rs:491` and
   `names/role.rs:197` each open the impl the row names.
2. **The sixth was wrong and is corrected in this change.**
   `crates/profile/src/lib.rs:704` is `SketchPlane::u`; `bit_eq` is at
   **`:733`**, and it reads twelve coordinates through `origin()` and
   `placement.linear`, exactly as the row describes.
3. **Ownership is unchanged**, re-derived from the open programs'
   `paths` globs: `names/role.rs` and `program.rs` are EDIT's,
   `mate/coset.rs` MSOLVE's, `profile/src/lib.rs` BOOL's, and
   **`expr.rs` and `topo/src/props.rs` are claimed by no open
   program.** Re-derive this at the commit that lands, not from this
   page.
4. **`KNOWN_HAND_LISTED` holds five entries**, not four —
   `clearance.rs` (SHELL's own row) plus the four of these six it can
   see: `expr.rs`, `mate/coset.rs`, `program.rs` and `props.rs`. I
   read the array's length wrong on a first pass and the row was right;
   check it yourself rather than believing either of us.
5. **The census forces your bookkeeping, and this is the mechanical
   fact to plan around.**
   `every_known_hand_listed_impl_is_still_found`
   (`crates/test-utils/tests/hand_written_impl_census.rs:774`) reds
   when an entry names an impl the walk no longer finds hand-listed.
   So **repairing a suppressed site without removing its entry in the
   same diff reds the suite** — which is the loud direction and is
   working as designed.

## The repair, and the one site where it is not a destructure

Four of the six are one destructure each: bind **every** field by name,
and where a field is deliberately outside the comparison or the
rendering, bind it to `_` *with the reason at the binding* — so a field
added later is an E0027 at the site rather than a silent omission. The
deliberate omissions are already documented and already right: `expr.rs`'s
display unit (D7, presentation metadata) and `names/role.rs`'s `stamp`
(D9, a cache and never a decision). **This unit does not change what any
of these comparisons MEANS.** It ties them to their declarations.

**Never a `..` rest pattern.** It binds nothing, so it is the same
defect wearing a destructure's clothes — and this is not a style
preference: the census beside these sites already refuses a destructure
with a rest arm by name. A fix that reached for `..` would be this
program's standing trap, landed in the one file that would catch it.

**`crates/topo/src/props.rs`'s `SignCertificate` is the hard one and is
not a destructure at all.** It `write!`s a braced struct shape —
`SignCertificate { volume in […], surface_area …, open_at …,
target_refusal … }` — and **none of the four things it prints is a
field**. So it makes `finish()`'s completeness claim by hand while
having no tie to the declaration to restore. Two shapes, and **I am not
choosing between them**: stop it looking like a struct, or give it a
tie to the declaration that a new field breaks. Take the measurement
that decides it — what does a reader of this output believe, and what
would a new field on `SignCertificate` do to that belief — and say why
at the site.

## The two the census cannot see are the interesting half

`SketchPlane::bit_eq` (behind a `PartialEq` delegation, reading fields
through a METHOD, which no text reader can tell from any other call)
and `NameRef` (through a tuple index into an inner type whose
declaration no text walk reaches). Nothing but the row holds either.

**An entry in `KNOWN_HAND_LISTED` for them would red every run**, per
fact 5 — the walk never reports them hand-listed, so the entry would
name an impl it cannot find. So if your repair leaves either still
invisible, say at the site what holds it and why the census cannot,
and leave the row carrying it. **If your repair makes one of them
visible, that is a result worth naming** — and then its suppression
entry becomes possible and the row shrinks.

## The trap, named as this unit's specific growth direction

The standing trap is that **the fix mints a fresh instance of the
defect it closes**, and it has sprung on all seven units of this
program.

This unit's version: the defect is *a hand-written list of fields that
looks complete and is not tied to the declaration.* Three ways to mint
a fresh one —

- a `..` rest pattern (above), which is the defect exactly;
- a destructure that binds every field **today** and a `match` or a
  helper beside it that still reads a subset by name, so the tie is at
  one site and the reading is at another;
- `SignCertificate`'s repair producing a second hand-written list —
  a format string, a field roster, a helper — that no declaration
  holds.

State, in writing at each site, **before** you write the repair: what
holds this complete, and what would still be silent if a field were
added tomorrow. Then execute that second question at every site — add
a field, see what fails, remove it — and report what came back.

## The silent-omission obligation

This unit changes sites a census reads and may change what that census
can see, so it carries the obligation: **an instrument that stops
seeing part of its population reports agreement over what it can still
read, and an invariant row goes green.** If anything you do changes
what `hand_written_impl_census.rs` finds, mutation-prove it both ways.

## Acceptance

1. Every one of the six sites ends in a repair with the added-field
   case **executed**, or a written refusal at the site with its own
   execution. No site ends in a sentence alone.
2. Suppression entries for repaired sites are removed **in the same
   diff**, per fact 5.
3. No `..` rest pattern anywhere in the diff.
4. `SignCertificate`'s disposition is chosen by a stated measurement,
   not by preference.
5. Every count or citation you write is re-derived at the commit that
   writes it. This program has had a stale count born stale six times,
   and this row shipped with a wrong line number.
6. Hosted CI green; `python3 scripts/work.py lint` clean.

## Territory — this unit crosses four fences and you announce each

CENSUS claims no paths. `crates/editor-core/src/{names/role.rs,program.rs}`
are **EDIT's**, `crates/editor-core/src/mate/coset.rs` is **MSOLVE's**,
`crates/profile/src/lib.rs` is **BOOL's**;
`crates/editor-core/src/expr.rs` and `crates/topo/src/props.rs` are
claimed by no open program. `crates/test-utils/*` is **TCOST's and
TINT's** — removing a `KNOWN_HAND_LISTED` entry is the bookkeeping that
census demands of a repair, not a change to the instrument.

Name every crossing in the PR body, and re-derive the list with
`python3 scripts/work.py territory --base origin/main` rather than
copying it from here. **A site whose owner you cannot establish is one
to split out and leave**, per the row's own closing line: a lane may
take any subset and split its site rather than carry the rest.

## What this unit is NOT

- It is not a change to what any comparison or rendering MEANS.
- It is not `clearance.rs` — that is SHELL's, on their own row.
- It is not a widening of `hand_written_impl_census.rs`'s walk. If you
  find you want one, file it; `crates/test-utils/*` is not CENSUS's.
