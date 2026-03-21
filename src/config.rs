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

        let jwt_secret = gen_secret();
        tracing::info!("JWT secret generated (32 chars, alphanumeric)");

        Self {
            database_url: "sqlite:model_hub.db?mode=rwc".into(),
            jwt_secret,
            server_host: host,
            server_port: port,
        }
    }
}

/// Generate a 32-character alphanumeric random secret using OS entropy (getrandom via uuid).
fn gen_secret() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    // Two UUIDs = 32 bytes of OS entropy; map each byte to a charset character.
    let a = uuid::Uuid::new_v4();
    let b = uuid::Uuid::new_v4();
    a.as_bytes()
        .iter()
        .chain(b.as_bytes().iter())
        .map(|&byte| CHARSET[byte as usize % CHARSET.len()] as char)
        .collect()
}

