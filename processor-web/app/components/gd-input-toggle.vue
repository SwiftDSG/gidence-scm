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
      <div class="gd-input-slider"></div>
    </label>
  </client-only>
</template>

<script lang="ts" setup>
  const props = defineProps<{
    modelValue: boolean;
    name?: string;
    disabled?: boolean;
  }>();
  const emits = defineEmits<{
    "update:modelValue": [value: boolean];
  }>();

  const value = ref(props.modelValue);

  const id = ref(
    `gd-input-toggle-${Math.random().toString(36).substring(2, 15)}`
  );

  watch(
    () => props.modelValue,
    (newValue) => {
      value.value = newValue;
    }
  );
</script>

<style lang="scss" scoped>
  .gd-input {
    cursor: pointer;
    position: relative;
    width: 1.5rem;
    height: 1rem;
    background: var(--border-color);
    border-radius: 0.75rem;
    padding: 0.125rem;
    box-sizing: border-box;
    transition: 0.25s linear background-color;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    &-checkbox {
      pointer-events: none;
      position: absolute;
      opacity: 0;
    }
    &-slider {
      position: absolute;
      top: 0.125rem;
      left: 0.125rem;
      width: 0.75rem;
      height: 0.75rem;
      border-radius: 0.375rem;
      box-sizing: border-box;
      background: var(--background-depth-two-color);
      overflow: hidden;
      display: flex;
      align-items: center;
      transition: width 0.5s cubic-bezier(0.16, 1, 0.3, 1),
        transform 0.5s cubic-bezier(0.16, 1, 0.3, 1);
    }
    &:active {
      .gd-input-slider {
        width: 1rem;
      }
    }
    &.--active {
      background: var(--primary-color);
      .gd-input-slider {
        transform: translateX(0.5rem);
      }
      &:active {
        .gd-input-slider {
          transform: translateX(0.25rem);
          width: 1rem;
        }
      }
    }
    &.--disabled {
      pointer-events: none;
      opacity: 0.5;
    }
  }
</style>
