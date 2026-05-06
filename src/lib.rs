use schemars::JsonSchema;
use serde::Deserialize;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const NPM_PACKAGE: &str = "@postman/postman-mcp-server";
const NPM_PACKAGE_VERSION: &str = "2.8.7";

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

// Common npx locations for macOS (Homebrew, nvm, system) and Linux.
// Zed GUI apps don't inherit the shell PATH, so we probe known paths.
const NPX_CANDIDATE_PATHS: &[&str] = &[
    "/opt/homebrew/bin/npx",
    "/usr/local/bin/npx",
    "/usr/bin/npx",
    // nvm default locations
    "/Users/Shared/.nvm/versions/node/default/bin/npx",
];

fn find_npx() -> String {
    for path in NPX_CANDIDATE_PATHS {
        if std::fs::metadata(path).is_ok() {
            return path.to_string();
        }
    }
    "npx".to_string()
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
        let settings = ContextServerSettings::for_project("postman", project)?;
        let Some(settings) = settings.settings else {
            return Err(
                "Set postman_api_key in the Postman MCP Server extension settings. \
                 Get a key at https://postman.postman.co/settings/me/api-keys"
                    .into(),
            );
        };
        let settings: PostmanSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        if settings.postman_api_key.trim().is_empty() {
            return Err(
                "postman_api_key is required. Get a key at https://postman.postman.co/settings/me/api-keys".into(),
            );
        }

        let tool_flag = match settings.toolset {
            Toolset::Full => "--full",
            Toolset::Code => "--code",
            Toolset::Minimal => "--minimal",
        };

        let npx = find_npx();

        Ok(Command {
            command: npx,
            args: vec![
                "-y".into(),
                format!("{}@{}", NPM_PACKAGE, NPM_PACKAGE_VERSION),
                tool_flag.into(),
            ],
            env: vec![("POSTMAN_API_KEY".into(), settings.postman_api_key)],
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
