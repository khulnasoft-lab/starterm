# Starterm Workflows

This directory contains the workflow system for Starterm, which provides a way to define and manage shell command workflows that can be executed from the terminal emulator.

## Overview

The workflow system consists of three main components:

1. **Workflow Types** (`workflow-types/`) - Rust types and structures for defining workflows
2. **Workflow Generator** (`workflows/`) - Build-time code generator that converts YAML workflow definitions into Rust code
3. **Workflow Specifications** (`specs/`) - YAML files defining individual workflows

## Architecture

```
workflows/
├── Cargo.toml              # Workspace configuration
├── workflow-types/         # Rust types for workflows
│   ├── Cargo.toml
│   └── src/lib.rs         # Workflow and Argument structs
├── workflows/             # Main workflow crate
│   ├── Cargo.toml
│   ├── build.rs           # Code generator
│   └── src/
│       ├── lib.rs         # Public API
│       └── generated_workflows/  # Auto-generated workflow modules
└── specs/                 # YAML workflow definitions
    └── brew/              # Homebrew-related workflows
        ├── backup_installed_dependencies.yaml
        └── update_all_packages.yaml
```

## Workflow Format

Workflows are defined in YAML files with the following structure:

```yaml
---
name: "Workflow Name"
command: "shell command to execute"
tags:
  - "tag1"
  - "tag2"
description: "Optional description of what the workflow does"
arguments:
  - name: "arg_name"
    description: "Description of the argument"
    default_value: "optional default value"
source_url: "https://source.url"
author: "Author Name"
author_url: "https://author.url"
shells: ["zsh", "bash", "fish"]  # Optional shell restrictions
```

### Required Fields

- `name`: The display name of the workflow
- `command`: The shell command to execute

### Optional Fields

- `tags`: Array of tags for categorization
- `description`: Human-readable description
- `arguments`: Array of parameterized arguments
- `source_url`: URL where the workflow originated
- `author`: Original author of the workflow
- `author_url`: URL to the author's profile
- `shells`: Array of supported shells (`zsh`, `bash`, `fish`)

### Arguments

Arguments allow workflows to be parameterized. Use `{{argument_name}}` in the command to reference arguments:

```yaml
name: "Search files"
command: "find {{directory}} -name '{{pattern}}'"
arguments:
  - name: "directory"
    description: "Directory to search in"
    default_value: "."
  - name: "pattern"
    description: "File pattern to search for"
    default_value: "*.txt"
```

## Building

The workflow system uses a build script (`build.rs`) that:

1. Scans the `specs/` directory for YAML files
2. Parses each YAML file into a `Workflow` struct
3. Generates Rust modules for each workflow
4. Creates a `workflows()` function that returns all workflows

To build the workflows:

```bash
cd workflows
cargo build
```

## Adding New Workflows

To add a new workflow:

1. Create a new YAML file in the `specs/` directory (organize by category)
2. Define the workflow using the format above
3. Run `cargo build` to generate the Rust code
4. The workflow will be automatically available through the `workflows()` function

## Example

Here's a complete example of adding a new workflow:

1. Create `specs/git/commit_with_message.yaml`:

```yaml
---
name: "Git commit with message"
command: "git add . && git commit -m '{{message}}'"
tags:
  - "git"
  - "version-control"
description: "Stage all changes and commit with a custom message"
arguments:
  - name: "message"
    description: "Commit message"
    default_value: "Update"
source_url: "https://git-scm.com/docs/git-commit"
author: "starterm"
author_url: "https://github.com/khulnasoft-lab/starterm"
shells: []
```

2. Build the project:

```bash
cargo build
```

3. The workflow will be available in the generated code and can be accessed programmatically.

## Integration

The workflow system is designed to be integrated into the main Starterm terminal emulator. The `workflows()` function provides access to all defined workflows, which can then be presented to users for selection and execution.

## Testing

Run the test suite to verify workflow generation:

```bash
cargo test
```

The tests verify that:
- Workflows are properly generated from YAML files
- All workflow properties are correctly parsed
- The `workflows()` function returns the expected workflows

## Dependencies

- `serde` - Serialization/deserialization
- `serde_yaml` - YAML parsing
- `anyhow` - Error handling
- `walkdir` - Directory traversal
- `convert_case` - String case conversion
- `memmap` - Memory-mapped file I/O
- `uneval` - Rust code generation

## License

This project is licensed under the Apache 2.0 License. 