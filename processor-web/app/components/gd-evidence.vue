<template>
  <div class="gd-evidence" @click="emits('click')">
    <div class="gd-evidence-header">
      <div class="gd-evidence-header-information">
        <span class="gd-evidence-header-information-title gd-headline-5">{{
          formatTime(props.evidence.timestamp)
        }}</span>
        <div class="gd-evidence-header-information-id gd-body-5">
          {{ props.evidence.id }}
        </div>
      </div>
      <div class="gd-evidence-header-chip">
        <div class="gd-evidence-header-chip-icon">
          <gd-svg name="account-group" color="tertiary" />
        </div>
        <span class="gd-evidence-header-chip-count gd-headline-6">
          {{ violationCount }}
        </span>
      </div>
    </div>
    <div class="gd-evidence-body">
      <gd-camera-evidence v-if="evidence" :saved="true" :evidence="evidence" />
    </div>
  </div>
</template>

<script lang="ts" setup>
  import type { Evidence } from "~/types/evidence";

  const props = defineProps<{
    evidence: Evidence;
  }>();

  const emits = defineEmits<{
    (event: "click"): void;
  }>();

  const violationCount = computed<number>(() => {
    return props.evidence.person.reduce((acc, person) => {
      return acc + person.violation.length;
    }, 0);
  });

  const formatTime = (timestamp: number): string => {
    const date = new Date(timestamp);
    return date.toLocaleString();
  };
</script>

<style lang="scss" scoped>
  .gd-evidence {
    cursor: pointer;
    position: relative;
    width: 100%;
    border: var(--border);
    border-radius: 0.75rem;
    background: var(--background-depth-one-color);
    padding: 0.75rem;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    overflow: hidden;

    &-header {
      position: relative;
      width: 100%;
      display: flex;
      justify-content: space-between;
      align-items: center;

      &-information {
        position: relative;
        display: flex;
        flex-direction: column;
        &-title {
          position: relative;
        }
        &-id {
          position: relative;
          color: var(--font-secondary-color);
        }
      }
      &-chip {
        position: relative;
        height: 1.5rem;
        padding: 0 0.5rem;
        border-radius: 0.75rem;
        background: var(--warning-color);
        display: flex;
        align-items: center;
        gap: 0.375rem;
        &-icon {
          position: relative;
          width: 0.75rem;
          display: flex;
        }
        &-count {
          position: relative;
          color: var(--font-tertiary-color);
        }
      }
    }

    &-body {
      position: relative;
      width: 100%;
      aspect-ratio: 16 / 9;
      background-color: var(--background-depth-two-color);
      border-radius: 0.5rem;
      display: flex;
      overflow: hidden;
    }
  }
</style>
