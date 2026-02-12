<template>
  <gd-menu title="Evidence Detail" @close="emits('close')">
    <div class="gd-evidence-menu">
      <div class="gd-evidence-menu-image">
        <img
          v-if="imageUrl"
          :src="imageUrl"
          alt="Evidence"
          class="gd-evidence-menu-image-preview"
        />
        <div v-else class="gd-evidence-menu-image-placeholder">
          <gd-svg name="image" color="secondary" />
          <span class="gd-body-4">No image available</span>
        </div>
      </div>

      <div class="gd-evidence-menu-info">
        <div class="gd-evidence-menu-info-row">
          <span class="gd-evidence-menu-info-row-label gd-body-4">Camera</span>
          <span class="gd-evidence-menu-info-row-value gd-headline-5">{{
            cameraName
          }}</span>
        </div>
        <div class="gd-evidence-menu-info-row">
          <span class="gd-evidence-menu-info-row-label gd-body-4">Time</span>
          <span class="gd-evidence-menu-info-row-value gd-headline-5">{{
            formatTime(evidence.timestamp)
          }}</span>
        </div>
        <div class="gd-evidence-menu-info-row">
          <span class="gd-evidence-menu-info-row-label gd-body-4">Frame ID</span>
          <span class="gd-evidence-menu-info-row-value gd-body-4">{{
            evidence.frame_id
          }}</span>
        </div>
      </div>

      <div class="gd-evidence-menu-persons">
        <span class="gd-headline-4">Detected Persons ({{ evidence.person.length }})</span>
        <div
          v-for="person in evidence.person"
          :key="person.id"
          class="gd-evidence-menu-persons-person"
        >
          <div class="gd-evidence-menu-persons-person-header">
            <span class="gd-headline-5">Person {{ person.id.slice(0, 8) }}</span>
            <span class="gd-body-5">Confidence: {{ (person.confidence * 100).toFixed(1) }}%</span>
          </div>

          <div v-if="person.violation.length > 0" class="gd-evidence-menu-persons-person-violations">
            <span class="gd-body-4">Violations:</span>
            <div class="gd-evidence-menu-persons-person-violations-list">
              <gd-violation
                v-for="(violation, i) in person.violation"
                :key="i"
                :violation="violation"
              />
            </div>
          </div>

          <div v-if="person.equipment.length > 0" class="gd-evidence-menu-persons-person-equipment">
            <span class="gd-body-4">Equipment detected:</span>
            <div class="gd-evidence-menu-persons-person-equipment-list">
              <span
                v-for="eq in person.equipment"
                :key="eq.label"
                class="gd-evidence-menu-persons-person-equipment-item gd-headline-6"
              >
                {{ eq.label }}
              </span>
            </div>
          </div>

          <div v-if="person.part.length > 0" class="gd-evidence-menu-persons-person-parts">
            <span class="gd-body-4">Body parts detected:</span>
            <div class="gd-evidence-menu-persons-person-parts-list">
              <span
                v-for="part in person.part"
                :key="part.label"
                class="gd-evidence-menu-persons-person-parts-item gd-body-5"
              >
                {{ part.label }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </gd-menu>
</template>

<script lang="ts" setup>
import type { Evidence } from "~/types/evidence";

const props = defineProps<{
  evidence: Evidence;
  cameraName?: string;
  imageUrl?: string;
}>();

const emits = defineEmits<{
  (event: "close"): void;
}>();

const formatTime = (timestamp: number): string => {
  const date = new Date(timestamp);
  return date.toLocaleString();
};
</script>

<style lang="scss" scoped>
.gd-evidence-menu {
  position: relative;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1rem;
  max-height: 70vh;
  overflow-y: auto;

  &-image {
    position: relative;
    width: 100%;
    border-radius: 0.5rem;
    overflow: hidden;
    background: var(--background-depth-two-color);

    &-preview {
      width: 100%;
      height: auto;
      display: block;
    }

    &-placeholder {
      width: 100%;
      height: 12rem;
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
      gap: 0.5rem;
      color: var(--font-secondary-color);
    }
  }

  &-info {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    &-row {
      display: flex;
      justify-content: space-between;
      align-items: center;

      &-label {
        color: var(--font-secondary-color);
      }

      &-value {
        color: var(--font-primary-color);
      }
    }
  }

  &-persons {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;

    &-person {
      position: relative;
      padding: 0.75rem;
      border: var(--border);
      border-radius: 0.5rem;
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      &-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        color: var(--font-primary-color);
      }

      &-violations {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;

        &-list {
          display: flex;
          flex-wrap: wrap;
          gap: 0.25rem;
        }
      }

      &-equipment,
      &-parts {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        color: var(--font-secondary-color);

        &-list {
          display: flex;
          flex-wrap: wrap;
          gap: 0.25rem;
        }

        &-item {
          padding: 0.125rem 0.5rem;
          border-radius: 0.25rem;
          background: var(--background-depth-two-color);
          color: var(--font-primary-color);
        }
      }
    }
  }
}
</style>
