//! Probability distributions.

use random::Source;

/// A probability distribution.
pub trait Distribution {
    type Value;

    /// Compute the expected value.
    fn mean(&self) -> f64;

    /// Compute the variance.
    fn var(&self) -> f64;

    /// Compute the standard deviation.
    #[inline]
    fn sd(&self) -> f64 {
        self.var().sqrt()
    }

    /// Compute the skewness.
    fn skewness(&self) -> f64;

    /// Compute the excess kurtosis.
    fn kurtosis(&self) -> f64;

    /// Compute the median.
    fn median(&self) -> f64;

    /// Compute the modes.
    fn modes(&self) -> Vec<Self::Value>;

    /// Compute the differential entropy in nats.
    fn entropy(&self) -> f64;

    /// Compute the cumulative distribution function.
    fn cdf(&self, Self::Value) -> f64;

    /// Compute the inverse of the cumulative distribution function.
    fn inv_cdf(&self, f64) -> Self::Value;

    /// Compute the probability density function.
    fn pdf(&self, Self::Value) -> f64;

    /// Draw a sample.
    fn sample<S>(&self, &mut S) -> Self::Value where S: Source;
}

pub use self::bernoulli::Bernoulli;
pub use self::beta::Beta;
pub use self::binomial::Binomial;
pub use self::categorical::Categorical;
pub use self::exponential::Exponential;
pub use self::gamma::Gamma;
pub use self::gaussian::Gaussian;
pub use self::uniform::Uniform;

mod bernoulli;
mod beta;
mod binomial;
mod categorical;
mod exponential;
mod gamma;
mod gaussian;
mod uniform;
