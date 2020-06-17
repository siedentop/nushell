use crate::commands::classified::block::run_block;
use crate::commands::WholeStreamCommand;
use crate::context::CommandRegistry;
use crate::prelude::*;

use futures::stream::once;

use nu_errors::ShellError;
use nu_protocol::{CallInfo, ReturnSuccess, Scope, Signature, SyntaxShape, UntaggedValue, Value};

pub struct Do;

impl WholeStreamCommand for Do {
    fn name(&self) -> &str {
        "do"
    }

    fn signature(&self) -> Signature {
        Signature::build("do").required("block",
         SyntaxShape::Block, "A block as line continuation.")
    }

    fn usage(&self) -> &str {
        "Run a block which might span multiple lines"
    }
    fn run(
        &self,
        args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        let stream =
            async_stream! {  yield Err(ShellError::untagged_runtime_error("Not Implemented Yet")) };

        Ok(stream.to_output_stream())
    }
}
