# Anticipate Documentation Scope Rule

## Context
When asked to create documentation, the request often implies a comprehensive documentation suite, not just surface-level guides.

## The Rule
**For any documentation request, ALWAYS include these components unless explicitly told otherwise:**

### Core Documentation Set
1. **Overview & Getting Started** - Philosophy and quick wins
2. **Core Concepts** - Architecture and design decisions
3. **API Reference** - Complete public API documentation
4. **Best Practices** - Patterns and anti-patterns
5. **Testing Strategies** - How to test implementations
6. **Migration Guide** - Moving from other solutions
7. **Performance Guide** - Optimization strategies
8. **Security Guide** - Security considerations and best practices
9. **Troubleshooting** - Common issues and solutions
10. **Governance** - Contribution and maintenance processes

### Framework-Specific Additions
- Integration examples for popular frameworks
- Framework-specific performance considerations
- Ecosystem compatibility matrices

## Example from Today
**Initial Request:** "Create extensive documentation on jupiter design system best practices"

**What Was Delivered (First Pass):**
- Basic guides for colors, typography, spacing, components

**What Was Actually Needed (After Two Prompts):**
- Testing strategies with complete test examples
- Migration guide with systematic approaches
- Security guide with XSS prevention
- Framework integration for Dioxus, Yew, Leptos
- Governance with quality gates
- Troubleshooting with debug utilities

## Red Flags That Scope Is Too Narrow
- Documentation only covers "happy path" scenarios
- No mention of testing or validation
- Missing migration or adoption strategies
- No security or performance considerations
- Lacks troubleshooting or debugging guides

## Proactive Scope Expansion
When creating documentation, ask yourself:
- "How would someone test this?"
- "How would someone migrate to this?"
- "What could go wrong?"
- "How does this perform at scale?"
- "What security issues should users consider?"

## Implementation Checklist
- [ ] Core concept documentation
- [ ] Practical usage examples
- [ ] Testing approaches
- [ ] Migration strategies
- [ ] Performance considerations
- [ ] Security implications
- [ ] Troubleshooting guides
- [ ] Governance/contribution docs