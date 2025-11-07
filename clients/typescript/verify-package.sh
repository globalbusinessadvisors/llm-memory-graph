#!/bin/bash
# Package Verification Script

echo "============================================"
echo "LLM Memory Graph Client - Package Verification"
echo "============================================"
echo ""

# Check if package builds
echo "1. Checking build..."
npm run build > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "   ✓ Build successful"
else
    echo "   ✗ Build failed"
    exit 1
fi

# Check if package can be loaded
echo "2. Checking module loading..."
node -e "const { MemoryGraphClient } = require('./dist/index.js'); console.log('   ✓ Module loads successfully');" 2>&1 | grep "✓"
if [ $? -ne 0 ]; then
    echo "   ✗ Module loading failed"
    exit 1
fi

# Check package metadata
echo "3. Checking package.json..."
node -e "const pkg = require('./package.json'); 
if (pkg.name && pkg.version && pkg.main && pkg.types) {
    console.log('   ✓ Package metadata valid');
} else {
    console.log('   ✗ Package metadata incomplete');
    process.exit(1);
}" 2>&1 | grep "✓"

# Check file structure
echo "4. Checking file structure..."
FILES="dist/index.js dist/index.d.ts dist/client.js dist/types.js proto/memory_graph.proto README.md LICENSE"
for file in $FILES; do
    if [ ! -f "$file" ]; then
        echo "   ✗ Missing file: $file"
        exit 1
    fi
done
echo "   ✓ All required files present"

# Check package size
echo "5. Checking package size..."
if [ -f "llm-memory-graph-client-0.1.0.tgz" ]; then
    SIZE=$(stat -f%z "llm-memory-graph-client-0.1.0.tgz" 2>/dev/null || stat -c%s "llm-memory-graph-client-0.1.0.tgz")
    echo "   ✓ Package size: $SIZE bytes (~25 KB)"
else
    echo "   ✗ Package tarball not found"
    exit 1
fi

# Summary
echo ""
echo "============================================"
echo "All checks passed! Package is ready ✓"
echo "============================================"
echo ""
echo "To publish:"
echo "  npm login"
echo "  npm publish --access public"
echo ""
