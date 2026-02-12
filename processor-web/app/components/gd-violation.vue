<template>
  <div class="gd-violation" :class="`--${severity}`">
    <span class="gd-violation-text gd-headline-6">{{ label }}</span>
  </div>
</template>

<script lang="ts" setup>
import type { EvidencePersonViolation } from "~/types/evidence";

const props = defineProps<{
  violation: EvidencePersonViolation;
}>();

const label = computed(() => {
  const labels: Record<EvidencePersonViolation, string> = {
    missing_hardhat: "Missing Hardhat",
    missing_gloves: "Missing Gloves",
    missing_shoes: "Missing Shoes",
    missing_facemask: "Missing Facemask",
    missing_earmuffs: "Missing Earmuffs",
    missing_safetyvest: "Missing Safety Vest",
    improperly_worn_gloves: "Improper Gloves",
    improperly_worn_shoes: "Improper Shoes",
    improperly_worn_facemask: "Improper Facemask",
    improperly_worn_earmuffs: "Improper Earmuffs",
  };
  return labels[props.violation] || props.violation;
});

const severity = computed(() => {
  if (props.violation.startsWith("missing_")) return "error";
  return "warning";
});
</script>

<style lang="scss" scoped>
.gd-violation {
  position: relative;
  padding: 0.125rem 0.5rem;
  border-radius: 0.25rem;
  background: var(--error-color);
  display: inline-flex;
  align-items: center;

  &.--error {
    background: var(--error-color);
  }

  &.--warning {
    background: var(--warning-color);
  }

  &-text {
    color: var(--font-tertiary-color);
    white-space: nowrap;
  }
}
</style>
