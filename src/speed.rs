use std::{ops::Add, time::Duration};

use anyhow::Result;
use curl::easy::{Easy2, Handler};
use size::Size;

pub struct Statistic {
    info: Vec<Size>,
    target_limit: Size,
}

impl Statistic {
    fn new() -> Self {
        Self {
            info: Vec::new(),
            target_limit: Size::from_exabytes(1), // FIXME: use a proper way to do max limit
        }
    }
    pub fn max(&self) -> Size {
        *(self.info.iter().max().unwrap())
    }
    pub fn min(&self) -> Size {
        *(self.info.iter().filter(|e| e.bytes() > 0).min().unwrap())
    }
    pub fn avg(&self) -> Size {
        self.info.iter().fold(Size::from_bytes(0), Add::add) / self.info.len()
    }
    pub fn detail(&self) -> &[Size] {
        &self.info
    }
    pub fn top_n(&self, n: usize) -> Vec<Size> {
        let mut sorted = self.info.clone();
        sorted.sort();
        sorted.reverse();
        sorted.truncate(n);
        sorted
    }
    pub fn last_n(&self, n: usize) -> Vec<Size> {
        let mut sorted: Vec<Size> = self
            .info
            .iter()
            .filter(|e| e.bytes() > 0)
            .cloned()
            .collect();
        sorted.sort();
        sorted.truncate(n);
        sorted
    }
}

impl Handler for Statistic {
    fn progress(&mut self, _: f64, dlnow: f64, _: f64, _: f64) -> bool {
        let rate = Size::from_bytes(dlnow);
        self.info.push(rate);
        rate <= self.target_limit
    }
}

pub struct SpeedTest {
    handle: Easy2<Statistic>,
}

impl SpeedTest {
    fn setup_handle(handle: &mut Easy2<Statistic>) -> Result<()> {
        handle.get(true)?;
        handle.progress(true)?;
        handle.follow_location(true)?;
        handle.timeout(Duration::from_secs(30))?;
        Ok(())
    }

    pub fn new(target: &str) -> Result<Self> {
        let mut handle = Easy2::new(Statistic::new());
        handle.url(target)?;
        Self::setup_handle(&mut handle)?;
        Ok(SpeedTest { handle })
    }

    pub fn new_with_limit(target: &str, limit_min: Size, limit_max: Size) -> Result<Self> {
        let mut stat = Statistic::new();
        stat.target_limit = limit_max;
        let mut handle = Easy2::new(stat);
        handle.url(target)?;
        Self::setup_handle(&mut handle)?;
        handle.low_speed_limit(limit_min.bytes() as u32)?;
        handle.low_speed_time(Duration::from_secs(10))?;
        Ok(SpeedTest { handle })
    }

    pub fn set_timeout(&mut self, timeout: Duration) -> Result<(), curl::Error> {
        self.handle.timeout(timeout)
    }

    pub fn speedtest(&self) -> Result<(&Statistic, bool), curl::Error> {
        self.handle
            .perform()
            .map(|_| false)
            .or_else(|e| {
                (e.is_aborted_by_callback() || e.is_operation_timedout())
                    .then_some(true)
                    .ok_or(e)
            })
            .map(|recover| (self.handle.get_ref(), recover))
    }
}
