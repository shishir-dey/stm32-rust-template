#![no_std]
#![no_main]

use defmt_rtt as _;

/// Sets up the logging before entering the test-body, so that embedded-test internal logs (e.g. Running Test <...>)  can also be printed.
/// Note: you can also inline this method in the attribute. e.g. `#[embedded_test::tests(setup=rtt_target::rtt_init_log!())]`
fn setup_log() {
    #[cfg(feature = "log")]
    rtt_target::rtt_init_log!();
    #[cfg(feature = "defmt")]
    rtt_target::rtt_init_defmt!();
}

#[cfg(test)]
#[embedded_test::tests(setup=crate::setup_log())]
mod unit_tests {
    // A test which is conditionally enabled
    #[test]
    #[cfg(feature = "log")]
    fn log() {
        log::info!("Hello, log!");
        assert!(true)
    }

    // Another example for a conditionally enabled test
    #[test]
    #[cfg(feature = "defmt")]
    fn defmt() {
        defmt::info!("Hello, defmt!");
        assert!(true)
    }

    // Tests can be ignored with the #[ignore] attribute
    #[test]
    #[ignore]
    fn it_works_ignored() {
        assert!(false)
    }

    // A test that fails with a panic
    #[test]
    fn it_fails1() {
        assert!(false)
    }

    // A test that fails with a returned Err(&str)
    #[test]
    fn it_fails2() -> Result<(), &'static str> {
        Err("It failed because ...")
    }

    // Tests can be annotated with #[should_panic] if they are expected to panic
    #[test]
    #[should_panic]
    fn it_passes() {
        assert!(false)
    }

    // This test should panic, but doesn't => it fails
    #[test]
    #[should_panic]
    fn it_fails3() {}

    // Tests can be annotated with #[timeout(<secs>)] to change the default timeout of 60s
    #[test]
    #[timeout(10)]
    fn it_timeouts() {
        loop {} // should run into the 10s timeout
    }
}
