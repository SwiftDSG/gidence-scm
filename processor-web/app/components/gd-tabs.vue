<template>
  <div class="gd-tabs">
    <div class="gd-tabs-header">
      <button
        v-for="tab in tabs"
        :key="tab.name"
        :class="[
          'gd-tabs-header-tab',
          'gd-body-3',
          { '--active': modelValue === tab.name },
        ]"
        @click="selectTab(tab.name)"
        type="button"
      >
        {{ tab.label }}
      </button>
    </div>
    <div class="gd-tabs-content">
      <slot></slot>
    </div>
  </div>
</template>

<script lang="ts" setup>
  interface Tab {
    name: string;
    label: string;
  }

  interface TabsProps {
    modelValue: string;
  }

  const props = defineProps<TabsProps>();

  const emits = defineEmits<{
    (event: 'update:modelValue', value: string): void;
  }>();

  const tabs = ref<Tab[]>([]);

  const selectTab = (name: string) => {
    emits('update:modelValue', name);
  };

  const registerTab = (tab: Tab) => {
    const existingIndex = tabs.value.findIndex(t => t.name === tab.name);
    if (existingIndex === -1) {
      tabs.value.push(tab);
    }
  };

  const unregisterTab = (name: string) => {
    const index = tabs.value.findIndex(t => t.name === name);
    if (index !== -1) {
      tabs.value.splice(index, 1);
    }
  };

  provide('tabs', {
    activeTab: computed(() => props.modelValue),
    registerTab,
    unregisterTab,
  });
</script>

<style lang="scss" scoped>
  .gd-tabs {
    position: relative;
    width: 100%;
    display: flex;
    flex-direction: column;

    &-header {
      position: relative;
      display: flex;
      border-bottom: var(--border);
      background-color: var(--background-depth-one-color);

      &-tab {
        position: relative;
        padding: 0.75rem 1rem;
        background: transparent;
        border: none;
        cursor: pointer;
        color: var(--font-secondary-color);
        border-bottom: 2px solid transparent;
        transition: color 0.2s ease-in-out, border-bottom-color 0.2s ease-in-out;

        &:hover {
          color: var(--font-primary-color);
          background-color: var(--background-depth-two-color);
        }

        &.--active {
          color: var(--primary-color);
          border-bottom-color: var(--primary-color);
          background-color: var(--background-depth-two-color);
        }

        &:focus {
          outline: none;
          background-color: var(--background-depth-two-color);
        }
      }
    }

    &-content {
      position: relative;
      background-color: var(--background-depth-two-color);
      min-height: 10rem;
    }
  }
</style>