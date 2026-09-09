---
id: LIB-EDITS
kind: unit
title: the five DocEdit arms with no Python door get their constructors
status: review
opened: 2026-09-09
branch: lib/edits
refs: [five-doc-edit-arms-have-no-python-door]
pr: 2251
---



Closes `work/lib/five-doc-edit-arms-have-no-python-door.md` (census
family `B-DOC-EDITS`) for the arms that could be closed, and files the
three that could not.

## Delivered

**Two doors, not five.** `DocEdit.set_param(node, slot, expr)` and
`DocEdit.rebind(from_name, to_name)` are built, with their `pncad.pyi`
stanzas, one test row per tag each can raise and one success row each
read back off a fresh evaluation. `DocEdit` has fourteen static
constructors. The other three arms stop, each for a reason it states:

- the two witness arms carry `WitnessDatum` and `BranchCertification`,
  which the façade does not curate at all —
  `work/lib/the-witness-edits-need-a-facade-type.md`;
- `SetExpression` refuses a bad address with a message that renders
  the address through `Debug`, which the binding's prose gate panics
  on, so the door would panic exactly where it must refuse —
  `work/lib/the-expression-path-edit-cannot-refuse-as-prose.md`
  (parked on DOCM's `debug-in-prose-residue-after-finding-sink`).
  The door was written and its refusal executed; that is how this was
  found.

**A slot crosses as its WORD, not as an enum mirror.** `set_param`
takes the word `EditError.slot` answers in, so a refusal is an address
the caller retries at unchanged. The alphabet is read inward by
`crates/pncad-py/src/slot_word.rs`, sited apart from `tags.rs` because
that module's `pub fn`s are read as a table by the tag-value guard,
which understands `-> &'static str` and nothing else. The round trip
is pinned against that guard's own committed inventory
(`tests::every_slot_word_reads_back_to_the_slot_it_names`), so a slot
the kernel adds arrives through a table already required to move.

**One door for every continuous slot, against one door per structural
slot.** The `bind_*_param` trio names its slot in the door; this one
takes the word. The two decisions differ because the vocabularies do:
the structural slots are four, each a different concept, and the
continuous ones are the whole named alphabet a refusal already
publishes.

**`profile` refuses in its own sentence.** It is a word of the
alphabet with no slot to read back — the rest of that address is two
integers and an argument role the word does not carry — so it is not
reported as a misspelling. Any other unknown word is the boundary
`ValueError` a non-name text already raises.

**`rebind`'s halves are `from_name` / `to_name`**, the role suffix
`EditError.from_kind` / `to_kind` take, for the reason those two take
it: `from` is a Python keyword.

**Census.** Two `MEMBERS_NOT_BOUND` `gap:` rows leave the roster
ENTIRELY rather than moving to `MEMBERS_BOUND_AS`: `set_param` and
`rebind` spell `SetParam` and `Rebind` namesake for namesake, and rule
1 accounts a member the stub spells (`test_the_member_rosters_decay`
says so in both directions). `B-DOC-EDITS` stays in `FAMILIES` with
three rows and a re-written charter that says what each needs first.

**Prose at the sites.** `Node.extrude`, `revolve`, `fillet`, `chamfer`
and `shell` each name the door that moves the literal they mint and
the slot's word; `test_shell.py`'s docstring no longer says the
Python surface cannot rebuild through a slot edit, and carries the row
that does it on the live document.
