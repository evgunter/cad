//! **Each verdict's role is the scope its caller opened**
//! (`geom_core::k_stats::Role`): these rows go red if a check is recorded
//! as a choice, or a choice as a check, through any door that hands a
//! recording on. The first-turn reading (`editor_core::resolve::vdiff`)
//! reads only choices, and the certifying vector hashes each row's role,
//! so a mistagged row is a flip the resolver cannot see or a key that
//! moves with the schedule.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom_core::k_stats::{
    Bracket, Mark, Recorded, Role, Verdict, answered, checking, decide, map_detached,
    mark_candidates, splice,
};
use geom_core::{Band, Margin, Sign, Tol};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn one(name: &'static str) {
    let _ = decide(name, Margin::of(1.0), band());
}

fn roles(r: &Recorded) -> Vec<(&'static str, Role)> {
    r.verdicts.iter().map(|v| (v.predicate, v.role)).collect()
}

/// A check scope is not reopened by a nested one, and a panic inside it
/// restores the outer role on unwind.
#[test]
fn a_check_scope_is_not_reopened_and_unwinds() {
    let b = Bracket::open();
    checking(|| {
        checking(one_in);
        one("t_after_inner");
    });
    let _ = std::panic::catch_unwind(|| checking(|| panic!("inside a check")));
    one("t_out");
    let r = b.finish();
    assert_eq!(
        roles(&r),
        [
            ("t_in", Role::Check),
            ("t_after_inner", Role::Check),
            ("t_out", Role::Choice),
        ]
    );
}

fn one_in() {
    one("t_in");
}

/// `map_detached` hands its caller's role to every job, and a job
/// spliced back keeps it.
#[test]
fn map_detached_hands_the_check_role() {
    let b = Bracket::open();
    for (_, rec) in checking(|| map_detached(&[0, 1, 2, 3], |_| one("t_job"))) {
        splice(rec);
    }
    for (_, rec) in map_detached(&[0, 1], |_| one("t_choice")) {
        splice(rec);
    }
    let r = b.finish();
    assert_eq!(
        roles(&r),
        [
            ("t_job", Role::Check),
            ("t_job", Role::Check),
            ("t_job", Role::Check),
            ("t_job", Role::Check),
            ("t_choice", Role::Choice),
            ("t_choice", Role::Choice),
        ]
    );
}

/// **A job runs in its spawner's role, whatever thread runs it**: a
/// worker blocked inside a check-scoped inner map steals work from the
/// pool, and an outer job it runs there was spawned as a choice. Each
/// outer job records a choice before and after a check-scoped inner map;
/// none of those may read as a check. Under a role taken as the max of
/// the job's and the running thread's, a stolen outer job's choices read
/// as checks, so this row goes red with the schedule.
#[test]
fn a_stolen_job_runs_in_its_spawners_role() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();
    let outer: Vec<usize> = (0..256).collect();
    let inner: Vec<usize> = (0..16).collect();
    let mismarked = pool.install(|| {
        map_detached(&outer, |_| {
            one("t_outer_before");
            for (_, rec) in checking(|| {
                map_detached(&inner, |_| {
                    // Enough work that the inner collect waits, and its
                    // thread steals.
                    let mut x = 0.0_f64;
                    for k in 0..2_000 {
                        x += f64::from(k).sqrt();
                    }
                    let _ = decide("t_inner", Margin::of(1.0 + x * 0.0), band());
                })
            }) {
                splice(rec);
            }
            one("t_outer_after");
        })
        .into_iter()
        .flat_map(|(_, rec)| rec.recorded().verdicts.clone())
        .filter(|v| (v.predicate == "t_inner") != (v.role == Role::Check))
        .count()
    });
    assert_eq!(
        mismarked, 0,
        "a job ran in the role of the thread that stole it"
    );
}

/// `answered` and `mark_candidates` take this thread's role.
#[test]
fn an_answer_and_a_mark_take_the_scopes_role() {
    let b = Bracket::open();
    answered("t_answer", Sign::Positive);
    mark_candidates("t_site", [3]);
    checking(|| {
        answered("t_answer", Sign::Negative);
        mark_candidates("t_site", [4]);
    });
    let r = b.finish();
    assert_eq!(
        roles(&r),
        [("t_answer", Role::Choice), ("t_answer", Role::Check)]
    );
    assert_eq!(
        r.marks.iter().map(|m| (m.item, m.role)).collect::<Vec<_>>(),
        [
            (Some(3), Role::Choice),
            (None, Role::Choice),
            (Some(4), Role::Check),
            (None, Role::Check),
        ]
    );
}

/// A record appended after another places its marks after the other's
/// verdicts.
#[test]
fn extend_places_marks_after_the_prefix() {
    let v = Verdict {
        predicate: "t",
        sign: Sign::Positive,
        role: Role::Choice,
    };
    let mut a = Recorded {
        verdicts: vec![v; 3],
        ..Recorded::default()
    };
    a.extend(&Recorded {
        verdicts: vec![v],
        marks: vec![Mark {
            before: 1,
            site: "s",
            item: Some(0),
            role: Role::Choice,
        }],
        ..Recorded::default()
    });
    assert_eq!(a.marks[0].before, 4);
    assert_eq!(a.verdicts.len(), 4);
}
