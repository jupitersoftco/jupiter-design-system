# Workflow Rule: Example-First Development

## Overview
Always start by examining and running the examples directory. Examples reveal the intended usage patterns and often expose gaps between design and implementation.

## Why Examples First?

Examples are where theory meets reality. They show:
- Actual usage patterns (not theoretical)
- Integration requirements
- Hidden dependencies
- Missing functionality
- Real developer experience

## The Example-First Workflow

### Step 1: Find and Categorize Examples
```bash
# Find all examples
find . -name "examples" -type d
find . -name "*example*" -type f | grep -v ".git"
find . -name "demo" -o -name "sample" -type d

# Categorize by type
ls examples/ | while read example; do
    echo "=== $example ==="
    head -20 "examples/$example" | grep -E "//!|use|fn main"
done
```

### Step 2: Run Each Example
```bash
# For Rust examples
for example in examples/*.rs; do
    echo "Running $example..."
    rustc "$example" --edition 2021 -L target/debug/deps || echo "❌ Failed: $example"
done

# Or with cargo
cd examples/basic_usage
cargo run
```

### Step 3: Analyze Example Output
```bash
# What do examples actually produce?
cargo run --example basic_usage 2>&1 | tee example_output.txt

# Look for:
# - Generated values
# - Error messages  
# - Missing dependencies
# - Actual vs expected output
```

### Step 4: Identify Gaps
Compare examples with documentation:
- Do examples match documented API?
- Are there undocumented features in examples?
- Do examples actually work?
- What's missing for real-world usage?

## Today's Discovery Through Examples

### What We Found
```rust
// In basic_usage.rs
println!("Primary text: {}", colors.text_class(Color::Primary));
// Output: "text-jupiter-blue-500"

// In button_usage.rs  
let primary = ButtonStyles::new(colors.clone()).primary().classes();
// Discovered: Entire ButtonStyles builder pattern not documented!
```

### What Examples Revealed
1. **ButtonStyles Builder** - Complex builder pattern undocumented
2. **Real Usage Patterns** - Different from what we assumed
3. **Missing Integration** - Examples show API but not web integration
4. **Output Without Rendering** - Generates classes but no CSS

## Example Analysis Checklist

### For Each Example
- [ ] Does it compile and run?
- [ ] What does it output?
- [ ] Does output actually work (e.g., CSS classes)?
- [ ] What patterns does it demonstrate?
- [ ] What's missing for real usage?

### Red Flags in Examples
- [ ] Only shows API calls, no integration
- [ ] Prints output but doesn't use it
- [ ] No error handling
- [ ] No complete application
- [ ] Missing build/run instructions

## Example-Driven Documentation

### Good Example Structure
```
examples/
├── 01-basic-usage/        # Start simple
│   ├── Cargo.toml
│   ├── src/main.rs
│   └── README.md         # How to run
├── 02-complete-app/      # Full integration
│   ├── Cargo.toml
│   ├── package.json      # CSS dependencies
│   ├── tailwind.config.js
│   ├── src/main.rs
│   └── README.md
└── 03-custom-theme/      # Advanced usage
    └── ...
```

### What Examples Should Show
1. **Minimal Usage** - Simplest possible case
2. **Complete Integration** - With CSS pipeline
3. **Common Patterns** - Real-world scenarios
4. **Error Cases** - What can go wrong
5. **Performance** - Large-scale usage

## Commands for Example Discovery

### Find Undocumented Features
```bash
# Compare examples with main source
echo "=== Features in examples ==="
grep -h "use jupiter" examples/*.rs | sort -u

echo "=== Public API in source ==="
grep "pub fn\|pub struct\|pub trait" src/**/*.rs | grep -v test

# Find mismatches
diff <(grep -h "ButtonStyles\|CardStyles" examples/*.rs | sort -u) \
     <(grep "pub struct.*Styles" src/**/*.rs | sort -u)
```

### Verify Example Completeness
```bash
# Create example runner script
cat > run_all_examples.sh << 'EOF'
#!/bin/bash
FAILED=0
for example in examples/*.rs; do
    name=$(basename "$example" .rs)
    echo "Running $name..."
    if cargo run --example "$name" > /dev/null 2>&1; then
        echo "✅ $name"
    else
        echo "❌ $name"
        FAILED=$((FAILED + 1))
    fi
done
echo "Failed: $FAILED examples"
EOF

chmod +x run_all_examples.sh
./run_all_examples.sh
```

## Today's Lesson

Started with core implementation documentation, but examples revealed:
1. **ButtonStyles builder pattern** - Completely missed
2. **Different usage patterns** - Not what we expected
3. **Integration gaps** - Examples don't show full setup
4. **Non-functional output** - Classes don't work without config

Should have started with:
```bash
cd examples
ls -la
cargo run --example basic_usage
# Immediately would have seen ButtonStyles and integration issues
```

## The Rule

**Always start with examples:**
1. Run all examples first
2. Analyze their output
3. Identify gaps and patterns
4. Then document the system

**Examples are documentation** - they show what developers actually need to know.

Time saved by running examples first: **2+ hours**