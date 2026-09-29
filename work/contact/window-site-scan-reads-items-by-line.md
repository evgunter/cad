---
id: window-site-scan-reads-items-by-line
kind: issue
title: the window-site guard reads items line by line, and a nested fn keeps the name after it closes
status: open
opened: 2026-09-29
priority: P3
cost: E
---

## Finding

`crates/topo/src/boolean/wall_section_rows.rs`,
`the_window_construction_sites_are_the_ones_listed` (its inner `sites`,
`:~411`), attributes each cosine-window mark to the item it falls in by
walking the code view LINE by line and setting `current` at every line
whose code starts with a `fn` head. It never leaves an item: a `fn`
declared inside a site's body takes `current` for the rest of the host,
so a mark after the nested helper's closing brace is filed under the
helper's name, and the host drops out of the set without a mark having
moved. A head split so that `fn` is not at the start of a line after
`pub `/`) `/nothing (`pub(crate) const fn`, `pub async fn`) is not seen
at all, and the marks under it go to the item before.

Dormant: in `solid_contain.rs` and `contain.rs` today no site hosts a
nested `fn`.

## The close

`crate::source_walk::CodeOnly::fns` is this crate's item scan; it reads
nested items as rows of their own and every qualifier run, and each
`FnItem::span` says which item a byte offset falls in — the innermost
holding span is the item a mark belongs to (the same attribution
`live::tests::every_door_that_hands_out_a_live_looks_up_first` makes for
its construction sites). The row should read `sites` through it rather
than keep a second, line-shaped reader.

Found by the ORIGIN `origin-live` lane's sweep for item readers that
assume no nesting, when `CodeOnly::fns` learned to recurse.

## Home (2026-09-29, ORIGIN orchestrator)

Filed in `work/issues/` by ORIGIN's Live-guard lane (PR 3424), then
placed on CONTACT's slate by that PR's review: `wall_section_rows.rs`
is in no program's `paths`, but the guard reads marks in `contain.rs`
and `solid_contain.rs` (CONTACT's) and CONTACT last maintained the row
(CONTACT-3). The fix it points at — read `sites` through
`source_walk::CodeOnly::fns`, which now recurses into nested items and
offers each item's own text (`FnItem::own_body`, PR 3424) — is
ORIGIN's ground; announce the seam there.
