
use std::any::Any;

use super::bench::BenchSamples;
use super::time;
use super::types::TestDesc;
use super::options::ShouldPanic;

pub use self::TestResult::*;

// Return codes for secondary process.
// Start somewhere other than 0 so we know the return code means what we think
// it means.
pub const TR_OK: i32 = 50;
pub const TR_FAILED: i32 = 51;

#[derive(Debug, Clone, PartialEq)]
pub enum TestResult {
    TrOk,
    TrFailed,
    TrFailedMsg(String),
    TrIgnored,
    TrAllowedFail,
    TrBench(BenchSamples),
    TrTimedFail,
}

unsafe impl Send for TestResult {}


pub fn calc_result<'a>(
    desc: &TestDesc,
    task_result: Result<(), &'a (dyn Any + 'static + Send)>,
    time_opts: &Option<time::TestTimeOptions>,
    exec_time: &Option<time::TestExecTime>
) -> TestResult {
    let result = match (&desc.should_panic, task_result) {
        (&ShouldPanic::No, Ok(())) | (&ShouldPanic::Yes, Err(_)) => TestResult::TrOk,
        (&ShouldPanic::YesWithMessage(msg), Err(ref err)) => {
            if err
                .downcast_ref::<String>()
                .map(|e| &**e)
                .or_else(|| err.downcast_ref::<&'static str>().map(|e| *e))
                .map(|e| e.contains(msg))
                .unwrap_or(false)
            {
                TestResult::TrOk
            } else {
                if desc.allow_fail {
                    TestResult::TrAllowedFail
                } else {
                    TestResult::TrFailedMsg(format!("panic did not include expected string '{}'", msg))
                }
            }
        }
        (&ShouldPanic::Yes, Ok(())) => TestResult::TrFailedMsg("test did not panic as expected".to_string()),
        _ if desc.allow_fail => TestResult::TrAllowedFail,
        _ => TestResult::TrFailed,
    };

    // If test is already failed (or allowed to fail), do not change the result.
    if result != TestResult::TrOk {
        return result;
    }

    // Check if test is failed due to timeout.
    if let (Some(opts), Some(time)) = (time_opts, exec_time) {
        if opts.error_on_excess && opts.is_critical(desc, time) {
            return TestResult::TrTimedFail;
        }
    }

    result
}

pub fn get_result_from_exit_code(
    desc: &TestDesc,
    code: i32,
    time_opts: &Option<time::TestTimeOptions>,
    exec_time: &Option<time::TestExecTime>,
) -> TestResult {
    let result = match (desc.allow_fail, code) {
        (_, TR_OK) => TestResult::TrOk,
        (true, TR_FAILED) => TestResult::TrAllowedFail,
        (false, TR_FAILED) => TestResult::TrFailed,
        (_, _) => TestResult::TrFailedMsg(format!("got unexpected return code {}", code)),
    };

    // If test is already failed (or allowed to fail), do not change the result.
    if result != TestResult::TrOk {
        return result;
    }

    // Check if test is failed due to timeout.
    if let (Some(opts), Some(time)) = (time_opts, exec_time) {
        if opts.error_on_excess && opts.is_critical(desc, time) {
            return TestResult::TrTimedFail;
        }
    }

    result
}
