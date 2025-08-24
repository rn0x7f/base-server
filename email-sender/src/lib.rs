pub mod email;

// Re-export main types for easy access
pub use email::{EmailSender, EmailContent};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
