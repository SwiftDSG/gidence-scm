<template>
  <client-only>
    <label
      v-for="option in options"
      class="gd-input"
      :class="{
        '--disabled': props.disabled,
        '--active': value === option.value,
      }"
      :for="option.value"
    >
      <input
        :id="option.value"
        :name="name"
        :disabled="disabled"
        v-model="value"
        type="radio"
        class="gd-input-radio"
        @change="handleChange(option.value)"
      />
      <div class="gd-input-indicator">
        <div class="gd-input-indicator-inner"></div>
      </div>
      <span class="gd-input-label gd-body-4">
        {{ option.label }}
      </span>
    </label>
  </client-only>
</template>

<script lang="ts" setup>
type SelectOption = {
  label: string;
  value: string;
};

const props = defineProps<{
  modelValue: string;
  options: SelectOption[];
  name: string;
  disabled?: boolean;
}>();
const emits = defineEmits<{
  "update:modelValue": [value: string];
}>();

const value = ref(props.modelValue);

function handleChange(value: string) {
  if (props.modelValue === value) return;
  emits("update:modelValue", value);
}

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
  align-items: center;
  gap: 0.5rem;
  &-radio {
    pointer-events: none;
    position: absolute;
    opacity: 0;
  }
  &-indicator {
    position: relative;
    width: 0.75rem;
    height: 0.75rem;
    padding: 0.125rem;
    background: var(--border-color);
    border-radius: 0.5rem;
    box-sizing: border-box;
    transition: 0.5s cubic-bezier(0.16, 1, 0.3, 1) background-color;
    &-inner {
      position: absolute;
      top: 50%;
      left: 50%;
      width: 0;
      height: 0;
      border-radius: 50%;
      background-color: var(--font-tertiary-color);
      transform: translate(-50%, -50%);
      transform-origin: center center;
      transition:
        opacity 0.5s cubic-bezier(0.16, 1, 0.3, 1),
        width 0.5s cubic-bezier(0.16, 1, 0.3, 1),
        height 0.5s cubic-bezier(0.16, 1, 0.3, 1);
      opacity: 0;
    }
  }
  &.--active {
    .gd-input-indicator {
      background: var(--primary-color);
      &-inner {
        width: 60%;
        height: 60%;
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
