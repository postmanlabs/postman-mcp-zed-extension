# Postman MCP Server — Zed Extension

A [Zed](https://zed.dev/) context server extension that connects the Zed AI Agent to your [Postman](https://postman.com) workspace via the [Postman MCP Server](https://github.com/postmanlabs/postman-mcp-server).

## Installation

Install from the [Zed Extensions marketplace](https://zed.dev/extensions?filter=context-servers) by searching for **"Postman MCP Server"**.

## Configuration

Add to your Zed settings (`~/.config/zed/settings.json`):

```json
{
  "context_servers": {
    "postman": {
      "settings": {
        "postman_api_key": "YOUR_API_KEY",
        "tool_config": "minimal"
      }
    }
  }
}
```

Get your API key at [postman.postman.co/settings/me/api-keys](https://postman.postman.co/settings/me/api-keys).

## Tool Configurations

| Config | Description |
|--------|-------------|
| `minimal` (default) | Essential tools for collections, workspaces, environments |
| `full` | All 100+ Postman API tools |
| `code` | API search and client code generation |

## What you can do

- Create, update, and run Postman collections
- Manage environments and variables
- Generate and sync API specs
- Search and generate client code from API definitions
- Manage workspaces and collaborate with your team

## Requirements

- Node.js 18+ (for `npx`)
- A Postman API key

## Links

- [Postman MCP Server](https://github.com/postmanlabs/postman-mcp-server)
- [Postman MCP Server docs](https://learning.postman.com/docs/postman-ai-services/mcp-server/postman-mcp-server/)
- [npm package](https://www.npmjs.com/package/@postman/postman-mcp-server)

## License

Apache-2.0
