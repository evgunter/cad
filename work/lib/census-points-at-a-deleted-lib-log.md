---
id: census-points-at-a-deleted-lib-log
kind: issue
title: The census's three docs/LIB-LOG.md pointers name a deleted file
status: closed
opened: 2026-09-03
refs: [1661]
closed: 2026-09-03
---

Noticed at LIB-B-PICKING while re-cutting the census; **banked rather
than fixed in that unit's diff**, because the fix touches three
separate hunks of the most contended file on the LIB track
(`test_binding_census.py` is shared with every concurrent B-family
lane) for a reason unrelated to any family. It is a one-commit repair
whenever no two lanes are in it.

## The finding

`crates/pncad-py/tests/test_binding_census.py` points at
`docs/LIB-LOG.md` three times, in the present tense, and that file no
longer exists — the LIB program log migrated into the `work/` tracker,
and the register it names now lives at `work/lib/log.md:439`
("## LIB residual register").

- `:109` — "`docs/LIB-LOG.md`'s residual register, category B, points
  here for the enumeration rather than carrying one in prose."
- `:559` — "`B-` is the register category these entries used to point
  at in prose — `docs/LIB-LOG.md`, 'LIB residual register', category B
  — which now points HERE for its enumeration".
- `:1305` — "a pointer at a PARAGRAPH of `docs/LIB-LOG.md`, which is
  not an [id space]".

The census's whole argument is that a pointer which stops resolving is
a pointer nobody is reading, and
`test_every_gap_entry_names_a_defined_id` enforces exactly that
property in both directions — but for the AUDIT PAGE's ids. These three
pointers are outside that guard's reach: it reads
`docs/guide/north-star-audit.md`, not this filename, so nothing fires.

`docs/DOC-LEDGER.md:395` covers the general case ("append-only logs …
still name deleted files. Those are not broken: the filename plus the
recovery recipe … resolves any of them"). That dispensation is for
HISTORY. Two of these three are live claims about where a reader should
go NOW, which is the same distinction the ledger itself drew when it
edited `crates/mesh/src/sizing.rs` rather than leaving it (sweep 2,
"because its tense made a live claim").

## The fix

Re-point all three at `work/lib/log.md`'s "LIB residual register"
section. The lineage sentence at `:559` should KEEP the old filename as
the historical spelling — it is explaining where the `B-` letter came
from — and only update the destination.

Worth one grep at the same time for the same dangling pointer elsewhere
under `crates/`. At the time of filing the census was the only
non-`work/` file naming it in a live tense; `docs/DESIGN.md:1931` and
`docs/ASM-EXIT-WALK.md:26` name it historically and are covered by the
ledger's dispensation.

## Closed

All three pointers re-pointed at `work/lib/log.md`'s "LIB residual
register", which was confirmed to exist (heading at `work/lib/log.md:439`)
and to enumerate a category B ("B. Bindings-parity residuals (the
north-star audit, executable)", `work/lib/log.md:529`) — the destination
resolves, which is the whole point of the repair.

- module docstring (`test_binding_census.py:109`) — live claim, now
  names `work/lib/log.md`'s "LIB residual register".
- `FAMILIES`' spelling note (`:637`) — the lineage sentence, so it
  KEEPS `docs/LIB-LOG.md` as the historical spelling the entries were
  written against and says where the register lives now. A reader
  arriving with the old filename in hand still lands.
- `test_every_gap_entry_names_a_defined_id`'s docstring (`:1608`) — the
  "pointer at a PARAGRAPH" argument, which is about the register's
  SHAPE and stays true wherever it lives, re-pointed to the new path.

The sweep the issue asked for was run over the whole tree
(`grep -rn "LIB-LOG"`): outside `work/` and `docs/`, the census was and
is the ONLY file naming it — nothing else under `crates/`, nothing in
`demos/`, `scripts/` or `local-scripts/`. The `docs/` hits
(`DESIGN.md:1937`, `DOC-LEDGER.md`, `MODEL-AB-LOG.md`,
`PATHS-DESIGN.md`) are all historical or ledger-covered and were left
alone; `docs/ASM-EXIT-WALK.md` no longer exists, so that citation from
the filing has already gone.

WHAT THIS DOES NOT COVER: nothing guards the class. These three
pointers were found by eye and fixed by hand, and the next dangling
path in a test docstring will be too —
`work/code-quality/doc-line-citations-rot-silently.md` is the open item
for the general sweep, and it is not closed by this.
