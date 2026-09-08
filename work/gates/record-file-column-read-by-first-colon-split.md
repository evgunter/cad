---
id: record-file-column-read-by-first-colon-split
kind: issue
title: the class - every reader that takes a record's FILE column as everything before the first colon
status: review
branch: gates/record-column-parser
pr: 2174
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

## Landed

One reading, in `lib.sh`'s new §"THE COLUMNS OF A RECORD":
`GATE_RECORD_LINE_RE=':[0-9]+:'` is where the FILE column ends, and
three ways in read it — `gate_record_file` and `gate_record_text` for a
pipeline stage, and `gate_record_awk`, the one way into the `awk`
function `gate_record_split(rec)` (GR_FILE / GR_LINE / GR_TEXT): it
puts the constant in the environment and prepends the snippet, so a
caller writes its program and nothing else. Spelled by hand the call is
two parts that must agree, and forgetting the env half is SILENT —
`match(rec, "")` succeeds at position 1 and every record reads as an
empty FILE column — so the function refuses an empty constant through
the marker, and a scratch gate in `gate_selftest_clean` plants exactly
that. `GATE_RECORD_PREFIX_RE` stays, built from the same constant, and
now says at its definition that it is an ANCHOR and not a parser, with
no caller today.

The row's open sites, converted: `viewer-module-kinds.sh`'s union dedupe
key; `lib.sh`'s `gate_test_only_mounts` (the `-oE` narrowing, and the
three columns now arriving as `LINE:NAME:FILE` so the one field that may
carry a colon is the one `read` hands the remainder to),
`gate_declaration_shape` and `gate_exact_skip_record_for`;
`bounds-allowlist.sh`'s local `BOUNDS_RECORD_LINE_RE` /
`gate_record_file_column`, which are now the shared helper.

The class was bigger than the row's grep. That grep cannot see a column
read done inside an `awk` body with a hand-written `index()`/`substr()`
pair — the row says so — and there were seven of them, in six gates,
each the same
first-colon split: `panic-free-macro-bodies.sh` (its `macro_bodies`
reader AND its `PANIC_RE` fence, which is why a panic token in a macro
body in such a file was seen by nothing), `interval-square-allowlist.sh`
(the binding hop and the census report), `loop-boundary-discards.sh`,
`bit-identity-debug-only.sh`, `no-extra-real-bounds.sh` and
`viewer-vocab-declared-once.sh`. All converted.

The residual ambiguity — `foo:12:bar.rs`, a path spelling a `:LINE:` of
its own — is registered once, in `lib.sh`'s new section, as the one
shape no reader of a `FILE:LINE:TEXT` record can resolve; this reading
takes the first `:digits:`. `bounds-allowlist.sh`'s KNOWN GAP 8 keeps
what the shape costs THAT gate and points there for the rest.

Fixtures, one per converted reader, each mutation-proved by restoring
the first-colon reading at that site: the viewer gate counts two sites
in `a:b.rs` as two (and reds on a third); the resolver excludes the
subtree a declarer at `a:b.rs` mounts, in all three of its callers;
`panic-free-macro-bodies.sh` fires on a macro body in `a:b.rs`;
`loop-boundary-discards.sh` names that path whole in its UNREG line;
`viewer-vocab-declared-once.sh` names it whole with its line;
`interval-square-allowlist.sh` raises the binding-hop candidate in it;
`no-extra-real-bounds.sh` stays quiet on a SOLE bound in a path whose
tail the walk used to read as a predicate; and `bit-identity-debug-only.sh`
takes one further SUBJECT at such a path — its row list is what decides
what it reads, so the fixture adds a row through the environment the way
`viewer-module-kinds.sh` adds an exception entry (pins are not compared
under `--root`, so the extra row costs nothing else) — green while the
use is gated and red naming that path AND the leak's line, which is both
columns the reading decides.

One text for a reader that could not run, `gate_reader_died_refusal`:
the class had four spellings (`gate_grep`'s, the shared Rust reader's,
the record columns' and `viewer-vocab-declared-once.sh`'s
`reader_failed`), so a CI reader met one event under four descriptions.
Every `: >> "$GATE_MATCHER_FAILED"` site was swept: the other seven are
not this class — four are a SKIP or a LIST being wrong, and three are a
reader that read something it could not place — and they keep their own
texts.

Every gate's live stdout and stderr is byte-identical to the merge
base's, statuses included, and every `--selftest` passes.

A rider from PR 2170's review, since these files were this lane's for
the hour: `gate_selftest_homes --subject` is adopted at the six callers
that still checked their homes by path alone —
`bit-identity-consumer.sh`, `bit-identity-punning.sh`,
`evalscalar-allowlist.sh`, `interval-square-allowlist.sh`,
`no-ambient-env.sh` and `register-equal-allowlist.sh` — so each
home-gone case now wants the SENTENCE the gate hands
`gate_require_homes` as well as the path. `register-equal-allowlist.sh`
merged two subject groups into one call, which is the shape the flag
exists for: it is split into one call per subject, so a definition home
can no longer pass wearing the caller subject. Mutation-proved by
swapping that gate's two subjects, which reds its home-gone case.
