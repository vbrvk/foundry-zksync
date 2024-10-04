use super::{VerifyArgs, ZksyncSourceProvider};
use crate::zk_provider::ZkVerificationContext;
use eyre::{Context, Result};
use foundry_compilers::zksolc::input::StandardJsonCompilerInput;

#[derive(Debug)]
pub struct ZksyncStandardJsonSource;

impl ZksyncSourceProvider for ZksyncStandardJsonSource {
    fn zk_source(
        &self,
        _args: &VerifyArgs,
        context: &ZkVerificationContext,
    ) -> Result<(StandardJsonCompilerInput, String)> {
        let input = foundry_compilers::zksync::project_standard_json_input(
            &context.project,
            &context.target_path,
        )
        .wrap_err("failed to get zksolc standard json")?;

        // Extract the path relative to the project root
        let relative_path = context
            .target_path
            .strip_prefix(context.project.root())
            .unwrap_or(context.target_path.as_path())
            .display()
            .to_string();

        // Ensure the path uses forward slashes consistently (handles Windows paths)
        let normalized_path = relative_path.replace("\\", "/");

        // Format the name as <path>/<file>:<contract_name>
        let name = format!("{}:{}", normalized_path, context.target_name);

        Ok((input, name))
    }
}
