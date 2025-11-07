# llm-memory-graph-cli

Command-line management tool for LLM Memory Graph databases.

## Installation

```bash
cargo install llm-memory-graph-cli
```

Or build from source:

```bash
cargo build --release
```

## Usage

### Show Database Statistics

```bash
llm-memory-graph --db-path ./data stats
```

Output:
```
Database Statistics
===================
Total Nodes:         1,234
Total Edges:         2,456
Total Sessions:      45
```

### List Sessions

```bash
# Show recent sessions
llm-memory-graph sessions --limit 10

# JSON output
llm-memory-graph --format json sessions
```

### View Session Details

```bash
llm-memory-graph session <session-id>
```

### List Nodes

```bash
# List all nodes
llm-memory-graph nodes --limit 20

# Filter by type
llm-memory-graph nodes --node-type prompt --limit 10
```

### View Node Details

```bash
llm-memory-graph node <node-id>
```

### Query Session Prompts

```bash
llm-memory-graph prompts <session-id>
```

### Export Session Data

```bash
llm-memory-graph export <session-id> --output session-backup.json
```

### Database Maintenance

```bash
# Compact database
llm-memory-graph compact

# Verify integrity
llm-memory-graph verify
```

## Configuration

### Environment Variables

- `LMG_DB_PATH`: Default database path (default: `./data`)

### Command-Line Options

- `-d, --db-path <PATH>`: Database directory path
- `-f, --format <FORMAT>`: Output format (text, json)

## Output Formats

### Text Format (Default)

Human-readable tables and formatted output with colors.

### JSON Format

Machine-readable JSON output for scripting and integration:

```bash
llm-memory-graph --format json stats | jq .
```

## Examples

### Find Sessions by Metadata

```bash
# List all sessions with JSON output
llm-memory-graph --format json sessions | \
  jq '.[] | select(.metadata.user_id == "user-123")'
```

### Export Multiple Sessions

```bash
# Export all sessions
for session_id in $(llm-memory-graph --format json sessions | jq -r '.[].id'); do
  llm-memory-graph export "$session_id" --output "backups/${session_id}.json"
done
```

### Monitor Database Growth

```bash
# Watch statistics in real-time
watch -n 5 'llm-memory-graph stats'
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
