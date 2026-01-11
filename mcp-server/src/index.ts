#!/usr/bin/env node

import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
  Tool,
} from "@modelcontextprotocol/sdk/types.js";
import { execSync } from "child_process";
import { existsSync } from "fs";
import { resolve } from "path";

// Find the repo-tasks binary
function findRepoTasksBinary(): string {
  // Try common locations and names
  const locations = [
    "tasks", // Homebrew default name
    "repo-tasks", // Full name in PATH
    "./target/release/repo-tasks", // Local release build
    "./target/debug/repo-tasks", // Local debug build
  ];

  for (const loc of locations) {
    try {
      execSync(`command -v ${loc}`, { stdio: "ignore" });
      return loc;
    } catch {
      if (existsSync(loc)) {
        return resolve(loc);
      }
    }
  }

  throw new Error(
    "repo-tasks binary not found. Please install it (as 'tasks' or 'repo-tasks') or build from source."
  );
}

const REPO_TASKS_BIN = findRepoTasksBinary();

// Execute repo-tasks command
function executeCommand(args: string[]): string {
  try {
    return execSync(`${REPO_TASKS_BIN} ${args.join(" ")}`, {
      encoding: "utf-8",
      stdio: ["pipe", "pipe", "pipe"],
    });
  } catch (error: any) {
    throw new Error(error.stderr || error.message);
  }
}

// Define all available tools
const TOOLS: Tool[] = [
  {
    name: "list_tasks",
    description:
      "List tasks in a given status (todo, in-progress, testing, done). Returns tasks with their IDs, titles, priorities, and tags.",
    inputSchema: {
      type: "object",
      properties: {
        status: {
          type: "string",
          description: "Status to filter by (todo, in-progress, testing, done)",
          default: "todo",
        },
        priority: {
          type: "string",
          description: "Filter by priority (Low, Medium, High, Critical)",
        },
        tag: {
          type: "string",
          description: "Filter by tag",
        },
      },
    },
  },
  {
    name: "show_task",
    description:
      "Show detailed information about a specific task including title, status, priority, tags, and full description.",
    inputSchema: {
      type: "object",
      properties: {
        task_id: {
          type: "string",
          description: "Task ID or slug to show",
        },
      },
      required: ["task_id"],
    },
  },
  {
    name: "create_task",
    description:
      "Create a new task with the given title, priority, and optional tags/notes.",
    inputSchema: {
      type: "object",
      properties: {
        title: {
          type: "string",
          description: "Task title",
        },
        priority: {
          type: "string",
          description: "Priority level (Low, Medium, High, Critical)",
          default: "Medium",
        },
        tags: {
          type: "string",
          description: "Comma-separated tags",
        },
        notes: {
          type: "string",
          description: "Task description/notes",
        },
      },
      required: ["title"],
    },
  },
  {
    name: "update_task",
    description:
      "Update task properties (title, priority, tags). Opens an interactive editor.",
    inputSchema: {
      type: "object",
      properties: {
        task_id: {
          type: "string",
          description: "Task ID or slug to update",
        },
      },
      required: ["task_id"],
    },
  },
  {
    name: "move_task",
    description:
      "Move a task to a different status (todo, in-progress, testing, done).",
    inputSchema: {
      type: "object",
      properties: {
        task_id: {
          type: "string",
          description: "Task ID or slug to move",
        },
        new_status: {
          type: "string",
          description: "New status (todo, in-progress, testing, done)",
          enum: ["todo", "in-progress", "testing", "done"],
        },
      },
      required: ["task_id", "new_status"],
    },
  },
  {
    name: "start_task",
    description:
      "Start working on a task: moves it to in-progress and creates a git branch with format {id}-{slug}. This is the recommended way to begin work on a task.",
    inputSchema: {
      type: "object",
      properties: {
        task_id: {
          type: "string",
          description: "Task ID or slug to start working on",
        },
      },
      required: ["task_id"],
    },
  },
  {
    name: "search_tasks",
    description:
      "Search for tasks by content using regex patterns. Searches across all task files.",
    inputSchema: {
      type: "object",
      properties: {
        query: {
          type: "string",
          description: "Search query (regex supported)",
        },
      },
      required: ["query"],
    },
  },
  {
    name: "save_tasks",
    description:
      "Commit changes to task files using git. Use this after creating or modifying tasks to save them to version control.",
    inputSchema: {
      type: "object",
      properties: {
        message: {
          type: "string",
          description: "Commit message (auto-generated if not provided)",
        },
      },
    },
  },
  {
    name: "open_task",
    description:
      "Open a task file in the default editor for manual editing.",
    inputSchema: {
      type: "object",
      properties: {
        task_id: {
          type: "string",
          description: "Task ID or slug to open",
        },
      },
      required: ["task_id"],
    },
  },
];

// Create server instance
const server = new Server(
  {
    name: "repo-tasks",
    version: "0.1.0",
  },
  {
    capabilities: {
      tools: {},
    },
  }
);

// Handle list tools request
server.setRequestHandler(ListToolsRequestSchema, async () => {
  return { tools: TOOLS };
});

// Handle tool execution
server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args } = request.params;

  try {
    switch (name) {
      case "list_tasks": {
        const cmdArgs = ["list"];
        if (args?.status) cmdArgs.push(args.status as string);
        if (args?.priority) cmdArgs.push("--priority", args.priority as string);
        if (args?.tag) cmdArgs.push("--tag", args.tag as string);

        const output = executeCommand(cmdArgs);
        return {
          content: [{ type: "text", text: output }],
        };
      }

      case "show_task": {
        const output = executeCommand(["show", args?.task_id as string]);
        return {
          content: [{ type: "text", text: output }],
        };
      }

      case "create_task": {
        const cmdArgs = ["new"];
        if (args?.title) cmdArgs.push("--title", args.title as string);
        if (args?.priority) cmdArgs.push("--priority", args.priority as string);
        if (args?.tags) cmdArgs.push("--tags", args.tags as string);
        if (args?.notes) cmdArgs.push("--notes", args.notes as string);

        const output = executeCommand(cmdArgs);
        return {
          content: [{ type: "text", text: output }],
        };
      }

      case "update_task": {
        const output = executeCommand(["update", args?.task_id as string]);
        return {
          content: [{ type: "text", text: output }],
        };
      }

      case "move_task": {
        const output = executeCommand([
          "move",
          args?.task_id as string,
          args?.new_status as string,
        ]);
        return {
          content: [{ type: "text", text: output }],
        };
      }

      case "start_task": {
        const output = executeCommand(["start", args?.task_id as string]);
        return {
          content: [{ type: "text", text: output }],
        };
      }

      case "search_tasks": {
        const output = executeCommand(["search", args?.query as string]);
        return {
          content: [{ type: "text", text: output }],
        };
      }

      case "save_tasks": {
        const cmdArgs = ["save"];
        if (args?.message) cmdArgs.push("--message", args.message as string);

        const output = executeCommand(cmdArgs);
        return {
          content: [{ type: "text", text: output }],
        };
      }

      case "open_task": {
        const output = executeCommand(["open", args?.task_id as string]);
        return {
          content: [{ type: "text", text: output }],
        };
      }

      default:
        throw new Error(`Unknown tool: ${name}`);
    }
  } catch (error: any) {
    return {
      content: [{ type: "text", text: `Error: ${error.message}` }],
      isError: true,
    };
  }
});

// Start the server
async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error("repo-tasks MCP server running on stdio");
}

main().catch((error) => {
  console.error("Fatal error:", error);
  process.exit(1);
});
