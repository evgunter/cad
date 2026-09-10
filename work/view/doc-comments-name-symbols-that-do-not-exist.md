---
id: doc-comments-name-symbols-that-do-not-exist
kind: issue
title: an unbracketed code span in a doc comment names a symbol nothing checks, and two of them name symbols that have never existed
status: open
opened: 2026-09-09
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
| `crates/viewer/src/session/op.rs:756` | `frame::dropped_hide_notice` | open. The real renderer is `crate::frame::Withdrawal::dropped_hide` (`frame.rs:707`), reached at `app.rs:940-943` |
| `crates/viewer/src/app.rs:947` | `frame::dropped_hide_notice` | open, and **outside the rule above** — see the blind spot |

The sibling at `session/op.rs:756` sits thirteen lines below the one #2272
fixed, in the doc for the neighbouring field, and was left there
deliberately: #2272's fence was the wasm-dead items and the class is
this file's, not that PR's.

## What the rule could not match

1. **Plain `//` comments.** `app.rs:947` is one — same dead name, same
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
   that crate's tree; not attempted here. 85 of the 97 names are in
   this set.
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
