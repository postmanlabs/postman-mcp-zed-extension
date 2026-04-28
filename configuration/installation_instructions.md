# Postman MCP Server — Installation

## Requirements

- [Node.js](https://nodejs.org/) 18 or later (for `npx`)
- A [Postman API key](https://postman.postman.co/settings/me/api-keys)

## Setup

1. **Get your Postman API key**
   Open [Postman API Keys settings](https://postman.postman.co/settings/me/api-keys) and create a new key.

2. **Configure the extension**
   Open the Zed Agent Panel → Settings → Find "Postman MCP Server" and add your key:
   ```json
   {
     "context_servers": {
       "postman": {
         "settings": {
           "postman_api_key": "YOUR_API_KEY"
         }
       }
     }
   }
   ```

3. **Choose a tool configuration** (optional, default is `minimal`):
   - `"minimal"` — essential tools, fastest performance
   - `"full"` — all 100+ Postman API tools
   - `"code"` — API search and client code generation

## What you can do

- Create, update, and run Postman collections
- Manage environments and variables
- Generate and sync API specs
- Search and generate client code from API definitions
- Manage workspaces and collaborate with your team

## EU Region

If your Postman account is in the EU region, use the remote server directly:
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
And set `POSTMAN_API_BASE_URL=https://api.eu.postman.com` in your environment.

## More information

- [Postman MCP Server docs](https://learning.postman.com/docs/postman-ai-services/mcp-server/postman-mcp-server/)
- [GitHub repository](https://github.com/postmanlabs/postman-mcp-server)
