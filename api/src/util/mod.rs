mod auth;

pub use auth::*;

use crate::error::InputValidationError;

/// A trait that denotes that this type represents some user input that needs
/// to be validated, and in the process of validation will be converted to
/// some other output type. Typically, any type that implements this trait is
/// **always** considered invalid, and should generally be validated as soon as
/// possible in the API chain. Then, the validated version of the type can be
/// used internally and we know for sure that the value is valid.
///
/// Valid structs should _only_ be constructable via this trait implementation,
/// to prevent sidestepping validation by directly creating the "valid" value.
pub trait Validate: Sized {
    type Output;

    /// Validate the user-provided value. If it's valid, return the validated
    /// form of it. If not, return a validation error.
    ///
    /// `field` represents the GraphQL field that is being validated. This is
    /// used in the event of an error, to tell the user which field was invalid.
    fn validate(
        self,
        field: &str,
    ) -> Result<Self::Output, InputValidationError>;
}
