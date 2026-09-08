---
id: record-file-column-read-by-first-colon-split
kind: issue
title: the class - every reader that takes a record's FILE column as everything before the first colon
status: open
opened: 2026-09-08
refs: [bounds-allowlist-select-cuts-at-the-first-colon]
---

## Finding

Turned up by `bounds-allowlist-select-cuts-at-the-first-colon`'s sweep.
That row's subject is one SELECT; the class is every reader in this
directory that answers "which file is this record from" by splitting at
the first colon. A record is `FILE:LINE:TEXT` and a `:` is legal in a
path here and in git, so the reading is exact only while no scanned path
carries one.

**The class IS its grep**, so run it rather than trusting the list below:

    grep -nE 'cut -d:|awk -F:|%%:\*|sub\(PFX|GATE_RECORD_PREFIX_RE' scripts/gates/*.sh

19 hits on this tree. What it cannot match is a column read done inside
an `awk` body with a hand-written `index()`/`substr()` pair, or a `sed`
address spelled some third way; those would have to be read for.

Hit list and disposition:

- `bounds-allowlist.sh` — **fixed** in PR 2157, at all three of its
  readings: the scan's select (now `gate_record_anchor_any`, on the
  record), the diagnosis column, and the READER's own prefix strip
  (`sub(PFX, "", line)`, which left the path standing in the text so
  the walk read `a:Bounds.rs` as a target `a` keyed with `Bounds` and
  `rs` — a sole bracket bound reported as compound). The one reading
  that survives there is the first `:LINE:`, held in one constant and
  planted in both directions.
- `probe-suite-census.sh:492-494,602,797,811,970,976,1173` and
  `:539,687,1178` — split that gate's OWN colon-joined entry strings
  (`mode:crate:module:want`), not records. **Not this class.**
- `viewer-module-kinds.sh:469` — the line-arm/window-arm union is
  deduplicated on a site key built as

      awk -F: '{ k = $1 ":" $2 } !(k in seen) { seen[k] = 1; print }'

  `$1 ":" $2` is `FILE:LINE` only while the FILE column carries no colon.
  For `vocab/forms.rs:x.rs` every record shares ONE key whatever line it
  is on and all but the first are dropped:

      printf 'a/b.rs:x.rs:3: mod one\na/b.rs:x.rs:9: mod two\n' \
        | awk -F: '{ k = $1 ":" $2 } !(k in seen) { seen[k] = 1; print }'
      a/b.rs:x.rs:3: mod one

  Direction is BLIND: sites vanish from the count the `VOCAB_EXCEPTIONS`
  entries are compared against, so an exception count reads low and the
  gate can go green over a site nobody argued. **Open.**
- `lib.sh:743` — `GATE_RECORD_PREFIX_RE='^[^:]*:[0-9]+:'` itself. As an
  ANCHOR (`gate_record_anchor`) the first-colon shape is deliberate and
  argued at its header: a record from a colon-carrying path is NOT
  claimed by an entry's anchor, which is the direction a skip wants.
  Used as a PARSER the same expression is a silent drop, and that use is
  not argued there. **Open, and it is the repair site the whole class
  turns on**: three callers below.
- `lib.sh:1195-1196`, consumed at `:1150` — `gate_test_only_mounts`
  narrows `mod` declarations with `gate_grep -oE
  "$GATE_RECORD_PREFIX_RE.*…mod …"`. A record from a colon-carrying path
  matches nowhere, so `-oE` emits nothing:

      printf 'a/b.rs:x.rs:3: #[cfg(test)] mod one\n' \
        | grep -oE '^[^:]*:[0-9]+:.*[[:space:]]mod [a-z_][a-z0-9_]*$'   # no output

  The declaration is never registered as a test-only mount and the
  mounted subtree is read as PRODUCTION by every caller of
  `gate_production_sources`. `${decl%%:*}` at `:1150` is the same reading
  again and is consistent with the pattern above rather than a second
  defect. **Open.**
- `lib.sh:1041` — `gate_declaration_shape` skips any record the prefix
  RE does not match (`if (!match(s, …)) next`), so a colon-carrying
  file's `mod` declaration is invisible to the resolver that places it.
  **Open**, same repair.
- `lib.sh:844` — `gate_exact_skip_record_for` strips the prefix with
  `sed -E "s/$GATE_RECORD_PREFIX_RE//"` to get a skip's TEXT. Its input
  is a process substitution (`/dev/fd/N`), a path that carries no colon
  by construction, so nothing is reachable here today; it is the same
  expression and moves with the repair. **Open, low.**

**Population is zero today**: `find crates/*/src -name '*:*'` returns
nothing.

## Why it is worth a row

Construction value, as the row that disclosed it: the direction is blind
at `viewer-module-kinds.sh:469` and at `lib.sh:1041`, cry-wolf at
`lib.sh:1195` (a test-only tree read as production), and each wants a
fixture rather than a claim. The `lib.sh` half is ONE decision — a
parser that reads the column the way a record is actually shaped (up to
the first `:LINE:`), or an explicit refusal at the sites that cannot —
made once for every gate that reads records, which is why it is not a
rider on any single gate's unit. `bounds-allowlist.sh` now spells that
reading locally (`BOUNDS_RECORD_LINE_RE`) and says so at the constant;
whichever way this row goes, that constant is what it replaces.
