---
id: joined-notices-nest-their-own-separator
kind: issue
title: A frame's joined notices nest NOTICE_SEPARATOR and the em-dash inside themselves, so the line is ambiguous at two notices
status: open
opened: 2026-09-05
refs: [the-news-vocabulary-has-no-expiry, status-line-writers-bypass-the-ranking, 1886]
---


Found by #1886's style review. Pre-existing in shape, made reachable
by that unit: `frame` had one notice producer and now has two, so two
notices in one frame is an ordinary state rather than a hypothetical.

## What the line reads

`frame::frame_status` joins rank-2 notices with `NOTICE_SEPARATOR`
(`"; "`), and `render_causes` joins each notice's own causes with the
same string. With two superseded placements and one dropped hide the
status line is:

> free move: 2 committed placements were discarded — A; B; hide: … — C

A reader cannot tell where the cause list ends and the next notice
begins, because the inner join and the outer join are the same
character. The em-dash nests the same way: `DisplayFault`'s own
`Display` arms contain one (`FusedGeometry`,
`crates/viewer/src/display.rs:190-200`), and so does each notice's
preamble.

## Why it is a design question and not a formatting nit

The obvious fix — a different inner separator — is a choice about how a
composed sentence tells a reader its own structure, and this crate has
no rule for that. It is also the same question from a different side as
`the-news-vocabulary-has-no-expiry`, which is on `[ev]` PR #1883: if a
message carried its subject, several notices about different subjects
would have a structure to render rather than a string to concatenate,
and the ambiguity would not arise. Answering the separator alone would
be a local patch over a vocabulary gap.

One hazard worth recording separately, because it is mechanical and
present today: **`DisplayFault::NonRigidFrame`'s `Display` contains a
`"; "` of its own** (`display.rs:180`). Any reading of the joined line
that counts separators — including the assertion at
`crates/viewer/src/frame.rs:1122` — is wrong the moment that arm
reaches the line. #1886's fix pass was asked to stop that assertion
lying; the ambiguity it is a symptom of is this file.

## Home

VIEW's: `crates/viewer/src/frame.rs`. Sequence after #1883's answer on
the news vocabulary.
