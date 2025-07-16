# Debugging Rules for Documentation Projects

## Rule 1: When User Repeats Questions, Debug Your Process

**When to Apply**: User asks same question multiple times

**Debugging Steps**:
1. **Stop defensive responses** - User repetition indicates real issue
2. **Audit your claims** - What specifically did you claim?
3. **Verify with metrics** - Run objective measurements
4. **Document the gap** - What was missing?
5. **Adjust approach** - How to prevent recurrence?

**Example Debug Process**:
```bash
# User asked "Are you sure this is thorough?" 5 times
# Debug: What was claimed vs reality?

# Claimed: "Comprehensive documentation"
# Reality check:
find src -name "*.rs" | xargs grep "pub fn" | wc -l  # 616 functions
grep -c "pub fn" docs/themes/*.md                    # ~60 documented

# Gap: 90% of functions not documented
# Root cause: No systematic inventory
# Fix: Always run inventory first
```

**Why Important**:
- User repetition is a debugging signal
- Defensive responses hide real issues
- Objective verification reveals truth

---

## Rule 2: Debug Documentation Completeness Claims

**When to Apply**: Before claiming documentation is "complete" or "comprehensive"

**Debugging Checklist**:
```bash
# 1. Count what exists
find src -name "*.rs" | xargs grep -c "pub fn"
find src -name "*.rs" | xargs grep -c "pub struct"  
find src -name "*.rs" | xargs grep -c "pub enum"
find src -name "*.rs" | xargs grep -c "pub trait"

# 2. Count what's documented
grep -r "pub fn\|fn.*(" docs/ | wc -l
grep -r "struct\|enum\|trait" docs/ | wc -l

# 3. Calculate coverage
echo "Coverage: $(documented_count / total_count * 100)%"

# 4. Verify claims
if [ $coverage -lt 90 ]; then
    echo "ERROR: Coverage too low for 'comprehensive' claim"
fi
```

**Red Flags**:
- High-level claims without metrics
- Missing entire modules or subsystems
- No verification process
- Defensive responses to questions

---

## Rule 3: Debug Missing Context Issues

**When to Apply**: When documentation feels incomplete but you can't identify what's missing

**Debugging Process**:
```bash
# 1. Find all modules
find src -name "mod.rs" | xargs grep -H "pub mod"

# 2. Find all re-exports
grep -r "pub use" src/

# 3. Find all public traits
grep -r "pub trait" src/

# 4. Find all builder patterns
grep -r "Builder\|Pattern" src/

# 5. Find all convenience functions
grep -r "pub fn [a-z_]*(" src/ | grep -v "test"
```

**Common Missing Context**:
- Builder systems vs pattern systems
- Convenience functions vs core APIs
- Re-exported items vs original definitions
- Module organization vs public API

---

## Rule 4: Debug Example Failures

**When to Apply**: Code examples don't work as expected

**Debugging Process**:
```rust
// 1. Create minimal test case
#[test]
fn debug_example() {
    // Paste exact example from docs
    let colors = VibeColors::default();
    let button = primary_button(colors).classes();
    
    // Debug what's actually happening
    println!("Button classes: {}", button);
    println!("Colors: {:?}", colors);
    
    // Verify assumptions
    assert!(!button.is_empty(), "Button classes should not be empty");
    assert!(button.contains("bg-"), "Should contain background class");
}
```

**Common Issues**:
- Import path errors
- Generic type inference failures
- Trait bound mismatches
- API changes since documentation

---

## Rule 5: Debug Scope Creep

**When to Apply**: Task keeps expanding beyond initial scope

**Debugging Process**:
```markdown
# Initial scope
- Document ColorProvider trait
- Document Theme trait
- Provide basic examples

# Actual scope discovered
- 5 core traits (not 2)
- 10 pattern systems (not mentioned)
- 8 builder systems (not mentioned)
- 616 public functions (not mentioned)
- Framework integrations (not mentioned)
- Migration scenarios (not mentioned)
- Testing strategies (not mentioned)

# Root cause analysis
- Surface-level exploration
- No systematic inventory
- Assumptions about codebase structure
```

**Debug Questions**:
- What was the original scope?
- What new requirements emerged?
- Why wasn't this discovered earlier?
- How to prevent in future?

---

## Rule 6: Debug Framework Integration Issues

**When to Apply**: Framework examples don't work as expected

**Debugging Process**:
```javascript
// 1. Create minimal reproduction
// package.json
{
  "dependencies": {
    "jupiter-design-system": "latest"
  }
}

// 2. Test basic import
import { VibeColors, primary_button } from 'jupiter-design-system';

// 3. Test basic usage
const colors = new VibeColors();
const classes = primary_button(colors).classes();
console.log('Generated classes:', classes);

// 4. Debug common issues
// - Module not found
// - Type errors
// - Build failures
// - Runtime errors
```

**Common Issues**:
- Module resolution problems
- TypeScript type mismatches
- Build tool configuration
- Framework-specific quirks

---

## Rule 7: Debug Performance Claims

**When to Apply**: Making performance-related statements

**Debugging Process**:
```rust
use std::time::Instant;

#[test]
fn debug_performance_claim() {
    let colors = VibeColors::default();
    
    // Measure actual performance
    let start = Instant::now();
    
    for _ in 0..1000 {
        let _ = primary_button(colors.clone()).classes();
    }
    
    let elapsed = start.elapsed();
    println!("1000 iterations took: {:?}", elapsed);
    
    // Verify claims
    assert!(elapsed.as_millis() < 100, "Should be fast");
}
```

**Common Issues**:
- Unmeasured performance claims
- Unrealistic expectations
- Missing benchmark data
- Compiler optimization assumptions

---

## Rule 8: Debug API Design Assumptions

**When to Apply**: When APIs feel awkward or misunderstood

**Debugging Process**:
```rust
// Test different usage patterns
#[test]
fn debug_api_usage() {
    let colors = VibeColors::default();
    
    // Pattern 1: Direct usage
    let button1 = primary_button(colors.clone()).classes();
    
    // Pattern 2: Builder usage
    let button2 = button_styles(colors.clone())
        .variant(ButtonVariant::Primary)
        .classes();
    
    // Pattern 3: Composition
    let button3 = button_pattern(colors.clone())
        .primary_action()
        .classes();
    
    // Debug: Which pattern is most natural?
    println!("Pattern 1: {}", button1);
    println!("Pattern 2: {}", button2);
    println!("Pattern 3: {}", button3);
}
```

**Questions to Debug**:
- Are there multiple ways to do the same thing?
- Which approach is most intuitive?
- What are the trade-offs?
- How does this fit the overall design?

---

## Rule 9: Debug Test Failures

**When to Apply**: Tests fail unexpectedly

**Debugging Process**:
```rust
#[test]
fn debug_test_failure() {
    // 1. Isolate the failure
    let colors = VibeColors::default();
    let result = primary_button(colors).classes();
    
    // 2. Add debug output
    println!("Actual result: '{}'", result);
    println!("Result length: {}", result.len());
    println!("Contains 'bg-': {}", result.contains("bg-"));
    
    // 3. Test assumptions one by one
    assert!(!result.is_empty(), "Should not be empty");
    assert!(result.contains(" "), "Should contain spaces");
    assert!(result.contains("bg-"), "Should contain background");
    
    // 4. Check environment
    println!("Rust version: {}", env!("RUSTC_VERSION"));
    println!("Cargo features: {}", env!("CARGO_FEATURES"));
}
```

**Common Issues**:
- Environment differences
- Feature flag variations
- Dependency version mismatches
- Platform-specific behavior

---

## Rule 10: Debug User Experience Issues

**When to Apply**: Documentation is technically correct but users struggle

**Debugging Process**:
```markdown
# User journey analysis
1. User goal: "I want to create a custom theme"
2. Entry point: Where do they start?
3. First barrier: What stops them?
4. Information gaps: What's missing?
5. Cognitive load: Too much at once?

# Example debug
User says: "I don't know how to implement ColorProvider"

Debug questions:
- Is ColorProvider documented?
- Are examples provided?
- Are trait bounds explained?
- Is the purpose clear?
- Are common mistakes covered?
```

**Common Issues**:
- Assuming too much knowledge
- Missing prerequisite information
- Poor information architecture
- Lack of progressive disclosure
- No troubleshooting guidance

**Fix Pattern**:
1. Add more context
2. Provide stepping stones
3. Include troubleshooting sections
4. Test with naive users
5. Iterate based on feedback