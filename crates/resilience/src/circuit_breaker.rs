use aiot_core::{AiotError, CoreError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for CircuitState.
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Documentation for CircuitBreakerProfile.
pub enum CircuitBreakerProfile {
    Storage,
    Plugin,
    Http,
    Llm,
    Default,
}

impl CircuitBreakerProfile {
    #[must_use]
    /// Documentation for fn.
    pub const fn default_thresholds(&self) -> (u32, u32, u64) {
        match self {
            Self::Storage => (3, 2, 5000), // Fail fast, recover fast
            Self::Plugin => (5, 1, 10000),
            Self::Http => (5, 3, 30000),
            Self::Llm => (2, 1, 60000), // LLM is expensive, backoff longer
            Self::Default => (5, 2, 10000),
        }
    }
}

/// A standard Circuit Breaker pattern.
pub struct CircuitBreaker {
    /// Documentation for field `state`.
    pub state: CircuitState,
    /// Documentation for field `profile`.
    pub profile: CircuitBreakerProfile,
    /// Documentation for field `failure_count`.
    pub failure_count: u32,
    /// Documentation for field `failure_threshold`.
    pub failure_threshold: u32,
    /// Documentation for field `success_count`.
    pub success_count: u32,
    /// Documentation for field `success_threshold`.
    pub success_threshold: u32,
    /// Documentation for field `last_failure_timestamp`.
    pub last_failure_timestamp: u64,
    /// Documentation for field `reset_timeout`.
    pub reset_timeout: u64,
}

impl CircuitBreaker {
    #[must_use]
    /// Documentation for fn.
    pub const fn new(failure_threshold: u32, success_threshold: u32, reset_timeout: u64) -> Self {
        Self {
            state: CircuitState::Closed,
            profile: CircuitBreakerProfile::Default,
            failure_count: 0,
            failure_threshold,
            success_count: 0,
            success_threshold,
            last_failure_timestamp: 0,
            reset_timeout,
        }
    }

    #[must_use]
    /// Documentation for fn.
    pub const fn with_profile(profile: CircuitBreakerProfile) -> Self {
        let (ft, st, rt) = profile.default_thresholds();
        Self {
            state: CircuitState::Closed,
            profile,
            failure_count: 0,
            failure_threshold: ft,
            success_count: 0,
            success_threshold: st,
            last_failure_timestamp: 0,
            reset_timeout: rt,
        }
    }

    /// Update the state before executing a task. Returns error if Open.
    pub fn allow_request(&mut self, current_time: u64) -> Result<(), AiotError> {
        match self.state {
            CircuitState::Closed => Ok(()),
            CircuitState::Open => {
                if current_time >= self.last_failure_timestamp + self.reset_timeout {
                    self.state = CircuitState::HalfOpen;
                    Ok(())
                } else {
                    Err(AiotError::Core(CoreError::Timeout))
                }
            }
            CircuitState::HalfOpen => Ok(()),
        }
    }

    /// Record a successful execution.
    pub fn record_success(&mut self) {
        match self.state {
            CircuitState::Closed => {
                self.failure_count = 0;
            }
            CircuitState::HalfOpen => {
                self.success_count += 1;
                if self.success_count >= self.success_threshold {
                    self.state = CircuitState::Closed;
                    self.failure_count = 0;
                    self.success_count = 0;
                }
            }
            CircuitState::Open => {}
        }
    }

    /// Record a failed execution.
    pub fn record_failure(&mut self, current_time: u64) {
        self.last_failure_timestamp = current_time;
        match self.state {
            CircuitState::Closed => {
                self.failure_count += 1;
                if self.failure_count >= self.failure_threshold {
                    self.state = CircuitState::Open;
                }
            }
            CircuitState::HalfOpen => {
                self.state = CircuitState::Open;
                self.success_count = 0;
            }
            CircuitState::Open => {}
        }
    }
}
