---
id: step-writer-hardcodes-user-header-fields
kind: issue
title: The STEP writer still hardcodes two Part 21 header fields the standard assigns to the user
status: open
opened: 2026-08-20
github: 742
refs: [732, C14]
---

## From GitHub issue 742

Opened 2026-08-20; 0 comments.

Found by #732's style review, out of that unit's scope. **"Which STEP header fields are the caller's" is a design call, not a cleanup — its plan goes to Ev before implementation.** Filed as a takeable Track C row.

## The two fields

`step-export/src/writer.rs:913-930` builds the Part 21 header from a fixed template. Four things in it are hardcoded, and they are **not all the same kind of thing**:

| hardcoded | Part 21 assigns it to | verdict |
|---|---|---|
| `FILE_SCHEMA(('AUTOMOTIVE_DESIGN { … }'))` | the standard | correct — it names the AP the data conforms to |
| `FILE_DESCRIPTION`'s `'2;1'` | the standard | correct — implementation level |
| `FILE_NAME`'s `'step-export'` (`preprocessor_version`) | **the software** that wrote the file | correct — a caller who could set it could misrepresent the producer |
| **`FILE_NAME`'s 7th argument, `''` — `authorisation`** | **the user** (the person who authorised the file) | **hardcoded empty, unreachable** |
| **`FILE_DESCRIPTION`'s description list, `('')`** | **the user** (free text describing the file) | **hardcoded empty, unreachable** |

So the writer already distinguishes software fields from user fields for `originating_system` (which **is** a `StepOptions` field) and does not for these two.

## Why this matters beyond tidiness

It is **H16's own class, in H16's own sibling exporter**. #732's §7 argues that STL's 80-byte header belongs to the caller *because Part 21 assigns `preprocessor_version` to the software and `originating_system` to the user* — that argument is exactly what these two fields fail. After #732, STL's free text is settable and STEP keeps two caller-facing free-text fields the caller cannot reach. The asymmetry the finding named is **reduced, not eliminated**, and this is the remainder.

## The design part

Not "add two `String` fields to `StepOptions`". `authorisation` is a claim about a *person*, in a file format used for exchange; a default that is empty is honest and a default that is anything else is not. The questions are which of the two are worth exposing, whether either wants validation beyond Part 21's basic alphabet (which `quoted` already enforces), and whether the description list should be one string or the list the standard allows. Plan first.

## Home

LIB: its code-quality row `C14` (Track U) is parked under the same UV-R5 hold as `C13` — LIB drafts the plan, Ev signs off — and `work/lib/log.md` carries #743/#742/#741 together as the export-option surface in the program's register fold.
