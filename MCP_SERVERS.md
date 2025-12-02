# MCP Servers Configuration

This project is configured with the following Model Context Protocol (MCP) servers to enhance AI assistance capabilities.

## Configured Servers

### 1. Sequential Thinking MCP
**Package**: `@modelcontextprotocol/server-sequential-thinking`

Enables structured, step-by-step problem-solving through a dynamic thinking process.

**Features**:
- Breaks down complex problems into manageable steps
- Revises and refines thoughts as understanding deepens
- Branches into alternative reasoning paths
- Adjusts thought process dynamically

**Use Cases**: System design planning, architectural decisions, refactoring strategies

---

### 2. Puppeteer MCP
**Package**: `@modelcontextprotocol/server-puppeteer`

Provides browser automation capabilities using Puppeteer.

**Features**:
- Navigate websites programmatically
- Take screenshots
- Interact with web pages
- Execute JavaScript in browser context

**Use Cases**: UI testing, web scraping, automated browser interactions

---

### 3. Memory Bank MCP
**Package**: `@memory-bank/mcp`

Organizes project knowledge hierarchically for better context retention.

**Features**:
- Persistent storage using file-system approach
- Branch-specific memory banks for isolated feature work
- Global memory bank for project-wide information
- Structured JSON-based document format

**Use Cases**: Complex projects requiring context preservation across sessions

---

### 4. Playwright MCP
**Package**: `@playwright/mcp` (Official Microsoft implementation)

Modern, feature-rich browser automation using Playwright.

**Features**:
- Cross-browser testing (Chromium, Firefox, WebKit)
- Accessibility tree-based interaction (fast & lightweight)
- Screenshot capture
- JavaScript execution
- Console log monitoring

**Use Cases**: Cross-browser testing, advanced web automation

---

### 5. GitHub MCP
**Package**: `@modelcontextprotocol/server-github`

Provides GitHub API access for repository management.

**Features**:
- File operations on repositories
- Repository management
- Search functionality (code, issues, PRs, users)
- Automatic branch creation
- Batch operations

**Setup Required**:
Set the `GITHUB_TOKEN` environment variable with your GitHub Personal Access Token.

```bash
export GITHUB_TOKEN="your_github_personal_access_token"
```

**Use Cases**: Repository management, code search, issue tracking

**Note**: This package is deprecated. Check [github/github-mcp-server](https://github.com/github/github-mcp-server) for the actively maintained version.

---

### 6. Knowledge Graph Memory MCP
**Package**: `mcp-knowledge-graph`

Creates persistent memory using a local knowledge graph.

**Features**:
- Stores entities, relations, and observations
- Persistent memory across conversations
- Works with any MCP-compatible AI platform
- Local data storage for privacy

**Use Cases**: Maintaining context across sessions, building project knowledge base

---

### 7. DuckDuckGo MCP
**Package**: `duckduckgo-mcp-server`

Lightweight web search capabilities without API keys.

**Features**:
- Web search through DuckDuckGo
- Content fetching and parsing
- No API key required
- Privacy-focused

**Use Cases**: Accessing current documentation, error solutions, up-to-date information

---

### 8. MCP Compass
**Package**: `@liuyoshio/mcp-compass`

Discovery and recommendation service for MCP servers.

**Features**:
- Smart search using natural language queries
- Rich metadata about MCP services
- Real-time updates on available services
- Easy integration

**Use Cases**: Discovering appropriate MCP servers for specific tasks

---

## Setup Instructions

### 1. Environment Variables

For the GitHub MCP server, set up your token:

```bash
# Add to your shell profile (~/.zshrc, ~/.bashrc, etc.)
export GITHUB_TOKEN="your_personal_access_token"
```

To create a GitHub Personal Access Token:
1. Go to GitHub Settings > Developer settings > Personal access tokens > Tokens (classic)
2. Click "Generate new token (classic)"
3. Select appropriate scopes (repo, read:org, etc.)
4. Copy the token and set it as the GITHUB_TOKEN environment variable

### 2. Restart Claude Code

After updating the `.mcp.json` file, restart Claude Code to load the new MCP servers.

### 3. Verify MCP Servers

Use the `/context` command in Claude Code to verify that all servers are loaded successfully.

## Configuration File

The MCP servers are configured in `.mcp.json` at the project root. This file can be checked into version control to share the configuration with your team.

## Troubleshooting

### Server fails to start
- Check that Node.js and npm are installed
- Verify environment variables are set correctly
- Check console output for specific error messages

### GitHub server authentication issues
- Ensure `GITHUB_TOKEN` is set and valid
- Verify token has appropriate permissions
- Check token expiration date

### Performance issues
- Some servers (like Puppeteer/Playwright) may consume significant resources
- Consider disabling unused servers by removing them from `.mcp.json`

## Resources

- [Model Context Protocol Documentation](https://modelcontextprotocol.io/)
- [Claude Code MCP Guide](https://code.claude.com/docs/en/mcp.md)
- [Awesome MCP Servers](https://github.com/punkpeye/awesome-mcp-servers)
