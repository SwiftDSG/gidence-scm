<template>
  <client-only>
    <label
      class="gd-input"
      :class="{
        '--disabled': props.disabled,
        '--active': modelValue,
      }"
      :for="id"
    >
      <input
        :id="id"
        :name="name || id"
        :disabled="disabled"
        v-model="value"
        type="checkbox"
        class="gd-input-checkbox"
        @change="emits('update:modelValue', value)"
      />
      <div class="gd-input-indicator">
        <div class="gd-input-indicator-inner">
          <gd-svg name="check" color="tertiary" />
        </div>
      </div>
      <span class="gd-input-label gd-body-4">
        {{ props.label }}
      </span>
    </label>
  </client-only>
</template>

<script lang="ts" setup>
const props = defineProps<{
  modelValue: boolean;
  label?: string;
  name?: string;
  disabled?: boolean;
}>();
const emits = defineEmits<{
  "update:modelValue": [value: boolean];
}>();

const value = ref(props.modelValue);

const id = ref(
  `gd-input-checkbox-${Math.random().toString(36).substring(2, 15)}`,
);

watch(
  () => props.modelValue,
  (newValue) => {
    value.value = newValue;
  },
);
</script>

<style lang="scss" scoped>
.gd-input {
  cursor: pointer;
  position: relative;
  display: flex;
  gap: 0.5rem;
  &-checkbox {
    pointer-events: none;
    position: absolute;
    opacity: 0;
  }
  &-indicator {
    position: relative;
    width: 0.75rem;
    height: 0.75rem;
    background: var(--border-color);
    border-radius: 0.25rem;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    transition: background-color 0.25s cubic-bezier(0.16, 1, 0.3, 1);
    &-inner {
      position: relative;
      width: 100%;
      height: 100%;
      padding: 0.0625rem;
      box-sizing: border-box;
      display: flex;
      align-items: center;
      justify-content: center;
      transition:
        opacity 0.5s cubic-bezier(0.16, 1, 0.3, 1),
        transform 0.5s cubic-bezier(0.16, 1, 0.3, 1);
      opacity: 0;
      transform: scale(0);
    }
  }
  &.--active {
    .gd-input-indicator {
      background: var(--primary-color);
      &-inner {
        transform: scale(1);
        opacity: 1;
      }
    }
  }
  &.--disabled {
    pointer-events: none;
    opacity: 0.5;
  }
}
</style>
