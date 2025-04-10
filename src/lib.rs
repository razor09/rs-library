pub mod constants {
    pub mod authorization;
    pub mod error;
    pub mod users;
}

pub mod database {
    pub mod adapter;
    pub mod tokens;
    pub mod users;
}

pub mod guards {
    pub mod authorization;
}

pub mod handlers {
    pub mod authorization;
    pub mod authors;
    pub mod routes;
    pub mod users;
}

pub mod models {
    pub mod authorization;
    pub mod config;
    pub mod state;
    pub mod users;
}

pub mod utils {
    pub mod macros;
    pub mod traits;
}
