pub const INSERT_TOKEN: &str = "INSERT INTO tokens (user_id, hash) VALUES (?, ?)";
pub const SELECT_TOKEN: &str = "
    SELECT id FROM tokens
    WHERE user_id = ? AND hash = ? AND datetime(expires_at) > datetime('now')
";
