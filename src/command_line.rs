use std::collections::HashMap;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLine {
    /// Path for the query template
    #[arg(long, env)]
    pub template_path: String,

    /// Connection string for Trino formated as trino://<user>:<password>@<host>:<port>/<catalog>
    #[arg(long, env)]
    pub connection_string: String,

    /// Symbol to split queries in template
    #[arg(long, env, default_value = "---*---")]
    pub query_spliter: String,

    /// Parameters in the format -p chave1=valor1 -p chave2=valor2
    #[arg(short = 'p', long = "parameter", use_value_delimiter = true, value_delimiter = ',')]
    parameters: Vec<String>,    
}

impl CommandLine {
    pub fn context_parameters(&self) -> HashMap<String, String> {
        self.parameters.iter()
            .map(|param| {
                let parts: Vec<&str> = param.split('=').collect();
                (parts[0].to_string(), parts[1].to_string())
            })
            .collect()
    }
}
