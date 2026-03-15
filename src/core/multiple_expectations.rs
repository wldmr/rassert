use super::ExpectationChain;

/// Abstracts over different [`ExpectationChain`]s.
pub(crate) trait Conclude {
    fn conclude(&mut self) -> Result<(), String>;
}

impl<'a, T> Conclude for ExpectationChain<'a, T> {
    fn conclude(&mut self) -> Result<(), String> {
        ExpectationChain::conclude(self)
    }
}

/// Checks multiple expectations
#[must_use = "This doesn't do anything without calling a `conclude_*()` method"]
pub struct MultipleExpectations<'a> {
    pub(crate) many: Vec<Box<dyn Conclude + 'a>>,
    pub(crate) panic_on_drop: bool,
}

/// If your test needs to check more than one expectation.
///
/// Fluently build up expectations with [`.and()`][MultipleExpectations::and] or
/// stepwise expectations with [`.now()`][MultipleExpectations::now].
pub fn start_expectations<'a>() -> MultipleExpectations<'a> {
    MultipleExpectations {
        many: Vec::new(),
        panic_on_drop: true,
    }
}

impl<'a> MultipleExpectations<'a> {
    /// Fluently adds an expectation.
    ///
    /// ```rust
    /// # use rassert::prelude::*;
    /// # fn main() {
    /// let numbers = vec![1, 2, 3];
    /// start_expectations()
    ///     .and(expect!(&numbers).to_have_length(3))
    ///     .and(expect!(&numbers[0]).to_be(&1))
    ///     .and(expect!(&numbers[1]).to("be even", |it| it % 2 == 0))
    ///     .and(expect!(&numbers[2]).to_be(&3))
    ///     .conclude_panic();
    /// # }
    /// ```
    pub fn and<T>(mut self, chain: ExpectationChain<'a, T>) -> Self {
        self.here(chain);
        self
    }

    /// Mutably adds an expectation.
    ///
    /// Use this style if you want to check intermediate results, but keep the
    /// “important” check at the end, so that it is nice and isolated.
    ///
    /// ``` rust
    /// # use rassert::prelude::*;
    /// # fn main() {
    /// let mut checks = start_expectations();
    ///
    /// let is_even = |n: &u8| (*n) % 2 == 0;
    ///
    /// let a = [1, 2].iter().sum();
    /// checks.here(expect!(&a).not().to("be even", is_even));
    ///
    /// let b = [3, 6].iter().sum();
    /// checks.here(expect!(&b).not().to("be even", is_even));
    ///
    /// // This this the important one!
    /// let sum_of_two_odd_numbers = a + b;
    /// checks.here(expect!(&sum_of_two_odd_numbers)
    ///     .to("be even", is_even));
    ///
    /// checks.conclude_panic();
    /// # }
    /// ```
    pub fn here<T>(&mut self, chain: ExpectationChain<'a, T>) -> &mut Self {
        self.many.push(Box::new(chain));
        self
    }

    pub fn conclude_panic(self) {
        if let Err(err) = self.conclude_result() {
            panic!("{}", err);
        }
    }

    pub fn conclude_result(mut self) -> Result<(), String> {
        self.panic_on_drop = false;
        let msg = self
            .many
            .iter_mut()
            .filter_map(|it| it.conclude().err())
            .fold(String::new(), |mut acc, err| {
                acc.push_str(&err);
                acc
            });
        if msg.trim().is_empty() {
            Ok(())
        } else {
            Err(msg)
        }
    }
}
