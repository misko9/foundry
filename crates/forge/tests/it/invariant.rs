//! Invariant tests.

use crate::config::*;
use alloy_primitives::U256;
use forge::{fuzz::CounterExample, result::TestStatus, TestOptions};
use foundry_test_utils::Filter;
use std::collections::BTreeMap;

#[tokio::test(flavor = "multi_thread")]
async fn test_invariant() {
    let mut runner = runner().await;

    let results = runner
        .test_collect(
            &Filter::new(".*", ".*", ".*fuzz/invariant/(target|targetAbi|common)"),
            test_opts(),
        )
        .await;

    assert_multiple(
        &results,
        BTreeMap::from([
            (
                "fuzz/invariant/common/InvariantHandlerFailure.t.sol:InvariantHandlerFailure",
                vec![("statefulFuzz_BrokenInvariant()", true, None, None, None)],
            ),
            (
                "fuzz/invariant/common/InvariantInnerContract.t.sol:InvariantInnerContract",
                vec![(
                    "invariantHideJesus()",
                    false,
                    Some("revert: jesus betrayed".into()),
                    None,
                    None,
                )],
            ),
            (
                "fuzz/invariant/common/InvariantReentrancy.t.sol:InvariantReentrancy",
                vec![("invariantNotStolen()", true, None, None, None)],
            ),
            (
                "fuzz/invariant/common/InvariantTest1.t.sol:InvariantTest",
                vec![
                    ("invariant_neverFalse()", false, Some("revert: false".into()), None, None),
                    (
                        "statefulFuzz_neverFalseWithInvariantAlias()",
                        false,
                        Some("revert: false".into()),
                        None,
                        None,
                    ),
                ],
            ),
            (
                "fuzz/invariant/target/ExcludeContracts.t.sol:ExcludeContracts",
                vec![("invariantTrueWorld()", true, None, None, None)],
            ),
            (
                "fuzz/invariant/target/TargetContracts.t.sol:TargetContracts",
                vec![("invariantTrueWorld()", true, None, None, None)],
            ),
            (
                "fuzz/invariant/target/TargetSenders.t.sol:TargetSenders",
                vec![(
                    "invariantTrueWorld()",
                    false,
                    Some("revert: false world".into()),
                    None,
                    None,
                )],
            ),
            (
                "fuzz/invariant/target/TargetInterfaces.t.sol:TargetWorldInterfaces",
                vec![(
                    "invariantTrueWorld()",
                    false,
                    Some("revert: false world".into()),
                    None,
                    None,
                )],
            ),
            (
                "fuzz/invariant/target/ExcludeSenders.t.sol:ExcludeSenders",
                vec![("invariantTrueWorld()", true, None, None, None)],
            ),
            (
                "fuzz/invariant/target/TargetSelectors.t.sol:TargetSelectors",
                vec![("invariantTrueWorld()", true, None, None, None)],
            ),
            (
                "fuzz/invariant/targetAbi/ExcludeArtifacts.t.sol:ExcludeArtifacts",
                vec![("invariantShouldPass()", true, None, None, None)],
            ),
            (
                "fuzz/invariant/targetAbi/TargetArtifacts.t.sol:TargetArtifacts",
                vec![
                    ("invariantShouldPass()", true, None, None, None),
                    (
                        "invariantShouldFail()",
                        false,
                        Some("revert: false world".into()),
                        None,
                        None,
                    ),
                ],
            ),
            (
                "fuzz/invariant/targetAbi/TargetArtifactSelectors.t.sol:TargetArtifactSelectors",
                vec![("invariantShouldPass()", true, None, None, None)],
            ),
            (
                "fuzz/invariant/targetAbi/TargetArtifactSelectors2.t.sol:TargetArtifactSelectors2",
                vec![(
                    "invariantShouldFail()",
                    false,
                    Some("revert: it's false".into()),
                    None,
                    None,
                )],
            ),
            (
                "fuzz/invariant/common/InvariantShrinkWithAssert.t.sol:InvariantShrinkWithAssert",
                vec![(
                    "invariant_with_assert()",
                    false,
                    Some("<empty revert data>".into()),
                    None,
                    None,
                )],
            ),
            (
                "fuzz/invariant/common/InvariantShrinkWithAssert.t.sol:InvariantShrinkWithRequire",
                vec![(
                    "invariant_with_require()",
                    false,
                    Some("revert: wrong counter".into()),
                    None,
                    None,
                )],
            ),
            (
                "fuzz/invariant/common/InvariantPreserveState.t.sol:InvariantPreserveState",
                vec![("invariant_preserve_state()", true, None, None, None)],
            ),
        ]),
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_invariant_override() {
    let mut runner = runner().await;

    let mut opts = test_opts();
    opts.invariant.call_override = true;
    runner.test_options = opts.clone();

    let results = runner
        .test_collect(
            &Filter::new(".*", ".*", ".*fuzz/invariant/common/InvariantReentrancy.t.sol"),
            opts,
        )
        .await;

    assert_multiple(
        &results,
        BTreeMap::from([(
            "fuzz/invariant/common/InvariantReentrancy.t.sol:InvariantReentrancy",
            vec![("invariantNotStolen()", false, Some("revert: stolen".into()), None, None)],
        )]),
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_invariant_fail_on_revert() {
    let mut runner = runner().await;

    let mut opts = test_opts();
    opts.invariant.fail_on_revert = true;
    opts.invariant.runs = 1;
    opts.invariant.depth = 10;
    runner.test_options = opts.clone();

    let results = runner
        .test_collect(
            &Filter::new(".*", ".*", ".*fuzz/invariant/common/InvariantHandlerFailure.t.sol"),
            opts,
        )
        .await;

    assert_multiple(
        &results,
        BTreeMap::from([(
            "fuzz/invariant/common/InvariantHandlerFailure.t.sol:InvariantHandlerFailure",
            vec![(
                "statefulFuzz_BrokenInvariant()",
                false,
                Some("revert: failed on revert".into()),
                None,
                None,
            )],
        )]),
    );
}

#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn test_invariant_storage() {
    let mut runner = runner().await;

    let mut opts = test_opts();
    opts.invariant.depth = 100 + (50 * cfg!(windows) as u32);
    opts.fuzz.seed = Some(U256::from(6u32));
    runner.test_options = opts.clone();

    let results = runner
        .test_collect(
            &Filter::new(".*", ".*", ".*fuzz/invariant/storage/InvariantStorageTest.t.sol"),
            opts,
        )
        .await;

    assert_multiple(
        &results,
        BTreeMap::from([(
            "fuzz/invariant/storage/InvariantStorageTest.t.sol:InvariantStorageTest",
            vec![
                ("invariantChangeAddress()", false, Some("changedAddr".to_string()), None, None),
                ("invariantChangeString()", false, Some("changedString".to_string()), None, None),
                ("invariantChangeUint()", false, Some("changedUint".to_string()), None, None),
                ("invariantPush()", false, Some("pushUint".to_string()), None, None),
            ],
        )]),
    );
}

#[tokio::test(flavor = "multi_thread")]
#[cfg_attr(windows, ignore = "for some reason there's different rng")]
async fn test_invariant_shrink() {
    let mut runner = runner().await;

    let mut opts = test_opts();
    opts.fuzz.seed = Some(U256::from(119u32));
    runner.test_options = opts.clone();

    let results = runner
        .test_collect(
            &Filter::new(".*", ".*", ".*fuzz/invariant/common/InvariantInnerContract.t.sol"),
            opts,
        )
        .await;

    let results =
        results.values().last().expect("`InvariantInnerContract.t.sol` should be testable.");

    let result =
        results.test_results.values().last().expect("`InvariantInnerContract` should be testable.");

    let counter = result
        .counterexample
        .as_ref()
        .expect("`InvariantInnerContract` should have failed with a counterexample.");

    match counter {
        CounterExample::Single(_) => panic!("CounterExample should be a sequence."),
        // `fuzz_seed` at 119 makes this sequence shrinkable from 4 to 2.
        CounterExample::Sequence(sequence) => {
            assert_eq!(sequence.len(), 2);

            // call order should always be preserved
            let create_fren_sequence = sequence[0].clone();
            assert_eq!(
                create_fren_sequence.contract_name.unwrap(),
                "fuzz/invariant/common/InvariantInnerContract.t.sol:Jesus"
            );
            assert_eq!(create_fren_sequence.signature.unwrap(), "create_fren()");

            let betray_sequence = sequence[1].clone();
            assert_eq!(
                betray_sequence.contract_name.unwrap(),
                "fuzz/invariant/common/InvariantInnerContract.t.sol:Judas"
            );
            assert_eq!(betray_sequence.signature.unwrap(), "betray()");
        }
    };
}

#[tokio::test(flavor = "multi_thread")]
#[cfg_attr(windows, ignore = "for some reason there's different rng")]
async fn test_invariant_assert_shrink() {
    let mut opts = test_opts();
    opts.fuzz.seed = Some(U256::from(119u32));

    // ensure assert and require shrinks to same sequence of 3 or less
    test_shrink(opts.clone(), "InvariantShrinkWithAssert").await;
    test_shrink(opts.clone(), "InvariantShrinkWithRequire").await;
}

async fn test_shrink(opts: TestOptions, contract_pattern: &str) {
    let mut runner = runner().await;
    runner.test_options = opts.clone();
    let results = runner
        .test_collect(
            &Filter::new(
                ".*",
                contract_pattern,
                ".*fuzz/invariant/common/InvariantShrinkWithAssert.t.sol",
            ),
            opts,
        )
        .await;
    let results = results.values().last().expect("`InvariantShrinkWithAssert` should be testable.");

    let result = results
        .test_results
        .values()
        .last()
        .expect("`InvariantShrinkWithAssert` should be testable.");

    assert_eq!(result.status, TestStatus::Failure);

    let counter = result
        .counterexample
        .as_ref()
        .expect("`InvariantShrinkWithAssert` should have failed with a counterexample.");

    match counter {
        CounterExample::Single(_) => panic!("CounterExample should be a sequence."),
        CounterExample::Sequence(sequence) => {
            assert!(sequence.len() <= 3);
        }
    };
}

#[tokio::test(flavor = "multi_thread")]
async fn test_invariant_preserve_state() {
    let mut runner = runner().await;

    // should not fail with default options
    let mut opts = test_opts();
    opts.invariant.fail_on_revert = true;
    runner.test_options = opts.clone();
    let results = runner
        .test_collect(
            &Filter::new(".*", ".*", ".*fuzz/invariant/common/InvariantPreserveState.t.sol"),
            opts,
        )
        .await;
    assert_multiple(
        &results,
        BTreeMap::from([(
            "fuzz/invariant/common/InvariantPreserveState.t.sol:InvariantPreserveState",
            vec![("invariant_preserve_state()", true, None, None, None)],
        )]),
    );

    // same test should revert when preserve state enabled
    let mut opts = test_opts();
    opts.invariant.fail_on_revert = true;
    opts.invariant.preserve_state = true;
    runner.test_options = opts.clone();

    let results = runner
        .test_collect(
            &Filter::new(".*", ".*", ".*fuzz/invariant/common/InvariantPreserveState.t.sol"),
            opts,
        )
        .await;
    assert_multiple(
        &results,
        BTreeMap::from([(
            "fuzz/invariant/common/InvariantPreserveState.t.sol:InvariantPreserveState",
            vec![(
                "invariant_preserve_state()",
                false,
                Some("EvmError: Revert".into()),
                None,
                None,
            )],
        )]),
    );
}
