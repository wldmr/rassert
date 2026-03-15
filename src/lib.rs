mod core;
mod expectations;

pub use crate::core::{
    start_expectations, Expectation, ExpectationChain, ExpressionUnderTest, MultipleExpectations,
    SourceLocation,
};

pub mod prelude {
    pub use super::expect;
    pub use super::expect_matches;
    pub use super::start_expectations;

    pub use super::expectations::*;
}
