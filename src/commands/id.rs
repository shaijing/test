use clap::Subcommand;

#[derive(Subcommand)]
pub enum IdCommand {
    /// Generate ULID using ferroid
    Ulid {
        /// Number of IDs to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        /// Output in JSON format
        #[arg(short, long)]
        json: bool,
    },
    /// Generate UUID v4
    Uuid {
        /// Number of IDs to generate
        #[arg(short, long, default_value = "1")]
        count: u32,
        /// Output in JSON format
        #[arg(short, long)]
        json: bool,
    },
}

impl IdCommand {
    pub fn execute(&self) {
        match self {
            IdCommand::Ulid { count, json } => generate_ulids(*count, *json),
            IdCommand::Uuid { count, json } => generate_uuids(*count, *json),
        }
    }
}

fn generate_ulids(count: u32, json: bool) {
    use ulid::Ulid;
    let ids: Vec<String> = (0..count).map(|_| Ulid::new().to_string()).collect();
    output_ids(&ids, json);
}

fn generate_uuids(count: u32, json: bool) {
    use uuid::Uuid;
    let ids: Vec<String> = (0..count).map(|_| Uuid::new_v4().to_string()).collect();
    output_ids(&ids, json);
}

fn output_ids(ids: &[String], json: bool) {
    if json {
        println!("{}", serde_json::to_string_pretty(ids).unwrap_or_else(|e| e.to_string()));
    } else {
        for id in ids {
            println!("{}", id);
        }
    }
}