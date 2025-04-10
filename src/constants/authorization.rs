pub struct AuthorizationMessages;
pub struct AuthorizationValidation;

impl AuthorizationMessages {
    pub const UNATHORIZED: &str = "Unathorized";
}

impl AuthorizationValidation {
    pub const TOKEN_PATTERN: &str = r"^[0-9]+:[a-zA-Z0-9]{100}$";
}
