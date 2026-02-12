<template>
  <div class="gd-input" :class="{ '--disabled': disabled }">
    <label v-if="label" class="gd-input-label gd-headline-6">
      {{ label }}
    </label>
    <input
      :id="inputId"
      :name="name"
      type="date"
      class="gd-input-date"
      :class="inputClass || 'gd-body-4'"
      :disabled="disabled"
      :placeholder="placeholder"
      :min="min"
      :max="max"
      :value="modelValue"
      @input="handleInput"
      @focus="handleFocus"
      @blur="handleBlur"
    />
  </div>
</template>

<script lang="ts" setup>
  import { computed } from "#imports";

  // Types
  interface InputDateProps {
    modelValue: string;
    label?: string;
    placeholder?: string;
    name?: string;
    min?: string;
    max?: string;
    error?: string;
    disabled?: boolean;
    class?: string;
  }

  interface InputDateEmits {
    "update:modelValue": [value: string];
    change: [value: string];
    focus: [event: FocusEvent];
    blur: [event: FocusEvent];
  }

  // Props with defaults
  const props = withDefaults(defineProps<InputDateProps>(), {
    disabled: false,
  });

  // Emits
  const emit = defineEmits<InputDateEmits>();

  // Computed
  const inputId = computed(
    () => `gd-input-date-${Math.random().toString(36).substring(2, 15)}`
  );

  const inputClass = computed(() => props.class);

  // Methods
  const handleInput = (event: Event) => {
    const target = event.target as HTMLInputElement;
    const value = target.value;

    emit("update:modelValue", value);
    emit("change", value);
  };

  const handleFocus = (event: FocusEvent) => {
    emit("focus", event);
  };

  const handleBlur = (event: FocusEvent) => {
    emit("blur", event);
  };
</script>

<style lang="scss" scoped>
  .gd-input {
    cursor: pointer;
    position: relative;
    width: 100%;
    display: flex;
    flex-direction: column;

    &-label {
      position: relative;
      width: 100%;
      height: 1rem;
      display: flex;
      align-items: center;
      color: var(--font-secondary-color);
    }

    &-date {
      position: relative;
      width: 100%;
      height: 2rem;
      padding: 0 0.5rem;
      border-radius: 0.5rem;
      border: var(--border);
      color: var(--font-primary-color);
      box-sizing: border-box;
      background-color: var(--background-depth-one-color);
      transition: border-color 0.25s ease-in-out;

      &::placeholder {
        opacity: 0.5;
        transition: opacity 0.25s ease-in-out;
      }

      &:focus {
        outline: none;
        border-color: var(--primary-color);

        &::placeholder {
          opacity: 1;
        }
      }

      &:hover {
        &::placeholder {
          opacity: 1;
        }
      }

      // Date input specific styles
      &::-webkit-calendar-picker-indicator {
        cursor: pointer;
        opacity: 0.6;
        transition: opacity 0.25s ease-in-out;

        &:hover {
          opacity: 1;
        }
      }

      // Firefox date input styles
      &::-moz-focus-inner {
        border: 0;
      }
    }

    // Disabled styles moved to global.scss with --disabled modifier
  }
</style>