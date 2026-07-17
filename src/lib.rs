use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "@postman/postman-mcp-server";
const SERVER_PATH: &str = "node_modules/@postman/postman-mcp-server/dist/src/index.js";

#[derive(Debug, Deserialize, JsonSchema)]
struct PostmanSettings {
    /// Your Postman API key.
    /// Get one at: https://postman.postman.co/settings/me/api-keys
    postman_api_key: String,

    /// Toolset to load.
    /// - "minimal" (default): essential tools, fastest performance
    /// - "full": all 100+ Postman API tools
    /// - "code": API search and client code generation tools
    #[serde(default = "default_toolset")]
    toolset: Toolset,

    /// Whether to send anonymous usage telemetry to Postman.
    /// Enabled by default. Set to false to opt out.
    #[serde(rename = "telemetryEnabled", default = "default_telemetry_enabled")]
    telemetry_enabled: bool,
}

fn default_telemetry_enabled() -> bool {
    true
}

#[derive(Debug, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "lowercase")]
enum Toolset {
    #[default]
    Minimal,
    Full,
    Code,
}

fn default_toolset() -> Toolset {
    Toolset::Minimal
}

struct PostmanExtension;

impl zed::Extension for PostmanExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let latest_version = zed::npm_package_latest_version(PACKAGE_NAME)?;
        let installed_version = zed::npm_package_installed_version(PACKAGE_NAME)?;

        if installed_version.as_deref() != Some(latest_version.as_ref()) {
            zed::npm_install_package(PACKAGE_NAME, &latest_version)?;
        }

        let settings = ContextServerSettings::for_project("postman", project)?;
        let Some(settings) = settings.settings else {
            return Err(
                "Postman API key not set. Open the Postman extension's Configure panel \
                 and paste your key into postman_api_key. \
                 Get a key at https://postman.postman.co/settings/me/api-keys"
                    .into(),
            );
        };
        let settings: PostmanSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        if settings.postman_api_key.trim().is_empty() {
            return Err(
                "postman_api_key is required. Open the Postman extension's Configure panel \
                 and paste your key into postman_api_key. \
                 Get a key at https://postman.postman.co/settings/me/api-keys"
                    .into(),
            );
        }

        let tool_flag = match settings.toolset {
            Toolset::Full => "--full",
            Toolset::Code => "--code",
            Toolset::Minimal => "--minimal",
        };

        let node_path = zed::node_binary_path()?;
        let server_path = env::current_dir()
            .unwrap()
            .join(SERVER_PATH)
            .to_string_lossy()
            .to_string();

        Ok(Command {
            command: node_path,
            args: vec![server_path, tool_flag.into()],
            env: vec![
                ("POSTMAN_API_KEY".into(), settings.postman_api_key),
                (
                    "POSTMAN_MCP_TELEMETRY".into(),
                    settings.telemetry_enabled.to_string(),
                ),
            ],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings =
            include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema = serde_json::to_string(&schemars::schema_for!(PostmanSettings))
            .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(PostmanExtension);
