# Implementation Notes & Configuration

This document covers important implementation details and configuration requirements for the Jupiter Design System color system.

## ⚠️ Critical Configuration Requirements

### Tailwind CSS Configuration Required

The Jupiter Design System references custom color names (`jupiter-blue-500`, `jupiter-green-500`, etc.) that **must be configured in your Tailwind CSS setup**. Without this configuration, the generated CSS classes will not work.

#### Required Tailwind Config

Add this to your `tailwind.config.js`:

```javascript
module.exports = {
  // ... other config
  theme: {
    extend: {
      colors: {
        'jupiter-blue': {
          50: '#eff6ff',
          100: '#dbeafe',
          200: '#bfdbfe',
          300: '#93c5fd',
          400: '#60a5fa',
          500: '#3b82f6',  // Primary Jupiter blue
          600: '#2563eb',
          700: '#1d4ed8',
          800: '#1e40af',
          900: '#1e3a8a',  // Used in branded gradients
        },
        'jupiter-green': {
          50: '#f0fdf4',
          100: '#dcfce7',
          200: '#bbf7d0',
          300: '#86efac',
          400: '#4ade80',
          500: '#22c55e',  // Primary Jupiter green
          600: '#16a34a',
          700: '#15803d',
          800: '#166534',
          900: '#14532d',
        },
        'jupiter-orange': {
          50: '#fff7ed',
          100: '#ffedd5',
          200: '#fed7aa',
          300: '#fdba74',
          400: '#fb923c',
          500: '#f97316',  // Primary Jupiter orange
          600: '#ea580c',
          700: '#c2410c',
          800: '#9a3412',
          900: '#7c2d12',
        },
        'jupiter-navy': {
          50: '#f8fafc',
          100: '#f1f5f9',
          200: '#e2e8f0',
          300: '#cbd5e1',
          400: '#94a3b8',
          500: '#64748b',  // Primary Jupiter navy
          600: '#475569',
          700: '#334155',
          800: '#1e293b',
          900: '#0f172a',  // Used in branded gradients
        },
      },
    },
  },
  // ... rest of config
}
```

#### Alternative: CSS Custom Properties

If you prefer CSS custom properties, add this to your CSS:

```css
:root {
  /* Jupiter Blue Scale */
  --jupiter-blue-50: #eff6ff;
  --jupiter-blue-100: #dbeafe;
  --jupiter-blue-200: #bfdbfe;
  --jupiter-blue-300: #93c5fd;
  --jupiter-blue-400: #60a5fa;
  --jupiter-blue-500: #3b82f6;
  --jupiter-blue-600: #2563eb;
  --jupiter-blue-700: #1d4ed8;
  --jupiter-blue-800: #1e40af;
  --jupiter-blue-900: #1e3a8a;

  /* Jupiter Green Scale */
  --jupiter-green-50: #f0fdf4;
  --jupiter-green-100: #dcfce7;
  --jupiter-green-200: #bbf7d0;
  --jupiter-green-300: #86efac;
  --jupiter-green-400: #4ade80;
  --jupiter-green-500: #22c55e;
  --jupiter-green-600: #16a34a;
  --jupiter-green-700: #15803d;
  --jupiter-green-800: #166534;
  --jupiter-green-900: #14532d;

  /* Jupiter Orange Scale */
  --jupiter-orange-50: #fff7ed;
  --jupiter-orange-100: #ffedd5;
  --jupiter-orange-200: #fed7aa;
  --jupiter-orange-300: #fdba74;
  --jupiter-orange-400: #fb923c;
  --jupiter-orange-500: #f97316;
  --jupiter-orange-600: #ea580c;
  --jupiter-orange-700: #c2410c;
  --jupiter-orange-800: #9a3412;
  --jupiter-orange-900: #7c2d12;
}

/* Tailwind utilities */
.text-jupiter-blue-500 { color: var(--jupiter-blue-500); }
.bg-jupiter-blue-500 { background-color: var(--jupiter-blue-500); }
.border-jupiter-blue-500 { border-color: var(--jupiter-blue-500); }

/* Repeat for all Jupiter colors and scales... */
```

## Implementation Details

### Color Resolution Flow

```rust
// 1. Component requests a color
let theme = VibeColors::default();
let primary_class = theme.text_class(Color::Primary);

// 2. ColorProvider resolves the token
// Color::Primary -> "jupiter-blue-500"

// 3. CSS class is generated
// "jupiter-blue-500" -> "text-jupiter-blue-500"

// 4. Tailwind/CSS resolves the final color
// "text-jupiter-blue-500" -> color: #3b82f6
```

### Missing Features in Current Implementation

The following features are referenced in the codebase but not fully implemented:

#### 1. Theme Management System
```rust
// Referenced but minimal implementation
pub struct VibeTheme {
    colors: VibeColors,
}

impl Theme for VibeTheme {
    fn name(&self) -> &str {
        "Jupiter"
    }
}
```

#### 2. Theme Registry
```rust
// Available but only returns one theme
pub fn available_themes() -> Vec<&'static str> {
    vec!["jupiter"] // Only Jupiter theme available now
}
```

### Color Value Mapping

The Jupiter colors are currently mapped to standard Tailwind CSS values:

| Jupiter Token | Equivalent Tailwind | Hex Value |
|---------------|-------------------|-----------|
| `jupiter-blue-500` | `blue-500` | `#3b82f6` |
| `jupiter-green-500` | `green-500` | `#22c55e` |
| `jupiter-orange-500` | `orange-500` | `#f97316` |

This means Jupiter colors are essentially branded aliases for Tailwind's default colors.

### Serialization Support

All color structures support JSON serialization:

```rust
use serde_json;

// Serialize a palette
let theme = VibeColors::default();
let json = serde_json::to_string(theme.palette())?;

// Serialize individual color tokens
let color_json = serde_json::to_string(&Color::Primary)?;
```

### Testing Infrastructure

The color system includes comprehensive tests in `src/core/color_test.rs`:

- Color enum variant testing
- Palette default verification
- Color resolution functionality
- CSS class generation
- Override system validation

## Common Integration Issues

### Issue 1: Missing CSS Classes

**Problem**: `text-jupiter-blue-500` class doesn't apply styles

**Solution**: Configure Tailwind CSS as shown above

### Issue 2: Inconsistent Colors

**Problem**: Colors don't match between components

**Solution**: Always use the `ColorProvider` methods instead of hardcoding:

```rust
// Wrong
let style = "text-blue-500";

// Correct
let style = theme.text_class(Color::Primary);
```

### Issue 3: Theme Switching Not Working

**Problem**: Custom themes don't override colors properly

**Solution**: Use the override system correctly:

```rust
let custom_theme = VibeColors::with_overrides(|palette| {
    palette.primary = "purple-600".to_string(); // Must be valid Tailwind class
});
```

## Build Integration

### Rust Project Setup

```toml
# Cargo.toml
[dependencies]
jupiter-design-system = { path = "." }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Web Framework Integration

#### Next.js Example

```javascript
// next.config.js
module.exports = {
  // ... other config
  experimental: {
    // Enable if using app directory
    appDir: true,
  },
}

// tailwind.config.js
module.exports = {
  content: [
    './pages/**/*.{js,ts,jsx,tsx}',
    './components/**/*.{js,ts,jsx,tsx}',
    // Include Rust-generated HTML if applicable
    './rust-generated/**/*.html',
  ],
  theme: {
    extend: {
      colors: {
        // Jupiter colors configuration (as shown above)
      },
    },
  },
}
```

#### Vite Example

```javascript
// vite.config.js
import { defineConfig } from 'vite'

export default defineConfig({
  css: {
    postcss: {
      plugins: [
        require('tailwindcss'),
        require('autoprefixer'),
      ],
    },
  },
})
```

## Performance Considerations

### Theme Instance Caching

```rust
use std::sync::OnceLock;

static DEFAULT_THEME: OnceLock<VibeColors> = OnceLock::new();

fn get_default_theme() -> &'static VibeColors {
    DEFAULT_THEME.get_or_init(|| VibeColors::default())
}
```

### Color Class Memoization

```rust
use std::collections::HashMap;

struct ColorClassCache {
    text_classes: HashMap<Color, String>,
    bg_classes: HashMap<Color, String>,
    border_classes: HashMap<Color, String>,
}

impl ColorClassCache {
    fn new(theme: &impl ColorProvider) -> Self {
        let mut cache = Self {
            text_classes: HashMap::new(),
            bg_classes: HashMap::new(),
            border_classes: HashMap::new(),
        };
        
        // Pre-compute all classes
        for color in ALL_COLORS {
            cache.text_classes.insert(color, theme.text_class(color));
            cache.bg_classes.insert(color, theme.bg_class(color));
            cache.border_classes.insert(color, theme.border_class(color));
        }
        
        cache
    }
}
```

## Migration Path

### From Hardcoded Colors

1. **Identify hardcoded colors** in your codebase
2. **Map to semantic tokens** using the token reference
3. **Replace with ColorProvider calls**
4. **Configure Tailwind** with Jupiter colors
5. **Test theme switching** to ensure flexibility

### From Other Design Systems

1. **Map existing tokens** to Jupiter Color enum
2. **Create custom overrides** for brand-specific colors
3. **Update component interfaces** to accept ColorProvider
4. **Gradually migrate** components one by one

This implementation guide should help you properly configure and use the Jupiter Design System color system in your projects.