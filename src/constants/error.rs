pub struct ErrorMessages;

impl ErrorMessages {
    pub const BIND_SERVER: &str = "Failed to bind server";
    pub const OPEN_CONFIG: &str = "Unable to open config file";
    pub const READ_CONFIG: &str = "Unable to read config file";
    pub const OPEN_DB_CONNECTION: &str = "Failed to opening database connection";
    pub const READ_SQL: &str = "Failed to read SQL file";
    pub const INITIALIZE_DB: &str = "Failed to initialize database";
    pub const CREATE_REGEXP: &str = "Failed to create regular expression";
    pub const LOCK_DB_CONNECTION: &str = "Failed to lock database connection";
    pub const PREPARE_STATEMENT: &str = "Failed to prepare statement";
    pub const QUERY_EXECUTION: &str = "Query execution failed";
}
