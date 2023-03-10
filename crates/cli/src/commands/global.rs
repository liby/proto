use crate::helpers::{enable_logging, get_global_version_path};
use crate::tools::{create_tool, ToolType};
use log::{info, trace};
use proto_core::{color, ProtoError};
use std::fs;

pub async fn global(tool_type: ToolType, version: String) -> Result<(), ProtoError> {
    enable_logging();

    let mut tool = create_tool(&tool_type)?;

    tool.resolve_version(&version).await?;

    let global_path = get_global_version_path(&tool)?;
    let handle_error = |e: std::io::Error| ProtoError::Fs(global_path.to_path_buf(), e.to_string());

    fs::create_dir_all(global_path.parent().unwrap()).map_err(handle_error)?;
    fs::write(&global_path, tool.get_resolved_version()).map_err(handle_error)?;

    trace!(
        target: "proto:global",
        "Wrote the global version to {}",
        color::path(&global_path),
    );

    info!(
        target: "proto:global",
        "Set the global {} version to {}",
        tool.get_name(),
        tool.get_resolved_version(),
    );

    Ok(())
}
