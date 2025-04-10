pub const INSERT_USER: &str = "INSERT INTO users (login, password) VALUES (?, ?)";
pub const SELECT_USER: &str = "SELECT * FROM users WHERE id = ?";
pub const UPDATE_USER: &str = "UPDATE users SET login = COALESCE(?, login), password = COALESCE(?, password) WHERE id = ?";
pub const DELETE_USER: &str = "DELETE FROM users WHERE id = ?";
pub const SELECT_USER_ID: &str = "SELECT id FROM users WHERE login = ? AND password = ?";
