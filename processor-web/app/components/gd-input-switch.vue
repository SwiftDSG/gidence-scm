<template>
  <client-only>
    <div class="gd-input" ref="gdInput">
      <input
        v-for="option in props.options"
        :id="option.value"
        :name="props.name"
        :value="option.value"
        v-model="value"
        type="radio"
        class="gd-input-radio"
        @change="emits('update:modelValue', option)"
      />
      <label
        v-for="option in props.options"
        :for="option.value"
        class="gd-input-label gd-headline-5"
        :class="{ '--active': value.value === option.value }"
        :style="{
          pointerEvents: value.value === option.value ? 'none' : 'auto',
        }"
      >
        {{ option.label }}
      </label>
      <div
        v-if="left"
        class="gd-input-slider"
        :style="
          rect
            ? {
                width: rect.width - 0.5 * rem + 'px',
                transform: `translateX(${
                  rect.left -
                  left +
                  0.25 * rem -
                  (index / (props.options.length - 1)) * 2
                }px)`,
              }
            : {}
        "
      ></div>
    </div>
  </client-only>
</template>

<script lang="ts" setup>
  type SelectOption = {
    label: string;
    value: string;
  };
  const props = defineProps<{
    modelValue: SelectOption;
    options: SelectOption[];
    name: string;
  }>();
  const emits = defineEmits<{
    "update:modelValue": [value: SelectOption];
  }>();
  const { rem } = useMain();

  const init = ref(false);

  const gdInput = ref<HTMLElement>();

  const value = ref(props.modelValue);

  const index = computed<number>(() => {
    return props.options.findIndex(
      (option) => option.value === value.value.value
    );
  });
  const left = computed<number>(() => {
    if (!init.value || !gdInput.value) return 0;
    return gdInput.value.getBoundingClientRect().left;
  });
  const rect = computed<DOMRect | null>(() => {
    if (index.value === -1) return null;
    const option = props.options[index.value];
    const gdLabel = document.querySelector(`[for="${option!.value}"]`);
    if (!gdLabel) return null;

    return gdLabel.getBoundingClientRect();
  });

  watch(
    () => props.modelValue,
    (newValue) => {
      value.value = newValue;
    }
  );

  onMounted(() => {
    setTimeout(() => {
      init.value = true;
    }, 100);
  });
</script>

<style lang="scss" scoped>
  .gd-input {
    position: relative;
    height: 3rem;
    background: var(--background-depth-one-color);
    border-radius: 1rem;
    border: var(--border);
    box-sizing: border-box;
    display: flex;
    align-items: center;
    &-radio {
      pointer-events: none;
      position: absolute;
      opacity: 0;
    }
    &-label {
      cursor: pointer;
      z-index: 2;
      height: 100%;
      padding: 0 1rem;
      box-sizing: border-box;
      color: var(--font-secondary-color);
      display: flex;
      justify-content: center;
      align-items: center;
      * {
        pointer-events: none;
      }
      &.--active {
        color: var(--primary-color);
      }
    }
    &-slider {
      pointer-events: none;
      position: absolute;
      left: 0;
      height: 2.5rem;
      border-radius: 0.75rem;
      box-sizing: border-box;
      background: var(--background-depth-two-color);
      overflow: hidden;
      display: flex;
      align-items: center;
      transition: width 0.5s cubic-bezier(0.16, 1, 0.3, 1),
        transform 0.5s cubic-bezier(0.16, 1, 0.3, 1);
    }
  }
</style>
