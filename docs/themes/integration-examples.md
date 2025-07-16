# Framework Integration Examples

This document shows how to integrate Jupiter Design System themes with popular web frameworks.

## React Integration

### Basic Setup
```jsx
// theme-context.jsx
import React, { createContext, useContext, useState } from 'react';

// Create theme instances
const themes = {
  vibe: new VibeColors(),
  corporate: new CorporateTheme('Acme Corp'),
  gaming: new GamingTheme(GamingIntensity.Standard),
  accessible: new AccessibilityTheme(ContrastLevel.AA),
};

// Create context
const ThemeContext = createContext(themes.vibe);

export function ThemeProvider({ children, defaultTheme = 'vibe' }) {
  const [currentTheme, setCurrentTheme] = useState(defaultTheme);
  
  return (
    <ThemeContext.Provider value={{ 
      theme: themes[currentTheme],
      setTheme: setCurrentTheme,
      themes: Object.keys(themes)
    }}>
      {children}
    </ThemeContext.Provider>
  );
}

export const useTheme = () => useContext(ThemeContext);
```

### Component Examples
```jsx
// Button.jsx
import { useTheme } from './theme-context';
import { primary_button, secondary_button, button_pattern } from 'jupiter-design-system';

export function Button({ 
  variant = 'primary', 
  size = 'medium',
  disabled = false,
  loading = false,
  onClick,
  children 
}) {
  const { theme } = useTheme();
  
  // Use pattern for semantic styling
  const classes = React.useMemo(() => {
    if (variant === 'primary') {
      return primary_button(theme)
        .disabled(disabled)
        .loading(loading)
        .classes();
    } else if (variant === 'secondary') {
      return secondary_button(theme)
        .disabled(disabled)
        .loading(loading)
        .classes();
    }
    
    // Custom composition
    return button_pattern(theme)
      .secondary_action()
      .disabled(disabled)
      .loading(loading)
      .classes();
  }, [theme, variant, disabled, loading]);
  
  return (
    <button 
      className={classes}
      disabled={disabled}
      onClick={onClick}
    >
      {loading && <Spinner />}
      {children}
    </button>
  );
}

// Typography.jsx
import { useTheme } from './theme-context';
import { 
  title_typography, 
  heading_typography, 
  body_typography 
} from 'jupiter-design-system';

export function Typography({ 
  variant = 'body',
  color = 'auto',
  align,
  children,
  as 
}) {
  const { theme } = useTheme();
  
  const { classes, element } = React.useMemo(() => {
    let pattern;
    
    switch (variant) {
      case 'title':
        pattern = title_typography(theme);
        break;
      case 'heading':
        pattern = heading_typography(theme);
        break;
      default:
        pattern = body_typography(theme);
    }
    
    if (color !== 'auto') {
      pattern = pattern.color(color);
    }
    if (align) {
      pattern = pattern.alignment(align);
    }
    
    return {
      classes: pattern.classes(),
      element: as || pattern.get_element()
    };
  }, [theme, variant, color, align, as]);
  
  const Component = element;
  
  return (
    <Component className={classes}>
      {children}
    </Component>
  );
}

// Card.jsx  
import { useTheme } from './theme-context';
import { card_pattern, CardElevation } from 'jupiter-design-system';

export function Card({ 
  elevation = 'raised',
  interactive = false,
  selected = false,
  children,
  onClick
}) {
  const { theme } = useTheme();
  
  const classes = React.useMemo(() => {
    return card_pattern(theme)
      .elevation(CardElevation[elevation])
      .interaction(interactive ? CardInteraction.Clickable : CardInteraction.Static)
      .selected(selected)
      .classes();
  }, [theme, elevation, interactive, selected]);
  
  return (
    <div 
      className={classes}
      onClick={onClick}
      role={interactive ? 'button' : undefined}
      tabIndex={interactive ? 0 : undefined}
    >
      {children}
    </div>
  );
}
```

### Complete App Example
```jsx
// App.jsx
import { ThemeProvider, useTheme } from './theme-context';
import { Button, Typography, Card } from './components';

function ThemeSelector() {
  const { themes, setTheme } = useTheme();
  
  return (
    <select onChange={(e) => setTheme(e.target.value)}>
      {themes.map(theme => (
        <option key={theme} value={theme}>{theme}</option>
      ))}
    </select>
  );
}

function ProductCard({ product }) {
  const { theme } = useTheme();
  
  // Use builders for fine control
  const priceClasses = text_styles(theme)
    .size(TextSize.LG)
    .weight(FontWeight.Bold)
    .color(Color.Primary)
    .classes();
    
  const badgeClasses = state_styles(theme)
    .success()
    .with_icon()
    .classes();
  
  return (
    <Card elevation="raised" interactive>
      <img src={product.image} alt={product.name} />
      <Typography variant="heading">{product.name}</Typography>
      <Typography>{product.description}</Typography>
      <div className={priceClasses}>${product.price}</div>
      {product.inStock && (
        <span className={badgeClasses}>In Stock</span>
      )}
      <Button variant="primary" onClick={() => addToCart(product)}>
        Add to Cart
      </Button>
    </Card>
  );
}

export function App() {
  return (
    <ThemeProvider defaultTheme="vibe">
      <div className="app">
        <header>
          <Typography variant="title">My Store</Typography>
          <ThemeSelector />
        </header>
        <main>
          <ProductGrid />
        </main>
      </div>
    </ThemeProvider>
  );
}
```

## Vue.js Integration

### Setup with Composition API
```vue
<!-- ThemeProvider.vue -->
<template>
  <div :class="backgroundClass">
    <slot />
  </div>
</template>

<script setup>
import { provide, ref, computed } from 'vue';
import { VibeColors, CorporateTheme } from 'jupiter-design-system';

const themes = {
  vibe: new VibeColors(),
  corporate: new CorporateTheme('Acme Corp'),
};

const currentTheme = ref('vibe');
const theme = computed(() => themes[currentTheme.value]);

const backgroundClass = computed(() => 
  theme.value.bg_class('Background')
);

// Provide theme to all children
provide('theme', {
  theme,
  setTheme: (name) => currentTheme.value = name,
  themes: Object.keys(themes)
});
</script>

<!-- useTheme.js -->
import { inject } from 'vue';

export function useTheme() {
  const themeContext = inject('theme');
  if (!themeContext) {
    throw new Error('useTheme must be used within ThemeProvider');
  }
  return themeContext;
}
```

### Component Examples
```vue
<!-- JupiterButton.vue -->
<template>
  <button 
    :class="classes" 
    :disabled="disabled"
    @click="$emit('click', $event)"
  >
    <span v-if="loading" class="spinner" />
    <slot />
  </button>
</template>

<script setup>
import { computed } from 'vue';
import { useTheme } from './useTheme';
import { primary_button, secondary_button } from 'jupiter-design-system';

const props = defineProps({
  variant: {
    type: String,
    default: 'primary',
    validator: (v) => ['primary', 'secondary', 'ghost'].includes(v)
  },
  disabled: Boolean,
  loading: Boolean,
  size: {
    type: String,
    default: 'medium'
  }
});

const { theme } = useTheme();

const classes = computed(() => {
  const base = props.variant === 'primary' 
    ? primary_button(theme.value)
    : secondary_button(theme.value);
    
  return base
    .disabled(props.disabled)
    .loading(props.loading)
    .classes();
});
</script>

<!-- JupiterCard.vue -->
<template>
  <div 
    :class="classes"
    @click="interactive && $emit('click')"
  >
    <slot />
  </div>
</template>

<script setup>
import { computed } from 'vue';
import { useTheme } from './useTheme';
import { card_pattern, CardElevation, CardInteraction } from 'jupiter-design-system';

const props = defineProps({
  elevation: {
    type: String,
    default: 'raised'
  },
  interactive: Boolean,
  selected: Boolean
});

const { theme } = useTheme();

const classes = computed(() => {
  return card_pattern(theme.value)
    .elevation(CardElevation[props.elevation])
    .interaction(props.interactive ? CardInteraction.Clickable : CardInteraction.Static)
    .selected(props.selected)
    .classes();
});
</script>
```

## Svelte Integration

### Store Setup
```js
// theme-store.js
import { writable, derived } from 'svelte/store';
import { VibeColors, CorporateTheme } from 'jupiter-design-system';

const themes = {
  vibe: new VibeColors(),
  corporate: new CorporateTheme('Acme Corp'),
};

export const currentTheme = writable('vibe');

export const theme = derived(
  currentTheme,
  $currentTheme => themes[$currentTheme]
);

// Helper functions
export function setTheme(name) {
  if (themes[name]) {
    currentTheme.set(name);
  }
}

export function getThemeNames() {
  return Object.keys(themes);
}
```

### Component Examples
```svelte
<!-- Button.svelte -->
<script>
  import { theme } from './theme-store';
  import { primary_button, secondary_button } from 'jupiter-design-system';
  
  export let variant = 'primary';
  export let disabled = false;
  export let loading = false;
  
  $: classes = variant === 'primary'
    ? primary_button($theme)
        .disabled(disabled)
        .loading(loading)
        .classes()
    : secondary_button($theme)
        .disabled(disabled)
        .loading(loading)
        .classes();
</script>

<button 
  class={classes} 
  {disabled}
  on:click
>
  {#if loading}
    <span class="spinner" />
  {/if}
  <slot />
</button>

<!-- Typography.svelte -->
<script>
  import { theme } from './theme-store';
  import { 
    title_typography, 
    heading_typography, 
    body_typography 
  } from 'jupiter-design-system';
  
  export let variant = 'body';
  export let color = null;
  export let align = null;
  export let as = null;
  
  $: pattern = variant === 'title' 
    ? title_typography($theme)
    : variant === 'heading'
    ? heading_typography($theme)
    : body_typography($theme);
    
  $: if (color) pattern = pattern.color(color);
  $: if (align) pattern = pattern.alignment(align);
  
  $: classes = pattern.classes();
  $: element = as || pattern.get_element();
</script>

<svelte:element this={element} class={classes}>
  <slot />
</svelte:element>

<!-- ProductCard.svelte -->
<script>
  import { theme } from './theme-store';
  import Button from './Button.svelte';
  import Card from './Card.svelte';
  import { product_builder, text_styles } from 'jupiter-design-system';
  
  export let product;
  
  $: productClasses = product_builder($theme)
    .variant('card')
    .on_sale(product.onSale)
    .classes();
    
  $: priceClasses = text_styles($theme)
    .size('lg')
    .weight('bold')
    .color('primary')
    .classes();
</script>

<Card interactive elevation="raised">
  <div class={productClasses}>
    <img src={product.image} alt={product.name} />
    <h3>{product.name}</h3>
    <p>{product.description}</p>
    <div class={priceClasses}>${product.price}</div>
    <Button variant="primary" on:click={() => addToCart(product)}>
      Add to Cart
    </Button>
  </div>
</Card>
```

## Angular Integration

### Service Setup
```typescript
// theme.service.ts
import { Injectable, signal, computed } from '@angular/core';
import { VibeColors, CorporateTheme, Theme } from 'jupiter-design-system';

interface ThemeConfig {
  [key: string]: Theme & ColorProvider;
}

@Injectable({
  providedIn: 'root'
})
export class ThemeService {
  private themes: ThemeConfig = {
    vibe: new VibeColors(),
    corporate: new CorporateTheme('Acme Corp'),
  };
  
  private currentTheme = signal<string>('vibe');
  
  theme = computed(() => this.themes[this.currentTheme()]);
  themeNames = Object.keys(this.themes);
  
  setTheme(name: string): void {
    if (this.themes[name]) {
      this.currentTheme.set(name);
    }
  }
}
```

### Component Examples
```typescript
// button.component.ts
import { Component, Input, Output, EventEmitter, computed } from '@angular/core';
import { ThemeService } from './theme.service';
import { primary_button, secondary_button } from 'jupiter-design-system';

@Component({
  selector: 'jupiter-button',
  template: `
    <button 
      [class]="classes()" 
      [disabled]="disabled"
      (click)="onClick.emit($event)"
    >
      <span *ngIf="loading" class="spinner"></span>
      <ng-content></ng-content>
    </button>
  `
})
export class ButtonComponent {
  @Input() variant: 'primary' | 'secondary' = 'primary';
  @Input() disabled = false;
  @Input() loading = false;
  @Output() onClick = new EventEmitter<Event>();
  
  constructor(private themeService: ThemeService) {}
  
  classes = computed(() => {
    const theme = this.themeService.theme();
    const builder = this.variant === 'primary' 
      ? primary_button(theme)
      : secondary_button(theme);
      
    return builder
      .disabled(this.disabled)
      .loading(this.loading)
      .classes();
  });
}

// typography.component.ts
import { Component, Input, computed } from '@angular/core';
import { ThemeService } from './theme.service';
import { 
  title_typography, 
  heading_typography, 
  body_typography,
  TypographyColor,
  TypographyAlignment
} from 'jupiter-design-system';

@Component({
  selector: 'jupiter-typography',
  template: `
    <component 
      [ngSwitch]="element()"
      [class]="classes()"
    >
      <h1 *ngSwitchCase="'h1'"><ng-content></ng-content></h1>
      <h2 *ngSwitchCase="'h2'"><ng-content></ng-content></h2>
      <h3 *ngSwitchCase="'h3'"><ng-content></ng-content></h3>
      <p *ngSwitchDefault><ng-content></ng-content></p>
    </component>
  `
})
export class TypographyComponent {
  @Input() variant: 'title' | 'heading' | 'body' = 'body';
  @Input() color?: TypographyColor;
  @Input() align?: TypographyAlignment;
  
  constructor(private themeService: ThemeService) {}
  
  pattern = computed(() => {
    const theme = this.themeService.theme();
    let pattern = this.variant === 'title' 
      ? title_typography(theme)
      : this.variant === 'heading'
      ? heading_typography(theme)
      : body_typography(theme);
      
    if (this.color) {
      pattern = pattern.color(this.color);
    }
    if (this.align) {
      pattern = pattern.alignment(this.align);
    }
    
    return pattern;
  });
  
  classes = computed(() => this.pattern().classes());
  element = computed(() => this.pattern().get_element());
}
```

## Solid.js Integration

### Context Setup
```jsx
// ThemeContext.jsx
import { createContext, createSignal, useContext } from 'solid-js';
import { VibeColors, CorporateTheme } from 'jupiter-design-system';

const themes = {
  vibe: new VibeColors(),
  corporate: new CorporateTheme('Acme Corp'),
};

const ThemeContext = createContext();

export function ThemeProvider(props) {
  const [currentTheme, setCurrentTheme] = createSignal('vibe');
  
  const theme = () => themes[currentTheme()];
  const themeNames = Object.keys(themes);
  
  return (
    <ThemeContext.Provider value={{ 
      theme, 
      setTheme: setCurrentTheme,
      themeNames 
    }}>
      {props.children}
    </ThemeContext.Provider>
  );
}

export const useTheme = () => {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme must be used within ThemeProvider');
  }
  return context;
};
```

### Component Examples
```jsx
// Button.jsx
import { createMemo } from 'solid-js';
import { useTheme } from './ThemeContext';
import { primary_button, secondary_button } from 'jupiter-design-system';

export function Button(props) {
  const { theme } = useTheme();
  
  const classes = createMemo(() => {
    const builder = props.variant === 'primary'
      ? primary_button(theme())
      : secondary_button(theme());
      
    return builder
      .disabled(props.disabled || false)
      .loading(props.loading || false)
      .classes();
  });
  
  return (
    <button 
      class={classes()} 
      disabled={props.disabled}
      onClick={props.onClick}
    >
      {props.loading && <span class="spinner" />}
      {props.children}
    </button>
  );
}
```

## Web Components Integration

### Custom Element with Theme
```js
// jupiter-button.js
import { primary_button, secondary_button, VibeColors } from 'jupiter-design-system';

class JupiterButton extends HTMLElement {
  static observedAttributes = ['variant', 'disabled', 'loading', 'theme'];
  
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
    this._theme = new VibeColors();
  }
  
  connectedCallback() {
    this.render();
  }
  
  attributeChangedCallback(name, oldValue, newValue) {
    if (oldValue !== newValue) {
      this.render();
    }
  }
  
  set theme(value) {
    this._theme = value;
    this.render();
  }
  
  get theme() {
    return this._theme;
  }
  
  render() {
    const variant = this.getAttribute('variant') || 'primary';
    const disabled = this.hasAttribute('disabled');
    const loading = this.hasAttribute('loading');
    
    const builder = variant === 'primary'
      ? primary_button(this._theme)
      : secondary_button(this._theme);
      
    const classes = builder
      .disabled(disabled)
      .loading(loading)
      .classes();
    
    this.shadowRoot.innerHTML = `
      <style>
        @import url('/path/to/tailwind.css');
        :host {
          display: inline-block;
        }
      </style>
      <button class="${classes}" ${disabled ? 'disabled' : ''}>
        ${loading ? '<span class="spinner"></span>' : ''}
        <slot></slot>
      </button>
    `;
  }
}

customElements.define('jupiter-button', JupiterButton);

// Usage
// <jupiter-button variant="primary">Click me</jupiter-button>
```

## Next.js App Router Integration

### Theme Provider
```tsx
// app/providers.tsx
'use client';

import { createContext, useContext, useState, ReactNode } from 'react';
import { VibeColors, CorporateTheme } from 'jupiter-design-system';

const themes = {
  vibe: new VibeColors(),
  corporate: new CorporateTheme('Acme Corp'),
} as const;

type ThemeName = keyof typeof themes;

interface ThemeContextType {
  theme: typeof themes[ThemeName];
  themeName: ThemeName;
  setTheme: (name: ThemeName) => void;
}

const ThemeContext = createContext<ThemeContextType | null>(null);

export function ThemeProvider({ 
  children,
  defaultTheme = 'vibe'
}: { 
  children: ReactNode;
  defaultTheme?: ThemeName;
}) {
  const [themeName, setThemeName] = useState<ThemeName>(defaultTheme);
  
  return (
    <ThemeContext.Provider value={{ 
      theme: themes[themeName],
      themeName,
      setTheme: setThemeName
    }}>
      {children}
    </ThemeContext.Provider>
  );
}

export const useTheme = () => {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme must be used within ThemeProvider');
  }
  return context;
};
```

### Server Component Support
```tsx
// app/components/ServerButton.tsx
import { primary_button, VibeColors } from 'jupiter-design-system';

interface ServerButtonProps {
  children: React.ReactNode;
  variant?: 'primary' | 'secondary';
}

// Server component with default theme
export function ServerButton({ 
  children, 
  variant = 'primary' 
}: ServerButtonProps) {
  // Use default theme for server rendering
  const theme = new VibeColors();
  const classes = primary_button(theme).classes();
  
  return (
    <button className={classes}>
      {children}
    </button>
  );
}
```

## Best Practices

1. **Create a theme context/store** - Centralize theme management
2. **Use composition** - Combine patterns and builders as needed
3. **Memoize classes** - Prevent unnecessary recalculations
4. **Type your themes** - Use TypeScript for better DX
5. **Server-side considerations** - Have a default theme for SSR
6. **Lazy load themes** - Load themes on demand for better performance
7. **Cache theme instances** - Reuse theme objects to save memory