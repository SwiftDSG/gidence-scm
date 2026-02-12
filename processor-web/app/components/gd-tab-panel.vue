<template>
  <div
    v-if="isActive"
    class="gd-tab-panel"
  >
    <slot></slot>
  </div>
</template>

<script lang="ts" setup>
  interface TabPanelProps {
    name: string;
    label: string;
  }

  const props = defineProps<TabPanelProps>();

  const tabs = inject('tabs') as {
    activeTab: ComputedRef<string>;
    registerTab: (tab: { name: string; label: string }) => void;
    unregisterTab: (name: string) => void;
  };

  const isActive = computed(() => tabs.activeTab.value === props.name);

  onMounted(() => {
    tabs.registerTab({
      name: props.name,
      label: props.label,
    });
  });

  onUnmounted(() => {
    tabs.unregisterTab(props.name);
  });
</script>

<style lang="scss" scoped>
  .gd-tab-panel {
    position: relative;
    padding: 1rem;
    color: var(--font-primary-color);
  }
</style>