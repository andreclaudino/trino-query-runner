pub fn load_template_from_file(template_source_path: &str) -> anyhow::Result<String> {
    let template_as_string = std::fs::read_to_string(template_source_path)?;

    Ok(template_as_string)
}
