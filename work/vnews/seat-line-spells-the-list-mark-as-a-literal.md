---
id: seat-line-spells-the-list-mark-as-a-literal
kind: issue
title: seat_line joins its items with a bare "; " rather than frame::LIST_SEPARATOR
status: closed
opened: 2026-09-15
closed: 2026-09-20
refs: [withdrawal-causes-join-on-a-mark-a-fault-may-contain]
branch: vnews/seat-line-list-separator
pr: 2917
---



Found by the same sweep as
`startup-notices-join-on-a-mark-a-prefs-notice-contains`, and much
smaller.

## The site

`seats::seat_line` (`crates/viewer/src/seats.rs`) renders one item per
seat and joins them with a bare `"; "`:

    .collect::<Vec<_>>()
    .join("; ")

That string is `frame::LIST_SEPARATOR`'s value. The row read that as a
mark spelled twice and asked only for the direction of the dependency.

## Closed: NOT A DEFECT (`vnews/seat-line-list-separator`, 2026-09-20)

**`seat_line` is not one of `LIST_SEPARATOR`'s consumers and must not
become one.** The two strings are the same characters and are not the
same mark, which is the whole point of the crate's two-level split.
The row's premise — *"what `seat_line` produces is exactly what that
constant is for"* — is false, and the fix it asked for would have been
a regression. No code changed; `seat_line` keeps its own `"; "`, and
its doc comment now says whose mark it is and why it is not that one,
so the next sweep meets the argument at the site.

### The constant's contract, and how this line fails it

`LIST_SEPARATOR`'s own doc (`crates/viewer/src/frame.rs`) states it in
two parts: **"What ONE notice puts between the items of a list of its
own — a `Withdrawal`'s causes, which are the items its counted preamble
introduces."** `crates/viewer/README.md` carries the same test in
"The third consumer was the second level misread": *"nothing counts
them and no preamble introduces them, so there is no enclosing
sentence for them to be the items of"*.

`seat_line` fails both halves, and the evidence is this unit's own
correction to the item (below): the line's only production caller is
`pane::create`, which renders it as a `ui.weak` panel label. **It is
not a notice** — it reaches no `frame::Message` and no rank in
`frame_status` — and nothing counts the seats and no preamble
introduces them. The counter-argument the README disposes of in
advance applies exactly: the seats *happen to share a subject*, and
sharing a subject was explicitly not enough for the startup notices
either.

### The stated benefit inverts into the harm

The row argued that *"a change to `LIST_SEPARATOR` moves the
withdrawal join and the preferences join and leaves this one behind."*
**#2710 removed the preferences join from `LIST_SEPARATOR` on
2026-09-16** — the day after this row was filed — for failing the same
test this line fails. So the population the row wanted this line to
join is smaller than it was, and after the substitution a
`LIST_SEPARATOR` edit would have moved a line from outside that
population. That is the defect the two-level split exists to prevent,
and it would have been the **fourth** consumer misreading the second
level, eight days after the third was removed.

### What this item claimed, checked

- `seat_line` joins with a bare `"; "` and that is `LIST_SEPARATOR`'s
  value: **true**, and it was the only `"; "` whole-literal in
  `crates/viewer/src/` other than the constant's own definition.
- Neither half of an item can carry the mark: **true**, and stronger
  than the item says — `Seat::name()` is a `vocabulary!` word and
  `node.0` is a `u64`, so neither half is user text at all. Irrelevant
  to the disposition: the objection is not ambiguity.
- *"The line reaches the chrome as a lost-pick notice's text"*:
  **false**, and this is what decided the row. `seat_line`'s only
  production caller is `pane::create`'s `ui.weak`; the lost-pick notice
  is `SeatEvent::PickLost`'s own `Display`, a separate sentence built
  from the same VOCABULARY (`Seat::name()`) — which is what
  `seat_line`'s doc comment says. The item over-read the doc comment
  into a claim about the DOOR, and every later step rested on it.
- *"The only question is the direction of the dependency"*: **false.**
  The question was whether there should be a dependency, and there
  should not.
- `withdrawal-causes-…` and `startup-notices-…` as live siblings:
  **both closed**, at #2693 (2026-09-15) and #2710 (2026-09-16).

### The `", "` family is not the same question, and gets no row

`prefs`, `frame`, `app`, `display` (x2), `session::delete` and
`session::refuse` join with a literal `", "`, and `idpass` with
`" and "`. They are not members of anything this row is about: `frame`
names **no constant** for either, so there is nothing to misread and
nothing to reach for. Whether a comma-list deserves a name is a
question about whether to MINT a constant, which is the residue below
asked one level out, and neither is answered here.

### Residue

`seat-lines-item-mark-has-no-name` — the real finding underneath the
false one. `seat_line`'s mark is an unnamed literal, and so is the
mate panel's.
