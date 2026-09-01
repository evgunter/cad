/// R2 PROBE (BOOL-11): the lifting door's refusal is REACHABLE and
/// TYPED — a recorded chain carrying the verb refuses
/// `VerbNotInDocumentVocabulary(ContinueTo)` naming the verb, with the
/// schema-version story in the message.
#[test]
fn r2_probe_the_lifting_door_refuses_continue_to_typed() {
    use editor_core::RecordedProgramError;
    let steps: Vec<profile::Step<f64>> = vec![
        profile::Step::At(geom_core::Point2::new(0.0, 0.0)),
        profile::Step::ContinueTo(profile::Target::Start),
    ];
    match LoopProgram::from_recorded(&steps) {
        Err(RecordedProgramError::VerbNotInDocumentVocabulary(v)) => {
            assert_eq!(v, Verb::ContinueTo);
            let msg = RecordedProgramError::VerbNotInDocumentVocabulary(v).to_string();
            assert!(msg.contains("schema version"), "{msg}");
            assert!(msg.contains("ContinueTo"), "{msg}");
        }
        other => panic!("expected the typed vocabulary refusal, got {other:?}"),
    }
}
