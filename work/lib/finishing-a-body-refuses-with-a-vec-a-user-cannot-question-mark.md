---
id: finishing-a-body-refuses-with-a-vec-a-user-cannot-question-mark
kind: issue
title: AtRestBody::validate refuses with Vec<ValidationError>, which a user's ? cannot carry into Box<dyn Error>, so every boolean caller writes a map_err
status: open
opened: 2026-10-03
priority: P3
cost: E
refs: [boolean-door-adopts-the-finished-body-type]
---


Found by `boolean-door-adopts-the-finished-body-type`: the boolean
doors take `&AtRestBody`, so a user holding a sweep's body finishes it
first with `AtRestBody::validate(body, tol)`, whose error is
`Vec<ValidationError>`. `Vec<_>` implements no `Error`, so the guide's
`slab` (`docs/GUIDE.md` §2.2, and every hidden copy, and
`docs/guide/fail-loud.md` §3) has to write
`.map_err(|errors| format!("not a finished body: {errors:?}"))?` where
every other door in the journey is a bare `?`. The tier gates
(`validate`, `validate_closed`, `validate_pseudomanifold`) return the
same vector and the guide `.expect`s them for the same reason.

What closes it: a findings type that implements `Error` (its `Display`
the first finding and the count, as `BooleanError::ResultInvalid`
renders them), returned by the finishing door, or a `From` the façade
provides; then the guide drops its `map_err`s.
