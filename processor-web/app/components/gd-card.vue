<template>
  <div
    :class="[
      'gd-card',
      `--type-${type}`,
      { '--elevated': elevated },
    ]"
  >
    <div v-if="title || subtitle || $slots.header" class="gd-card-header">
      <div v-if="title || subtitle" class="gd-card-header-content">
        <h3 v-if="title" class="gd-card-header-content-title gd-headline-3">
          {{ title }}
        </h3>
        <p v-if="subtitle" class="gd-card-header-content-subtitle gd-body-4">
          {{ subtitle }}
        </p>
      </div>
      <div v-if="$slots.header" class="gd-card-header-slot">
        <slot name="header"></slot>
      </div>
    </div>

    <div v-if="$slots.content || $slots.default" class="gd-card-body">
      <slot name="content">
        <slot></slot>
      </slot>
    </div>

    <div v-if="$slots.actions" class="gd-card-footer">
      <slot name="actions"></slot>
    </div>
  </div>
</template>

<script lang="ts" setup>
  interface CardProps {
    title?: string;
    subtitle?: string;
    type?: "primary" | "secondary" | "success" | "error" | "warning";
    elevated?: boolean;
  }

  const props = withDefaults(defineProps<CardProps>(), {
    type: "secondary",
    elevated: false,
  });
</script>

<style lang="scss" scoped>
  .gd-card {
    position: relative;
    width: 100%;
    background-color: var(--background-depth-one-color);
    border: var(--border);
    border-radius: 0.75rem;
    overflow: hidden;
    display: flex;
    flex-direction: column;

    &-header {
      position: relative;
      padding: 1rem;
      border-bottom: var(--border);
      display: flex;
      align-items: flex-start;
      justify-content: space-between;
      gap: 1rem;

      &-content {
        position: relative;
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        flex: 1;

        &-title {
          position: relative;
          color: var(--font-primary-color);
          margin: 0;
        }

        &-subtitle {
          position: relative;
          color: var(--font-secondary-color);
          margin: 0;
        }
      }

      &-slot {
        position: relative;
        display: flex;
        align-items: center;
        flex-shrink: 0;
      }
    }

    &-body {
      position: relative;
      padding: 1rem;
      flex: 1;
      color: var(--font-primary-color);
    }

    &-footer {
      position: relative;
      padding: 1rem;
      border-top: var(--border);
      display: flex;
      align-items: center;
      justify-content: flex-end;
      gap: 0.5rem;
    }

    &.--elevated {
      box-shadow: 0 0.25rem 0.5rem rgba(0, 0, 0, 0.1);
    }

    &.--type-primary {
      border-color: var(--primary-color);
      .gd-card-header {
        border-bottom-color: var(--primary-color);
      }
      .gd-card-footer {
        border-top-color: var(--primary-color);
      }
    }

    &.--type-success {
      border-color: var(--success-color);
      .gd-card-header {
        border-bottom-color: var(--success-color);
      }
      .gd-card-footer {
        border-top-color: var(--success-color);
      }
    }

    &.--type-error {
      border-color: var(--error-color);
      .gd-card-header {
        border-bottom-color: var(--error-color);
      }
      .gd-card-footer {
        border-top-color: var(--error-color);
      }
    }

    &.--type-warning {
      border-color: var(--warning-color);
      .gd-card-header {
        border-bottom-color: var(--warning-color);
      }
      .gd-card-footer {
        border-top-color: var(--warning-color);
      }
    }
  }
</style>