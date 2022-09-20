/// Validate the struct implementing this trait.
/// Required and used by _all_ generated code.
pub trait Validate {

    /// Return a unit, or a reason this struct is invalid.
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}