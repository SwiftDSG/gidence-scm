<template>
  <gd-popover :visible="isOpen" @close="handleClose">
    <template #trigger>
      <div
        class="gd-input"
        :class="{
          '--disabled': isDisabled,
          '--opened': isOpen,
        }"
      >
        <label v-if="label" class="gd-input-label gd-body-5">
          {{ label }}
        </label>
        <input
          :id="inputId"
          ref="inputRef"
          :name="name"
          class="gd-input-text"
          :class="inputClass || 'gd-body-4'"
          :disabled="isDisabled"
          :placeholder="placeholder"
          autocomplete="off"
          :value="inputValue"
          @input="handleInput"
          @focus="handleFocus"
          @blur="handleBlur"
        />
        <div class="gd-input-arrow">
          <gd-svg name="chevron-down" color="secondary" />
        </div>
      </div>
    </template>
    <template #content>
      <div ref="optionsRef" class="gd-input-options">
        <span
          v-if="!filteredOptions.length"
          class="gd-input-options-message gd-body-5"
        >
          Tidak ada opsi
        </span>
        <gd-button
          v-for="option in filteredOptions"
          :key="option.value"
          :ref="(el) => setOptionRef(el, option.value)"
          :text="option.label"
          alignment="left"
          type="tertiary"
          font="body"
          borderless
          focusable
          @click="handleSelect(option)"
        />
      </div>
    </template>
  </gd-popover>
</template>

<script lang="ts" setup>
// Types
type SelectOption = {
  label: string;
  value: string;
};

type SelectProps = {
  options: SelectOption[];
  modelValue: SelectOption | null;
  placeholder?: string;
  label?: string;
  name?: string;
  error?: string;
  class?: string;
  small?: boolean;
  strict?: boolean;
  disabled?: boolean;
};

type SelectEmits = {
  "update:modelValue": [value: SelectOption | null];
  change: [value: SelectOption | null];
  focus: [event: FocusEvent];
  blur: [event: FocusEvent];
};

// Props with defaults
const props = withDefaults(defineProps<SelectProps>(), {
  options: () => [],
  placeholder: "",
  disabled: false,
  strict: false,
  small: false,
});

// Emits
const emit = defineEmits<SelectEmits>();

// Refs
const inputRef = ref<HTMLInputElement>();
const optionsRef = ref<HTMLDivElement>();
const optionRefs = ref<Map<string, HTMLElement>>(new Map());
const focusedOptionValue = ref<string | null>(null);

// State
const isOpen = ref(false);
const inputValue = ref("");
const filteredOptions = ref<SelectOption[]>([]);

// Computed
const inputId = computed(
  () => `gd-input-select-${Math.random().toString(36).substring(2, 15)}`,
);

const isDisabled = computed(
  () => props.disabled || (props.strict && props.options.length === 0),
);

const inputClass = computed(() => props.class);

// Initialize input value from modelValue
watch(
  () => props.modelValue,
  (newValue) => {
    if (newValue) {
      inputValue.value = newValue.label;
    } else {
      inputValue.value = "";
    }
  },
  { immediate: true },
);

// Watch options changes
watch(
  () => props.options,
  () => {
    filterOptions();
  },
);

// Methods
const setOptionRef = (el: any, value: string) => {
  if (el) {
    optionRefs.value.set(value, el.$el || el);
  }
};

const filterOptions = () => {
  if (!inputValue.value) {
    filteredOptions.value = props.options;
    return;
  }

  const searchTerm = inputValue.value.toLowerCase().replace(/\s+/g, "");
  filteredOptions.value = props.options.filter((option) =>
    option.label.toLowerCase().replace(/\s+/g, "").includes(searchTerm),
  );
};

const handleInput = (event: Event) => {
  const target = event.target as HTMLInputElement;
  inputValue.value = target.value;

  if (!isOpen.value) {
    isOpen.value = true;
  }

  filterOptions();
};

const handleFocus = (event: FocusEvent) => {
  if (!isDisabled.value) {
    isOpen.value = true;
    filterOptions();
  }
  emit("focus", event);
};

const handleBlur = (event: FocusEvent) => {
  emit("blur", event);
};

const handleSelect = (option: SelectOption) => {
  focusedOptionValue.value = null;
  inputValue.value = option.label;

  emit("update:modelValue", option);
  emit("change", option);

  isOpen.value = false;
  nextTick(() => {
    inputRef.value?.focus();
  });
};

const handleClose = () => {
  isOpen.value = false;
  focusedOptionValue.value = null;

  // Validate and update value based on strict mode
  if (props.strict) {
    const matchingOption = props.options.find(
      (option) => option.label.toLowerCase() === inputValue.value.toLowerCase(),
    );

    if (!matchingOption) {
      // Reset if no match in strict mode
      inputValue.value = props.modelValue?.label || "";
      emit("update:modelValue", props.modelValue || null);
      emit("change", props.modelValue || null);
    } else if (matchingOption !== props.modelValue) {
      // Update to matching option
      emit("update:modelValue", matchingOption);
      emit("change", matchingOption);
    }
  } else if (inputValue.value) {
    // Allow custom value if not strict
    const matchingOption = props.options.find(
      (option) => option.label.toLowerCase() === inputValue.value.toLowerCase(),
    );

    const newValue = matchingOption || {
      label: inputValue.value,
      value: inputValue.value,
    };

    if (newValue !== props.modelValue) {
      emit("update:modelValue", newValue);
      emit("change", newValue);
    }
  }
};

const handleKeydown = (event: KeyboardEvent) => {
  if (!isOpen.value || !filteredOptions.value.length) return;

  switch (event.key) {
    case "ArrowUp":
      event.preventDefault();
      navigateOptions("up");
      break;
    case "ArrowDown":
      event.preventDefault();
      navigateOptions("down");
      break;
    case "Enter":
      event.preventDefault();
      selectFocusedOption();
      break;
    case " ":
      if (focusedOptionValue.value) {
        event.preventDefault();
        selectFocusedOption();
      }
      break;
    case "Escape":
      event.preventDefault();
      isOpen.value = false;
      inputRef.value?.focus();
      break;
    default:
      // Reset focus to input for typing
      if (focusedOptionValue.value && event.key.length === 1) {
        focusedOptionValue.value = null;
        inputRef.value?.focus();
      }
  }
};

const navigateOptions = (direction: "up" | "down") => {
  const currentIndex = focusedOptionValue.value
    ? filteredOptions.value.findIndex(
        (opt) => opt.value === focusedOptionValue.value,
      )
    : -1;

  let nextIndex: number;
  if (direction === "up") {
    nextIndex =
      currentIndex > 0 ? currentIndex - 1 : filteredOptions.value.length - 1;
  } else {
    nextIndex =
      currentIndex < filteredOptions.value.length - 1 ? currentIndex + 1 : 0;
  }

  const nextOption = filteredOptions.value[nextIndex];
  if (nextOption) {
    focusedOptionValue.value = nextOption.value;
    const optionEl = optionRefs.value.get(nextOption.value);
    optionEl?.focus();
  }
};

const selectFocusedOption = () => {
  if (focusedOptionValue.value) {
    const option = filteredOptions.value.find(
      (opt) => opt.value === focusedOptionValue.value,
    );
    if (option) {
      handleSelect(option);
    }
  }
};

// Keyboard event listeners
watch(isOpen, (open) => {
  if (open) {
    window.addEventListener("keydown", handleKeydown);
    filterOptions();
  } else {
    window.removeEventListener("keydown", handleKeydown);
    optionRefs.value.clear();
  }
});

// Cleanup
onUnmounted(() => {
  window.removeEventListener("keydown", handleKeydown);
});
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

  &-arrow {
    cursor: pointer;
    position: absolute;
    right: 0;
    bottom: 0;
    width: 2rem;
    height: 2rem;
    padding: 0 0.5rem;
    box-sizing: border-box;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: transform 0.25s ease;
  }

  &.--opened {
    .gd-input-arrow {
      transform: rotate(180deg);
    }
  }

  &.--disabled {
    pointer-events: none;
    opacity: 0.5;
    cursor: not-allowed;

    * {
      pointer-events: none;
    }
  }
}
</style>

<style lang="scss">
.gd-input-options {
  overflow-y: auto;
  display: flex;
  flex-direction: column;

  &-message {
    position: relative;
    width: 100%;
    height: 2rem;
    color: var(--font-secondary-color);
    display: flex;
    justify-content: center;
    align-items: center;
  }
}
</style>
