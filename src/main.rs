use std::{env, process::exit};

use size::Size;
use speed::SpeedTest;

mod speed;

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() <= 1 {
        println!(
            "Usage: {} <Connection target> [<Min Speed> <Max Speed>]",
            args[0]
        );
        exit(1);
    }
    let speedtest = if args.len() > 3 {
        SpeedTest::new_with_limit(
            &args[1],
            Size::from_str(&args[2])?,
            Size::from_str(&args[3])?,
        )
    } else {
        SpeedTest::new(&args[1])
    }?;
    let (stats, recover) = speedtest.speedtest()?;
    println!(
        "{} {} {} {}",
        stats.min(),
        stats.max(),
        stats.avg(),
        recover
    );
    Ok(())
}
