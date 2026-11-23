//! A merge-base differential over the crate's fixture corpus: every
//! scalar a validated profile stores, its directions included, written
//! as bits to the file `REVIEW_DOORS2_DUMP` names. It asserts nothing
//! about the values; two trees' dumps are compared by hand.
//!
//! `REVIEW_DOORS2_DUMP=/tmp/head.txt cargo nextest run -p profile --test all \
//!  --run-ignored only -E 'test(review_doors_read_2_differential)'`
//!
//! On a tree whose `ValidatedSegment` holds `directions` beside `kind`,
//! `dirs` reads that field instead.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::fmt::Write as _;

use crate::common;
use geom_core::{Bounds, Interval, Real, Vec2};
use profile::{ConstructedProfile, Profile, SegmentKind, SketchPlane, ValidatedProfile};

fn dirs<T: Real>(s: &profile::ValidatedSegment<T>) -> Vec<Vec2<T>> {
    match s.kind {
        SegmentKind::Line { director } => vec![director],
        SegmentKind::Arc {
            radials: [a, b], ..
        } => vec![a, b],
    }
}

trait Bits: Real {
    fn bits(self) -> String;
}
impl Bits for f64 {
    fn bits(self) -> String {
        format!("{:016x}", self.to_bits())
    }
}
impl Bits for Interval {
    fn bits(self) -> String {
        format!("{:016x}..{:016x}", self.lo().to_bits(), self.hi().to_bits())
    }
}

fn dump<T: Bits>(out: &mut String, what: &str, vp: &Result<ValidatedProfile<T>, String>) {
    let vp = match vp {
        Ok(vp) => vp,
        Err(e) => {
            let _ = writeln!(out, "{what}: refused {e}");
            return;
        }
    };
    for (l, lp) in vp.loops().iter().enumerate() {
        let _ = writeln!(
            out,
            "{what} loop {l} {:?} tangent {:?} cusp {:?}",
            lp.role(),
            lp.tangent_joints(),
            lp.cusp_joints()
        );
        for (j, s) in lp.segments().iter().enumerate() {
            let mut line = format!(
                "  {j} {} {} {} {}",
                s.start.x.bits(),
                s.start.y.bits(),
                s.end.x.bits(),
                s.end.y.bits()
            );
            if let SegmentKind::Arc { arc, turn, .. } = s.kind {
                let _ = write!(
                    line,
                    " arc {} {} {} {} {turn:?}",
                    arc.centre.x.bits(),
                    arc.centre.y.bits(),
                    arc.radius.bits(),
                    arc.sweep.bits()
                );
            } else {
                line.push_str(" line");
            }
            for d in dirs(s) {
                let _ = write!(line, " d {} {}", d.x.bits(), d.y.bits());
            }
            let _ = writeln!(out, "{line}");
        }
    }
}

#[test]
#[ignore = "a differential dump, compared across trees by hand; see the module doc"]
fn review_doors_read_2_differential() {
    let Ok(path) = std::env::var("REVIEW_DOORS2_DUMP") else {
        panic!("REVIEW_DOORS2_DUMP names the file to write");
    };
    let tol = common::tol();
    let text = |e: profile::ProfileError| format!("{e:?}");
    let mut out = String::new();
    let mut tables: Vec<(String, Profile<f64>)> = vec![
        (
            "rect".into(),
            common::profile(vec![common::rect(0.0, 0.0, 3.0, 2.0)]),
        ),
        ("l".into(), common::profile(vec![common::l_profile()])),
        (
            "rounded".into(),
            common::profile(vec![common::rounded_rect(4.0, 3.0, 0.5)]),
        ),
        ("bracket".into(), common::profile(vec![common::bracket()])),
        ("lens".into(), common::profile(vec![common::lens()])),
        (
            "circle_h".into(),
            common::profile(vec![common::circle_h(1.0, 2.0, 0.75)]),
        ),
        (
            "circle_v".into(),
            common::profile(vec![common::circle_v(1.0, 2.0, 0.75)]),
        ),
        ("annulus".into(), common::annulus()),
        ("tangent_hole".into(), common::tangent_hole()),
    ];
    for (k, closed) in common::coverage_corpus().into_iter().enumerate() {
        let constructed = ConstructedProfile::new(SketchPlane::xy(), vec![closed.loop_.clone()])
            .validate(tol)
            .map_err(text);
        dump(
            &mut out,
            &format!("corpus {k} constructed f64"),
            &constructed,
        );
        tables.push((
            format!("corpus {k} table"),
            common::profile(vec![closed.loop_.into_loop()]),
        ));
    }
    for (name, p) in &tables {
        let at_f64 = p.validate(tol).map_err(text);
        dump(&mut out, &format!("{name} f64"), &at_f64);
        let lane = common::lift::<Interval>(p).validate(tol).map_err(text);
        dump(&mut out, &format!("{name} interval"), &lane);
        if let Ok(vp) = &lane {
            let channel = vp.value_channel().map_err(|e| format!("{e:?}"));
            dump(&mut out, &format!("{name} interval channel"), &channel);
        }
        if let Ok(vp) = at_f64 {
            let lifted = vp
                .lift_onto(SketchPlane::<Interval>::xy())
                .map_err(|e| format!("{e:?}"));
            dump(&mut out, &format!("{name} lifted"), &lifted);
            if let Ok(lifted) = lifted {
                let channel = lifted.value_channel().map_err(|e| format!("{e:?}"));
                dump(&mut out, &format!("{name} lifted channel"), &channel);
            }
        }
    }
    std::fs::write(&path, out).expect("the dump file is writable");
}
