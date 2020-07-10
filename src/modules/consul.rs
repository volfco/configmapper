use consul;
use std::env;
use handlebars::{Handlebars, Output, RenderContext, Context, Helper, RenderError, JsonRender};
use consul::kv::KV;
use consul::catalog::Catalog;

pub fn get_consul(helper: &Helper, _: &Handlebars, ctx: &Context, _: &mut RenderContext, out: &mut dyn Output) -> Result<(), RenderError> {
    let key = helper.param(0).and_then(|ref v| v.value().as_str())
        .ok_or(RenderError::new("Param 0 with string type is required for rank helper."))? as &str;

    let default = helper.param(1).and_then(|ref v| v.value().as_str());

    let config = consul::Config::new();
    let mut config = config.unwrap();
    let client = consul::Client::new(config);

    let lookup = client.get(key, None);
    if lookup.is_err() {
        let err = format!("error occurred during consul lookup. {:?}", lookup.err().unwrap());
        if default.is_some() {
            warn!("{}", err);
            out.write(default.unwrap().as_ref())?;
            return Ok(());
        } else {
            return Err(RenderError::new(err));
        }
    }
    let kv = lookup.unwrap().0;
    let inner = if kv.is_none() {
        if default.is_some() {
            warn!("{} does not exist in consul, using provided default", key);
            out.write(default.unwrap().as_ref())?;
        } else {
            out.write("".as_ref())?;
        }
        return Ok(())

    } else {
        kv.unwrap().Value
    };

    let decoder = base64::decode(&inner);
    if decoder.is_err() {
        if default.is_some() {
            warn!("unable to decode {}, using provided default", key);
            out.write(default.unwrap().as_ref())?;
        } else {
            out.write("".as_ref())?;
        }
        return Ok(())
    }

    let decoded = String::from_utf8(decoder.unwrap());
    if decoded.is_err() {
        warn!("decode failed for {}. {:?}", key, decoded.err().unwrap());
        if default.is_some() {
            warn!("unable to decode {}, using provided default", key);
            out.write(default.unwrap().as_ref())?;
        } else {
            out.write("".as_ref())?;
        }
        return Ok(())
    }
    out.write(decoded.unwrap().as_ref())?;
    return Ok(());
}

pub fn get_consul_service(helper: &Helper, _: &Handlebars, ctx: &Context, _: &mut RenderContext, out: &mut dyn Output) -> Result<(), RenderError> {
    let name = helper.param(0).and_then(|ref v| v.value().as_str())
        .ok_or(RenderError::new("Param 0 with string type is required for rank helper."))? as &str;

    let tags_src: Vec<&str>= helper.param(1).and_then(|ref v| v.value().as_str())
        .ok_or(RenderError::new("Param 2 with string type is required for consul_service."))?.split(",").collect::<Vec<&str>>();

    let config = consul::Config::new();
    let mut config = config.unwrap();
    let client = consul::Client::new(config);


    client.services()


    return Ok(());
}