<template>
  <div class="gd-table">
    <table class="gd-table-content">
      <thead v-if="props.header" class="gd-table-content-header">
        <th
          class="gd-table-content-header-item gd-headline-6"
          v-for="(text, i) in props.header"
          :key="i"
        >
          {{ text }}
        </th>
      </thead>
      <tbody class="gd-table-content-body">
        <slot />
      </tbody>
    </table>
  </div>
</template>

<script lang="ts" setup>
  const props = defineProps<{
    header: string[];
  }>();
</script>

<style lang="scss" scoped>
  .gd-table {
    position: relative;
    width: 100%;
    border-radius: 0.5rem;
    border: var(--border);
    box-sizing: border-box;
    display: flex;
    overflow: auto;
    &-content {
      position: relative;
      width: 100%;
      border-collapse: collapse;

      &-header {
        width: 100%;
        background-color: var(--background-depth-two-color);
        border-bottom: var(--border);
        box-sizing: border-box;

        &-item {
          position: sticky;
          top: 0;
          left: 0;
          padding: 0.5rem;
          box-sizing: border-box;
          background-color: var(--background-depth-two-color);
          color: var(--font-secondary-color);
          text-align: left;
          white-space: nowrap;
        }
      }
    }
  }
</style>
