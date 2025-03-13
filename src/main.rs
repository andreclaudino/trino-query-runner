use std::collections::HashMap;

use clap::Parser;
use trino_query_runner::{
    persistence::load_template_from_file,
    CommandLine, TemplateRenderer, TrinoClient
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    let command_line = CommandLine::parse();

    let template_as_string = load_template_from_file(&command_line.template_path)?;
    let template_renderer = TemplateRenderer::new(&template_as_string)?;
    let trino_client = TrinoClient::new(&command_line.connection_string, &command_line.query_spliter)?;

    let context_parameters = command_line.context_parameters();
    let rendered_template = template_renderer.render(&context_parameters)?;
    trino_client.run_sql::<HashMap<String, String>>(&rendered_template).await?;

    Ok(())
}
