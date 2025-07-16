# Critical Rule: Output Validation Before Documentation

## Overview
Never document what a system claims to produce - validate that the output actually works. Generated code, CSS classes, or any output must be tested in its intended environment.

## The Validation Principle

**Generated != Functional**

Just because code generates a string doesn't mean that string does anything useful.

## Output Validation Protocol

### Step 1: Capture Generated Output
```bash
# Run the system and capture output
cargo run --example basic_usage > output.txt

# Extract generated values
grep -E "text-|bg-|border-" output.txt | sort -u > generated_classes.txt

# Example from today:
# Generated: "text-jupiter-blue-500"
# But this CSS class doesn't exist!
```

### Step 2: Test in Target Environment
```bash
# For CSS classes - create test HTML
cat > test.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <!-- Include supposed CSS -->
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <div class="text-jupiter-blue-500">Does this have color?</div>
    <button class="bg-jupiter-green-500 text-white">Am I green?</button>
</body>
</html>
EOF

# Open in browser
open test.html

# If no styling appears, the output is non-functional
```

### Step 3: Trace the Pipeline
```bash
# For CSS generation systems:
# 1. Where are classes defined?
find . -name "*.css" -exec grep -l "jupiter-blue-500" {} \;

# 2. Is there a build process?
find . -name "package.json" -o -name "webpack.config.js" -o -name "tailwind.config.js"

# 3. Are there missing steps?
grep -r "TODO\|FIXME" . | grep -i "css\|style\|build"
```

## Validation Patterns by Output Type

### CSS Classes
```bash
# Validate CSS class generation
GENERATED_CLASS="text-jupiter-blue-500"

# Check 1: Is it defined anywhere?
grep -r "$GENERATED_CLASS" . --include="*.css" --include="*.js" || echo "❌ Not defined"

# Check 2: Can it be generated?
if [ -f "tailwind.config.js" ]; then
    grep "jupiter-blue" tailwind.config.js || echo "❌ Not in Tailwind config"
fi

# Check 3: Does build process exist?
npm run build-css 2>/dev/null || echo "❌ No CSS build process"
```

### HTML Generation
```bash
# Validate HTML output
cargo run --example generate_html > output.html

# Check 1: Valid HTML?
html-validate output.html 2>/dev/null || echo "⚠️  Invalid HTML"

# Check 2: References work?
grep -o 'class="[^"]*"' output.html | cut -d'"' -f2 | while read class; do
    grep "$class" styles.css >/dev/null || echo "❌ Missing class: $class"
done
```

### Configuration Generation
```bash
# Validate config output
cargo run --example generate_config > config.json

# Check 1: Valid JSON?
jq . config.json >/dev/null || echo "❌ Invalid JSON"

# Check 2: Required fields?
REQUIRED_FIELDS=("colors" "theme" "version")
for field in "${REQUIRED_FIELDS[@]}"; do
    jq ".$field" config.json >/dev/null || echo "❌ Missing field: $field"
done
```

## Today's Validation Failure

### What We Documented
```rust
// Elegant color system
theme.text_class(Color::Primary)  // Returns: "text-jupiter-blue-500"
theme.bg_class(Color::Surface)     // Returns: "bg-white"
```

### What Actually Happened
```html
<!-- Generated HTML -->
<div class="text-jupiter-blue-500">This text is unstyled</div>

<!-- Because jupiter-blue-500 is not defined anywhere! -->
```

### Root Cause
- System generates Tailwind classes
- But no Tailwind configuration exists
- No CSS build process
- Output is just useless strings

## Validation Checklist

### Before Documentation
- [ ] Generate sample output
- [ ] Test output in real environment
- [ ] Verify output is functional
- [ ] Trace full pipeline
- [ ] Document missing pieces

### Red Flags
- [ ] Output is just strings
- [ ] No build process
- [ ] No configuration files
- [ ] No working examples
- [ ] Can't find output definitions

## Creating Validation Tests

### Quick Validation Script
```bash
#!/bin/bash
# validate_output.sh

echo "=== Generating Output ==="
cargo run --example basic_usage > generated.txt

echo "=== Extracting Classes ==="
grep -o '[a-z-]*-[a-z-]*-[0-9]*' generated.txt | sort -u > classes.txt

echo "=== Validating Classes ==="
while read class; do
    # Check if class is defined
    if grep -r "$class" . --include="*.css" --include="*.js" >/dev/null; then
        echo "✅ $class"
    else
        echo "❌ $class (undefined)"
    fi
done < classes.txt

echo "=== Checking Build Process ==="
if [ -f "package.json" ]; then
    npm run build 2>/dev/null || echo "❌ Build failed"
else
    echo "❌ No package.json"
fi
```

### Integration Test
```rust
#[test]
fn test_generated_output_is_usable() {
    let theme = VibeColors::default();
    let class = theme.text_class(Color::Primary);
    
    // Check 1: Non-empty
    assert!(!class.is_empty());
    
    // Check 2: Valid CSS class format
    assert!(class.starts_with("text-"));
    
    // Check 3: TODO - Actually verify CSS exists
    // This test is incomplete without CSS validation!
}
```

## Documentation Template

When output validation fails, document it:

```markdown
## ⚠️ Output Validation Status

The system generates CSS classes like `text-jupiter-blue-500`, but these require additional configuration:

### Required Setup
1. Install Tailwind CSS
2. Configure Jupiter colors (see `tailwind.config.example.js`)
3. Build CSS pipeline
4. Include generated CSS

### Without This Setup
- Generated classes are non-functional
- No styling will appear
- System outputs are just strings

See [Complete Setup Guide](./setup.md) for configuration instructions.
```

## Key Lesson

**Output that looks correct but doesn't work is worse than obvious errors.**

Today we spent hours documenting a color system that generates useless strings. A 5-minute validation test would have revealed this immediately:

```bash
echo '<div class="text-jupiter-blue-500">Test</div>' > test.html
open test.html  # No color? System is broken.
```

Always validate output in its intended environment before documentation.