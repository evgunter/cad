//! The controls guide and the input mapper read one table.
//!
//! [`InputMap::bindings`] is both what the mapper honours and what the
//! guide lists. These rows hold the two readings together from
//! outside, over every registered preset: each row the guide shows is
//! a gesture the mapper answers with that row's action, and each
//! gesture the mapper answers is a row the guide shows.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use viewer::camera::{Camera, CameraOp};
use viewer::input::{
    Action, Binding, Held, InputMap, Navigation, PRESETS, PickAction, PointerButton, ScrollAxis,
    ViewportEvent, ViewportSize,
};

use crate::common;

fn viewport() -> ViewportSize {
    ViewportSize {
        width_px: 1024.0,
        height_px: 768.0,
    }
}

fn camera() -> Camera {
    common::framed(viewport().aspect().expect("a positive aspect"))
}

const AT: [f64; 2] = [10.0, 20.0];

/// Both mappings' answers to `event`, in the guide's terms. Leaving
/// the viewport is not asked here: it is no gesture of its own in the
/// guide.
fn did(map: &InputMap, event: &ViewportEvent) -> Vec<Action> {
    let moved = map.map(event, viewport(), &camera()).map(|op| match op {
        CameraOp::Orbit { .. } => Action::Orbit,
        CameraOp::Pan { .. } => Action::Pan,
        CameraOp::Dolly { .. } => Action::Zoom,
        CameraOp::Frame { .. } => panic!("{event:?} framed the model"),
    });
    let picked = map.pick(event).map(|action| match action {
        PickAction::Select(_) => Action::Select,
        PickAction::Hover(_) => Action::Highlight,
        PickAction::ClearHover => panic!("{event:?} cleared the hover"),
    });
    moved.into_iter().chain(picked).collect()
}

fn drag(button: PointerButton, held: Held) -> ViewportEvent {
    ViewportEvent::Drag {
        button,
        held,
        delta_px: [7.0, -3.0],
    }
}

fn scroll(axis: ScrollAxis, held: Held) -> ViewportEvent {
    ViewportEvent::Scroll {
        held,
        axis,
        units: 1.0,
    }
}

/// The event that is exactly `binding`'s gesture.
fn performed(binding: &Binding) -> ViewportEvent {
    match *binding {
        Binding::Drag { button, held, .. } => drag(button, held),
        Binding::Scroll { axis, held } => scroll(axis, held),
        Binding::Pinch => ViewportEvent::Pinch { factor: 1.25 },
        Binding::Click { button } => ViewportEvent::Click { button, pos_px: AT },
        Binding::Hover => ViewportEvent::Hover { pos_px: AT },
    }
}

/// Whether `binding` is a row that names `event`'s gesture.
fn names(binding: &Binding, event: &ViewportEvent) -> bool {
    match (*binding, *event) {
        (
            Binding::Drag { button, held, .. },
            ViewportEvent::Drag {
                button: dragged,
                held: down,
                ..
            },
        ) => button == dragged && held.met_by(down),
        (
            Binding::Scroll { axis, held },
            ViewportEvent::Scroll {
                axis: scrolled,
                held: down,
                ..
            },
        ) => axis == scrolled && held.met_by(down),
        (Binding::Pinch, ViewportEvent::Pinch { .. })
        | (Binding::Hover, ViewportEvent::Hover { .. }) => true,
        (
            Binding::Click { button },
            ViewportEvent::Click {
                button: clicked, ..
            },
        ) => button == clicked,
        _ => false,
    }
}

/// Every event of the vocabulary that could denote something: each
/// button dragged under each modifier state and clicked, each axis
/// scrolled both ways under each modifier state, a pinch, a hover.
fn every_gesture() -> Vec<ViewportEvent> {
    let mut events = vec![
        ViewportEvent::Pinch { factor: 1.25 },
        ViewportEvent::Hover { pos_px: AT },
    ];
    for held in Held::ALL {
        for axis in ScrollAxis::ALL {
            for units in [1.0, -2.0] {
                events.push(ViewportEvent::Scroll { held, axis, units });
            }
        }
    }
    for button in PointerButton::ALL {
        events.push(ViewportEvent::Click { button, pos_px: AT });
        for held in Held::ALL {
            events.push(drag(button, held));
        }
    }
    events
}

/// `held` with each key moved to the next: ctrl to shift, shift to
/// alt, alt to ctrl.
fn rotated(held: Held) -> Held {
    Held {
        ctrl: held.alt,
        shift: held.ctrl,
        alt: held.shift,
    }
}

/// The registered presets, and one table no preset holds: the
/// default's rows reversed, with every key moved to the next and every
/// scroll on the other axis, so a mapper arm that read a fixed button,
/// modifier or axis instead of the table disagrees with it.
fn tables() -> Vec<(String, InputMap)> {
    let swapped: Vec<Binding> = InputMap::DEFAULT
        .bindings
        .iter()
        .rev()
        .map(|binding| match *binding {
            Binding::Drag {
                button,
                held,
                moves,
            } => Binding::Drag {
                button,
                held: rotated(held),
                moves,
            },
            Binding::Scroll { axis, held } => Binding::Scroll {
                axis: match axis {
                    ScrollAxis::Wheel => ScrollAxis::Sideways,
                    ScrollAxis::Sideways => ScrollAxis::Wheel,
                },
                held: rotated(held),
            },
            Binding::Click { .. } => Binding::Click {
                button: PointerButton::Middle,
            },
            other => other,
        })
        .collect();
    PRESETS
        .iter()
        .map(|(name, map)| ((*name).to_owned(), *map))
        .chain([(
            "the swapped table".to_owned(),
            InputMap {
                bindings: Vec::leak(swapped),
                ..InputMap::DEFAULT
            },
        )])
        .collect()
}

/// **Every row the guide shows is honoured**: performing exactly the
/// row's gesture does exactly the row's action. A row shadowed by
/// another, or one the mapper has no arm for, is red here.
#[test]
fn every_guide_row_is_a_gesture_the_mapper_answers_with_that_action() {
    for (name, map) in tables() {
        assert!(!map.bindings.is_empty(), "{name}: a table with no rows");
        for binding in map.bindings {
            assert_eq!(
                did(&map, &performed(binding)),
                vec![binding.does()],
                "{name}: the guide says `{}` does `{}`",
                binding.gesture(),
                binding.action(),
            );
        }
    }
}

/// **Everything the mapper honours is a row the guide shows**: over
/// the whole event vocabulary, an event that denotes something is
/// named by a row whose action is what it did. A binding hard-coded in
/// the mapper beside the table is red here.
#[test]
fn every_gesture_the_mapper_answers_is_a_guide_row() {
    for (name, map) in tables() {
        for event in every_gesture() {
            for done in did(&map, &event) {
                assert!(
                    map.bindings
                        .iter()
                        .any(|binding| names(binding, &event) && binding.does() == done),
                    "{name}: {event:?} did {done:?} and no guide row says so"
                );
            }
        }
    }
}

/// A keyed row's gesture with no keys, and the keys it requires: a
/// drag's button, a scroll's axis.
fn keyed(binding: &Binding) -> Option<(Binding, Held)> {
    match *binding {
        Binding::Drag { button, held, .. } => Some((
            Binding::Drag {
                button,
                held: Held::NONE,
                moves: Navigation::Orbit,
            },
            held,
        )),
        Binding::Scroll { axis, held } => Some((
            Binding::Scroll {
                axis,
                held: Held::NONE,
            },
            held,
        )),
        Binding::Pinch | Binding::Click { .. } | Binding::Hover => None,
    }
}

/// **A key a row does not name is not asked about**: a drag's or a
/// scroll's row with any further keys down still does the row's
/// action, unless that state is another row's — one on the same button
/// or axis naming more keys, or as many and listed earlier.
#[test]
fn a_keyed_row_holds_under_every_key_it_does_not_name() {
    let (mut drags, mut scrolls) = (0, 0);
    for (name, map) in tables() {
        for (at, row) in map.bindings.iter().enumerate() {
            let Some((gesture, held)) = keyed(row) else {
                continue;
            };
            for down in Held::ALL.into_iter().filter(|down| held.met_by(*down)) {
                let taken = map.bindings.iter().enumerate().any(|(other_at, other)| {
                    keyed(other).is_some_and(|(other_gesture, other_held)| {
                        other_at != at
                            && other_gesture == gesture
                            && other_held.met_by(down)
                            && (other_held.count() > held.count()
                                || (other_held.count() == held.count() && other_at < at))
                    })
                });
                if taken || down == held {
                    continue;
                }
                let event = match gesture {
                    Binding::Drag { button, .. } => {
                        drags += 1;
                        drag(button, down)
                    }
                    Binding::Scroll { axis, .. } => {
                        scrolls += 1;
                        scroll(axis, down)
                    }
                    _ => unreachable!("`keyed` reads drags and scrolls"),
                };
                assert_eq!(
                    did(&map, &event),
                    vec![row.does()],
                    "{name}: `{}` with {down:?} down",
                    row.gesture(),
                );
            }
        }
    }
    assert!(
        drags >= 16 && scrolls >= 8,
        "the tables hold rows with a key to spare: {drags} drag and {scrolls} scroll states asked"
    );
}

/// **The default table's wheel zooms under every key, and its
/// sideways scroll does nothing under any** — the shift+wheel the
/// toolkit folds sideways among the first, a trackpad's sideways
/// swipe the second.
#[test]
fn the_default_wheel_zooms_under_every_key_and_a_sideways_scroll_does_nothing() {
    for held in Held::ALL {
        assert_eq!(
            did(&InputMap::DEFAULT, &scroll(ScrollAxis::Wheel, held)),
            vec![Action::Zoom],
            "the wheel under {held:?}"
        );
        assert_eq!(
            did(&InputMap::DEFAULT, &scroll(ScrollAxis::Sideways, held)),
            vec![],
            "a sideways scroll under {held:?}"
        );
    }
}

/// **Between two rows a drag fits that name equally many keys, the
/// earlier is the one it means** — whichever of the two that is.
#[test]
fn of_two_rows_naming_equally_many_keys_the_earlier_wins() {
    const ORBIT: Binding = Binding::Drag {
        button: PointerButton::Middle,
        held: Held::SHIFT,
        moves: Navigation::Orbit,
    };
    const PAN: Binding = Binding::Drag {
        button: PointerButton::Middle,
        held: Held::ALT,
        moves: Navigation::Pan,
    };
    let both = drag(
        PointerButton::Middle,
        Held {
            shift: true,
            alt: true,
            ..Held::NONE
        },
    );
    for (bindings, first) in [(&[ORBIT, PAN], Action::Orbit), (&[PAN, ORBIT], Action::Pan)] {
        let map = InputMap {
            bindings,
            ..InputMap::DEFAULT
        };
        assert_eq!(did(&map, &both), vec![first], "{bindings:?}");
    }
}

/// The words of `text`, in lower case.
fn words(text: &str) -> Vec<String> {
    text.split(|letter: char| !letter.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .collect()
}

/// **A row's wording says what its fields say**: a drag's gesture
/// names its button and no other, a scroll's says `sideways` exactly
/// on that axis, and either names a key exactly when the row requires
/// it; a drag's action is its `moves`; and no two actions, and no two
/// gestures, read alike. Every button and axis under every key state,
/// so the rows no preset holds are worded too.
#[test]
fn a_rows_wording_names_its_button_its_keys_and_its_action() {
    let names_only = |said: &[String], button: PointerButton, row: &Binding| {
        for other in PointerButton::ALL {
            assert_eq!(
                said.iter().any(|word| word == other.name()),
                other == button,
                "{row:?} names the {} button in `{}`",
                other.name(),
                row.gesture(),
            );
        }
    };
    let names_keys = |said: &[String], held: Held, row: &Binding| {
        for (key, required) in [
            ("ctrl", held.ctrl),
            ("shift", held.shift),
            ("alt", held.alt),
        ] {
            assert_eq!(
                said.iter().any(|word| word == key),
                required,
                "{row:?} says {key} in `{}`",
                row.gesture()
            );
        }
    };
    let mut gestures = Vec::new();
    for held in Held::ALL {
        for axis in ScrollAxis::ALL {
            let row = Binding::Scroll { axis, held };
            let said = words(&row.gesture());
            names_keys(&said, held, &row);
            assert!(
                said.iter().any(|word| word == "scroll"),
                "{row:?}: {said:?}"
            );
            assert_eq!(
                said.iter().any(|word| word == "sideways"),
                axis == ScrollAxis::Sideways,
                "{row:?}: {said:?}"
            );
            assert!(
                PointerButton::ALL
                    .iter()
                    .all(|button| !said.iter().any(|word| word == button.name())),
                "{row:?} names a button: {said:?}"
            );
            assert_eq!(row.does(), Action::Zoom, "{row:?}");
            gestures.push(row.gesture());
        }
    }
    for button in PointerButton::ALL {
        for held in Held::ALL {
            for (moves, does) in [
                (Navigation::Orbit, Action::Orbit),
                (Navigation::Pan, Action::Pan),
            ] {
                let row = Binding::Drag {
                    button,
                    held,
                    moves,
                };
                let said = words(&row.gesture());
                names_only(&said, button, &row);
                names_keys(&said, held, &row);
                assert!(said.iter().any(|word| word == "drag"), "{row:?}: {said:?}");
                assert_eq!(row.does(), does, "{row:?}");
                assert_eq!(row.action(), does.label(), "{row:?}");
                if moves == Navigation::Orbit {
                    gestures.push(row.gesture());
                }
            }
        }
        let row = Binding::Click { button };
        let said = words(&row.gesture());
        names_only(&said, button, &row);
        assert!(said.iter().any(|word| word == "click"), "{row:?}: {said:?}");
        gestures.push(row.gesture());
    }
    gestures.extend([Binding::Pinch, Binding::Hover].map(|row| row.gesture()));
    for (at, gesture) in gestures.iter().enumerate() {
        assert!(
            !gestures[..at].contains(gesture),
            "two gestures read `{gesture}`: {gestures:?}"
        );
    }
    let labels = Action::ALL.map(Action::label);
    for (at, label) in labels.iter().enumerate() {
        assert!(
            !label.is_empty() && !labels[..at].contains(label),
            "two actions read `{label}`: {labels:?}"
        );
    }
}

/// **A map with no rows honours nothing** — leaving the viewport
/// included, which rides on the hover row.
#[test]
fn an_empty_table_honours_no_gesture() {
    let map = InputMap {
        bindings: &[],
        ..InputMap::DEFAULT
    };
    for event in every_gesture() {
        assert_eq!(did(&map, &event), vec![], "{event:?} with no bindings");
    }
    assert_eq!(map.pick(&ViewportEvent::Leave), None, "leave, no hover row");
    assert_eq!(
        InputMap::DEFAULT.pick(&ViewportEvent::Leave),
        Some(PickAction::ClearHover),
        "leave, under the hover row"
    );
}

/// **No two rows of a guide read alike**: a guide with two rows for
/// one gesture cannot say which of them holds.
#[test]
fn a_guide_names_each_gesture_once() {
    for (name, map) in tables() {
        let gestures: Vec<String> = map.bindings.iter().map(Binding::gesture).collect();
        for (at, gesture) in gestures.iter().enumerate() {
            assert!(
                !gestures[..at].contains(gesture),
                "{name}: `{gesture}` is listed twice in {gestures:?}"
            );
        }
    }
}

/// The README's bindings table, as the guide writes it.
fn readme_table(map: &InputMap) -> String {
    let rows: String = map
        .bindings
        .iter()
        .map(|binding| format!("| {} | {} |\n", binding.gesture(), binding.action()))
        .collect();
    format!("| Gesture | Action |\n| --- | --- |\n{rows}")
}

/// **`README.md`'s "Mouse bindings" table is the default guide, row
/// for row.** The page is read before the viewer is run, so it keeps a
/// table; this is what stops that table being a second copy.
#[test]
fn the_readme_bindings_table_is_the_default_guide() {
    let readme = include_str!("../README.md");
    let section = readme
        .split_once("\n## Mouse bindings\n")
        .expect("the README has a `## Mouse bindings` section")
        .1;
    let table: String = section
        .lines()
        .skip_while(|line| !line.starts_with('|'))
        .take_while(|line| line.starts_with('|'))
        .map(|line| format!("{line}\n"))
        .collect();
    assert_eq!(
        table,
        readme_table(&InputMap::DEFAULT),
        "crates/viewer/README.md's bindings table against `InputMap::DEFAULT.bindings`"
    );
}
