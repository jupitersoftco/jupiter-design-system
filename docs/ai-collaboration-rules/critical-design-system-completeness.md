# Critical Rule: Design System Completeness Check

## Overview
Before documenting any design system, verify it's actually usable. A well-architected system that generates non-functional output is worse than no system at all.

## The Problem
Today's work revealed that hours of documentation effort was partially wasted because:
- The Jupiter Design System generates CSS classes that don't exist
- Required Tailwind configuration is missing
- No setup documentation exists
- The system is unusable without undocumented dependencies

## Pre-Documentation Checklist

### 1. Verify Basic Functionality
```bash
# Create minimal test project
mkdir test-integration && cd test-integration

# Initialize with the design system
cargo init
echo '[dependencies]' >> Cargo.toml
echo 'jupiter-design-system = { path = ".." }' >> Cargo.toml

# Create minimal usage
cat > src/main.rs << 'EOF'
use jupiter_design_system::prelude::*;
fn main() {
    let theme = VibeColors::default();
    let button_class = theme.bg_class(Color::Primary);
    println!("Generated class: {}", button_class);
    
    // Generate HTML to test
    let html = format!(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <style>/* Where are the styles? */</style>
    </head>
    <body>
        <button class="{}">Test Button</button>
    </body>
    </html>
    "#, button_class);
    
    std::fs::write("test.html", html).unwrap();
}
EOF

cargo run
open test.html  # Does the button have styling?
```

### 2. Check for CSS Pipeline
```bash
# Essential files for CSS generation
REQUIRED_FILES=(
    "package.json"
    "tailwind.config.js"
    "postcss.config.js"
    "webpack.config.js|vite.config.js|rollup.config.js"
)

for file_pattern in "${REQUIRED_FILES[@]}"; do
    if ! ls $file_pattern 2>/dev/null; then
        echo "❌ Missing: $file_pattern"
    fi
done

# Check for CSS build scripts
if [ -f package.json ]; then
    grep -E "build.*css|css.*build|tailwind|postcss" package.json || echo "❌ No CSS build scripts"
fi
```

### 3. Trace Generated Classes
```bash
# What classes does the system generate?
grep -r "format!" . --include="*.rs" | grep -E "text-|bg-|border-" | head -20

# Are these standard or custom?
# Custom patterns to look for:
grep -r "jupiter-|brand-|custom-" . --include="*.rs" | grep -o '"[^"]*"' | sort -u

# If custom classes found, check for their definitions
CUSTOM_PREFIX="jupiter"
echo "Searching for $CUSTOM_PREFIX class definitions..."
grep -r "$CUSTOM_PREFIX" . --include="*.js" --include="*.css" --include="*.json" || echo "❌ No definitions found!"
```

### 4. Verify Documentation Completeness
```bash
# Look for setup instructions
grep -r "setup\|install\|getting started" . --include="*.md" -i

# Check for configuration examples
find . -name "*.example" -o -name "*.sample" -o -name "*example*" -type f

# Look for integration guides
ls docs/ | grep -iE "setup|install|integration|guide|start"
```

## Red Flags Checklist

- [ ] **No package.json** - If generating CSS classes, need CSS tooling
- [ ] **Custom class prefixes** - Need configuration for non-standard classes
- [ ] **No build directory** - Where does compiled CSS go?
- [ ] **No example app** - How do users actually integrate this?
- [ ] **API-only examples** - Show usage but not integration
- [ ] **No CSS imports** - How do styles get included?

## Required Documentation

If building a design system, MUST document:

1. **Prerequisites**
   ```markdown
   ## Prerequisites
   - Node.js and npm (for CSS tooling)
   - Rust (for the library)
   - Tailwind CSS configuration
   ```

2. **Setup Steps**
   ```markdown
   ## Setup
   1. Install dependencies: `npm install`
   2. Configure Tailwind: Copy `tailwind.config.example.js`
   3. Build CSS: `npm run build-css`
   4. Include CSS: `<link rel="stylesheet" href="dist/jupiter.css">`
   ```

3. **Integration Example**
   ```markdown
   ## Complete Example
   See `examples/complete-app/` for a working integration
   ```

## Today's Lesson

We documented an elegant color system with:
- 19 semantic color tokens
- Custom Jupiter color families
- Theme customization
- Type-safe implementation

But it's all useless because:
- Classes like `text-jupiter-blue-500` aren't defined anywhere
- No Tailwind configuration exists
- No build process documented
- Users can't actually use it

## The Rule

**Before any design system documentation:**
1. Build a minimal working app
2. Verify styles actually render
3. Document the COMPLETE setup process
4. Include all configuration files

**Time spent on setup verification: 30 minutes**
**Time wasted on unusable documentation: 2+ hours**

Always verify completeness first.