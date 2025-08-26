use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use zed_extension_api::{
    serde_json, settings::ContextServerSettings, Command, ContextServerConfiguration,
    ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "tavily-mcp";
const PACKAGE_VERSION: &str = "0.2.9";
const SERVER_PATH: &str = "node_modules/tavily-mcp/build/index.js";
const CONTEXT_SERVER_ID: &str = "mcp-server-tavily";
struct TavilyContextServerExtension;

#[derive(Debug, Deserialize, JsonSchema)]
struct TavilyContextServerSettings {
    tavily_api_key: String,
}

impl zed_extension_api::Extension for TavilyContextServerExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        assert_eq!(
            context_server_id.as_ref(),
            CONTEXT_SERVER_ID,
            "Unexpected context server ID"
        );
        let version = zed_extension_api::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed_extension_api::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }
        let settings = ContextServerSettings::for_project(CONTEXT_SERVER_ID, project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `tavily_api_key` setting".into());
        };
        let settings: TavilyContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        Ok(Command {
            command: zed_extension_api::node_binary_path()?,
            args: vec![env::current_dir()
                .unwrap()
                .join(SERVER_PATH)
                .to_string_lossy()
                .to_string()],
            env: vec![("TAVILY_API_KEY".into(), settings.tavily_api_key)],
        })
    }

    fn context_server_configuration(
        &mut self,
        context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        assert_eq!(
            context_server_id.as_ref(),
            CONTEXT_SERVER_ID,
            "Unexpected context server ID"
        );
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(TavilyContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed_extension_api::register_extension!(TavilyContextServerExtension);
