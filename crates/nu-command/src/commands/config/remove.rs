use crate::prelude::*;
use nu_engine::WholeStreamCommand;
use nu_errors::ShellError;
use nu_protocol::{Primitive, ReturnSuccess, Signature, SyntaxShape, UntaggedValue, Value};
use nu_source::Tagged;

pub struct SubCommand;

#[derive(Deserialize)]
pub struct Arguments {
    remove: Tagged<String>,
}

impl WholeStreamCommand for SubCommand {
    fn name(&self) -> &str {
        "config remove"
    }

    fn signature(&self) -> Signature {
        Signature::build("config remove").required(
            "remove",
            SyntaxShape::Any,
            "remove a value from the config",
        )
    }

    fn usage(&self) -> &str {
        "Removes a value from the config"
    }

    fn run(&self, args: CommandArgs) -> Result<OutputStream, ShellError> {
        remove(args)
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Remove the startup commands",
            example: "config remove startup",
            result: None,
        }]
    }
}

pub fn remove(args: CommandArgs) -> Result<OutputStream, ShellError> {
    let name_span = args.call_info.name_tag.clone();
    let scope = args.scope.clone();
    let (Arguments { remove }, _) = args.process()?;

    let path = match scope.get_var("config-path") {
        Some(Value {
            value: UntaggedValue::Primitive(Primitive::FilePath(path)),
            ..
        }) => Some(path),
        _ => nu_data::config::default_path().ok(),
    };

    let mut result = nu_data::config::read(name_span, &path)?;

    let key = remove.to_string();

    if result.contains_key(&key) {
        result.swap_remove(&key);
        config::write(&result, &path)?;
        Ok(vec![ReturnSuccess::value(
            UntaggedValue::Row(result.into()).into_value(remove.tag()),
        )]
        .into_iter()
        .to_output_stream())
    } else {
        Err(ShellError::labeled_error(
            "Key does not exist in config",
            "key",
            remove.tag(),
        ))
    }
}
