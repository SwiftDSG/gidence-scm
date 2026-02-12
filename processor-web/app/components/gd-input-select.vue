<template>
  <gd-popover :visible="opened" @close="opened = false">
    <template #trigger>
      <div class="gd-input">
        <label v-if="label" class="gd-input-label gd-body-5">{{ label }}</label>
        <button
          :class="{
            '--small': props.small,
            '--disabled': props.disabled,
            '--opened': opened,
          }"
          class="gd-input-select"
          @click.prevent="handleClick"
        >
          <span
            class="gd-input-select-text gd-body-4"
            :class="{ '--placeholder': !props.modelValue.value }"
          >
            {{ props.modelValue.label || props.placeholder }}
          </span>
          <div class="gd-input-select-icon">
            <gd-svg name="chevron-down" color="secondary" />
          </div>
        </button>
      </div>
    </template>
    <template #content>
      <gd-button
        v-for="option in props.options"
        :key="option.value"
        :text="option.label"
        alignment="left"
        type="tertiary"
        font="body"
        borderless
        focusable
        @click="
          () => {
            emits('update:modelValue', option);
            opened = false;
          }
        "
      />
    </template>
  </gd-popover>
</template>

<script lang="ts" setup>
// Types
type SelectOption = {
  label: string;
  value: string;
};
const props = defineProps<{
  modelValue: SelectOption;
  options: SelectOption[];
  label?: string;
  placeholder?: string;
  small?: boolean;
  disabled?: boolean;
}>();
const emits = defineEmits<{
  "update:modelValue": [value: SelectOption];
}>();

const opened = ref(false);

const handleClick = () => {
  if (!props.disabled) {
    opened.value = !opened.value;
  }
};
</script>

<style lang="scss" scoped>
.gd-input {
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
  &-select {
    cursor: pointer;
    position: relative;
    width: 100%;
    height: 2rem;
    padding: 0 0.5rem;
    border-radius: 0.5rem;
    border: var(--border);
    background-color: var(--background-depth-one-color);
    box-sizing: border-box;
    display: flex;
    align-items: center;
    gap: 0.25rem;
    transition:
      background-color 0.25s,
      transform 0.8s cubic-bezier(0.16, 1, 0.3, 1),
      opacity 0.2s ease-in-out,
      border-color 0.25s ease-in-out;

    * {
      pointer-events: none;
    }

    &-text {
      position: relative;
      width: calc(100% - 1rem);
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
      text-align: left;
      color: var(--font-primary-color);
      transition: 0.25s opacity;
      &.--placeholder {
        opacity: 0.5;
        color: var(--font-secondary-color);
        transition: opacity 0.25s ease-in-out;
      }
    }
    &-icon {
      position: absolute;
      right: 0.5rem;
      width: 1rem;
      height: 1rem;
      display: flex;
      justify-content: center;
      align-items: center;
      transition: transform 0.25s ease;
    }

    &:hover {
      .gd-input-select-icon,
      .gd-input-select-text.--placeholder {
        opacity: 0.8;
      }
    }
    &:active {
      opacity: 0.8;
    }

    &.--small {
      height: 1rem;
      padding: 0 0.125rem 0 0.25rem;
      gap: 0.125rem;
    }
    &.--disabled {
      pointer-events: none;
      opacity: 0.5;
      cursor: not-allowed;
    }
    &.--opened {
      border-color: var(--primary-color);
      .gd-input-select-icon {
        transform: rotate(180deg);
      }
    }
  }
}
</style>
