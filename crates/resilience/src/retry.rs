#[derive(Clone, Copy, Debug, PartialEq)]
/// Documentation for BackoffStrategy.
pub enum BackoffStrategy {
    Constant(u64),
    Exponential {
        initial: u64,
        multiplier: f32,
        max: u64,
    },
}

/// A simple Retry Policy generator.
pub struct RetryPolicy {
    /// Documentation for field `max_retries`.
    pub max_retries: u32,
    /// Documentation for field `strategy`.
    pub strategy: BackoffStrategy,
    /// Documentation for field `jitter_max`.
    pub jitter_max: u64,
}

impl RetryPolicy {
    #[must_use]
    /// Documentation for fn.
    pub const fn new(max_retries: u32, strategy: BackoffStrategy, jitter_max: u64) -> Self {
        Self {
            max_retries,
            strategy,
            jitter_max,
        }
    }

    /// Calculate the delay for a given retry attempt (0-indexed).
    #[must_use]
    pub fn calculate_delay(&self, attempt: u32, pseudo_random_jitter: u64) -> u64 {
        if attempt >= self.max_retries {
            return 0; // Or indicate failure
        }

        let base_delay = match self.strategy {
            BackoffStrategy::Constant(delay) => delay,
            BackoffStrategy::Exponential {
                initial,
                multiplier,
                max,
            } => {
                // For no_std, we avoid powf and just multiply iteratively or use simple math
                let mut delay = initial as f32;
                for _ in 0..attempt {
                    delay *= multiplier;
                }
                let mut delay_u64 = delay as u64;
                if delay_u64 > max {
                    delay_u64 = max;
                }
                delay_u64
            }
        };

        let jitter = if self.jitter_max > 0 {
            pseudo_random_jitter % self.jitter_max
        } else {
            0
        };

        base_delay + jitter
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Documentation for RetryClassification.
pub enum RetryClassification {
    Transient,
    Permanent,
    Timeout,
}

/// Documentation for RetryBudget.
pub struct RetryBudget {
    /// Documentation for field `max_tokens`.
    pub max_tokens: u32,
    /// Documentation for field `current_tokens`.
    pub current_tokens: u32,
    /// Documentation for field `token_regen_rate`.
    pub token_regen_rate: u32,
}

impl RetryBudget {
    #[must_use]
    /// Documentation for new.
    pub fn new(max_tokens: u32, token_regen_rate: u32) -> Self {
        Self {
            max_tokens,
            current_tokens: max_tokens,
            token_regen_rate,
        }
    }

    /// Documentation for try_consume.
    pub fn try_consume(&mut self) -> bool {
        if self.current_tokens > 0 {
            self.current_tokens -= 1;
            true
        } else {
            false
        }
    }

    /// Documentation for replenish.
    pub fn replenish(&mut self) {
        let new_tokens = self.current_tokens.saturating_add(self.token_regen_rate);
        self.current_tokens = core::cmp::min(self.max_tokens, new_tokens);
    }
}
