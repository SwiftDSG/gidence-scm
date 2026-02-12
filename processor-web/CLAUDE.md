# Claude Code Instructions

## CSS Class Naming Convention

This project uses a **hierarchical naming convention** with **separate modifier classes** for maintainable and scalable CSS architecture.

### Structure Classes (Hierarchical)

Follow the pattern: `gd-[parent]-[child]-[grandchild]`

**Rules:**
- All structural classes start with `gd-` prefix
- Each level represents DOM hierarchy **exactly as it appears in the HTML structure**
- Use single words only for each segment
- Each dash adds one level of hierarchy
- **IMPORTANT**: The class name must follow the actual DOM nesting, not logical grouping

**Examples:**
```html
<!-- Simple hierarchy -->
<div class="gd-button">
  <!-- Child elements follow parent class name -->
  <div class="gd-button-text">Button Text</div>
  <div class="gd-button-icon">Icon</div>
</div>

<!-- Complex hierarchy - each level adds one segment -->
<div class="gd-panel">
  <div class="gd-panel-header">
    <div class="gd-panel-header-item">
      <div class="gd-panel-header-item-icon">Icon</div>
      <div class="gd-panel-header-item-name">Name</div>
    </div>
  </div>
  <div class="gd-panel-body">Content</div>
</div>

<!-- Real-world example showing DOM structure MUST match class hierarchy -->
<div class="gd-testpage">
  <div class="gd-testpage-container">
    <!-- This is INSIDE container, so it's gd-testpage-container-header -->
    <header class="gd-testpage-container-header">
      <h1>Page Title</h1>
    </header>
    <!-- This is INSIDE container, so it's gd-testpage-container-section -->
    <section class="gd-testpage-container-section">
      <!-- This is INSIDE section, so it's gd-testpage-container-section-group -->
      <div class="gd-testpage-container-section-group">
        <!-- This is INSIDE group, so it's gd-testpage-container-section-group-item -->
        <div class="gd-testpage-container-section-group-item">
          Content
        </div>
      </div>
    </section>
  </div>
</div>
```

**WRONG Example (don't do this):**
```html
<!-- ❌ INCORRECT - class hierarchy doesn't match DOM hierarchy -->
<div class="gd-testpage">
  <div class="gd-testpage-container">
    <!-- ❌ This should be gd-testpage-container-header, not gd-testpage-header -->
    <header class="gd-testpage-header">Title</header>
    <!-- ❌ This should be gd-testpage-container-section, not gd-testpage-section -->
    <section class="gd-testpage-section">Content</section>
  </div>
</div>
```

### Modifier Classes (State/Variant)

Use the pattern: `--[modifier]` (double dash prefix, no `gd-`)

**Available modifiers:**

#### Color modifiers:
- `--primary`
- `--secondary` 
- `--tertiary`
- `--success`
- `--error`
- `--warning`

#### State modifiers:
- `--active`
- `--disabled`
- `--borderless`

**Usage:**
```html
<!-- Button with primary color and disabled state -->
<button class="gd-button --primary --disabled">
  <span class="gd-button-text">Submit</span>
</button>

<!-- Panel item with active state -->
<div class="gd-panel-header-item --active">
  <div class="gd-panel-header-item-icon">Icon</div>
</div>

<!-- Input with disabled state -->
<div class="gd-input --disabled">
  <input class="gd-input-text" />
</div>
```

### Vue.js Implementation

**Flexible button configuration:**
```vue
<template>
  <!-- Icon button (auto-detected) -->
  <gd-button 
    icon="plus" 
    type="primary"
    :tooltip="{ text: 'Add item', position: 'top' }"
  />

  <!-- Default button with custom alignment and font -->
  <gd-button 
    text="Submit" 
    type="primary" 
    icon="check"
    alignment="center"
    font="button"
  />

  <!-- Left-aligned menu item -->
  <gd-button 
    text="Menu Item" 
    type="secondary" 
    alignment="left"
    font="body"
    icon="star"
  />
  
  <!-- Right-aligned button -->
  <gd-button 
    text="Next" 
    type="primary"
    alignment="right"
    font="headline"
  />
</template>
```

### CSS Implementation

**Structure styles stay in component files:**
```scss
.gd-button {
  // Base button styles
  &-text {
    // Text styles
  }
  &-icon {
    // Icon styles
  }
}
```

**Modifier styles are in global.scss:**
```scss
.--primary {
  &.gd-button {
    background-color: var(--primary-color);
  }
  &.gd-component-svg {
    // SVG color styles
  }
}

.--disabled {
  &.gd-button,
  &.gd-input {
    opacity: 0.5;
    pointer-events: none;
  }
}
```

## Button Component

The `gd-button` component automatically detects its type based on props:

- **Icon button**: When only `icon` prop is provided (no `text`)
- **Default button**: When `text` prop is provided (with optional `icon`)

**API:**
```typescript
interface ButtonProps {
  text?: string;           // Button text
  icon?: string;          // Icon name
  type?: "primary" | "secondary" | "tertiary" | "success" | "error";
  alignment?: "left" | "center" | "right";  // Text/content alignment
  font?: "button" | "body" | "headline";    // Font style to use
  disabled?: boolean;     // Disabled state
  borderless?: boolean;   // Remove border
  tooltip?: {             // Only for icon buttons
    text: string;
    position?: "top" | "bottom" | "left" | "right";
  };
}
```

### Benefits

1. **Clear separation** between structure and state
2. **Reusable modifiers** across components
3. **JavaScript-friendly** for class toggling
4. **Maintainable** and **scalable** architecture
5. **Readable HTML** with obvious hierarchy
6. **Automatic type detection** reduces prop complexity

### Typography Classes

The project also includes typography utilities:
- `gd-title-1`, `gd-title-2`
- `gd-headline-1` through `gd-headline-6`
- `gd-body-1` through `gd-body-5`
- `gd-subtitle-text`
- `gd-caption-text`
- `gd-button-text`

Always follow this convention when creating new components or modifying existing ones.