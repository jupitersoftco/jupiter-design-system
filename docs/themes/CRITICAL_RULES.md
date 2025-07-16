# Critical Rules for Documentation Tasks

## Rule 1: Pre-Documentation Inventory is Mandatory

**When to Apply**: Before ANY documentation task claiming to be "comprehensive" or "complete"

**What to Do**:
```bash
# Step 1: Count all public APIs
find src -name "*.rs" -type f | xargs grep -E "pub (fn|struct|enum|trait)" | wc -l

# Step 2: List all modules
find src -type d | sort

# Step 3: Create inventory file
find src -name "*.rs" | xargs grep -E "pub (fn|struct|enum|trait)" > api_inventory.txt

# Step 4: Calculate documentation target
# If < 100 items: document all
# If > 100 items: create systematic plan
```

**Why Critical**: 
- Prevents claiming 10% coverage as "complete"
- Reveals true scope before starting
- Enables systematic approach

**Example Failure**: Today's session initially documented 2 traits out of 5, ~60 functions out of 616

---

## Rule 2: User Repetition = Stop and Audit

**When to Apply**: User asks same question 2+ times

**What to Do**:
1. STOP current approach
2. Acknowledge potential issue: "You've asked this multiple times. Let me audit..."
3. Run objective verification
4. Report findings honestly
5. Adjust approach based on findings

**Why Critical**:
- Users don't repeat without reason
- Indicates fundamental issue with approach
- Prevents defensive responses

**Example**: "Are you sure you've been thorough?" asked 5 times

---

## Rule 3: Never Trust Surface-Level File Inspection

**When to Apply**: Exploring any codebase structure

**What to Do**:
```bash
# BAD: Just looking at obvious files
ls src/
cat src/lib.rs

# GOOD: Systematic exploration
find src -name "*.rs" -type f | head -20
grep -r "pub mod" src/
grep -r "pub trait" src/
find src -name "mod.rs" | xargs grep "pub"
```

**Why Critical**:
- Modules can be deeply nested
- Public APIs can be re-exported
- Important code often in subdirectories

---

## Rule 4: Documentation Must Be Testable

**When to Apply**: Writing any code examples

**What to Do**:
1. Create test file with all examples
2. Verify compilation
3. Test actual output
4. Document verification status

```rust
// verification_test.rs
#[test]
fn verify_example_1() {
    // Paste example here
    // Should compile and run
}
```

**Why Critical**:
- Untested examples mislead users
- Compilation errors destroy credibility
- Testing reveals API misunderstandings

---

## Rule 5: Scope Creep Is Information

**When to Apply**: Task scope expanding during work

**What to Do**:
1. Document scope expansion immediately
2. Update task list
3. Communicate new scope to user
4. Get confirmation before proceeding

**Why Critical**:
- Original estimates become invalid
- Hidden complexity revealed
- User expectations must be reset

**Example**: "Document theme system" expanded from 2 traits to entire design system