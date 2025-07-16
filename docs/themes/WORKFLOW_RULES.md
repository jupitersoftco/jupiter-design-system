# Workflow Rules for Documentation Tasks

## Rule 1: Use TodoWrite Immediately for Multi-Step Tasks

**When to Apply**: Any task with 3+ distinct steps

**Pattern**:
```rust
// FIRST action in any complex task
TodoWrite::new([
    "Explore codebase structure",
    "Document core patterns", 
    "Create examples",
    "Verify examples work"
])
```

**Why Important**:
- Prevents forgetting subtasks
- Shows progress to user
- Reveals true scope early

**Today's Example**: System reminded about TodoWrite multiple times before it was used

---

## Rule 2: Documentation Structure Before Content

**When to Apply**: Creating documentation for complex systems

**Pattern**:
1. Create README with table of contents FIRST
2. Create empty files for each section
3. Fill sections systematically
4. Update README as structure evolves

```bash
# Create structure first
touch docs/architecture.md
touch docs/core-traits.md
touch docs/examples.md
# Then fill content
```

**Why Important**:
- Prevents ad-hoc organization
- Shows user what's coming
- Enables parallel work

---

## Rule 3: Grep-First, Read-Second

**When to Apply**: Exploring unfamiliar codebases

**Pattern**:
```bash
# Get overview first
grep -r "pub trait" src/
grep -r "pub fn.*pattern" src/
grep -r "pub struct.*Builder" src/

# Then read specific files
cat src/patterns/button.rs
```

**Why Important**:
- Reveals patterns before details
- Identifies key concepts quickly
- Prevents getting lost in implementation

---

## Rule 4: Example-Driven Documentation

**When to Apply**: Documenting APIs or libraries

**Pattern**:
1. Write example FIRST
2. Verify example works
3. Write documentation around example
4. Include example in docs

```rust
// Start with working example
let button = primary_button(colors).classes();

// Then document around it
/// Creates a primary button with default styling
/// 
/// # Example
/// ```rust
/// let button = primary_button(colors).classes();
/// ```
```

**Why Important**:
- Examples must work
- Shows actual usage
- Prevents theoretical documentation

---

## Rule 5: Systematic File Organization

**When to Apply**: Creating multiple documentation files

**Pattern**:
```
docs/
├── README.md           # Overview and navigation
├── architecture.md     # High-level design
├── core-concepts.md    # Essential understanding
├── api-reference.md    # Complete API listing
├── examples.md         # Working examples
├── integration.md      # Framework integration
├── testing.md          # Testing strategies
└── migration.md        # Migration guides
```

**Why Important**:
- Users can find information easily
- Logical progression of complexity
- Maintainable structure

---

## Rule 6: Cross-Reference Everything

**When to Apply**: Writing technical documentation

**Pattern**:
```markdown
The `primary_button` function (src/patterns/button.rs:370) creates...
```

**Why Important**:
- Enables verification
- Helps maintenance
- Shows credibility

---

## Rule 7: Metric-Based Progress

**When to Apply**: Large documentation tasks

**Pattern**:
```markdown
## Progress
- ✅ 5/5 Core traits documented
- ✅ 10/10 Pattern systems documented  
- ✅ 8/8 Builder systems documented
- ✅ 616/616 Public functions referenced
```

**Why Important**:
- Shows objective progress
- Identifies gaps
- Prevents premature completion claims