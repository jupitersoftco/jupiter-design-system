# Workflow Rule: Build Pipeline Discovery

## Overview
Many systems have hidden build steps that are critical for functionality. Always discover and document the complete build pipeline before documenting features.

## Build Pipeline Discovery Protocol

### Step 1: Find Build Configuration Files
```bash
# Common build files
BUILD_FILES=(
    "package.json"
    "Cargo.toml"
    "build.rs"
    "Makefile"
    "webpack.config.js"
    "vite.config.js"
    "rollup.config.js"
    "tailwind.config.js"
    "postcss.config.js"
    ".github/workflows/*.yml"
)

echo "=== Build Configuration Files ==="
for file in "${BUILD_FILES[@]}"; do
    if ls $file 2>/dev/null; then
        echo "✅ Found: $file"
    else
        echo "❌ Missing: $file"
    fi
done
```

### Step 2: Analyze Build Scripts
```bash
# Check package.json scripts
if [ -f "package.json" ]; then
    echo "=== NPM Scripts ==="
    cat package.json | jq '.scripts' 2>/dev/null || grep -A 10 '"scripts"' package.json
fi

# Check Cargo.toml build dependencies
if [ -f "Cargo.toml" ]; then
    echo "=== Cargo Build Dependencies ==="
    grep -A 20 "\[build-dependencies\]" Cargo.toml
fi

# Check for build.rs
if [ -f "build.rs" ]; then
    echo "=== Build Script Found ==="
    cat build.rs | head -20
fi
```

### Step 3: Trace Build Dependencies
```bash
# For CSS generation systems
if [ -f "tailwind.config.js" ]; then
    echo "=== Tailwind Pipeline ==="
    echo "Config: tailwind.config.js"
    echo "Input: $(grep -o 'input.*' tailwind.config.js)"
    echo "Output: $(grep -o 'output.*' tailwind.config.js)"
    
    # Check for PostCSS
    if [ -f "postcss.config.js" ]; then
        echo "PostCSS: postcss.config.js"
        cat postcss.config.js | grep -E "plugins|tailwind"
    fi
fi
```

### Step 4: Test Build Process
```bash
# Try to run build
echo "=== Testing Build Process ==="

# NPM builds
if [ -f "package.json" ]; then
    npm install 2>/dev/null
    npm run build 2>/dev/null || echo "❌ npm run build failed"
fi

# Cargo builds
if [ -f "Cargo.toml" ]; then
    cargo build 2>/dev/null || echo "❌ cargo build failed"
fi

# Check for generated files
echo "=== Generated Files ==="
find . -name "dist" -o -name "build" -o -name "target" -o -name "output" -type d
```

## Today's Build Pipeline Discovery

### What We Found (Eventually)
- System generates CSS classes
- No `package.json` exists
- No `tailwind.config.js` exists
- No build process documented

### What We Should Have Found First
```bash
# This command would have revealed the problem:
grep -r "jupiter-blue" . --include="*.css" --include="*.js" --include="*.json"
# Result: No files found - classes are generated but not defined!
```

### The Missing Pipeline
```
Rust Code → CSS Classes → ❌ (No Tailwind) → ❌ (No CSS) → ❌ (No Styling)
```

### Required Pipeline
```
Rust Code → CSS Classes → Tailwind Config → PostCSS → Generated CSS → Browser
```

## Build Pipeline Patterns

### Pattern 1: CSS Generation
```bash
# Detect CSS generation systems
echo "=== CSS Generation Detection ==="

# Check for CSS class generation
grep -r "format!" . --include="*.rs" | grep -E "text-|bg-|border-" | wc -l
echo "CSS class generation points: $(grep -r "format!" . --include="*.rs" | grep -E "text-|bg-|border-" | wc -l)"

# Check for CSS pipeline
PIPELINE_SCORE=0
[ -f "package.json" ] && PIPELINE_SCORE=$((PIPELINE_SCORE + 1))
[ -f "tailwind.config.js" ] && PIPELINE_SCORE=$((PIPELINE_SCORE + 1))
[ -f "postcss.config.js" ] && PIPELINE_SCORE=$((PIPELINE_SCORE + 1))
grep -q "tailwindcss" package.json 2>/dev/null && PIPELINE_SCORE=$((PIPELINE_SCORE + 1))

echo "CSS pipeline completeness: $PIPELINE_SCORE/4"
```

### Pattern 2: Asset Processing
```bash
# Check for asset processing
echo "=== Asset Processing ==="

# Static files
find . -name "assets" -o -name "static" -o -name "public" -type d

# Image processing
grep -r "image\|img\|svg" . --include="*.json" --include="*.js" | head -5

# Font processing
grep -r "font\|woff\|ttf" . --include="*.json" --include="*.js" | head -5
```

### Pattern 3: Code Generation
```bash
# Check for code generation
echo "=== Code Generation ==="

# Build scripts
[ -f "build.rs" ] && echo "✅ Rust build script" || echo "❌ No build.rs"

# Template processing
find . -name "*.template" -o -name "*.hbs" -o -name "*.mustache"

# Macro usage
grep -r "macro_rules!\|derive(" . --include="*.rs" | wc -l
echo "Macro usage: $(grep -r "macro_rules!\|derive(" . --include="*.rs" | wc -l)"
```

## Build Pipeline Validation

### Create Build Test
```bash
#!/bin/bash
# test_build_pipeline.sh

echo "=== Build Pipeline Test ==="

# Step 1: Fresh environment
rm -rf node_modules target dist build 2>/dev/null

# Step 2: Install dependencies
if [ -f "package.json" ]; then
    echo "Installing npm dependencies..."
    npm install || echo "❌ npm install failed"
fi

# Step 3: Run build
if [ -f "package.json" ]; then
    echo "Running npm build..."
    npm run build 2>/dev/null || echo "❌ npm build failed"
fi

echo "Running cargo build..."
cargo build || echo "❌ cargo build failed"

# Step 4: Check output
echo "=== Build Output ==="
find . -name "dist" -o -name "build" -o -name "target" -type d | while read dir; do
    echo "Directory: $dir"
    ls -la "$dir" | head -10
done
```

### Validate Generated Assets
```bash
# Check if generated assets work
echo "=== Asset Validation ==="

# For CSS systems
if [ -f "dist/style.css" ]; then
    echo "CSS size: $(wc -c < dist/style.css) bytes"
    echo "CSS contains custom colors: $(grep -c "jupiter-" dist/style.css)"
fi

# For JS systems
if [ -f "dist/main.js" ]; then
    echo "JS size: $(wc -c < dist/main.js) bytes"
    echo "JS is minified: $(grep -c ";" dist/main.js)"
fi
```

## Build Pipeline Documentation

### Template: Build Requirements
```markdown
## Build Requirements

### Prerequisites
- Node.js 18+ (for CSS processing)
- Rust 1.70+ (for library)
- npm or yarn

### Build Process
1. **Install dependencies**
   ```bash
   npm install
   cargo build
   ```

2. **Generate CSS**
   ```bash
   npm run build-css
   ```

3. **Build library**
   ```bash
   cargo build --release
   ```

### Output Files
- `dist/jupiter.css` - Generated CSS styles
- `target/release/` - Compiled Rust library

### Troubleshooting
- **CSS classes not working**: Run `npm run build-css`
- **Missing styles**: Check `dist/jupiter.css` exists
- **Build fails**: Ensure Node.js 18+ installed
```

## Build Pipeline Commands

### Complete Pipeline Discovery
```bash
#!/bin/bash
# discover_build_pipeline.sh

echo "=== Complete Build Pipeline Discovery ==="

echo "1. Configuration Files:"
ls -la | grep -E "config\.|\.config\.|package\.json|Cargo\.toml|Makefile"

echo "2. Build Scripts:"
find . -name "build.rs" -o -name "*.sh" | grep -v ".git"

echo "3. Dependencies:"
if [ -f "package.json" ]; then
    echo "NPM deps: $(cat package.json | jq '.dependencies | keys | length')"
fi
if [ -f "Cargo.toml" ]; then
    echo "Rust deps: $(grep -c "^[a-z]" Cargo.toml)"
fi

echo "4. Output Directories:"
find . -type d -name "dist" -o -name "build" -o -name "target" -o -name "output"

echo "5. Build Commands:"
if [ -f "package.json" ]; then
    echo "NPM scripts:"
    cat package.json | jq '.scripts | keys[]' 2>/dev/null
fi
```

### Generate Build Documentation
```bash
# Auto-generate build docs
cat > BUILD.md << 'EOF'
# Build Pipeline

## Discovered Files
EOF

ls -la | grep -E "config\.|\.config\.|package\.json|Cargo\.toml" >> BUILD.md

echo "## Build Commands" >> BUILD.md
if [ -f "package.json" ]; then
    echo "### NPM Scripts" >> BUILD.md
    cat package.json | jq '.scripts' >> BUILD.md
fi
```

## Key Lesson

**Build pipelines are often the missing link between elegant code and working systems.**

Today's system had:
- Elegant Rust architecture ✅
- Type-safe color system ✅
- Generated CSS classes ✅
- No build pipeline ❌

The missing build pipeline made the entire system non-functional.

**Always discover and document the complete build pipeline first.**

Time to discover pipeline: **10 minutes**
Time wasted documenting unbuildable system: **Hours**