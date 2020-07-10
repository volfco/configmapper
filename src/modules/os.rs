use std::env;
use handlebars::{Handlebars, Output, RenderContext, Context, Helper, RenderError, JsonRender};

/// get_env function
pub fn get_env(helper: &Helper, _: &Handlebars, ctx: &Context, _: &mut RenderContext, out: &mut dyn Output) -> Result<(), RenderError> {
    let param = helper.param(0).and_then(|ref v| v.relative_path())
        .ok_or(RenderError::new("Param 0 with string type is required for rank helper."))? as &str;

    let default = helper.param(1).and_then(|ref v| v.relative_path());

    let value = match env::var(param) {
        Ok(value) => value,
        _                => {
            // check if we have a default value
            if default.is_some() {
                // if so, let's use that value
                default.unwrap().clone()
            } else {
                // no default value, so return an error
                return Err(RenderError::new(
                    format!("`{}` does not exist in the environment variables", param)
                ))
            }
        }
    };

    out.write(value.as_ref())?;
    Ok(())
}
