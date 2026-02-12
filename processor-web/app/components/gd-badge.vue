<template>
  <div class="gd-badge" :class="`--${props.type}`">
    <span class="gd-badge-text gd-caption-text">{{ props.text }}</span>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  text: string;
  type: "success" | "warning" | "error" | "primary";
}>();
</script>

<style lang="scss" scoped>
.gd-badge {
  position: absolute;
  top: 0;
  right: 0;
  height: 1rem;
  padding: 0 0.25rem;
  border-radius: 0.5rem;
  box-sizing: border-box;
  display: flex;
  align-items: center;
  transform: translate(50%, -50%);
  &-text {
    position: relative;
  }
  &.--success {
    background: var(--success-color);
    .gd-badge-text {
      color: var(--font-tertiary-color);
    }
  }
  &.--warning {
    background: var(--warning-color);
    .gd-badge-text {
      color: var(--font-tertiary-color);
    }
  }
  &.--error {
    background: var(--error-color);
    .gd-badge-text {
      color: var(--font-primary-color);
    }
  }
  &.--primary {
    background: var(--primary-color);
    .gd-badge-text {
      color: var(--font-primary-color);
    }
  }
}
</style>
