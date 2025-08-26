# mcp-server-tavily

Zed extension that wraps the [`tavily-mcp`](https://www.npmjs.com/package/tavily-mcp) MCP server.

## Configuration

This MCP server requires an API key.

1. Sign up for a [Tavily API account](https://tavily.com)
2. Choose a plan (Free tier available with 1,000 queries/month)
3. Generate your API key [from the dashboard](https://app.tavily.com/home)

In your Zed settings:
```json
{
    "context_servers": {
        "mcp-server-tavily": {
          "settings": {
              "tavily_api_key": "YOUR_API_KEY"
          }
        }
    }
}

```
