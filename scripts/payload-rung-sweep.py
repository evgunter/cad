#!/usr/bin/env python3
"""**The payload-rung sweep, run rather than re-derived.**

A curated façade list carries a refusal and, one rung down, that refusal's
payload: the discriminant a caller branches on once it has matched the arm. If
the payload's type is on no curated list, the arm is matchable and its content
is not — the caller reads it out of `Display` prose or not at all. Finding
those rungs is the sweep.

WHY IT IS A SCRIPT. The pattern was described in prose and re-implemented by
each unit that ran it, and no two implementations agreed on a number: three
runs reported 115, 137 and 208 raw hits over a tree whose curated set moved by
four names, and one of them disagreed with itself by a row inside one document.
A count nobody can re-derive is not a measurement. Here the pattern is code, the
counts come out of one implementation, and a count that moves is a diff.

WHAT IT COMPUTES, in the four numbers `--counts` prints:

  curated   every name the façade's curated lists introduce, with WHICH list
            each is on (`crates/pncad/src/{document,select,prelude,profile}.rs`)
  declared  every `pub enum` / `pub struct` declared in the façade's own
            path-dependency set (the transitive non-dev `path =` closure of
            `crates/pncad`), indexed by crate
  raw       (carrier, payload) pairs: for every curated name that resolves to a
            declared type, every type identifier in that declaration's body —
            an enum arm's leading identifier dropped, a struct's bare-`pub`
            fields only — that is itself declared in the set and on NO curated
            list
  narrowed  the CUR3/CUR4 shape: the payload is an ENUM (a discriminant a
            caller branches on), is not itself an error type, and is declared
            in the same crate as its carrier

THE FOURTH AND FIFTH LISTS. Every early run read three — `document.rs`,
`select.rs`, `prelude.rs` — because that is the set
`crates/pncad-py/tests/test_binding_census.py` read. The façade curates
`profile.rs` by hand as well, and a scan that does not read it cannot tell
"uncurated" from "curated on a list I do not read": six hits one run reported
as the former were already on the fourth. `analysis.rs` is the fifth, added
when the census started reading it too — the two readers share this file's
resolver, so a census over five lists and a sweep over four would be the same
"no two runs agree on a number" defect one layer up. Adding a file is not the
whole fix, because the lists are not one surface — `profile.rs` is the profile
layer's whole presented root, the prelude is the glob surface, and
`analysis.rs` splits into one ungated `pub use` and five behind
`#[cfg(feature = "interval")]` that this reader cannot see as gated at all
(blind spot (j) below) — so this script reports which list each side of a row
is on, and separates two states that a three-list run flattened into one:

  UNCURATED  the payload is on no list at all
  CROSS-LIST the payload IS curated, on a list that does not carry its carrier

A CROSS-LIST row is not a defect by itself: what a curated list owes is
matchability THROUGH it, and a payload on the profile list while its carrier is
on the prelude is a real state with a real question. It is reported so the
question gets asked, and it is counted separately so it cannot inflate the
uncurated count.

BLIND SPOTS, each stated beside the code that has it and indexed here. The
letters are the ones the earlier runs' items assigned, kept so a reader can
follow one across four documents.

  (a) struct payloads              CLOSED  `_DECL`
  (b) alias / associated type      OPEN    `sweep`
  (c) refusals the façade declares OPEN    `curated`
  (d) generic parameters           OPEN    `payload_identifiers`
  (e) macro-MINTED names           OPEN    `_DECL` (a macro-written `pub enum`
                                           is indexed; a macro-minted NAME is
                                           not)
  (f) private struct fields        CLOSED  `payload_identifiers`
  (g) variant names as payloads    CLOSED  `payload_identifiers`
  (h) crate-aware, not module-     OPEN    `sweep`
      aware
  (i) registry dependencies        OPEN    `dependency_set`
  (j) `cfg`-gated `pub use` items  OPEN    `curated` (a name behind
                                           `#[cfg(feature = "…")]` is read as
                                           curated unconditionally, so a rung
                                           reachable only in one feature
                                           unification looks like any other)

THE DISPOSITION TABLE (`DISPOSITIONS`) is the argued set as data: every
narrowed name that has already been decided, with the home of the argument. It
is what makes the next run a diff — `--check` fails when the narrowed set stops
matching it, so a NEW rung fails a gate rather than waiting for the next
re-sweep to notice it.

A SECOND READER ON THE SAME DECLARATIONS, and no second implementation.
`declared_members` answers what a declaration's members are CALLED — an enum's
variant names, a struct's bare-`pub` field names — which is the other half of
the chunk `payload_identifiers` reads for types. Nothing in this script's own
report uses it: `crates/pncad-py/tests/test_binding_census.py` does, to ask
whether the Python namesake of a curated Rust name spells that name's MEMBERS
or only the name. It lives here because the resolver it needs is this one
(`dependency_set`, `declarations`, `curated`), and the defect the whole script
closes is a pattern re-implemented per caller. Its blind spots are this file's,
stated at the function, and the fixture battery below is where both readers are
pinned.

  payload-rung-sweep.py             the counts and the narrowed table
  payload-rung-sweep.py --json      the same, machine-readable
  payload-rung-sweep.py --check     narrowed names == DISPOSITIONS (the pin)
  payload-rung-sweep.py --lists document,select,prelude
                                    restrict the curated set to those lists,
                                    which is how an earlier run's numbers are
                                    reproduced
  payload-rung-sweep.py --selftest  the fixture battery below
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
import tomllib
from pathlib import Path

FACADE_CRATE_DIR = "crates/pncad"
FACADE_SRC = "crates/pncad/src"
ALL_LISTS = ("document", "select", "prelude", "profile", "analysis")


# --- the disposition table -------------------------------------------------
#
# Every name in the narrowed set, with where its disposition was argued. The
# sweep's `--check` mode requires this table's keys to equal the narrowed
# names exactly, in both directions: a new rung fails, and a name that stops
# being a rung fails too rather than leaving a stale row here.
#
# `argued` — decided, with the argument in the tree at the cited home.
# `filed`  — a rung nobody has decided yet, with the tracker item that holds it.
# `false-positive` — a row this scan reports and a reader has refuted, with the
#            blind spot that produces it.
DISPOSITIONS: dict[str, tuple[str, str]] = {
    "Attr": ("argued", "NOT_CARRIED, the appearance family (crates/pncad/tests/all.rs); "
                       "crates/pncad/src/document.rs says why the record types stay out"),
    "BandField": ("argued", "non-carriage with its falsifier, crates/pncad/src/prelude.rs"),
    "CarrierRelation": ("false-positive", "blind spot (b): the prelude carries this very "
                        "declaration under the alias `PlaneRelation` "
                        "(crates/topo/src/boolean/plane_eq.rs)"),
    "Diagnosis": ("argued", "the telemetry half, deliberately interior, "
                            "crates/pncad/src/select.rs"),
    "KProbe": ("filed", "work/lib/kprobe-is-a-rung-under-drive-config-on-the-analysis-list.md"),
    "MappedCurve": ("argued", "non-carriage with its falsifier, crates/pncad/src/prelude.rs"),
    "MetaValue": ("argued", "NOT_CARRIED, the metadata family; crates/pncad/src/document.rs "
                            "says why the value tree stays out"),
    "PairingViolation": ("argued", "NOT_CARRIED, the analysis lane's interior residue "
                                   "(crates/pncad/tests/all.rs)"),
    "ParamValue": ("argued", "NOT_CARRIED, a curated face of a different shape "
                             "(crates/pncad/tests/all.rs)"),
    "Qualifier": ("argued", "NOT_CARRIED, the naming interior (crates/pncad/tests/all.rs)"),
    "RecipeEditRef": ("argued", "the telemetry half, deliberately interior, "
                                "crates/pncad/src/select.rs"),
}

# The same table for the CROSS-LIST set — a payload that IS curated, on no list
# that carries its carrier. It is separate because the question is a different
# one: not "can a caller name this at all" but "can a caller name it through
# the door it arrived by".
CROSS_LIST_DISPOSITIONS: dict[str, tuple[str, str]] = {
    "EntityKind": ("argued", "the naming vocabulary is `select`'s and is spelled once, "
                             "crates/pncad/src/select.rs; the general rule is at the "
                             "payload-rule header of crates/pncad/src/document.rs"),
    "SplitHalf": ("argued", "the naming vocabulary is `select`'s and is spelled once, "
                            "crates/pncad/src/select.rs; the general rule is at the "
                            "payload-rule header of crates/pncad/src/document.rs"),
    # The same rule with the two lists swapped, and it decides these the same
    # way: the payload's vocabulary is the DOCUMENT layer's, so it is spelled
    # once on `document` and the analysis list points at it. That split is the
    # design's rather than the façade's — `crates/pncad/src/analysis.rs`'s own
    # head says a distribution is document state its doors author and persist,
    # while everything on the analysis list is DERIVED from it and never
    # stored — so the analysis lane is the clearest case the rule has.
    "Dimension": ("argued", "the parameter vocabulary is `document`'s and is spelled "
                            "once; the general rule is at the payload-rule header of "
                            "crates/pncad/src/document.rs"),
    "Distribution": ("argued", "the parameter vocabulary is `document`'s and is spelled "
                               "once; the general rule is at the payload-rule header of "
                               "crates/pncad/src/document.rs"),
    "MeasureUnavailableAt": ("argued", "the measurement vocabulary is `document`'s and is "
                                       "spelled once; the general rule is at the "
                                       "payload-rule header of "
                                       "crates/pncad/src/document.rs"),
}


# --- reading Rust ----------------------------------------------------------

# A string literal's opening token: an optional `b` byte prefix, an optional
# `r` raw prefix and its hash run. Matched only where the preceding character
# cannot be part of an identifier, so the `b` of `numb"…"` is not read as a
# byte-string prefix.
_STRING_OPEN = re.compile(r'(b?r?)(#*)"')
# A char literal, which is what separates `'a'` from the lifetime `'a`.
_CHAR_LITERAL = re.compile(r"'(\\.|[^\\'\n])'")


def code_only(src: str) -> str:
    """`src` with every comment and string literal blanked, newlines kept.

    Blanked rather than cut so a byte offset still names its line. The façade
    lists argue at length about types they deliberately do NOT export, and a
    scan that read prose would count a name a comment mentions as carried; the
    same holds one level down, where a doc comment on a refusal names the very
    payload types this sweep is looking for.

    Rust's lexical forms that can hide a `//`, a brace or a type name are all
    handled: line comments, NESTED block comments, strings with escapes, raw
    strings with any hash count, byte strings, and char literals. A `'` that
    does not open a char literal is a lifetime and is left alone.
    """
    out = list(src)
    i, n = 0, len(src)

    def blank(start: int, end: int) -> None:
        for k in range(start, min(end, n)):
            if out[k] != "\n":
                out[k] = " "

    def ident_char(k: int) -> bool:
        return 0 <= k < n and (src[k].isalnum() or src[k] == "_")

    while i < n:
        if src.startswith("//", i):
            j = src.find("\n", i)
            j = n if j < 0 else j
            blank(i, j)
            i = j
        elif src.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if src.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif src.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            blank(i, j)
            i = j
        elif (m := _STRING_OPEN.match(src, i)) and not ident_char(i - 1):
            if "r" in m.group(1):
                close = '"' + m.group(2)
                j = src.find(close, m.end())
                j = n if j < 0 else j + len(close)
            else:
                j = m.end()
                while j < n:
                    if src[j] == "\\":
                        j += 2
                    elif src[j] == '"':
                        j += 1
                        break
                    else:
                        j += 1
            blank(i, j)
            i = j
        elif m := _CHAR_LITERAL.match(src, i):
            blank(i, m.end())
            i = m.end()
        else:
            i += 1
    return "".join(out)


# A bare-`pub` type declaration. `pub(crate)` and `pub(super)` do not match —
# `pub` must be followed by whitespace — which is the same public/interior line
# the façade's own guards draw.
#
# BLIND SPOT (a), CLOSED HERE: a payload that is a STRUCT rather than an enum is
# indexed, so it enters the raw set and is narrowed out by shape rather than
# never being seen.
#
# BLIND SPOT (e), NARROWED AND STILL OPEN. The leading `[ \t]*` indexes a `pub
# enum` written literally inside a `macro_rules!` body — the declaration is
# real, and one payload rung was found exactly that way. What no
# declaration-level scan reaches is a type whose NAME is minted by a macro:
# every `slotmap` key in a payload position is spelled by no `pub enum` or `pub
# struct` line anywhere, so it is invisible here in both directions — never a
# carrier, never a payload.
_DECL = re.compile(r"(?m)^[ \t]*pub\s+(enum|struct)\s+([A-Za-z_][A-Za-z0-9_]*)")
# Every identifier, for reading a declaration's body.
_IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
# A `pub use` item's leading `pub `, for the bare-`pub` field filter.
_BARE_PUB_FIELD = re.compile(r"^\s*(?:#\[[^]]*\]\s*)*pub\s")


class Decl:
    """One `pub enum` / `pub struct` declaration in the dependency set."""

    __slots__ = ("body", "crate", "kind", "line", "name", "path")

    def __init__(self, name, kind, crate, path, line, body):
        self.name, self.kind, self.crate = name, kind, crate
        self.path, self.line, self.body = path, line, body

    @property
    def site(self) -> str:
        return f"{self.path}:{self.line}"


def _body_after(code: str, at: int) -> tuple[str, str]:
    """The declaration body starting at `at` (just past the type's name).

    Returns `(shape, text)` — `("brace", …)` for a braced enum or struct,
    `("tuple", …)` for a tuple struct's parenthesised element list, and
    `("unit", "")` for `pub struct Foo;`. Generic parameters and a `where`
    clause are stepped over by tracking angle-bracket depth, with `->` skipped
    whole so an `Fn(A) -> B` bound cannot close a bracket it never opened.
    """
    depth, i, n = 0, at, len(code)
    while i < n:
        two = code[i : i + 2]
        if two == "->":
            i += 2
            continue
        ch = code[i]
        if ch == "<":
            depth += 1
        elif ch == ">":
            depth = max(0, depth - 1)
        elif depth == 0 and ch in "{(;":
            if ch == ";":
                return "unit", ""
            j, d = i + 1, 1
            while j < n and d:
                if code[j] in "{(":
                    d += 1
                elif code[j] in "})":
                    d -= 1
                j += 1
            return ("brace" if ch == "{" else "tuple"), code[i + 1 : j - 1]
        i += 1
    return "unit", ""


def _split_top(text: str) -> list[str]:
    """`text` split on commas at nesting depth zero."""
    parts, depth, cur = [], 0, []
    for ch in text:
        if ch in "{([<":
            depth += 1
        elif ch in "})]>":
            depth = max(0, depth - 1)
        if ch == "," and depth == 0:
            parts.append("".join(cur))
            cur = []
        else:
            cur.append(ch)
    parts.append("".join(cur))
    return [p for p in parts if p.strip()]


def payload_identifiers(decl: Decl) -> set[str]:
    """Every identifier in `decl`'s body that could name a payload type.

    Two filters, each closing a blind spot an earlier run of this sweep had
    open, and each stated where it is applied:

    * **An enum arm's leading identifier is dropped.** A variant name is not a
      payload, and a variant whose name collides with a type declared in the
      same crate read as one — three did.
    * **A struct's fields are read only where they are bare `pub`.** A private
      field is not a payload a caller can reach; four false positives came from
      reading a body's private arenas.

    An enum's struct-variant fields are NOT `pub`-marked (a variant's fields
    are as public as the variant), so the second filter applies to structs
    only. Their field NAMES stay in the scanned text and cost nothing: a name
    is reported only if it resolves to a declared type, and a field name that
    collides with one is blind spot (h), not this filter's business.

    BLIND SPOT (d), OPEN: a generic parameter instantiated to an uncurated
    type. What is read here is the declaration as written, so a field typed
    `Slot<T>` names `Slot` and `T`, and the type `T` is actually instantiated
    to at the call sites is not in this text at all.
    """
    shape, text = decl.body
    if shape == "unit":
        return set()
    names: set[str] = set()
    for chunk in _split_top(text):
        if decl.kind == "enum":
            m = _IDENT.search(chunk)
            rest = chunk[m.end() :] if m else chunk
        else:
            if not _BARE_PUB_FIELD.match(chunk):
                continue
            rest = chunk
        names.update(_IDENT.findall(rest))
    return names


def _without_attributes(chunk: str) -> str:
    """`chunk` with every `#[…]` attribute cut, brackets nested correctly.

    `payload_identifiers` above does not need this — it drops ONE leading
    identifier and keeps the rest, so an attribute costs it a false name that
    resolves to no declaration. A reader that wants the member's OWN name
    needs the attribute gone: `#[default] Pinned` names `Pinned`, not
    `default`.
    """
    out, i, n = [], 0, len(chunk)
    while i < n:
        if chunk.startswith("#[", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if chunk[j] == "[":
                    depth += 1
                elif chunk[j] == "]":
                    depth -= 1
                j += 1
            i = j
        else:
            out.append(chunk[i])
            i += 1
    return "".join(out)


_VARIANT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
_PUB_FIELD = re.compile(r"pub\s+([A-Za-z_][A-Za-z0-9_]*)\s*:")


def declared_members(decl: Decl) -> list[str]:
    """The names `decl`'s own members carry, in declaration order.

    An enum's VARIANT names and a struct's bare-`pub` FIELD names — the two
    things a consumer of the declaration can name one level in. The same body
    split `payload_identifiers` uses, reading the other half of each chunk:
    that one wants the payload TYPES and drops the member's own name, this one
    wants the name and drops the types.

    The public/interior line is `payload_identifiers`': a struct's private
    field is not a member a caller reaches, so it is not listed. An enum's
    variants are as public as the enum, so all of them are.

    BLIND SPOTS, the same set the rest of this file has, plus one of its own.
    (h) is the sharpest here: two types with one name inside one crate are
    indistinguishable, so the first declaration in path order supplies the
    member list for both. (e) applies unchanged — a member whose NAME is
    minted by a macro is spelled by no line this reads.

    ITS OWN: a TUPLE struct's fields have no names, so `pub struct Ray(pub
    Vec3)` answers the empty list rather than a positional one. That is the
    honest answer to what a member is CALLED — there is nothing to call it —
    and it puts a tuple struct's fields outside every question asked of this
    list.
    """
    shape, text = decl.body
    if shape != "brace":
        return []
    out: list[str] = []
    for chunk in _split_top(text):
        body = _without_attributes(chunk).strip()
        if not body:
            continue
        if decl.kind == "enum":
            m = _VARIANT.match(body)
            if m:
                out.append(m.group(0))
        else:
            m = _PUB_FIELD.match(body)
            if m:
                out.append(m.group(1))
    return out


# --- the façade's path-dependency set --------------------------------------


def dependency_set(root: Path) -> dict[str, Path]:
    """`{crate ident: crate directory}` for the façade and its path closure.

    The transitive `[dependencies]` `path = …` closure of `crates/pncad`, keyed
    by the identifier a `use` statement spells (`editor-core` -> `editor_core`).
    Dev-dependencies are deliberately outside it: a payload a caller can only
    reach through a test-only edge is not on the façade's surface.

    BLIND SPOT (i), OPEN: registry dependencies have no `path` and drop out
    here. That is the right boundary — `getrandom`'s types are not this
    façade's rungs — but it is a boundary, so a payload whose type comes from
    a third-party crate is invisible to a scan that reads only this tree.
    """
    found: dict[str, Path] = {}
    stack = [(root / FACADE_CRATE_DIR).resolve()]
    while stack:
        crate = stack.pop()
        manifest = crate / "Cargo.toml"
        if not manifest.is_file():
            continue
        data = tomllib.loads(manifest.read_text())
        name = data.get("package", {}).get("name")
        if not name:
            continue
        ident = name.replace("-", "_")
        if ident in found:
            continue
        found[ident] = crate
        for spec in data.get("dependencies", {}).values():
            if isinstance(spec, dict) and "path" in spec:
                stack.append((crate / spec["path"]).resolve())
    return found


def declarations(root: Path, crates: dict[str, Path]) -> dict[str, list[Decl]]:
    """Every bare-`pub` enum and struct declared under those crates' `src/`.

    `src/` only: a type declared in a crate's `tests/` or `benches/` is not on
    anything a consumer can name. Sources are read as written, `#[cfg]`-gated
    items included — a payload behind `#[cfg(feature = "interval")]` is a real
    payload in the lane that builds it, and dropping gated code would make the
    sweep's answer depend on a feature selection it does not take.
    """
    index: dict[str, list[Decl]] = {}
    for ident in sorted(crates):
        src_dir = crates[ident] / "src"
        for path in sorted(src_dir.rglob("*.rs")):
            code = code_only(path.read_text())
            rel = os.path.relpath(path, root)
            for m in _DECL.finditer(code):
                kind, name = m.group(1), m.group(2)
                line = code.count("\n", 0, m.start()) + 1
                body = _body_after(code, m.end())
                index.setdefault(name, []).append(
                    Decl(name, kind, ident, rel, line, body)
                )
    return index


# --- the curated lists -----------------------------------------------------

_PUB_USE = "pub use "


def curated(root: Path, lists: tuple[str, ...]) -> dict[str, dict[str, set[str]]]:
    """`{name: {list stem: {root crate idents}}}` over the curated lists.

    The reader is `test_binding_census.py`'s: the leaf of each path item is the
    name that item introduces, and a braced group introduces each of its items.
    What it adds is the statement's ROOT — `pub use editor_core::{…}` says the
    names it introduces are `editor_core`'s — which is what lets a carrier be
    resolved crate-aware rather than by spelling. A leading `::` is stripped
    first: one list spells its own layer absolutely because that file's module
    shadows the crate name.

    BLIND SPOT (c), OPEN: a refusal the façade DECLARES rather than re-exports
    is on no `pub use` statement, so neither it nor its payloads ever enter the
    compared set.
    """
    out: dict[str, dict[str, set[str]]] = {}
    for stem in lists:
        code = code_only((root / FACADE_SRC / f"{stem}.rs").read_text())
        rest = code
        while (at := rest.find(_PUB_USE)) >= 0:
            rest = rest[at + len(_PUB_USE) :]
            end = rest.find(";")
            if end < 0:
                break
            stmt, rest = rest[:end].strip(), rest[end + 1 :]
            stmt = stmt.removeprefix("::")
            crate_root = stmt.split("::", 1)[0].strip() if "::" in stmt else stmt
            open_, close = stmt.find("{"), stmt.rfind("}")
            items = stmt[open_ + 1 : close] if 0 <= open_ < close else stmt.rsplit("::", 1)[-1]
            for item in items.split(","):
                item = item.strip()
                if not item:
                    continue
                name = item.rsplit("::", 1)[-1]
                out.setdefault(name, {}).setdefault(stem, set()).add(crate_root)
    return out


# --- the sweep -------------------------------------------------------------


# A refusal's name, by this tree's convention. Every `pub enum` in the façade's
# dependency set whose name ends in one of these is a refusal type: it carries
# its own `Display`, it is decided about AS a refusal, and the class was
# disposed of once rather than one row at a time.
REFUSAL_SUFFIXES = ("Error", "Refusal", "Fault")


def is_refusal_type(name: str) -> bool:
    """A payload that is itself a refusal is not a payload rung.

    The narrowing asks for a DISCRIMINANT — the value a caller reads once it
    has matched an arm. A nested refusal is a different question with a
    different answer, and it is settled where refusals are settled.

    THE TEST IS THE NAME, which is a convention and not a property. It holds
    over this tree in both directions today — every `*Error`, `*Refusal` and
    `*Fault` enum in the dependency set is a refusal, and no refusal is spelled
    otherwise — and it is stated here rather than inferred from a `Display`
    impl because a `Display` impl does not separate the two: five discriminants
    a curation pass CARRIED have one. Blind spot: a refusal named some other
    way stays in the narrowed set, and a discriminant named `…Error` drops out
    of it.
    """
    return name.endswith(REFUSAL_SUFFIXES)


class Row:
    """One (carrier, payload) pair the sweep found."""

    __slots__ = ("carrier", "carrier_lists", "hit", "hit_lists", "state")

    def __init__(self, carrier, carrier_lists, hit, hit_lists, state):
        self.carrier, self.carrier_lists = carrier, carrier_lists
        self.hit, self.hit_lists, self.state = hit, hit_lists, state

    def key(self) -> tuple:
        return (self.hit.name, self.carrier.name, self.carrier.site)

    def as_dict(self) -> dict:
        return {
            "payload": self.hit.name,
            "payload_kind": self.hit.kind,
            "payload_at": self.hit.site,
            "payload_lists": sorted(self.hit_lists),
            "carrier": self.carrier.name,
            "carrier_kind": self.carrier.kind,
            "carrier_at": self.carrier.site,
            "carrier_lists": sorted(self.carrier_lists),
            "state": self.state,
        }


def sweep(root: Path, lists: tuple[str, ...]) -> dict:
    """The counts and the rows behind them."""
    crates = dependency_set(root)
    index = declarations(root, crates)
    names = curated(root, lists)

    # A curated name resolves to the declaration in the crate the statement
    # that carried it named. That is the crate-awareness: `sweep::BlendError`
    # and a same-named type in another crate are two carriers, not one. A name
    # re-exported by a crate that does not declare it (the prelude lifts a few
    # through the layer that hands them out) falls back to the declaration set
    # for that spelling.
    #
    # BLIND SPOT (h), OPEN: this is crate-aware and not MODULE-aware, so two
    # types with one name inside one crate are indistinguishable here and the
    # first declaration in path order stands for both.
    #
    # BLIND SPOT (b), OPEN: a payload named only through a type ALIAS or an
    # associated type. Names are compared as spellings, so a curated list that
    # carries a declaration under an alias reads as carrying nothing, and the
    # declaration reads as uncurated — which is what produces this sweep's one
    # standing false positive, `CarrierRelation`, carried by the prelude as
    # `PlaneRelation`. The alias arm is mechanically closable and the
    # associated-type arm is not.
    carriers: list[tuple[Decl, set[str]]] = []
    for name, where in sorted(names.items()):
        roots = {r for rs in where.values() for r in rs}
        decls = [d for d in index.get(name, []) if d.crate in roots]
        if not decls:
            decls = index.get(name, [])
        for decl in decls:
            carriers.append((decl, set(where)))

    raw: list[Row] = []
    cross: list[Row] = []
    for decl, on_lists in carriers:
        for ident in sorted(payload_identifiers(decl)):
            if ident == decl.name:
                continue
            same = [d for d in index.get(ident, []) if d.crate == decl.crate]
            hits = same or index.get(ident, [])
            if not hits:
                continue
            hit_lists = set(names.get(ident, {}))
            if hit_lists & on_lists:
                continue  # curated beside its carrier: not a rung at all
            row = Row(decl, on_lists, hits[0], hit_lists, "uncurated" if not hit_lists else "cross-list")
            (raw if row.state == "uncurated" else cross).append(row)

    raw.sort(key=Row.key)
    cross.sort(key=Row.key)
    return {
        "lists": list(lists),
        "counts": {
            "curated": len(names),
            "declared": sum(len(v) for v in index.values()),
            "raw": len(raw),
            "narrowed": len(narrow(raw)),
        },
        "cross_list": len(cross),
        "raw": raw,
        "narrowed": narrow(raw),
        "cross": cross,
        "narrowed_cross": narrow(cross),
    }


def narrow(rows: list[Row]) -> list[Row]:
    """The CUR3/CUR4 shape, which is what a curation pass acts on.

    An ENUM, because the question is whether a caller can BRANCH on the value;
    not itself an error type, because a nested refusal is a different question
    settled as a class; and declared in the same crate as its carrier, because
    a payload from another crate is that crate\'s surface question and reaches
    the façade by a different door.
    """
    return [
        r
        for r in rows
        if r.hit.kind == "enum"
        and not is_refusal_type(r.hit.name)
        and r.hit.crate == r.carrier.crate
    ]


# --- reporting -------------------------------------------------------------


def render(result: dict) -> str:
    c = result["counts"]
    out = [
        f"curated names   {c['curated']}   ({', '.join(result['lists'])})",
        f"declared types  {c['declared']}",
        f"raw hits        {c['raw']}   (payload on no curated list)",
        f"narrowed        {c['narrowed']}   (enum, not a refusal, carrier's crate)",
        f"cross-list      {result['cross_list']} raw / "
        f"{len(result['narrowed_cross'])} narrowed   "
        "(payload curated, on no list carrying its carrier)",
        "",
        "NARROWED — a payload rung on no curated list",
    ]
    out += _table(result["narrowed"])
    out += ["", "NARROWED CROSS-LIST — curated, but not where its carrier is"]
    out += _table(result["narrowed_cross"])
    return "\n".join(out)


def _table(rows: list[Row]) -> list[str]:
    if not rows:
        return ["  (none)"]
    lines = []
    for r in rows:
        lists = "+".join(sorted(r.hit_lists)) or "-"
        disp = DISPOSITIONS.get(r.hit.name) or CROSS_LIST_DISPOSITIONS.get(r.hit.name)
        tail = f"  [{disp[0]}: {disp[1]}]" if disp else "  [UNDISPOSED]"
        lines.append(
            f"  {r.hit.name} ({r.hit.crate}, {lists}) at {r.hit.site}"
            f"\n      under {r.carrier.name} "
            f"({'+'.join(sorted(r.carrier_lists))}) at {r.carrier.site}{tail}"
        )
    return lines


def check(result: dict) -> int:
    """The pin: the narrowed NAMES are exactly the disposition tables' keys.

    Names and not counts, because the counts are the thing that drifted and the
    names are what a curation pass acts on. Both sets are pinned and both
    directions fail: a new rung is a rung nobody has argued, and a name that
    has stopped being a rung leaves a row here claiming a decision about
    nothing.
    """
    rc = 0
    for rows, table, what, verb in (
        (result["narrowed"], DISPOSITIONS, "DISPOSITIONS", "on no curated list"),
        (result["narrowed_cross"], CROSS_LIST_DISPOSITIONS, "CROSS_LIST_DISPOSITIONS",
         "curated on no list that carries its carrier"),
    ):
        found = {r.hit.name for r in rows}
        pinned = set(table)
        for name in sorted(found - pinned):
            where = "; ".join(
                f"{r.hit.site} under {r.carrier.name} ({r.carrier.site})"
                for r in rows
                if r.hit.name == name
            )
            print(
                f"NEW payload rung, {verb}: {name} at {where}. Decide it — carry "
                f"it where its carrier is carried, or argue the non-carriage "
                f"beside that carrier — and add it to {what} with the home of "
                f"the argument.",
                file=sys.stderr,
            )
            rc = 1
        for name in sorted(pinned - found):
            print(
                f"STALE row in {what}: {name} is no longer a narrowed rung. "
                "Delete the row, or find out what stopped the scan seeing it.",
                file=sys.stderr,
            )
            rc = 1
    return rc



# --- the fixture battery ---------------------------------------------------

# A scratch tree with the whole shape in miniature: a façade whose five lists
# curate names from two crates, one of which declares a type the other declares
# too. Every filter and every category the sweep draws has a witness here, and
# the counts below are asserted rather than printed, so a change to any of them
# fails this row rather than moving a number nobody re-derives.
_FIXTURE_ALPHA = """
/// A doc comment that names `pub enum Ghosted { }`, which is prose.
pub enum Carrier {
    Plain(Rung),
    /// Prose naming [`Secret`], which is not a payload of anything.
    Sibling(Shared),
    Named(CrossPayload),
    Ghost,
    Faulty(ThingError),
    Fielded(Detail),
    Foreign(BetaRung),
    #[default]
    Blank,
}
pub enum Rung { A, B }
pub enum GatedCarrier { Only(Rung) }
pub enum CrossPayload { X }
pub enum Shared { S }
pub enum Ghost { G }
pub enum ThingError { E }
pub enum Secret { S }
pub struct Detail { pub x: u8 }
pub struct DocCarrier {
    pub open: Rung,
    hidden: Secret,
}
pub(crate) enum Interior { I }
pub struct Wrapped(pub Rung);
pub struct Marker;
const SPELL: &str = "pub enum Bogus { }";
"""

_FIXTURE_BETA = """
pub enum BetaCarrier { Only(BetaRung) }
pub enum BetaRung { P }
pub enum Rung { Z }
"""

_FIXTURE = {
    "crates/pncad/Cargo.toml": '[package]\nname = "pncad"\n\n[dependencies]\nalpha = { path = "../alpha" }\n',
    "crates/pncad/src/document.rs": "pub use alpha::DocCarrier;\n",
    "crates/pncad/src/select.rs": "pub use alpha::CrossPayload;\n",
    "crates/pncad/src/prelude.rs": "// A bare `pub use alpha::Rung;` would carry the rung, and does not.\n"
                                  "pub use alpha::{Carrier, Shared};\n",
    "crates/pncad/src/profile.rs": "pub use abeta::BetaCarrier;\n",
    # Blind spot (j)'s witness: this reader strips comments and reads `pub use`
    # statements, so the `cfg` above one is invisible and the name is curated
    # unconditionally. A rung reachable only in one feature unification
    # therefore looks exactly like any other, which is why the disposition
    # tables carry that reading by hand.
    "crates/pncad/src/analysis.rs": '#[cfg(feature = "interval")]\n'
                                    "pub use alpha::GatedCarrier;\n",
    "crates/alpha/Cargo.toml": '[package]\nname = "alpha"\n\n[dependencies]\nabeta = { path = "../abeta" }\n',
    "crates/alpha/src/lib.rs": _FIXTURE_ALPHA,
    "crates/abeta/Cargo.toml": '[package]\nname = "abeta"\n',
    "crates/abeta/src/lib.rs": _FIXTURE_BETA,
}


def selftest() -> int:
    import tempfile

    failures: list[str] = []

    def want(claim: str, got, expected) -> None:
        if got != expected:
            failures.append(f"{claim}: expected {expected!r}, got {got!r}")

    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        for rel, text in _FIXTURE.items():
            path = root / rel
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(text)

        crates = dependency_set(root)
        want("the path closure is transitive", sorted(crates), ["abeta", "alpha", "pncad"])

        result = sweep(root, ALL_LISTS)
        counts = result["counts"]
        want("curated names", counts["curated"], 6)
        # alpha declares twelve bare-`pub` types — `pub(crate)` is not one, and
        # neither is a name that appears only in a doc comment or a string
        # literal; beta declares three. `Wrapped` and `Marker` are declared and
        # curated by nothing, so they are carriers of nothing and move only
        # this count; they are here as the two member-shape witnesses below.
        want("declared types", counts["declared"], 15)
        want("raw hits", counts["raw"], 7)
        want("narrowed", counts["narrowed"], 4)
        want("cross-list raw", result["cross_list"], 1)
        want("cross-list narrowed", len(result["narrowed_cross"]), 1)

        narrowed = sorted((r.hit.name, r.carrier.name) for r in result["narrowed"])
        want(
            "the narrowed rows",
            narrowed,
            [
                ("BetaRung", "BetaCarrier"),
                ("Rung", "Carrier"),
                ("Rung", "DocCarrier"),
                ("Rung", "GatedCarrier"),
            ],
        )
        want(
            "blind spot (j): a `cfg`-gated `pub use` is read as curated",
            ("Rung", "GatedCarrier") in narrowed,
            True,
        )
        want(
            "the cross-list row",
            [(r.hit.name, r.carrier.name) for r in result["narrowed_cross"]],
            [("CrossPayload", "Carrier")],
        )

        raw = sorted({r.hit.name for r in result["raw"]})
        want("blind spot (g) is closed: a variant name is not a payload", "Ghost" in raw, False)
        want("blind spot (f) is closed: a private field is not a payload", "Secret" in raw, False)
        want("a bare `pub` field IS a payload", ("Rung", "DocCarrier") in narrowed, True)
        want("prose is not a declaration", "Ghosted" in raw, False)
        want("a string literal is not a declaration", "Bogus" in raw, False)
        want("a refusal payload is raw and not narrowed", "ThingError" in raw, True)
        want("a struct payload is raw and not narrowed", "Detail" in raw, True)
        want(
            "a payload curated beside its carrier is no rung at all",
            "Shared" in raw or "Shared" in {r.hit.name for r in result["cross"]},
            False,
        )

        # THE MEMBER READER, on the same declarations the rows above came from.
        # It is the other half of each chunk: the member's own NAME, where
        # `payload_identifiers` reads the types beside it.
        index = declarations(root, crates)
        members = {name: declared_members(decls[0]) for name, decls in index.items()}
        want(
            "an enum's members are its variant names, in declaration order",
            members["Carrier"],
            ["Plain", "Sibling", "Named", "Ghost", "Faulty", "Fielded", "Foreign", "Blank"],
        )
        want(
            "an attribute is not the member's name",
            members["Carrier"][-1],
            "Blank",
        )
        want("a struct's members are its bare-`pub` fields", members["DocCarrier"], ["open"])
        want("a private field is not a member", "hidden" in members["DocCarrier"], False)
        want("a tuple struct's fields have no names", members["Wrapped"], [])
        want("a unit struct has no members", members["Marker"], [])
        # Crate-awareness reaches this reader too: `Rung` is declared in both
        # crates and the two declarations carry different members.
        want(
            "each declaration answers with its own members",
            sorted(d.crate + ":" + ",".join(declared_members(d)) for d in index["Rung"]),
            ["abeta:Z", "alpha:A,B"],
        )

        # Crate-awareness: `Rung` is declared in both crates, and `Carrier`'s
        # payload is the one in `Carrier`'s own crate.
        rung = [r for r in result["narrowed"] if (r.hit.name, r.carrier.name) == ("Rung", "Carrier")]
        # `abeta` sorts BEFORE `alpha`, so a scan that dropped the
        # same-crate preference would resolve this one to the wrong crate
        # rather than to the same answer by luck.
        want("the payload resolves in its carrier's crate", [r.hit.crate for r in rung], ["alpha"])

        want("two runs agree byte for byte", render(sweep(root, ALL_LISTS)), render(result))

        # The three-list run is the earlier runs' definition, and the fixture
        # shows what it costs: `BetaCarrier` is curated only on the fourth list,
        # so the rung under it is not even looked for.
        three = sweep(root, ("document", "select", "prelude"))
        want("the fourth list adds a carrier", three["counts"]["curated"], 4)
        want(
            "and the rung under it",
            sorted((r.hit.name, r.carrier.name) for r in three["narrowed"]),
            [("Rung", "Carrier"), ("Rung", "DocCarrier")],
        )

    for line in failures:
        print(f"payload-rung-sweep selftest: {line}", file=sys.stderr)
    if failures:
        return 1
    print("payload-rung-sweep selftest: ok")
    return 0



# --- entry point -----------------------------------------------------------


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--json", action="store_true", help="machine-readable form")
    ap.add_argument("--check", action="store_true", help="pin the narrowed names")
    ap.add_argument("--selftest", action="store_true", help="the fixture battery")
    ap.add_argument(
        "--lists",
        default=",".join(ALL_LISTS),
        help="curated lists to read, comma-separated stems of crates/pncad/src",
    )
    ap.add_argument("--root", default=None, help="repository root (default: this script's)")
    args = ap.parse_args(argv)

    if args.selftest:
        return selftest()

    root = Path(args.root).resolve() if args.root else Path(__file__).resolve().parents[1]
    lists = tuple(s.strip() for s in args.lists.split(",") if s.strip())
    unknown = [s for s in lists if not (root / FACADE_SRC / f"{s}.rs").is_file()]
    if unknown:
        print(f"no such curated list: {', '.join(unknown)}", file=sys.stderr)
        return 2
    result = sweep(root, lists)

    if args.check:
        if set(lists) != set(ALL_LISTS):
            print(
                "--check pins the whole-façade sweep; it is the surface the "
                "façade actually curates. Drop --lists.",
                file=sys.stderr,
            )
            return 2
        return check(result)
    if args.json:
        print(
            json.dumps(
                {
                    "lists": result["lists"],
                    "counts": result["counts"],
                    "cross_list": result["cross_list"],
                    "narrowed": [r.as_dict() for r in result["narrowed"]],
                    "narrowed_cross": [r.as_dict() for r in result["narrowed_cross"]],
                    "raw": [r.as_dict() for r in result["raw"]],
                },
                indent=2,
                sort_keys=True,
            )
        )
        return 0
    print(render(result))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
