# Reality-Based Documentation Rule

## Context
Documentation must reflect actual implementation reality, including limitations, workarounds, and incomplete features.

## The Rule
**Document the system as it IS, not as it SHOULD BE:**

### Reality Checks

1. **Check for TODOs and FIXMEs**
   ```bash
   # These indicate incomplete features
   rg "TODO|FIXME|HACK|XXX" --type rust
   
   # Document these as "Known Limitations"
   rg "unimplemented!|unreachable!|panic!" --type rust
   ```

2. **Verify Feature Completeness**
   ```bash
   # Look for feature flags or conditional compilation
   rg "#\[cfg\(" --type rust
   
   # Check for deprecated items
   rg "#\[deprecated" --type rust
   ```

3. **Find Workarounds**
   ```bash
   # Comments often reveal workarounds
   rg "workaround|hack|temporary|TEMP" --type rust -i
   
   # Long comments might explain limitations
   rg "// .{50,}" --type rust
   ```

## Example from Today

**Initial Documentation:** "The Color system provides semantic color values for all UI needs"

**Reality Check Revealed:**
```bash
rg "Color::" --type rust tests/

# Found: Color::Accent used in tests but not in main enum
# Found: Gradient colors mentioned but not implemented
# Found: TODO: Add color mixing functionality
```

**Corrected Documentation:** "The Color system provides core semantic colors (Primary, Secondary, Success, Error, Warning). Note: Color::Accent is currently test-only. Gradient support is planned but not yet implemented."

## Documentation Sections for Reality

### "Current Limitations" Section
Always include this section with:
- Features that are partially implemented
- Known performance bottlenecks  
- API inconsistencies
- Platform-specific issues

### "Workarounds" Section
Document temporary solutions:
- How to work around current limitations
- Alternative approaches until features are complete
- Version-specific considerations

### "Roadmap" Section
Be honest about future plans:
- Features that are planned but not implemented
- Breaking changes under consideration
- Deprecated features and migration timeline

## Reality Detection Patterns

```bash
# Find incomplete implementations
rg "impl.*for.*\{[\s]*\}" --type rust

# Find error-prone areas
rg "unwrap\(\)|expect\(" --type rust --count

# Find version-specific code
rg "@since|@version|Since:|Version:" --type rust

# Find experimental features
rg "experimental|unstable|nightly" --type rust -i
```

## Example Reality-Based Documentation

```markdown
## Color System

The Jupiter Design System provides semantic colors through the Color enum.

### Currently Implemented
- Color::Primary - Main brand color
- Color::Secondary - Secondary brand color  
- Color::Success - Success state (green)
- Color::Error - Error state (red)

### Known Limitations
- Color::Accent exists in tests but not in public API (see issue #45)
- No color mixing or manipulation functions yet
- Gradients are referenced but not implemented
- Dark mode color mappings are hardcoded

### Workarounds
- For accent colors, use Color::Secondary with opacity
- For gradients, use CSS gradients directly
- For color manipulation, use external crates like `palette`

### Roadmap
- v0.2.0: Add Color::Accent to public API
- v0.3.0: Implement color mixing functions
- v0.4.0: Add gradient support
```

## Key Principle
Users trust honest documentation that acknowledges limitations more than perfect documentation that doesn't match reality. Always err on the side of transparency.