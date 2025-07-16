# Debugging Rule: CSS Framework Dependencies in Design Systems

## Overview
Design systems that generate CSS class names have implicit dependencies on CSS frameworks. These dependencies are often undocumented and can make the system unusable without proper configuration.

## Today's Discovery: The Jupiter CSS Problem

### The Symptom
```rust
// Code generates classes
self.text_class(Color::Primary) // Returns: "text-jupiter-blue-500"
```

### The Problem
These CSS classes **don't exist** without configuration:
- No `jupiter-blue-500` color in standard Tailwind
- No CSS file defining these classes  
- No build process to generate them

### The Hidden Requirement
```javascript
// tailwind.config.js - REQUIRED but undocumented
module.exports = {
  theme: {
    extend: {
      colors: {
        'jupiter-blue': {
          500: '#3b82f6',
          // ... full scale needed
        }
      }
    }
  }
}
```

## Detection Strategies

### 1. CSS Class Audit
```bash
# Extract all generated CSS classes
grep -r "text-\|bg-\|border-" . --include="*.rs" | 
  grep -o '"[^"]*"' | 
  tr ' ' '\n' | 
  grep -E "^(text|bg|border)-" | 
  sort -u > generated-classes.txt

# Check for non-standard prefixes
grep -E "-(jupiter|brand|custom)-" generated-classes.txt
```

### 2. Framework Detection
```bash
# Look for CSS framework indicators
grep -r "tailwind\|bootstrap\|bulma" . --include="*.json" --include="*.js" --include="*.rs"

# Check package.json
cat package.json | jq '.dependencies, .devDependencies' | grep -i "css\|style\|tailwind"

# Check for config files
ls -la | grep -E "tailwind|postcss|webpack|vite"
```

### 3. Build Process Analysis
```bash
# Find CSS build commands
grep -r "build.*css\|css.*build" . --include="*.json" --include="Makefile"

# Check for PostCSS
find . -name "postcss.config.js" -o -name ".postcssrc*"

# Look for CSS imports
grep -r "@import\|@tailwind" . --include="*.css" --include="*.scss"
```

## Common CSS Dependency Patterns

### 1. Tailwind Extensions
```javascript
// Look for custom color definitions
colors: {
  'brand-primary': {...},
  'custom-gray': {...},
}

// Look for custom utilities
plugins: [
  function({ addUtilities }) {
    addUtilities({
      '.text-jupiter-blue-500': {
        color: '#3b82f6'
      }
    })
  }
]
```

### 2. CSS-in-JS Dependencies
```rust
// System might generate JS objects instead
format!("{{ color: '{}' }}", self.resolve_color(color))

// Requires runtime CSS-in-JS library
```

### 3. Build-Time Generation
```json
// package.json scripts
"scripts": {
  "generate-css": "node scripts/generate-colors.js",
  "build": "npm run generate-css && tailwindcss build"
}
```

## Validation Process

### Step 1: Trace a Single Class
```bash
# Pick one generated class
CLASS="text-jupiter-blue-500"

# Search for its definition
echo "=== Searching for $CLASS definition ==="
grep -r "$CLASS" . --include="*.css" --include="*.scss" --include="*.js"

# Search for the color value
COLOR="jupiter-blue-500"
grep -r "$COLOR" . --include="*.js" --include="*.json" --include="*.css"

# If not found, it needs configuration
```

### Step 2: Test Minimal Setup
```bash
# Create test HTML
cat > test.html << EOF
<!DOCTYPE html>
<html>
<head>
  <link href="output.css" rel="stylesheet">
</head>
<body>
  <p class="text-jupiter-blue-500">Test</p>
</body>
</html>
EOF

# Try to build CSS
npx tailwindcss build -o output.css

# Open test.html - is text colored?
```

### Step 3: Document Requirements
```markdown
## CSS Configuration Required

This design system generates Tailwind CSS classes that require configuration:

### Required Setup
1. Install Tailwind CSS
2. Configure custom colors in `tailwind.config.js`
3. Include all generated classes in content paths
4. Build CSS with PostCSS

### Without This Configuration
- Classes like `text-jupiter-blue-500` won't work
- Components will have no styling
- Colors will fall back to browser defaults
```

## Debug Checklist for CSS Dependencies

### Quick Checks
- [ ] Do generated classes use standard framework names?
- [ ] Are there custom prefixes (jupiter-, brand-, etc.)?
- [ ] Is there a CSS build step in package.json?
- [ ] Are there CSS framework config files?

### Deep Checks
- [ ] Create minimal HTML with generated classes
- [ ] Try to build/compile CSS
- [ ] Check if styles actually apply
- [ ] Look for CSS purging/tree-shaking configs

### Documentation Checks
- [ ] Is CSS framework mentioned in README?
- [ ] Are configuration steps documented?
- [ ] Are custom color values provided?
- [ ] Is build process explained?

## Common Mistakes

### 1. Assuming Standard Colors
```rust
// Code assumes these exist in Tailwind
"jupiter-blue-500" // ❌ Custom color
"blue-500"        // ✅ Standard Tailwind
```

### 2. Missing Purge Configuration
```javascript
// Tailwind needs to know about Rust-generated classes
content: [
  './src/**/*.rs',  // Often forgotten!
  './src/**/*.html',
]
```

### 3. Incomplete Color Scales
```javascript
// Only defining one shade
'jupiter-blue': {
  500: '#3b82f6'  // Only this
}
// But code uses jupiter-blue-300, jupiter-blue-700, etc.
```

## Recovery Strategy

When you find CSS framework dependencies:

1. **Document immediately** in implementation notes
2. **Provide complete configuration** examples
3. **Test with minimal setup** to verify
4. **Include troubleshooting** for common issues
5. **Consider providing** pre-built CSS option

## Key Lesson

**A design system that returns CSS class strings has a hard dependency on a CSS framework configuration.** This is rarely documented but always required.

Today's discovery: Jupiter Design System appears standalone but actually requires:
- Tailwind CSS installation
- Custom color configuration
- Build process setup
- Content path configuration

Without these, it generates useless strings.