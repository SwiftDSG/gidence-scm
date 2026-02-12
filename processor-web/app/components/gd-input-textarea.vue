<template>
  <div class="gd-input" :class="{ '--disabled': disabled }">
    <label v-if="label" class="gd-input-label gd-body-5">
      {{ label }}
    </label>
    <textarea
      :id="inputId"
      :name="name"
      class="gd-input-textarea"
      :class="inputClass || 'gd-body-4'"
      :disabled="disabled"
      :placeholder="placeholder"
      :maxlength="limit"
      :rows="rows"
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
  interface InputTextareaProps {
    modelValue: string;
    label?: string;
    placeholder?: string;
    name?: string;
    rows?: number;
    limit?: number;
    error?: string;
    disabled?: boolean;
    class?: string;
  }

  interface InputTextareaEmits {
    "update:modelValue": [value: string];
    change: [value: string];
    focus: [event: FocusEvent];
    blur: [event: FocusEvent];
  }

  // Props with defaults
  const props = withDefaults(defineProps<InputTextareaProps>(), {
    rows: 4,
    disabled: false,
  });

  // Emits
  const emit = defineEmits<InputTextareaEmits>();

  // Computed
  const inputId = computed(
    () => `gd-input-textarea-${Math.random().toString(36).substring(2, 15)}`
  );

  const inputClass = computed(() => props.class);

  // Methods
  const handleInput = (event: Event) => {
    const target = event.target as HTMLTextAreaElement;
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

    &-textarea {
      position: relative;
      width: 100%;
      min-height: 4rem;
      padding: 0.5rem;
      border-radius: 0.5rem;
      border: var(--border);
      color: var(--font-primary-color);
      box-sizing: border-box;
      background-color: var(--background-depth-one-color);
      transition: border-color 0.25s ease-in-out;
      resize: vertical;
      font-family: inherit;

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
    }

    // Disabled styles moved to global.scss with --disabled modifier
  }
</style>
