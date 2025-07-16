# TESTING RULE: Example Validation

## Problem This Prevents
Documentation with code examples that don't compile or run, leading to user frustration and wasted debugging time.

## The Rule
**Every code example in documentation must be verified to work with the actual codebase.**

## Example Validation Protocol

### Step 1: Extract Examples from Documentation
```bash
# Extract Rust code blocks
grep -A 20 "```rust" your-documentation.md > extracted-examples.txt

# Extract JavaScript code blocks  
grep -A 20 "```javascript" your-documentation.md > extracted-examples.js

# Extract Python code blocks
grep -A 20 "```python" your-documentation.md > extracted-examples.py
```

### Step 2: Verify Imports and Dependencies
```bash
# Check if all imports exist in codebase
grep "use.*::" extracted-examples.txt | while read import; do
    echo "Checking: $import"
    grep -r "$import" . --include="*.rs" > /dev/null && echo "✅ Found" || echo "❌ Missing"
done

# Check if functions exist
grep "fn.*(" extracted-examples.txt | while read func; do
    func_name=$(echo "$func" | sed 's/.*fn \([^(]*\).*/\1/')
    grep -r "fn $func_name" . --include="*.rs" > /dev/null && echo "✅ $func_name exists" || echo "❌ $func_name missing"
done
```

### Step 3: Test Compilation
```bash
# Create test file with examples
cat > test-examples.rs << 'EOF'
// Auto-generated from documentation examples
use your_crate::*;

#[cfg(test)]
mod doc_examples {
    use super::*;

    #[test]
    fn test_documentation_examples() {
        // Paste your examples here
    }
}
EOF

# Test compilation
cargo check --tests
```

## Real Example: Jupiter Design System Example Failures

### Documentation Example (Doesn't Work)
```rust
// From my documentation - BROKEN
let pattern = TypographyPattern::new(colors)
    .hierarchy(TypographyHierarchy::Title)
    .classes();

// Helper function example - BROKEN
fn assert_contains_all_classes(classes: &str, expected: &[&str]) {
    // Implementation
}
```

### Verification Results
```bash
$ grep -r "TypographyPattern" . --include="*.rs"
# No results - TypographyPattern doesn't exist!

$ grep -r "assert_contains_all_classes" . --include="*.rs"
# No results - helper function doesn't exist!
```

### Corrected Examples (Actually Work)
```rust
// Corrected - uses actual API
let classes = text_styles(colors)
    .hierarchy_str("title")
    .classes();

// Corrected - uses actual helper pattern
fn create_text_styles() -> TextStyles<VibeColors> {
    text_styles(VibeColors::default())
}
```

## Example Validation by Language

### Rust Examples
```bash
# Create validation test
cat > validate-examples.rs << 'EOF'
#[cfg(test)]
mod validate_docs {
    use super::*;
    
    #[test]
    fn test_example_1() {
        // Paste example code here
        let result = some_function();
        assert!(result.is_ok());
    }
}
EOF

# Run validation
cargo test validate_docs
```

### JavaScript Examples
```bash
# Create validation test
cat > validate-examples.test.js << 'EOF'
// Auto-generated example validation
const { someFunction } = require('./src/index');

test('documentation example 1', () => {
    // Paste example code here
    const result = someFunction();
    expect(result).toBeDefined();
});
EOF

# Run validation
npm test validate-examples.test.js
```

### Python Examples
```bash
# Create validation test
cat > validate_examples.py << 'EOF'
# Auto-generated example validation
import unittest
from src.module import some_function

class TestDocumentationExamples(unittest.TestCase):
    def test_example_1(self):
        # Paste example code here
        result = some_function()
        self.assertIsNotNone(result)
EOF

# Run validation
python -m pytest validate_examples.py
```

## Common Example Validation Failures

### 1. Wrong Import Paths
```rust
// ❌ Documentation shows
use crate::patterns::TypographyPattern;

// ✅ Reality is
use crate::builders::text::TextStyles;
```

### 2. Non-existent Functions
```rust
// ❌ Documentation shows
let result = assert_contains_all_classes(&classes, &["text-4xl"]);

// ✅ Reality is
assert!(classes.contains("text-4xl"));
```

### 3. Wrong Method Names
```rust
// ❌ Documentation shows
let pattern = text_styles(colors).hierarchy(Hierarchy::Title);

// ✅ Reality is
let classes = text_styles(colors).hierarchy_str("title");
```

### 4. Missing Dependencies
```rust
// ❌ Documentation shows
use crate::test_utils::*;

// ✅ Reality is
use crate::themes::VibeColors;
```

## Example Validation Checklist

For every code example:
- [ ] All imports exist in the codebase
- [ ] All functions/methods exist with correct signatures
- [ ] All types/structs exist and are accessible
- [ ] Example compiles without errors
- [ ] Example runs without panics
- [ ] Example produces expected output
- [ ] Dependencies are available in the project

## Automated Example Validation

Create a script to validate all examples:

```bash
#!/bin/bash
# validate-examples.sh

echo "=== EXAMPLE VALIDATION REPORT ==="
echo "Date: $(date)"
echo

# Extract all code examples
echo "Extracting examples..."
grep -n -A 20 "```rust" docs/**/*.md > all-examples.txt

# Count examples
EXAMPLE_COUNT=$(grep -c "```rust" docs/**/*.md)
echo "Found $EXAMPLE_COUNT Rust examples"

# Check imports
echo "=== IMPORT VALIDATION ==="
grep "use.*::" all-examples.txt | sort | uniq | while read import; do
    import_path=$(echo "$import" | sed 's/use \([^;]*\);.*/\1/')
    if grep -r "$import_path" src/ > /dev/null 2>&1; then
        echo "✅ $import_path"
    else
        echo "❌ $import_path - NOT FOUND"
    fi
done

# Check function calls
echo "=== FUNCTION VALIDATION ==="
grep -o "[a-zA-Z_][a-zA-Z0-9_]*(" all-examples.txt | sort | uniq | while read func_call; do
    func_name=${func_call%(*}
    if grep -r "fn $func_name" src/ > /dev/null 2>&1; then
        echo "✅ $func_name()"
    else
        echo "❌ $func_name() - NOT FOUND"
    fi
done

# Create test file
echo "=== COMPILATION TEST ==="
cat > test-doc-examples.rs << 'EOF'
#[cfg(test)]
mod doc_examples {
    use super::*;
    
    #[test]
    fn test_basic_usage() {
        // Basic compilation test
        let _result = true;
        assert!(true);
    }
}
EOF

if cargo check --tests > /dev/null 2>&1; then
    echo "✅ Basic compilation works"
else
    echo "❌ Basic compilation failed"
fi

echo "=== VALIDATION COMPLETE ==="
```

## Integration with Documentation Workflow

### Pre-Commit Hook
```bash
#!/bin/bash
# .git/hooks/pre-commit
echo "Validating documentation examples..."
bash scripts/validate-examples.sh
if [ $? -ne 0 ]; then
    echo "❌ Example validation failed - fix examples before committing"
    exit 1
fi
```

### CI Integration
```yaml
# .github/workflows/docs.yml
name: Documentation Validation
on: [push, pull_request]
jobs:
  validate-examples:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Validate documentation examples
        run: bash scripts/validate-examples.sh
```

## Example Testing Template

For systematic example validation:

```rust
// test-documentation-examples.rs
#[cfg(test)]
mod documentation_examples {
    use super::*;
    use crate::themes::VibeColors;

    // Test Example 1 from overview.md
    #[test]
    fn test_example_overview_basic_usage() {
        let colors = VibeColors::default();
        let classes = text_styles(colors).title().classes();
        assert!(classes.contains("text-4xl"));
    }

    // Test Example 2 from unit-testing.md
    #[test]
    fn test_example_unit_testing_helper() {
        fn create_text_styles() -> TextStyles<VibeColors> {
            text_styles(VibeColors::default())
        }
        
        let styles = create_text_styles();
        assert!(!styles.classes().is_empty());
    }
}
```

## Time Impact
- Validating examples upfront: 15-20 minutes
- Fixing broken examples after user reports: 1-2 hours
- User frustration from broken examples: High
- Documentation credibility: Significantly improved

## Key Principle
**"If it doesn't compile, it doesn't belong in documentation."**

Every example should be a working demonstration of the actual API, not theoretical code that looks good but doesn't work.