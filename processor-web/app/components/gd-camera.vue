<template>
  <div
    class="gd-camera"
    :class="{ '--active': hasViolation, '--offline': !isOnline }"
    @click="emits('click')"
  >
    <div class="gd-camera-header">
      <div class="gd-camera-header-icon">
        <gd-svg name="camera" :color="hasViolation ? 'error' : 'primary'" />
      </div>
      <div class="gd-camera-header-info">
        <span class="gd-camera-header-info-name gd-headline-4">{{
          camera.name
        }}</span>
        <span class="gd-camera-header-info-address gd-body-5">{{
          formatAddress(camera.address)
        }}</span>
      </div>
    </div>
    <div class="gd-camera-status">
      <div
        class="gd-camera-status-indicator"
        :class="isOnline ? '--online' : '--offline'"
      ></div>
      <span class="gd-camera-status-text gd-body-5">
        {{ isOnline ? "Online" : "Offline" }}
      </span>
    </div>
    <div v-if="hasViolation && latestEvidence" class="gd-camera-violation">
      <span class="gd-camera-violation-count gd-headline-6">
        {{ latestEvidence.person.reduce((sum, p) => sum + p.violation.length, 0) }} violations
      </span>
      <span class="gd-camera-violation-time gd-body-5">
        {{ formatTime(latestEvidence.timestamp) }}
      </span>
    </div>
  </div>
</template>

<script lang="ts" setup>
import type { Camera } from "~/types/camera";
import type { Evidence } from "~/types/evidence";

const props = defineProps<{
  camera: Camera;
  evidence?: Evidence | null;
  online?: boolean;
}>();

const emits = defineEmits<{
  (event: "click"): void;
}>();

const isOnline = computed(() => props.online ?? false);
const latestEvidence = computed(() => props.evidence);
const hasViolation = computed(() => {
  if (!latestEvidence.value) return false;
  return latestEvidence.value.person.some((p) => p.violation.length > 0);
});

const formatAddress = (address: Camera["address"]): string => {
  const host = address.host.join(".");
  const port = address.port;
  const path = address.path || "";
  return `${host}:${port}${path}`;
};

const formatTime = (timestamp: number): string => {
  const date = new Date(timestamp);
  return date.toLocaleTimeString();
};
</script>

<style lang="scss" scoped>
.gd-camera {
  cursor: pointer;
  position: relative;
  width: 100%;
  padding: 1rem;
  border: var(--border);
  border-radius: 0.75rem;
  background: var(--background-depth-one-color);
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  transition: transform 0.2s ease-in-out, box-shadow 0.2s ease-in-out;

  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  &.--active {
    border-color: var(--error-color);
    background: var(--error-background-color, rgba(239, 68, 68, 0.05));
  }

  &.--offline {
    opacity: 0.5;
  }

  &-header {
    position: relative;
    display: flex;
    align-items: center;
    gap: 0.75rem;

    &-icon {
      position: relative;
      width: 2.5rem;
      height: 2.5rem;
      border-radius: 0.5rem;
      background: var(--background-depth-two-color);
      display: flex;
      justify-content: center;
      align-items: center;
      flex-shrink: 0;
    }

    &-info {
      position: relative;
      display: flex;
      flex-direction: column;

      &-name {
        color: var(--font-primary-color);
      }

      &-address {
        color: var(--font-secondary-color);
      }
    }
  }

  &-status {
    position: relative;
    display: flex;
    align-items: center;
    gap: 0.5rem;

    &-indicator {
      position: relative;
      width: 0.5rem;
      height: 0.5rem;
      border-radius: 50%;
      background: var(--error-color);

      &.--online {
        background: var(--success-color);
      }

      &.--offline {
        background: var(--error-color);
      }
    }

    &-text {
      color: var(--font-secondary-color);
    }
  }

  &-violation {
    position: relative;
    padding: 0.5rem;
    border-radius: 0.5rem;
    background: var(--error-color);
    display: flex;
    justify-content: space-between;
    align-items: center;

    &-count {
      color: var(--font-tertiary-color);
    }

    &-time {
      color: var(--font-tertiary-color);
      opacity: 0.8;
    }
  }
}
</style>
