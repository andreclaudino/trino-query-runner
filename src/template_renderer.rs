use minijinja::{AutoEscape, Environment};
use serde::Serialize;

const DEFAULT_TEMPLATE_NAME: &str = "default";

pub struct TemplateRenderer<'source> {
    environment: Environment<'source>,
}

impl<'source> TemplateRenderer<'source> {

    pub fn new(template_as_string: &str) -> anyhow::Result<TemplateRenderer> {
        let mut environment = Environment::new();
        environment.set_auto_escape_callback(|_| AutoEscape::None);

        environment.add_template(DEFAULT_TEMPLATE_NAME, &template_as_string)?;

        let renderer = 
            TemplateRenderer {
                environment,
            };

        Ok(renderer)
    }

    pub fn render<S>(&self, context: S) -> anyhow::Result<String> where S: Serialize {
        let template = self.environment.get_template(DEFAULT_TEMPLATE_NAME)?;
        let rendered = template.render(context)?;

        Ok(rendered)
    }
}

