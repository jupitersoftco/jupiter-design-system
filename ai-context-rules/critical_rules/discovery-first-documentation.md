# Discovery-First Documentation Rule

## Context
Before creating any documentation, comprehensive discovery prevents multiple revision cycles and ensures accuracy.

## The Rule
**ALWAYS perform exhaustive discovery before writing any documentation:**

1. **Source Code Analysis**
   ```bash
   # Find all source files
   find . -name "*.rs" -type f | grep -E "(src|lib)" | wc -l
   
   # Search for key implementations
   rg -t rust "pub struct|pub enum|pub trait" --stats
   
   # Count API surface area
   rg -t rust "^pub " --count-matches
   ```

2. **Test Coverage Discovery**
   ```bash
   # Find all test files
   find . -name "*test*.rs" -o -name "*tests*" -type d
   
   # Count actual tests
   rg "#\[test\]" --count
   
   # Check for integration tests
   ls tests/ 2>/dev/null || echo "No integration tests directory"
   ```

3. **Documentation Inventory**
   ```bash
   # Check for existing docs
   find . -name "*.md" -type f | grep -v target
   
   # Search for inline documentation
   rg "///" --type rust --stats
   ```

## Example from Today
**What Happened:** Created color system documentation without discovering:
- Multiple color enum variants beyond the documented ones
- Hidden builder patterns and trait implementations
- Gradient and selection ring features

**What Should Have Happened:**
```bash
# Comprehensive color system discovery
rg "Color::" --type rust -A 2 -B 2
rg "impl.*Color" --type rust
rg "color|Color" --type rust --stats
```

## Consequences of Not Following
- 4x time increase due to revisions
- User frustration from incomplete documentation
- Loss of credibility requiring multiple "are you sure?" prompts

## Validation Checklist
- [ ] Searched for all relevant structs/enums/traits
- [ ] Counted test coverage for documented features
- [ ] Verified no existing documentation covers the topic
- [ ] Mapped actual vs documented feature set
- [ ] Checked example usage in tests