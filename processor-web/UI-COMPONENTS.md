# Gidence UI Components Documentation

This document provides comprehensive documentation for the Gidence UI component library used in the controller-web application. All components follow a consistent design system with support for theming and responsive design.

## Component Overview

The component library includes the following categories:
- **Typography**: Text styling classes
- **Buttons**: Interactive button components
- **Input Components**: Form elements for data entry
- **Display Components**: Data visualization and feedback
- **Layout Components**: Modals, popovers, and containers

---

## Typography System

The typography system uses CSS classes for consistent text styling across the application.

### Title Classes
```html
<h1 class="gd-title-1">Title 1 - Main heading</h1>
<h2 class="gd-title-2">Title 2 - Secondary heading</h2>
```

### Headline Classes
```html
<h3 class="gd-headline-1">Headline 1</h3>
<h4 class="gd-headline-2">Headline 2</h4>
<h5 class="gd-headline-3">Headline 3</h5>
<h6 class="gd-headline-4">Headline 4</h6>
<p class="gd-headline-5">Headline 5</p>
<p class="gd-headline-6">Headline 6</p>
```

### Body Text Classes
```html
<p class="gd-body-1">Body 1 - Large body text for important content</p>
<p class="gd-body-2">Body 2 - Regular body text for most content</p>
<p class="gd-body-3">Body 3 - Smaller body text</p>
<p class="gd-body-4">Body 4 - Even smaller text</p>
<p class="gd-body-5">Body 5 - Smallest body text</p>
```

### Utility Text Classes
```html
<p class="gd-subtitle-text">Subtitle text</p>
<p class="gd-caption-text">Caption text</p>
<p class="gd-button-text">Button text</p>
```

---

## Button Component (`gd-button`)

The button component supports multiple types, sizes, alignments, and interactive states.

### Basic Button Types

```html
<gd-button text="PRIMARY" type="primary" font="button" />
<gd-button text="SECONDARY" type="secondary" font="button" />
<gd-button text="TERTIARY" type="tertiary" font="button" />
<gd-button text="SUCCESS" type="success" font="button" />
<gd-button text="ERROR" type="error" font="button" />
<gd-button text="WARNING" type="warning" font="button" />
```

### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `text` | String | - | Button text content |
| `type` | String | `'primary'` | Button type: `primary`, `secondary`, `tertiary`, `success`, `error`, `warning` |
| `icon` | String | - | Icon name (when provided alone, creates icon-only button) |
| `font` | String | `'button'` | Font style: `button`, `body`, `headline` |
| `size` | String | `'medium'` | Button size: `small`, `medium`, `large` |
| `alignment` | String | `'center'` | Text alignment: `left`, `center`, `right` |
| `disabled` | Boolean | `false` | Disable button interaction |
| `borderless` | Boolean | `false` | Remove button border |
| `tooltip` | Object | - | Tooltip configuration: `{ text: string, position: string }` |

### Icon Buttons

Icon-only buttons are automatically detected when only the `icon` prop is provided:

```html
<gd-button
  icon="plus"
  type="primary"
  :tooltip="{ text: 'Add item', position: 'top' }"
/>
<gd-button
  icon="trash"
  type="error"
  :tooltip="{ text: 'Delete', position: 'left' }"
/>
```

### Text with Icon

```html
<gd-button text="With Icon" type="primary" icon="check" />
<gd-button text="Save Changes" type="success" icon="check" />
```

### Button Sizes

```html
<gd-button text="Small" type="primary" size="small" />
<gd-button text="Medium" type="primary" size="medium" />
<gd-button text="Large" type="primary" size="large" />
```

### Button Alignment

Useful for menu-style layouts:

```html
<gd-button
  text="Dashboard"
  type="tertiary"
  alignment="left"
  font="body"
  icon="grid-on"
  borderless
/>
```

### Disabled State

```html
<gd-button text="Disabled" type="primary" disabled />
```

### Tooltip Support

Tooltips can be positioned at `top`, `bottom`, `left`, or `right`:

```html
<gd-button
  icon="info"
  type="secondary"
  :tooltip="{ text: 'More information', position: 'top' }"
/>
```

---

## Input Components

### Text Input (`gd-input-text`)

```html
<gd-input-text
  v-model="textValue"
  label="Basic Text Input"
  placeholder="Enter some text"
/>
```

#### Input Types

```html
<!-- Email input -->
<gd-input-text
  v-model="emailValue"
  type="email"
  label="Email Input"
  placeholder="user@example.com"
/>

<!-- Password input -->
<gd-input-text
  v-model="passwordValue"
  type="password"
  label="Password Input"
  placeholder="Enter password"
/>

<!-- Number input with constraints -->
<gd-input-text
  v-model="numberValue"
  type="number"
  label="Number Input"
  placeholder="123"
  :min="0"
  :max="100"
/>
```

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `modelValue` | String/Number | - | Input value (use with v-model) |
| `type` | String | `'text'` | Input type: `text`, `email`, `password`, `number` |
| `label` | String | - | Input label |
| `placeholder` | String | - | Placeholder text |
| `disabled` | Boolean | `false` | Disable input |
| `min` | Number | - | Minimum value (for number inputs) |
| `max` | Number | - | Maximum value (for number inputs) |

### Select Input (`gd-input-select`)

```html
<gd-input-select
  v-model="selectValue"
  :options="selectOptions"
  type="secondary"
/>
```

#### Data Structure

```javascript
const selectOptions = [
  { label: "Option 1", value: "option1" },
  { label: "Option 2", value: "option2" },
  { label: "Option 3", value: "option3" }
];
```

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `modelValue` | Object | - | Selected option object |
| `options` | Array | `[]` | Array of option objects with `label` and `value` |
| `type` | String | `'primary'` | Select style type |
| `disabled` | Boolean | `false` | Disable select |

### Text Select with Autocomplete (`gd-input-text-select`)

```html
<gd-input-text-select
  v-model="textSelectValue"
  label="Searchable Select"
  placeholder="Type to search..."
  :options="countryOptions"
/>
```

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `modelValue` | Object | - | Selected option object |
| `label` | String | - | Input label |
| `placeholder` | String | - | Placeholder text |
| `options` | Array | `[]` | Array of searchable options |
| `disabled` | Boolean | `false` | Disable input |

### Date Input (`gd-input-date`)

```html
<gd-input-date
  v-model="dateValue"
  label="Basic Date Input"
  placeholder="Select a date"
/>
```

#### Date Range Example

```html
<!-- Start date -->
<gd-input-date
  v-model="dateRangeStart"
  label="Start Date"
  placeholder="Select start date"
  :max="dateRangeEnd"
/>

<!-- End date -->
<gd-input-date
  v-model="dateRangeEnd"
  label="End Date"
  placeholder="Select end date"
  :min="dateRangeStart"
/>
```

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `modelValue` | String | - | Date value (ISO format) |
| `label` | String | - | Input label |
| `placeholder` | String | - | Placeholder text |
| `min` | String | - | Minimum selectable date |
| `max` | String | - | Maximum selectable date |
| `disabled` | Boolean | `false` | Disable input |

### Textarea Input (`gd-input-textarea`)

```html
<gd-input-textarea
  v-model="textareaValue"
  label="Basic Textarea"
  placeholder="Enter multiple lines of text..."
  :rows="3"
/>
```

#### With Character Limit

```html
<gd-input-textarea
  v-model="textareaLongValue"
  label="Large Textarea"
  placeholder="Enter a longer description..."
  :rows="6"
  :limit="500"
/>
```

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `modelValue` | String | - | Textarea value |
| `label` | String | - | Textarea label |
| `placeholder` | String | - | Placeholder text |
| `rows` | Number | `3` | Number of visible rows |
| `limit` | Number | - | Character limit |
| `disabled` | Boolean | `false` | Disable textarea |

### Toggle Input (`gd-input-toggle`)

```html
<gd-input-toggle v-model="toggleValue" />
```

#### Toggle with Labels

```html
<div class="toggle-item">
  <span class="gd-body-3">Enable Feature</span>
  <gd-input-toggle v-model="featureToggle" />
</div>
```

#### Theme Switch Example

```html
<div class="theme-switch">
  <gd-svg name="sun" color="warning" />
  <gd-input-toggle v-model="isDarkTheme" @update:modelValue="toggleTheme" />
  <gd-svg name="moon" color="primary" />
</div>
```

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `modelValue` | Boolean | `false` | Toggle state |
| `disabled` | Boolean | `false` | Disable toggle |

---

## Display Components

### SVG Icons (`gd-svg`)

```html
<gd-svg name="plus" color="primary" />
<gd-svg name="check" color="success" />
<gd-svg name="error" color="error" />
<gd-svg name="warning" color="warning" />
```

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `name` | String | - | Icon name |
| `color` | String | `'primary'` | Icon color: `primary`, `secondary`, `success`, `error`, `warning` |

#### Available Icons

Based on usage in the test page:
- `plus`, `close`, `trash`, `check`
- `arrow-left`, `arrow-right`
- `grid-on`, `account`, `gear`, `exit`
- `link`, `sun`, `moon`

### Table (`gd-table`)

```html
<gd-table :header="tableHeaders">
  <tr v-for="row in tableData" :key="row.id" class="gd-table-row">
    <td class="gd-table-row-cell gd-body-3">{{ row.id }}</td>
    <td class="gd-table-row-cell gd-body-3">{{ row.name }}</td>
    <td class="gd-table-row-cell gd-body-3">{{ row.email }}</td>
    <td class="gd-table-row-cell gd-body-3">{{ row.status }}</td>
  </tr>
</gd-table>
```

#### Data Structure

```javascript
const tableHeaders = ["ID", "Name", "Email", "Status"];

const tableData = [
  { id: 1, name: "John Doe", email: "john@example.com", status: "Active" },
  { id: 2, name: "Jane Smith", email: "jane@example.com", status: "Inactive" }
];
```

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `header` | Array | `[]` | Array of header strings |

### Alert System (`gd-alert`)

Global alert system managed through the `useAlert` composable.

```html
<!-- Alert component (place once in layout) -->
<gd-alert />
```

#### Triggering Alerts

```javascript
const { setAlert } = useAlert();

// Success alert
setAlert({
  title: "Success!",
  type: "success",
  message: "Operation completed successfully."
});

// Error alert
setAlert({
  title: "Error!",
  type: "error", 
  message: "Something went wrong."
});

// Warning alert
setAlert({
  title: "Warning!",
  type: "warning",
  message: "Please check your input."
});
```

### Loader (`gd-loader`)

```html
<gd-loader v-if="showLoader" state="show" />
```

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `state` | String | `'show'` | Loader state |

---

## Layout Components

### Modal (`gd-modal`)

```html
<gd-modal 
  name="Test Modal" 
  :visible="showModal" 
  @close="showModal = false"
>
  <div>
    <h3 class="gd-headline-2">Modal Content</h3>
    <p class="gd-body-2">This is a test modal with some content.</p>
    <div style="margin-top: 1rem; display: flex; gap: 0.5rem">
      <gd-button text="Save" type="primary" @click="showModal = false" />
      <gd-button text="Cancel" type="secondary" @click="showModal = false" />
    </div>
  </div>
</gd-modal>
```

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `name` | String | - | Modal title |
| `visible` | Boolean | `false` | Modal visibility state |

#### Events

| Event | Description |
|-------|-------------|
| `close` | Emitted when modal should be closed |

### Popover (`gd-popover`)

```html
<gd-popover v-model:visible="showPopover">
  <template #trigger>
    <gd-button
      text="Open Popover"
      type="secondary"
      font="body"
      @click="showPopover = !showPopover"
    />
  </template>
  <template #content>
    <div class="popover-content">
      <h3 class="gd-headline-4">Popover Menu</h3>
      <p class="gd-body-4">Choose an action:</p>
      <div class="popover-actions">
        <gd-button
          text="Copy Link"
          type="tertiary"
          alignment="left"
          font="body"
          icon="link"
          borderless
          @click="showPopover = false"
        />
        <gd-button
          text="Delete"
          type="error"
          alignment="left"
          font="body"
          icon="trash"
          borderless
          @click="showPopover = false"
        />
      </div>
    </div>
  </template>
</gd-popover>
```

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `visible` | Boolean | `false` | Popover visibility state (use with v-model) |

#### Slots

| Slot | Description |
|------|-------------|
| `trigger` | Content that triggers the popover |
| `content` | Popover content |

---

## Composables

### useAlert

Manages global alert system.

```javascript
const { setAlert } = useAlert();

setAlert({
  title: string,      // Alert title
  type: string,       // 'success', 'error', 'warning'
  message: string     // Alert message
});
```

### useMain

Manages global application state including theming.

```javascript
const { theme, getTheme, setTheme } = useMain();

// Get current theme
getTheme();

// Set theme
setTheme('dark');   // or 'light'

// Reactive theme value
console.log(theme.value); // 'dark' or 'light'
```

---

## Styling System

### CSS Variables

The component system uses CSS variables for consistent theming:

#### Colors
- `--background-depth-one-color`
- `--background-depth-two-color`
- `--background-depth-three-color`
- `--font-secondary-color`
- `--border-color`
- `--border`

#### Usage Example

```scss
.custom-component {
  background-color: var(--background-depth-two-color);
  border: var(--border);
  color: var(--font-secondary-color);
}
```

### Responsive Design

Components are designed to be responsive. The test page includes media queries:

```scss
@media (max-width: 768px) {
  .component-row {
    flex-direction: column;
    align-items: stretch;
  }
}
```

---

## Component Development Guidelines

### Naming Convention
- All components use the `gd-` prefix
- Use kebab-case for component names
- Use descriptive names that indicate function

### Props Design
- Use clear, descriptive prop names
- Provide sensible defaults
- Support common variants (size, type, state)

### Styling Approach
- Use scoped styles with SCSS
- Leverage CSS variables for theming
- Follow BEM-like naming for CSS classes
- Support responsive design

### Accessibility
- Include proper ARIA attributes
- Support keyboard navigation
- Provide tooltips for icon-only buttons
- Use semantic HTML elements

---

## Testing

All components can be tested on the test page located at:
```
controller-web/app/pages/test.vue
```

This page demonstrates all component variations and serves as a living style guide for the application.

---

## Browser Support

The component library supports:
- Modern browsers (Chrome, Firefox, Safari, Edge)
- Responsive design for mobile devices
- Dark and light theme modes
- Touch interactions on mobile devices

---

## Missing Components for Complete UI Kit

This section documents components that would make the Gidence UI library fully reusable across multiple projects.

### High Priority (Most Commonly Needed)

#### 1. **Progress Bar (`gd-progress`)**
```html
<gd-progress :value="75" :max="100" />
<gd-progress :value="uploadedBytes" :max="totalBytes" label="Uploading..." />
<gd-progress :value="currentStep" :max="totalSteps" type="circular" />
```
**Use cases:** File uploads, multi-step processes, data loading with known completion

#### 2. **Checkbox Component (`gd-input-checkbox`)**
```html
<gd-input-checkbox v-model="agreeToTerms" label="I agree to the terms" />
<gd-input-checkbox v-model="notifications" label="Enable notifications" disabled />
```

#### 3. **Radio Button Component (`gd-input-radio`)**
```html
<gd-input-radio v-model="selectedOption" :options="radioOptions" />
```

#### 4. **Card/Panel Component (`gd-card`)**
```html
<gd-card title="Device Status" subtitle="Last updated: 2 min ago">
  <template #content>
    <!-- Card content -->
  </template>
  <template #actions>
    <gd-button text="Edit" type="secondary" />
  </template>
</gd-card>
```

#### 5. **Badge/Tag Component (`gd-badge`)**
```html
<gd-badge text="Active" type="success" />
<gd-badge text="Pending" type="warning" />
<gd-badge :count="5" type="primary" />
```

#### 6. **Tabs Component (`gd-tabs`)**
```html
<gd-tabs v-model="activeTab">
  <gd-tab-panel name="overview" label="Overview">
    <!-- Tab content -->
  </gd-tab-panel>
  <gd-tab-panel name="settings" label="Settings">
    <!-- Tab content -->
  </gd-tab-panel>
</gd-tabs>
```

### Medium Priority (Project-Specific but Useful)

#### 7. **File Upload (`gd-input-file`)**
```html
<gd-input-file 
  v-model="selectedFiles" 
  accept="image/*" 
  multiple 
  label="Upload Images" 
/>
```

#### 8. **Range/Slider (`gd-input-range`)**
```html
<gd-input-range 
  v-model="temperature" 
  :min="0" 
  :max="100" 
  :step="5" 
  label="Temperature" 
/>
```

#### 9. **Breadcrumb Component (`gd-breadcrumb`)**
```html
<gd-breadcrumb :items="breadcrumbItems" />
```

#### 10. **Pagination Component (`gd-pagination`)**
```html
<gd-pagination 
  v-model="currentPage" 
  :total="totalItems" 
  :per-page="itemsPerPage" 
/>
```

#### 11. **Standalone Tooltip Component (`gd-tooltip`)**
```html
<gd-tooltip text="This is helpful information" position="top">
  <span>Hover me</span>
</gd-tooltip>
```

### Lower Priority (Nice to Have)

#### 12. **Accordion Component (`gd-accordion`)**
```html
<gd-accordion>
  <gd-accordion-item title="Advanced Settings" :open="false">
    <!-- Accordion content -->
  </gd-accordion-item>
</gd-accordion>
```

#### 13. **Color Picker (`gd-input-color`)**
```html
<gd-input-color v-model="selectedColor" label="Theme Color" />
```

#### 14. **Drawer/Sidebar (`gd-drawer`)**
```html
<gd-drawer v-model:visible="showDrawer" position="right">
  <!-- Drawer content -->
</gd-drawer>
```

#### 15. **Grid System (`gd-grid`, `gd-col`)**
```html
<gd-grid>
  <gd-col :span="8">Main content</gd-col>
  <gd-col :span="4">Sidebar</gd-col>
</gd-grid>
```

#### 16. **Divider Component (`gd-divider`)**
```html
<gd-divider />
<gd-divider text="OR" />
```

#### 17. **Skeleton Loader (`gd-skeleton`)**
```html
<gd-skeleton :rows="3" :loading="isLoading">
  <div>Actual content when loaded</div>
</gd-skeleton>
```

#### 18. **Empty State (`gd-empty`)**
```html
<gd-empty 
  icon="inbox" 
  title="No data found" 
  description="Try adjusting your filters" 
>
  <gd-button text="Reset Filters" type="primary" />
</gd-empty>
```

#### 19. **Avatar Component (`gd-avatar`)**
```html
<gd-avatar src="/user.jpg" name="John Doe" size="large" />
<gd-avatar name="JD" background-color="primary" />
```

#### 20. **Stepper/Steps (`gd-stepper`)**
```html
<gd-stepper v-model="currentStep">
  <gd-step title="Basic Info" />
  <gd-step title="Configuration" />
  <gd-step title="Review" />
</gd-stepper>
```

### Existing Components ✅

Your current library already includes:

- ✅ **Typography System** (gd-title-*, gd-headline-*, gd-body-*)
- ✅ **Button Component** (gd-button) - Very feature-rich with icon support, tooltips, alignment
- ✅ **Input Components** (text, select, date, textarea, toggle, text-select)
- ✅ **SVG Icons** (gd-svg) with color variants
- ✅ **Table** (gd-table) with proper styling
- ✅ **Alert/Toast System** (gd-alert) - Sophisticated with GSAP animations
- ✅ **Loading Components** (gd-loader, gd-spinner) - Indeterminate loading states
- ✅ **Modal** (gd-modal) with overlay and content slots
- ✅ **Popover** (gd-popover) with trigger and content templates
- ✅ **Composables** (useAlert, useMain for theming)

### Development Recommendation

Start with **High Priority** components (1-6) as they're most commonly needed across different project types. Your current library covers approximately 60-70% of typical UI needs, so adding these would make it very complete and reusable across multiple projects.

---

This documentation reflects the current state of the component library as demonstrated in the test page. Components are designed for industrial automation interfaces with emphasis on reliability, clarity, and ease of use.