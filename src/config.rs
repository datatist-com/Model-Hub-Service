#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_host: String,
    pub server_port: u16,
}

impl AppConfig {
    pub fn from_args() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let mut host = "0.0.0.0".to_string();
        let mut port: u16 = 8080;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--host" => {
                    if let Some(v) = args.get(i + 1) {
                        host = v.clone();
                    }
                    i += 2;
                }
                "--port" => {
                    if let Some(v) = args.get(i + 1) {
                        port = v.parse().expect("--port must be a valid number");
                    }
                    i += 2;
                }
                _ => i += 1,
            }
        }

        let jwt_secret = uuid::Uuid::new_v4().to_string();
        tracing::info!("JWT secret auto-generated for this session");

        Self {
            database_url: "sqlite:model_hub.db?mode=rwc".into(),
            jwt_secret,
            server_host: host,
            server_port: port,
        }
    }
}
