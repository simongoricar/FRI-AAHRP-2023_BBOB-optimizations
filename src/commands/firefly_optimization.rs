use std::thread;
use std::time::Instant;

use miette::{miette, IntoDiagnostic, Result, WrapErr};
use rayon::ThreadPoolBuilder;

use crate::algorithms::firefly::perform_firefly_swarm_optimization;
use crate::core::names::ALL_BBOB_FUNCTION_NAMES;
use crate::core::suite::BBOBSuite;

pub fn cmd_run_firefly_optimization() -> Result<()> {
    // Make sure we're using as many cores as we can.
    // ThreadPoolBuilder::new()
    //     .num_threads(
    //         thread::available_parallelism()
    //             .into_diagnostic()
    //             .wrap_err_with(|| {
    //                 miette!("Could not get available parallelism.")
    //             })?
    //             .into(),
    //     )
    //     .build_global()
    //     .into_diagnostic()
    //     .wrap_err_with(|| {
    //         miette!("Could not modify rayon's global thread pool.")
    //     })?;

    // Initialize coco / the BBOB suite.
    let mut suite = BBOBSuite::new()?;

    let start_time = Instant::now();

    for problem_name in ALL_BBOB_FUNCTION_NAMES {
        let problem = suite.problem(problem_name, None)?;
        let problem_start_time = Instant::now();

        let minimum = perform_firefly_swarm_optimization(problem, None)?;

        let problem_delta_time = problem_start_time.elapsed().as_secs_f64();
        println!(
            "[{:02}/{:02}|{}] {}Minimum: {:.6}    ({:.4} seconds)",
            problem_name.function_index(),
            ALL_BBOB_FUNCTION_NAMES.len(),
            problem_name.function_name(),
            " ".repeat(32 - problem_name.function_name().len()),
            minimum.value,
            problem_delta_time
        );
    }

    let delta_time = start_time.elapsed().as_secs_f64();

    println!("-- Finished in {:.4} seconds --", delta_time);

    Ok(())
}
