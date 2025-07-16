# Iterative Validation Pattern Rule

## Context
User feedback like "are you sure you've been thorough?" is a signal that the work needs significant expansion, not minor tweaks.

## The Rule
**Implement a three-pass validation approach for all deliverables:**

### Pass 1: Initial Implementation
Complete the basic requirements as understood.

### Pass 2: Self-Audit Against Common Omissions
Before considering the task complete, check for:
- Edge cases and error scenarios
- Advanced use cases beyond basics
- Integration with other systems
- Performance implications
- Security considerations
- Testing strategies
- Migration paths

### Pass 3: Proactive 2-3x Scope Expansion
Ask yourself: "If the user asks 'are you sure?', what would I add?"
Then add it preemptively.

## Validation Questions for Each Pass

### Pass 1 Questions
- Does this meet the literal requirements?
- Are the examples functional?
- Is the basic use case covered?

### Pass 2 Questions  
- What would break this in production?
- How would someone test this?
- What happens at scale?
- Are there security implications?
- How does this integrate with existing systems?

### Pass 3 Questions
- What would an expert add to this?
- What questions might users have that aren't answered?
- What related topics should be covered?
- What would make this truly comprehensive?

## Example from Today

**Pass 1 Output:** Basic best practices for colors, typography, components

**Pass 2 Additions (After First "Are you sure?"):**
- Enhanced accessibility with ARIA patterns
- Interactive widget examples
- Form accessibility deep dive

**Pass 3 Additions (After Second "Are you sure?"):**
- Complete testing strategies guide
- Migration guide from other systems
- Security considerations
- Framework integration examples
- Governance documentation

## Red Flags You Need Another Pass
- Documentation feels "light" or surface-level
- No mention of edge cases or errors
- Missing practical, real-world examples
- No discussion of tradeoffs or alternatives
- Lacks depth in any area

## Implementation Pattern
```rust
// Pseudocode for iterative validation
let initial_work = complete_basic_requirements();
let expanded_work = add_common_omissions(initial_work);
let comprehensive_work = expand_scope_2x(expanded_work);

// Always deliver comprehensive_work, not initial_work
```

## Key Insight
When a user says "are you sure you've been thorough?", they're not questioning your competence—they're signaling that the expected scope is much larger than what was delivered. Anticipate this and deliver comprehensive work from the start.