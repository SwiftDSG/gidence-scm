<template>
  <button
    :class="[
      'gd-button',
      `--${type}`,
      `--align-${alignment}`,
      `--size-${size}`,
      { '--disabled': disabled },
      { '--borderless': borderless },
      { '--focusable': focusable },
      { '--loading': loading },
    ]"
    @click.stop="emits('click')"
    :disabled="disabled || loading"
    type="button"
  >
    <div v-if="icon" class="gd-button-icon">
      <gd-svg :name="icon" :color="svgColor" />
    </div>
    <div v-if="text" class="gd-button-label" :class="textFont">
      {{ text }}
    </div>
    <span
      v-if="tooltip"
      class="gd-button-tooltip gd-headline-6"
      :class="tooltipClass"
    >
      {{ tooltip.text }}
    </span>
    <gd-spinner :state="loading ? 'show' : 'hide'" />
  </button>
</template>

<script lang="ts" setup>
  interface ButtonProps {
    // Content
    text?: string;
    icon?: string;

    // Appearance
    type?:
      | "primary"
      | "secondary"
      | "tertiary"
      | "success"
      | "error"
      | "warning";
    alignment?: "left" | "center" | "right";
    font?: "button" | "body" | "headline";
    size?: "small" | "medium" | "large";

    // States
    disabled?: boolean;
    borderless?: boolean;
    focusable?: boolean;
    loading?: boolean;

    // Icon button specific
    tooltip?: {
      text: string;
      position?: "top" | "bottom" | "left" | "right";
    };
  }

  const props = withDefaults(defineProps<ButtonProps>(), {
    type: "secondary",
    alignment: "center",
    font: "button",
    size: "medium",
    disabled: false,
    borderless: false,
    loading: false,
  });

  const emits = defineEmits<{
    (event: "click"): void;
  }>();

  const svgColor = computed(() => {
    switch (props.type) {
      case "primary":
        return "tertiary";
      case "secondary":
        return "primary";
      case "tertiary":
        return "secondary";
      case "success":
        return "success";
      case "error":
        return "error";
      case "warning":
        return "warning";
      default:
        return "primary";
    }
  });
  const textFont = computed(() => {
    switch (props.font) {
      case "body":
        return "gd-body-4";
      case "button":
        return "gd-button-text";
      case "headline":
        return "gd-headline-5";
      default:
        return "gd-button-text";
    }
  });

  const tooltipClass = computed(() => {
    if (!props.tooltip?.position) return "--top";
    return `--${props.tooltip.position}`;
  });
</script>

<style lang="scss" scoped>
  .gd-button {
    cursor: pointer;
    position: relative;
    width: auto;
    height: 2rem;
    border-radius: 0.5rem;
    padding: 0 0.5rem;
    box-sizing: border-box;
    border: var(--border);
    background: transparent;
    display: flex;
    flex-shrink: 0;
    align-items: center;
    transition: transform 0.8s cubic-bezier(0.16, 1, 0.3, 1),
      opacity 0.2s ease-in-out, background-color 0.25s ease-in-out;

    * {
      pointer-events: none;
    }

    // Common elements
    &-icon {
      position: relative;
      width: 1rem;
      height: 1rem;
      display: flex;
      flex-shrink: 0;
      justify-content: center;
      align-items: center;
      transition: transform 0.8s cubic-bezier(0.16, 1, 0.3, 1);
      & + .gd-button-label {
        padding-left: 0 !important;
      }
    }

    &-label {
      position: relative;
      height: 100%;
      display: flex;
      align-items: center;
    }

    &-tooltip {
      pointer-events: none;
      position: absolute;
      height: 1rem;
      border-radius: 0.25rem;
      background-color: var(--font-primary-color);
      color: var(--background-depth-one-color);
      padding: 0 0.25rem;
      white-space: nowrap;
      display: flex;
      align-items: center;
      justify-content: center;
      transition: opacity 0.2s ease-in-out, transform 0.2s ease-in-out;
      opacity: 0;

      &.--top {
        bottom: calc(100% + 0.25rem);
        left: 50%;
        transform: translateX(-50%) scale(0.8);

        &::after {
          content: " ";
          position: absolute;
          top: 100%;
          left: 50%;
          margin-left: -0.25rem;
          border-width: 0.25rem;
          border-style: solid;
          border-color: var(--font-primary-color) transparent transparent
            transparent;
        }
      }

      &.--bottom {
        top: calc(100% + 0.25rem);
        left: 50%;
        transform: translateX(-50%) scale(0.8);

        &::after {
          content: " ";
          position: absolute;
          bottom: 100%;
          left: 50%;
          margin-left: -0.25rem;
          border-width: 0.25rem;
          border-style: solid;
          border-color: transparent transparent var(--font-primary-color)
            transparent;
        }
      }

      &.--left {
        right: calc(100% + 0.25rem);
        top: 50%;
        transform: translateY(-50%) scale(0.8);

        &::after {
          content: " ";
          position: absolute;
          top: 50%;
          left: 100%;
          margin-top: -0.25rem;
          border-width: 0.25rem;
          border-style: solid;
          border-color: transparent transparent transparent
            var(--font-primary-color);
        }
      }

      &.--right {
        left: calc(100% + 0.25rem);
        top: 50%;
        transform: translateY(-50%) scale(0.8);

        &::after {
          content: " ";
          position: absolute;
          top: 50%;
          right: 100%;
          margin-top: -0.25rem;
          border-width: 0.25rem;
          border-style: solid;
          border-color: transparent var(--font-primary-color) transparent
            transparent;
        }
      }
    }

    // Pseudo element for hovering
    &::before {
      content: "";
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      border-radius: calc(0.5rem - 1px);
      background-color: var(--font-primary-color);
      opacity: 0;
      transition: opacity 0.25s ease-in-out;
    }

    // Common active state
    &:active {
      transform: scale(0.95);
      opacity: 0.8;
      &::before {
        opacity: 0.5;
      }
    }

    // Focus state for accessibility
    &:focus {
      outline: none;
    }

    // Hover
    &:hover {
      .gd-button-tooltip {
        opacity: 1;

        &.--top,
        &.--bottom {
          transform: translateX(-50%) scale(1);
        }
        &.--left,
        &.--right {
          transform: translateY(-50%) scale(1);
        }
      }
      &::before {
        opacity: 0.05;
      }
    }

    &.--align-left {
      justify-content: flex-start;
    }

    &.--align-right {
      justify-content: flex-end;
    }

    &.--align-center {
      justify-content: center;
    }

    &.--size-small {
      height: 1rem;
      padding: 0;
      .gd-button-icon {
        width: calc(1rem - 2px);
        height: 1rem;
        padding: 0 0.125rem;
        box-sizing: border-box;
      }
      .gd-button-label {
        padding: 0 0.25rem;
      }
    }
    &.--size-medium {
      height: 2rem;
      padding: 0;
      .gd-button-icon {
        width: calc(2rem - 2px);
        height: 2rem;
        padding: 0 0.5rem;
        box-sizing: border-box;
      }
      .gd-button-label {
        padding: 0 0.5rem;
      }
    }
    &.--size-large {
      height: 2.5rem;
      padding: 0;
      border-radius: 1.25rem;
      .gd-button-icon {
        width: calc(2.5rem - 2px);
        height: 2.5rem;
        padding: 0 0.75rem;
        box-sizing: border-box;
      }
      .gd-button-label {
        padding: 0 0.75rem;
      }
      &::before {
        border-radius: 1.25rem;
      }
    }

    &.--loading {
      pointer-events: none !important;
      opacity: 0.75;
    }

    &.--borderless {
      border: none;
      background-color: transparent;
    }
    &.--focusable {
      &:focus {
        &::before {
          opacity: 0.1;
        }
      }
    }

    &.--disabled {
      pointer-events: none;
      opacity: 0.5;
      cursor: not-allowed;
    }

    &.--primary {
      background-color: var(--primary-color);
      border-color: transparent;
      .gd-button-label {
        color: var(--font-tertiary-color);
      }
    }

    &.--secondary {
      background-color: var(--background-depth-one-color);
      .gd-button-label {
        color: var(--font-primary-color);
      }
    }

    &.--tertiary {
      background-color: var(--background-depth-one-color);
      .gd-button-label {
        color: var(--font-primary-color);
      }
    }

    &.--success {
      background-color: var(--background-depth-one-color);
      .gd-button-label {
        color: var(--success-color);
      }
    }

    &.--error {
      background-color: var(--background-depth-one-color);
      .gd-button-label {
        color: var(--error-color);
      }
    }

    &.--warning {
      background-color: var(--background-depth-one-color);
      .gd-button-label {
        color: var(--warning-color);
      }
    }
  }
</style>
