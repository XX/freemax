use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Opts {
    /// Путь к файлу конфигурации (по умолчанию — `config.toml` в текущем или родительском каталоге).
    #[clap(short, long)]
    pub config: Option<PathBuf>,
}
