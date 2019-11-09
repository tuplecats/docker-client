
use serde::Serialize;

#[derive(Debug, Default)]
pub struct HealthCheckBuilder {
    test: Vec<String>,
    interval: Option<u64>,
    timeout: Option<u64>,
    retries: Option<u64>,
    start_period: Option<u64>
}

#[derive(Serialize, Debug, Clone)]
pub struct HealthCheck {

    #[serde(skip_serializing_if = "Vec::is_empty", rename(serialize = "Test"))]
    test: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "Interval"))]
    interval: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "Timeout"))]
    timeout: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "Retries"))]
    retries: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "StartPeriod"))]
    start_period: Option<u64>
}

impl HealthCheckBuilder {
    pub fn new() -> Self {
        HealthCheckBuilder::default()
    }

    pub fn test<T>(&mut self, cmd: T) -> &mut Self
        where T: Into<String>
    {
        self.test.push(cmd.into());

        self
    }

    pub fn interval(&mut self, interval: Option<u64>) -> &mut Self {
        self.interval = interval;

        self
    }

    pub fn timeout(&mut self, interval: Option<u64>) -> &mut Self {
        self.timeout = interval;

        self
    }

    pub fn retries(&mut self, interval: Option<u64>) -> &mut Self {
        self.retries = interval;

        self
    }

    pub fn start_period(&mut self, interval: Option<u64>) -> &mut Self {
        self.start_period = interval;

        self
    }

    pub fn build(&self) -> HealthCheck {
        HealthCheck {
            test: self.test.clone(),
            interval: self.interval.clone(),
            timeout: self.timeout.clone(),
            retries: self.retries.clone(),
            start_period: self.start_period.clone()
        }
    }
}

impl HealthCheck {
    pub fn new() -> HealthCheckBuilder {
        HealthCheckBuilder::default()
    }
}