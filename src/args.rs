use clap:: {Parser};

#[derive(Parser, Clone, Copy)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Maximum number of entries in a device consumption history
    #[arg(short('m'), long, default_value = "5")]
    pub max_entries: usize,

    /// Interval for updating device consumption history
    #[arg(short('r'), long, value_name = "seconds", default_value = "60")]
    pub update_interval: u64,

    /// Port for application to listen on
    #[arg(short('p'), long, value_name = "port", default_value = "3000")]
    pub port: u16,

}