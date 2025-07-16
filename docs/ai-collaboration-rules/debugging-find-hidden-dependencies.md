# Debugging Rule: Find Hidden Dependencies Before Documentation

## Overview
Systems often have hidden dependencies that aren't obvious from the main implementation. These must be discovered and documented to prevent user frustration.

## Today's Hidden Dependency: Tailwind Configuration

### What Happened
The Jupiter Design System uses custom color classes:
```rust
// In the code
primary: "jupiter-blue-500"
secondary: "jupiter-green-500"
```

But these CSS classes **don't exist** without Tailwind configuration!

### The Hidden Dependency
```javascript
// Required in tailwind.config.js (but not documented!)
module.exports = {
  theme: {
    extend: {
      colors: {
        'jupiter-blue': {
          500: '#3b82f6',
          600: '#2563eb',
          700: '#1d4ed8',
          // ... full scale
        },
        // ... other jupiter colors
      }
    }
  }
}
```

## How to Find Hidden Dependencies

### 1. CSS Class Dependencies
```bash
# Find all CSS classes generated
grep -r "text-\|bg-\|border-" . --include="*.rs" | grep -o '"[^"]*"' | sort -u

# Check if they're standard Tailwind
# If they have custom prefixes (jupiter-), they need configuration!
```

### 2. External Tool Dependencies
```bash
# Look for tool-specific patterns
grep -r "tailwind\|postcss\|webpack\|vite" . --include="*.json" --include="*.js"

# Check for build commands
grep -r "build\|compile\|generate" package.json Cargo.toml Makefile
```

### 3. Runtime Dependencies
```bash
# Find dynamic imports or lazy loading
grep -r "import(\|require(\|load\|fetch" . --include="*.rs" --include="*.js"

# Check for environment variables
grep -r "env!\|std::env\|process.env" . --include="*.rs" --include="*.js"
```

### 4. Configuration File Dependencies
```bash
# Common configuration files
ls -la | grep -E "config|rc|toml|json|yaml|yml"

# Check for config references
grep -r "config\|Config\|settings\|Settings" . --include="*.rs"
```

## Debugging Process for Hidden Dependencies

### Step 1: Test in Isolation
```bash
# Create minimal example
mkdir test-isolation
cd test-isolation

# Copy only the core files
cp ../src/core/color.rs .

# Try to use it without the full project
# This reveals missing dependencies quickly
```

### Step 2: Trace CSS Generation
```rust
// Add debug prints to find where CSS comes from
fn generate_css_class(&self, color: Color) -> String {
    let class = format!("text-{}", self.resolve_color(color));
    eprintln!("DEBUG: Generated class: {}", class);
    eprintln!("DEBUG: Requires Tailwind config for: {}", self.resolve_color(color));
    class
}
```

### Step 3: Build Process Analysis
```bash
# Check what build steps exist
find . -name "package.json" -exec grep -H "scripts" {} \;
find . -name "Cargo.toml" -exec grep -H "\[build" {} \;

# Look for pre/post build hooks
grep -r "prebuild\|postbuild\|prepare" . --include="*.json"
```

## Common Hidden Dependencies in Design Systems

### 1. CSS Framework Configuration
- Tailwind config for custom colors
- PostCSS plugins for transformations
- Sass variables or mixins

### 2. Build Tool Setup
- Webpack aliases for imports
- Vite configuration for CSS
- Rollup plugins for optimization

### 3. Type Definitions
```typescript
// May need separate installation
npm install @types/design-system
```

### 4. Font Files
```css
/* CSS references fonts that need to be included */
font-family: 'JupiterSans', sans-serif;
```

### 5. Icon Systems
```rust
// Code might reference icons that need separate setup
icon: "jupiter-icon-check"  // Requires icon font or SVG sprite
```

## Detection Commands

### Find All External References
```bash
# Find potential external dependencies
echo "=== Checking for External CSS Classes ==="
grep -r "class=" . --include="*.rs" | grep -o '"[^"]*"' | tr ' ' '\n' | sort -u | grep -v "^$"

echo "=== Checking for Import Paths ==="
grep -r "from\|import\|use" . --include="*.rs" --include="*.js" | grep -o '["\x27][^"\x27]*["\x27]' | sort -u

echo "=== Checking for File References ==="
grep -r "\./\|../\|@/" . --include="*.rs" --include="*.js" | grep -o '["\x27][^"\x27]*["\x27]' | sort -u
```

### Validation Script
```bash
#!/bin/bash
# validate-dependencies.sh

echo "Checking for hidden dependencies..."

# Check CSS classes
CUSTOM_CLASSES=$(grep -r "jupiter-\|brand-" . --include="*.rs" | wc -l)
if [ $CUSTOM_CLASSES -gt 0 ]; then
    echo "⚠️  Found $CUSTOM_CLASSES custom CSS classes - needs Tailwind config"
fi

# Check for config files
EXPECTED_CONFIGS="tailwind.config.js postcss.config.js"
for config in $EXPECTED_CONFIGS; do
    if [ ! -f $config ]; then
        echo "❌ Missing required config: $config"
    fi
done

# Check package.json for CSS build
if ! grep -q "tailwindcss" package.json 2>/dev/null; then
    echo "❌ Tailwind CSS not in package.json but custom classes used"
fi
```

## Documentation Template for Dependencies

```markdown
## Required Configuration

### Tailwind CSS Setup
This design system requires Tailwind CSS with custom configuration:

1. Install Tailwind CSS:
   ```bash
   npm install tailwindcss
   ```

2. Add to your `tailwind.config.js`:
   ```javascript
   // Custom colors configuration
   ```

3. Include in your CSS:
   ```css
   @tailwind base;
   @tailwind components;
   @tailwind utilities;
   ```

### Build Process
The CSS classes are generated at build time and require:
- PostCSS with Tailwind plugin
- PurgeCSS configuration for production
```

## Key Lesson
**A design system that generates CSS classes has an implicit dependency on a CSS framework configuration.** This wasn't documented initially, making the system unusable without reverse-engineering the requirements.

Always ask: "What else does this need to actually work?"