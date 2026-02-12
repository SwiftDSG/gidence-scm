<template>
  <div class="gd-evidence" @click="emits('click')">
    <div class="gd-evidence-thumbnail">
      <img
        v-if="imageUrl"
        :src="imageUrl"
        alt="Evidence"
        class="gd-evidence-thumbnail-image"
      />
      <div v-else class="gd-evidence-thumbnail-placeholder">
        <gd-svg name="image" color="secondary" />
      </div>
    </div>
    <div class="gd-evidence-info">
      <div class="gd-evidence-info-header">
        <span class="gd-evidence-info-header-camera gd-headline-5">{{
          cameraName
        }}</span>
        <span class="gd-evidence-info-header-time gd-body-5">{{
          formatTime(evidence.timestamp)
        }}</span>
      </div>
      <div class="gd-evidence-info-violations">
        <gd-violation
          v-for="(violation, index) in allViolations"
          :key="index"
          :violation="violation"
        />
      </div>
      <div class="gd-evidence-info-stats">
        <span class="gd-body-5">{{ evidence.person.length }} person(s)</span>
        <span class="gd-body-5">{{ allViolations.length }} violation(s)</span>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import type { Evidence, EvidencePersonViolation } from "~/types/evidence";

const props = defineProps<{
  evidence: Evidence;
  cameraName?: string;
  imageUrl?: string;
}>();

const emits = defineEmits<{
  (event: "click"): void;
}>();

const allViolations = computed<EvidencePersonViolation[]>(() => {
  return props.evidence.person.flatMap((p) => p.violation);
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
  box-sizing: border-box;
  display: flex;
  overflow: hidden;
  transition: transform 0.2s ease-in-out, box-shadow 0.2s ease-in-out;

  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  &-thumbnail {
    position: relative;
    width: 8rem;
    height: 6rem;
    flex-shrink: 0;
    background: var(--background-depth-two-color);
    display: flex;
    justify-content: center;
    align-items: center;

    &-image {
      width: 100%;
      height: 100%;
      object-fit: cover;
    }

    &-placeholder {
      width: 2rem;
      height: 2rem;
      opacity: 0.5;
    }
  }

  &-info {
    position: relative;
    flex: 1;
    padding: 0.75rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    &-header {
      display: flex;
      justify-content: space-between;
      align-items: center;

      &-camera {
        color: var(--font-primary-color);
      }

      &-time {
        color: var(--font-secondary-color);
      }
    }

    &-violations {
      display: flex;
      flex-wrap: wrap;
      gap: 0.25rem;
    }

    &-stats {
      display: flex;
      gap: 1rem;
      color: var(--font-secondary-color);
    }
  }
}
</style>
