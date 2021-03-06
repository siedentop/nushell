use crate::commands::PerItemCommand;
use crate::data::command_dict;

use crate::prelude::*;
use nu_errors::ShellError;
use nu_protocol::{
    CallInfo, NamedType, PositionalType, Primitive, ReturnSuccess, Signature, SyntaxShape,
    TaggedDictBuilder, UntaggedValue, Value,
};
use nu_source::SpannedItem;
use nu_value_ext::get_data_by_key;

pub struct Help;

impl PerItemCommand for Help {
    fn name(&self) -> &str {
        "help"
    }

    fn signature(&self) -> Signature {
        Signature::build("help").rest(SyntaxShape::Any, "the name of command(s) to get help on")
    }

    fn usage(&self) -> &str {
        "Display help information about commands."
    }

    fn run(
        &self,
        call_info: &CallInfo,
        registry: &CommandRegistry,
        _raw_args: &RawCommandArgs,
        _input: Value,
    ) -> Result<OutputStream, ShellError> {
        let tag = &call_info.name_tag;

        match call_info.args.nth(0) {
            Some(Value {
                value: UntaggedValue::Primitive(Primitive::String(document)),
                tag,
            }) => {
                let mut help = VecDeque::new();
                if document == "commands" {
                    let mut sorted_names = registry.names();
                    sorted_names.sort();
                    for cmd in sorted_names {
                        let mut short_desc = TaggedDictBuilder::new(tag.clone());
                        let value = command_dict(
                            registry.get_command(&cmd).ok_or_else(|| {
                                ShellError::labeled_error(
                                    format!("Could not load {}", cmd),
                                    "could not load command",
                                    tag,
                                )
                            })?,
                            tag.clone(),
                        );

                        short_desc.insert_untagged("name", cmd);
                        short_desc.insert_untagged(
                            "description",
                            get_data_by_key(&value, "usage".spanned_unknown())
                                .ok_or_else(|| {
                                    ShellError::labeled_error(
                                        "Expected a usage key",
                                        "expected a 'usage' key",
                                        &value.tag,
                                    )
                                })?
                                .as_string()?,
                        );

                        help.push_back(ReturnSuccess::value(short_desc.into_value()));
                    }
                } else if let Some(command) = registry.get_command(document) {
                    return Ok(
                        get_help(&command.name(), &command.usage(), command.signature()).into(),
                    );
                } else {
                    return Err(ShellError::labeled_error(
                        "Can't find command (use 'help commands' for full list)",
                        "can't find command",
                        tag,
                    ));
                }
                let help = futures::stream::iter(help);
                Ok(help.to_output_stream())
            }
            _ => {
                let msg = r#"Welcome to Nushell.

Here are some tips to help you get started.
  * help commands - list all available commands
  * help <command name> - display help about a particular command

Nushell works on the idea of a "pipeline". Pipelines are commands connected with the '|' character.
Each stage in the pipeline works together to load, parse, and display information to you.

[Examples]

List the files in the current directory, sorted by size:
    ls | sort-by size

Get information about the current system:
    sys | get host

Get the processes on your system actively using CPU:
    ps | where cpu > 0

You can also learn more at https://www.nushell.sh/book/"#;

                let output_stream = futures::stream::iter(vec![ReturnSuccess::value(
                    UntaggedValue::string(msg).into_value(tag),
                )]);

                Ok(output_stream.to_output_stream())
            }
        }
    }
}

pub(crate) fn get_help(
    cmd_name: &str,
    cmd_usage: &str,
    cmd_sig: Signature,
) -> impl Into<OutputStream> {
    let mut help = VecDeque::new();
    let mut long_desc = String::new();

    long_desc.push_str(&cmd_usage);
    long_desc.push_str("\n");

    let signature = cmd_sig;

    let mut one_liner = String::new();
    one_liner.push_str(&signature.name);
    one_liner.push_str(" ");

    for positional in &signature.positional {
        match &positional.0 {
            PositionalType::Mandatory(name, _m) => {
                one_liner.push_str(&format!("<{}> ", name));
            }
            PositionalType::Optional(name, _o) => {
                one_liner.push_str(&format!("({}) ", name));
            }
        }
    }

    if signature.rest_positional.is_some() {
        one_liner.push_str(" ...args");
    }

    if !signature.named.is_empty() {
        one_liner.push_str("{flags} ");
    }

    long_desc.push_str(&format!("\nUsage:\n  > {}\n", one_liner));

    if !signature.positional.is_empty() || signature.rest_positional.is_some() {
        long_desc.push_str("\nparameters:\n");
        for positional in signature.positional {
            match positional.0 {
                PositionalType::Mandatory(name, _m) => {
                    long_desc.push_str(&format!("  <{}> {}\n", name, positional.1));
                }
                PositionalType::Optional(name, _o) => {
                    long_desc.push_str(&format!("  ({}) {}\n", name, positional.1));
                }
            }
        }

        if let Some(rest_positional) = signature.rest_positional {
            long_desc.push_str(&format!("  ...args: {}\n", rest_positional.1));
        }
    }
    if !signature.named.is_empty() {
        long_desc.push_str("\nflags:\n");
        for (flag, ty) in signature.named {
            let msg = match ty.0 {
                NamedType::Switch(s) => {
                    if let Some(c) = s {
                        format!(
                            "  -{}, --{}{} {}\n",
                            c,
                            flag,
                            if !ty.1.is_empty() { ":" } else { "" },
                            ty.1
                        )
                    } else {
                        format!(
                            "  --{}{} {}\n",
                            flag,
                            if !ty.1.is_empty() { ":" } else { "" },
                            ty.1
                        )
                    }
                }
                NamedType::Mandatory(s, m) => {
                    if let Some(c) = s {
                        format!(
                            "  -{}, --{} <{}> (required parameter){} {}\n",
                            c,
                            flag,
                            m.display(),
                            if !ty.1.is_empty() { ":" } else { "" },
                            ty.1
                        )
                    } else {
                        format!(
                            "  --{} <{}> (required parameter){} {}\n",
                            flag,
                            m.display(),
                            if !ty.1.is_empty() { ":" } else { "" },
                            ty.1
                        )
                    }
                }
                NamedType::Optional(s, o) => {
                    if let Some(c) = s {
                        format!(
                            "  -{}, --{} <{}>{} {}\n",
                            c,
                            flag,
                            o.display(),
                            if !ty.1.is_empty() { ":" } else { "" },
                            ty.1
                        )
                    } else {
                        format!(
                            "  --{} <{}>{} {}\n",
                            flag,
                            o.display(),
                            if !ty.1.is_empty() { ":" } else { "" },
                            ty.1
                        )
                    }
                }
            };
            long_desc.push_str(&msg);
        }
    }

    help.push_back(ReturnSuccess::value(
        UntaggedValue::string(long_desc).into_value(Tag::from((0, cmd_name.len(), None))),
    ));
    help
}
