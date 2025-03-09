// Re-export all public items from submodules
pub use self::password::{hash_password, verify_password, validate_password};
pub use self::validation::{validate_email, validate_phone_number, validate_username, validate_full_name};
pub use self::roles::validate_role;
pub use self::jwt::{generate_token, verify_token, extract_token_from_header};
pub mod oauth;
pub mod jwt;

// Define submodules
mod password;
mod validation;
mod roles;