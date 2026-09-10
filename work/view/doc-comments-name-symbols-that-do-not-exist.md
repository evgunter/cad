---
id: doc-comments-name-symbols-that-do-not-exist
kind: issue
title: an unbracketed code span in a doc comment names a symbol nothing checks, and two of them name symbols #1957 deleted
status: closed
opened: 2026-09-09
closed: 2026-09-10
---



Found by the fix pass of #2272, from that PR's style review. The
review's finding was one dead symbol in shipped code; the sweep below
is what turned it into a class.

## The shape

`scripts/doc-gate.sh` fails only on rustdoc's **bracketed** intra-doc
links. A name written as a bare code span — `` `frame::foo` `` rather
than ``[`crate::frame::foo`]`` — is prose to rustdoc and prose to the
gate. So a doc comment can name a symbol that does not exist, in a file
the compiler builds every day, and nothing anywhere goes red.

This is `stale-file-citations-after-the-split`'s hole one level in.
That item's subject is a `file:line` in a Markdown tracker file, which
rustdoc has never seen; this one's subject is a symbol name in Rust
source, which rustdoc **would** have seen had it been bracketed. The
tracker case needs a gate that does not exist. This case needs one
character on each side.

## The sweep, and the rule that produces its population

Rule: every line under `crates/viewer/src` whose first non-space
characters are `///` or `//!`, scanned for backtick code spans whose
whole content matches `<mod>::<path>` with an optional trailing `()`,
where `<mod>` is one of this crate's own modules (`lib.rs`'s `mod`
list), and where the span is **not** immediately wrapped in `[` `]`.
Each distinct name is then checked for a definition anywhere under
`crates/viewer/src`.

Restricting `<mod>` to the crate's own modules is what makes the
check decidable: for `frame::x` or `session::x`, "not defined under
`crates/viewer/src`" means "does not exist". For `egui::Ui` or
`topo::EdgeKey` it means only that the definition is in another crate.

**Population at `243915f26`, before this PR's one fix: 19 spans over
12 distinct names with `<mod>` restricted to `frame`/`session` (all 19
are `frame::`; `session::` has no unbracketed span at all), and 132
spans over 97 names once every module prefix is admitted. Exactly two
of the 12 have no definition:** (head reads 18/11, the fixed one having
become a bracketed link.)

| Site | Names | Disposition |
|---|---|---|
| `crates/viewer/src/session/op.rs:742-743` | `frame::supersession_notice` (at `243915f26`) | **fixed in #2272**, to `[`crate::frame::Withdrawal::superseded`]` — bracketed, so rustdoc now holds it |
| `crates/viewer/src/session/op.rs:756` | `frame::dropped_hide_notice` | open. The real renderer is `crate::frame::Withdrawal::dropped_hide` (`frame.rs:713`), reached at `app.rs:951-954` |
| `crates/viewer/src/app.rs:950` | `frame::dropped_hide_notice` | open, and **outside the rule above** — see the blind spot |

The sibling at `session/op.rs:756` sits thirteen lines below the one #2272
fixed, in the doc for the neighbouring field, and was left there
deliberately: #2272's fence was the wasm-dead items and the class is
this file's, not that PR's.

## What the rule could not match

1. **Plain `//` comments.** `app.rs:950` is one — same dead name, same
   defect, invisible to a doc-comment rule *and* to rustdoc even if it
   were bracketed, because rustdoc never reads a non-doc comment. This
   is the blind spot that actually bites: it means a bracketing
   convention fixes the class only where rustdoc can reach, and a
   non-doc comment can never be reached at all.
2. **Prose without a code span.** A doc comment that says "the chrome
   renders this through supersession_notice" in bare words matches
   nothing.
3. **A span split across two `///` lines.** The scan is per-line.
4. **Other crates.** Scoped to `crates/viewer/src` because the two
   findings are there; the shape is repo-wide and nothing about it is
   viewer-specific.
5. **Names whose prefix is another crate.** Decidable only against
   that crate's tree; not attempted here. **This said 85 of the 97
   names are in this set, and it is 48** — see the Closed section.
   The decidable set is every own-module prefix, not just the two
   this item happened to look at.
6. **The definition check is a regex** over `fn|struct|enum|type|
   const|static|trait|mod|union` declarations, so an enum variant, a
   struct field, an associated item or a macro-generated name reads as
   undefined. Every such false positive in this sweep was resolved by
   hand; a mechanised version needs the compiler, not a regex.

## What resolving it looks like

Two shapes, and they are not exclusive:

- **Bracket what can be bracketed.** Convert every unbracketed
  `<own-mod>::<name>` span in a doc comment to `[`crate::…`]`, and
  rustdoc plus `scripts/doc-gate.sh` hold all of them from then on.
  Cheap, mechanical, and it converts an unchecked claim into a checked
  one rather than merely correcting today's instance. Note that a name
  in a *cfg-gated* configuration cannot always be linked — see
  `crates/viewer/README.md`'s **Rustdoc posture** ruling, and apply its
  test rather than this bullet: bracket freely on an item the HOST pass
  renders a page for, and on an item it does NOT, the bracket has to
  resolve at the browser target instead — which a host-only name cannot.
  `WebStartupError`'s five variant doc comments are exactly that case,
  and they carry no `cfg` of their own to warn you. So this is not
  unconditional.
- **Check the rest by rule.** The residue after bracketing is
  categories 1–3 above, and a gate for them is the `<file>.rs:<line>`
  gate `stale-file-citations-after-the-split` asks for, aimed at
  symbols instead of lines: a name of the shape `<own-mod>::<ident>`
  in any comment resolves, or the gate fails.

## Why two dead names is enough to file

The tracker learned the name from the code. Counting literal
occurrences of `supersession_notice` in `work/view/*.md`, excluding
`log.md` (append-only, a record of what was believed) and the two
files this item's own PR writes: **15 occurrences across 8 items, 4 of
them in items that are still `open`** —
`frame-module-has-eight-concerns-and-no-holds-row.md:20,77`,
`free-move-drag-dissolved-by-open.md:54` and
`rank-one-discards-the-frames-other-news.md:39`. One of the closed
ones even carries a line number for it:
`four-badges-five-spellings.md:107` cites `frame.rs:232`.

`stale-file-citations-after-the-split` records the `free-move` row as a
citation it must **not** repoint, and that is the right call about the
row. It is the wrong place to stop: the doc comment is where a wrong
symbol became a wrong tracker row, and the sweep that found the row
found the symptom rather than the cause.

## Closed (2026-09-10)

### The population, re-derived

The rule above was re-run as written, at the SHA this item states it
at and at the closing branch's merge base (`5ace0e9eb`). Instrument:
`///`/`//!` lines under `crates/viewer/src`, backtick spans whose
whole content is `<mod>::<path>` with an optional `()`, not wrapped in
`[` `]`, each name resolved against a declaration regex and every
regex miss resolved by hand.

| Reading | at `243915f26` | at `5ace0e9eb` | after this PR |
|---|---|---|---|
| `<mod>` ∈ {`frame`, `session`} | 19 spans / 12 names / 2 undefined | 16 / 10 / 1 | 0 / 0 / 0 |
| `<mod>` ∈ this crate's own modules | 66 / 49 / 3 | 64 / 47 / 2 | 0 / 0 / 0 |
| every module-shaped (lowercase) prefix | 132 / 97 | 132 / 96 | 68 / 49 |

**Both figures this item states reproduce exactly** — 19/12 with the
two-module restriction, and 132/97 unrestricted. The unrestricted one
needed its wording pinned to reproduce: *"every module prefix is
admitted"* means every **module-shaped** (lowercase) prefix, which is
132/97. Admitting type-qualified prefixes too (`Withdrawal::notice`)
gives 284/197, and restricting to this crate's own modules gives
66/49. Only the middle reading matches, and it is the one the
sentence means.

The 09-09→09-10 drift is two units editing `app.rs` and `frame.rs`;
this item's *"head reads 18/11"* was true of #2272's head and is 16/10
at `5ace0e9eb`. A head figure in an item goes stale by construction,
which is why the table above carries its SHA in the column heading.

### What this item got wrong

**1. The two names are not names that never existed.** The title said
so and the argument in `stale-file-citations-after-the-split` rested
on it — a claimed **fourth** class, *"a subject that was never
there"*, distinct from class 1, *"a subject that is gone"*. `git log
-S` over `crates/` refutes it: `supersession_notice` and
`dropped_hide_notice` were both `pub fn`s in `frame.rs`, present from
`6877a40ff` until `4db112ada` — **#1957, the badge PR**, which
replaced them with the `Withdrawal` vocabulary, rewrote the call sites
and left every prose mention behind. The fourth class does not exist;
this is class 1. That item is corrected.

`four-badges-five-spellings.md:107` is the proof rather than the
oddity this item made of it: it cites `frame.rs:232` for
`supersession_notice`, and `6877a40ff` puts it at exactly
`frame.rs:232`. The citation was right when it was written.

This correction makes the case for bracketing **stronger**, not
weaker. An invented name is one author's slip. A deleted one is a
rename that outran its prose, and a bracketed link would have reded
#1957 at `scripts/doc-gate.sh` on its own branch, before any of the
tracker rows learned the dead name.

**2. Blind spot 5's arithmetic.** *"85 of the 97 names are in this
set"* — names whose prefix belongs to another crate — implies 12 names
carry an own-module prefix, which is this item's `frame`/`session`
count. The real figure is **49** own-module-prefixed names of the 97,
so 48 are foreign. The decidability argument this item makes applies
to all 49, not to the 12 it happened to look at, and the closing pass
took the whole 49 because of it.

### What was bracketed, and what was named instead

**All 64 own-module spans (47 names) became `[`crate::…`]` links, and
the ruling's test was applied at every one.** The test is
`crates/viewer/README.md`'s **Rustdoc posture: the host pass is the
gate** — does the HOST pass render a page for the item the doc comment
sits on? — and it was answered from rustdoc's own output rather than
from an attribute grep:

- Every module holding one of the 64 sites has a page under
  `doc/viewer/` in the host pass (`cargo doc --no-deps
  --document-private-items -p viewer --all-features`). `mod app` is
  `cfg(feature = "app")` and the host pass documents that feature, so
  its page exists and its links are held. **So all 64 are the "it
  does" bullet: bracket freely.**
- **The trap's protected sites are outside this population.** The same
  host pass renders `enum.StartupError.html` and `fn.run.html` and no
  page for `WebStartupError` or `run_web`, exactly as the ruling
  describes. `WebStartupError`'s five variant doc comments carry no
  `<mod>::<path>` code span at all, so the rule never selected them
  and nothing in this diff brackets a host-absent item. The trap was
  live and did not fire; the reason it did not is population, not
  judgement, and it would have fired had one of those variants named a
  module path.
- **Receipt, not inference:** the browser pass reads **7 unresolved
  links over 4 identifiers** after this diff — `ThreadEvaluator` ×3,
  `StartupError::Worker` ×2, `crate::evalseam::ThreadEvaluator` ×1,
  `ThreadIndexer` ×1 — which is the README's dated population
  unchanged. 64 new links added **zero** browser-pass errors, so the
  per-site answers above are confirmed by the target the ruling says
  is the only reader for the other bullet.

**Named rather than linked: `app.rs:950`**, and it is the point of the
class. The same dead name sat in a plain `//` comment there. Rustdoc
never reads a non-doc comment, so a bracket there would be
punctuation, not a checked claim. It was corrected by hand to
`frame::Withdrawal::dropped_hide` and is held by nothing afterwards —
which is the residue, filed as
`comment-symbol-names-outside-rustdocs-reach-have-no-gate`.

### A third dead name, found by the bracketing itself

`frame.rs:216` named `pane::viewport::viewport_ui`. Bracketing it made
rustdoc read it and the host pass reded: `viewport_ui` is an inherent
method on `ViewerBehavior` — an `app` item — that is merely *written*
in `pane/viewport.rs:58`, so the module path names nothing. Corrected
to `crate::app::ViewerBehavior::viewport_ui`, which is how `frame.rs:6`
already spells it and which resolves.

This is the argument for candidate 1 in one site. The rule in this
item could not have found it: `viewport_ui` **is** declared under
`crates/viewer/src`, so the declaration regex resolves it and the name
reads as live. Only a resolver that checks the **path** rather than
the leaf can see it, and bracketing borrows rustdoc's.

### The tracker pass

`supersession_notice` in `work/view/*.md`, excluding `log.md` and this
item's own two files: **15 occurrences across 8 items, 4 of them in
items still open** — reproduced exactly, at `243915f26` and at
`5ace0e9eb` both. Of the four live ones, three were repaired and one
was deliberately left:

| Site | Disposition |
|---|---|
| `frame-module-…-no-holds-row.md:21` | **fixed** → `Withdrawal`. A present-tense census of `frame`'s public surface; every other name in it is live, so this was the single wrong element. The same row already lists `Withdrawal` at `:98` |
| `rank-one-discards-the-frames-other-news.md:39` | **fixed** → `frame::Withdrawal::superseded` |
| `free-move-drag-dissolved-by-open.md:54` | **symbol fixed** → `frame::Withdrawal::superseded`; the `app.rs:785` beside it **left as written**, because it was already wrong at this merge base and a fresher wrong number hides the breakage. `stale-file-citations-after-the-split` quoted this row verbatim and is updated so its quote stays true |
| `frame-module-…-no-holds-row.md:76` | **left as written, and correct.** The sentence is dated *"(#1886, 2026-09-05)"* and all three names it uses were real in `frame.rs` on that date. A dated record of a tree that existed is not a stale citation, and rewriting it to today's names would make it false |

That last row is the finding the *"never existed"* error would have
destroyed: believing the names were invented, the honest move is to
correct the sentence, and correcting it would have falsified a true
record. The class is *present-tense claim*, not *dead name*.

One more name fell out of the same commit and is **not** repaired:
`render_causes`, deleted by #1957 alongside the other two and still
named in the present tense at
`joined-notices-nest-their-own-separator.md:18`, an open row. #1957
gave it no named successor, so naming one is a guess — recorded in
`stale-file-citations-after-the-split` under the class that owns it
rather than edited.

### Line shifts

**None.** Every edit in `crates/viewer/src` is within-line — 58
insertions against 58 deletions across 16 files, and each file's line
count is unchanged — so no citation into any of them moved and the
band census does not arise. This was deliberate: the one fix that
wanted a second comment line (`app.rs:950`) fits in 72 characters
against a file whose comments already run to 89, and taking the extra
line would have shifted every `app.rs` citation below 950. The one
file that does grow is `stale-file-citations-after-the-split.md`,
whose own citations were re-read after the last edit.

### Residue

`comment-symbol-names-outside-rustdocs-reach-have-no-gate` — blind
spots 1–3, filed as its own file rather than disclosed here, with the
population measured: **26 plain-`//` own-module names under
`crates/viewer/src`, every one of them live today**, and **zero**
split-span own-module names. A clean population with no gate holding
it. Blind spot 2 is not measurable and is not claimed. Blind spot 4
(other crates) and blind spot 5 (foreign prefixes) are unchanged and
still out of this crate's fence.
