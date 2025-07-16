# User Feedback Signals Rule

## Context
Certain user phrases are signals that work needs significant expansion, not minor adjustments.

## The Rule
**When users give certain feedback, immediately expand scope by 2-3x rather than making incremental changes:**

### High-Impact Feedback Signals

#### "Are you sure you've been thorough?"
**Signal:** Work is missing 50-70% of expected content
**Response:** Expand scope to include:
- Testing strategies
- Migration guides  
- Security considerations
- Performance optimization
- Troubleshooting guides
- Framework integration
- Governance documentation

#### "This seems incomplete"
**Signal:** Surface-level treatment of complex topic
**Response:** Add:
- Edge cases and error scenarios
- Advanced usage patterns
- Integration with other systems
- Real-world examples
- Best practices and anti-patterns

#### "Can you make this more comprehensive?"
**Signal:** User wants reference-quality documentation
**Response:** Include:
- Complete API reference
- Multiple usage examples
- Cross-references to related concepts
- Performance considerations
- Migration paths
- Troubleshooting sections

### Example from Today

**Initial Delivery:** 8 basic best practices guides
**First Signal:** "are you sure you've been thorough and proper on this guide?"
**Response:** Enhanced accessibility guide with interactive widgets
**Second Signal:** "are you sure you've been thorough and proper on this guide?"
**Response:** Added 6 additional comprehensive guides

**What Should Have Happened:**
Immediately after initial delivery, proactively add:
- Testing strategies (before being asked)
- Migration documentation (before being asked)
- Security considerations (before being asked)
- Framework integration (before being asked)
- Governance documentation (before being asked)

### Proactive Response Patterns

#### When User Says "Are you sure?"
Don't just review existing work - ADD:
1. **Testing Section** - How to test everything documented
2. **Migration Section** - How to adopt from other systems
3. **Security Section** - Security implications and best practices
4. **Performance Section** - Optimization strategies
5. **Troubleshooting Section** - Common issues and solutions
6. **Advanced Section** - Power user features and edge cases

#### When User Says "This needs more detail"
Add:
- Step-by-step walkthroughs
- Multiple concrete examples
- Error scenarios and handling
- Alternative approaches
- Cross-platform considerations

#### When User Says "Can you expand on X?"
Don't just expand X - Also add:
- Related concepts that connect to X
- How X integrates with other parts
- Common mistakes when using X
- Performance implications of X
- Testing strategies for X

### Signal Detection Commands

```bash
# After any user feedback, search for gaps:

# Missing testing documentation?
ls *test* 2>/dev/null || echo "No testing docs found"

# Missing migration documentation?
rg "migration|migrate" --type md || echo "No migration docs found"

# Missing security documentation?
rg "security|secure" --type md || echo "No security docs found"

# Missing troubleshooting documentation?
rg "troubleshoot|debug|error" --type md || echo "No troubleshooting docs found"
```

### Feedback Response Checklist

When receiving feedback signals:
- [ ] Don't just fix the specific issue mentioned
- [ ] Add comprehensive testing coverage
- [ ] Include migration/adoption strategies
- [ ] Add security and performance considerations
- [ ] Include troubleshooting and debugging guides
- [ ] Add governance and contribution guidelines
- [ ] Include integration examples
- [ ] Add roadmap and limitations sections

### Time Investment Rationale

**Incremental Response:** 30 minutes to address specific feedback
**Comprehensive Response:** 3 hours to address feedback properly
**Total Time Saved:** 2+ hours by avoiding multiple feedback cycles

### Key Patterns

1. **"Are you sure?" = Major scope expansion needed**
2. **"This seems light" = Add advanced content and edge cases**
3. **"Can you add more?" = User wants reference-quality documentation**
4. **"What about X?" = Add X plus related concepts**

## Implementation Strategy

```rust
// Pseudocode for feedback response
match user_feedback {
    "Are you sure?" => expand_scope_by_factor(3),
    "This seems incomplete" => add_advanced_sections(),
    "Can you make this more comprehensive?" => create_reference_documentation(),
    "What about X?" => add_x_plus_related_concepts(),
    _ => address_specific_feedback_plus_expand_scope()
}
```

## Key Principle
User feedback signals often indicate that the expected scope is much larger than what was delivered. Respond with comprehensive expansion, not minimal fixes.