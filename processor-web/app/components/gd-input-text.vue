<template>
  <div class="gd-input" :class="{ '--disabled': disabled }">
    <label v-if="label" class="gd-input-label gd-body-5">
      {{ label }}
    </label>
    <input
      :id="inputId"
      :name="name"
      :type="currentType"
      class="gd-input-text"
      :class="inputClass || 'gd-body-4'"
      :disabled="disabled"
      :placeholder="placeholder"
      :maxlength="limit"
      :min="min"
      :max="max"
      :step="step"
      :autocomplete="autocomplete"
      :value="modelValue"
      @input="handleInput"
      @focus="handleFocus"
      @blur="handleBlur"
      :style="{ textAlign: props.alignment }"
    />
    <div
      v-if="type === 'password'"
      class="gd-input-show"
      @click="togglePasswordVisibility"
    >
      <gd-svg
        :name="currentType === 'text' ? 'eye' : 'eye-off'"
        color="secondary"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed, ref } from "#imports";

// Types
interface InputProps {
  modelValue: string;
  label?: string;
  placeholder?: string;
  name?: string;
  type?: "text" | "password" | "email" | "tel" | "url" | "number";
  alignment?: "left" | "center" | "right";
  min?: number;
  max?: number;
  step?: number;
  limit?: number;
  error?: string;
  disabled?: boolean;
  class?: string;
  autocomplete?: string;
}

interface InputEmits {
  "update:modelValue": [value: string];
  change: [value: string];
  focus: [event: FocusEvent];
  blur: [event: FocusEvent];
}

// Props with defaults
const props = withDefaults(defineProps<InputProps>(), {
  type: "text",
  autocomplete: "off",
  disabled: false,
});

// Emits
const emit = defineEmits<InputEmits>();

// State
const showPassword = ref(false);

// Computed
const inputId = computed(
  () => `gd-input-text-${Math.random().toString(36).substring(2, 15)}`,
);

const currentType = computed(() => {
  if (props.type === "password" && showPassword.value) {
    return "text";
  }
  return props.type;
});

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

const togglePasswordVisibility = () => {
  if (!props.disabled) {
    showPassword.value = !showPassword.value;
  }
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

  &-text {
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
      color: var(--font-secondary-color);
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

  &-show {
    cursor: pointer;
    position: absolute;
    right: 0.25rem;
    bottom: 0;
    width: 2rem;
    height: 2rem;
    padding: 0 0.5rem;
    box-sizing: border-box;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  // Disabled styles moved to global.scss with --disabled modifier
}
</style>
