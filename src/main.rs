use std::env;

use size::Size;
use speed::SpeedTest;

mod speed;

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>();
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
