# Installation and Publishing Guide

## NPM Package: llm-memory-graph-client

This document provides instructions for installing, using, and publishing the LLM Memory Graph TypeScript client package.

## Package Information

- **Name**: `llm-memory-graph-client`
- **Version**: 0.1.0
- **Size**: 24.9 KB (packed), 110.8 KB (unpacked)
- **Files**: 26 files total
- **License**: MIT OR Apache-2.0

## Installation

### From NPM (once published)

```bash
npm install llm-memory-graph-client
```

### From Local Package

```bash
# Install from the tarball
npm install /path/to/llm-memory-graph-client-0.1.0.tgz

# Or from the directory
npm install /path/to/clients/typescript
```

### From Git Repository

```bash
npm install git+https://github.com/globalbusinessadvisors/llm-memory-graph.git#main:clients/typescript
```

## Development Setup

### Prerequisites

- Node.js >= 16.0.0
- npm or yarn

### Building from Source

```bash
# Navigate to the package directory
cd clients/typescript

# Install dependencies
npm install

# Build the package
npm run build

# Run linting
npm run lint

# Format code
npm run format

# Create package tarball
npm pack
```

## Publishing to NPM

### First-time Setup

1. Create an NPM account at https://www.npmjs.com/signup
2. Login to NPM from the command line:
   ```bash
   npm login
   ```

### Publishing Steps

```bash
# 1. Navigate to the package directory
cd /workspaces/llm-memory-graph/clients/typescript

# 2. Ensure all changes are committed to git
git add .
git commit -m "Prepare v0.1.0 release"

# 3. Run build and tests
npm run build
npm test

# 4. Verify the package contents
npm pack --dry-run

# 5. Publish to NPM (public)
npm publish --access public

# For scoped packages (@your-org/llm-memory-graph-client):
# npm publish --access public
```

### Publishing to GitHub Packages

If you want to publish to GitHub Packages instead:

```bash
# 1. Configure npm to use GitHub Packages
npm config set @globalbusinessadvisors:registry https://npm.pkg.github.com

# 2. Authenticate with GitHub
npm login --scope=@globalbusinessadvisors --registry=https://npm.pkg.github.com

# 3. Update package.json to use scoped name
# Change "name" to "@globalbusinessadvisors/llm-memory-graph-client"

# 4. Publish
npm publish
```

### Version Management

```bash
# Patch version (0.1.0 -> 0.1.1)
npm version patch

# Minor version (0.1.0 -> 0.2.0)
npm version minor

# Major version (0.1.0 -> 1.0.0)
npm version major

# Then publish
npm publish --access public
```

## Using the Package

### TypeScript

```typescript
import { MemoryGraphClient, NodeType } from 'llm-memory-graph-client';

const client = new MemoryGraphClient({
  address: 'localhost:50051',
  useTls: false
});

async function example() {
  const session = await client.createSession();
  const prompt = await client.addPrompt({
    sessionId: session.id,
    content: 'Hello, world!'
  });
  console.log('Prompt ID:', prompt.id);
}

example().catch(console.error);
```

### JavaScript (CommonJS)

```javascript
const { MemoryGraphClient, NodeType } = require('llm-memory-graph-client');

const client = new MemoryGraphClient({
  address: 'localhost:50051',
  useTls: false
});

async function example() {
  const session = await client.createSession();
  console.log('Session created:', session.id);
}

example().catch(console.error);
```

## Package Structure

```
llm-memory-graph-client-0.1.0.tgz
├── dist/                           # Compiled JavaScript and type definitions
│   ├── client.js                   # Main client implementation
│   ├── client.d.ts                 # TypeScript definitions
│   ├── index.js                    # Package entry point
│   ├── index.d.ts                  # Entry point types
│   ├── types.js                    # Type definitions (runtime)
│   ├── types.d.ts                  # TypeScript type definitions
│   └── generated/
│       └── loader.js               # Proto loader
├── src/                            # Source TypeScript files
│   ├── client.ts                   # Main client class
│   ├── index.ts                    # Main exports
│   ├── types.ts                    # Type definitions
│   └── generated/
│       └── loader.ts               # Proto loader source
├── proto/                          # Protocol Buffer definitions
│   ├── memory_graph.proto          # Main service definition
│   └── google/protobuf/
│       ├── empty.proto
│       └── timestamp.proto
├── package.json                    # Package metadata
├── README.md                       # Documentation
└── LICENSE                         # Apache 2.0 license
```

## Testing the Package

### Local Testing

```bash
# Create a test project
mkdir test-client
cd test-client
npm init -y

# Install the local package
npm install ../llm-memory-graph-client-0.1.0.tgz

# Create a test file
cat > test.js << 'EOF'
const { MemoryGraphClient } = require('llm-memory-graph-client');
const client = new MemoryGraphClient({ address: 'localhost:50051' });
console.log('Client created successfully!');
EOF

# Run the test
node test.js
```

### Integration Testing

Run the quickstart example:

```bash
cd clients/typescript
npx ts-node examples/quickstart.ts
```

## Troubleshooting

### Proto file not found

If you encounter errors about missing proto files, ensure the `proto/` directory is included in your package:

```bash
# Check package contents
npm pack --dry-run
```

### Type definitions not working

Ensure your TypeScript configuration includes:

```json
{
  "compilerOptions": {
    "moduleResolution": "node",
    "esModuleInterop": true
  }
}
```

### Connection errors

Make sure the gRPC server is running:

```bash
# From the repository root
cargo run --bin server
```

## Support

- Issues: https://github.com/globalbusinessadvisors/llm-memory-graph/issues
- Documentation: https://github.com/globalbusinessadvisors/llm-memory-graph
- NPM Package: https://www.npmjs.com/package/llm-memory-graph-client (after publishing)

## Next Steps

1. Test the package locally with your gRPC server
2. Create a GitHub release for v0.1.0
3. Publish to NPM registry
4. Update main repository README with installation instructions
5. Create usage examples and tutorials
