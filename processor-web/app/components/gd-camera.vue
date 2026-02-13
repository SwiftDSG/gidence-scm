<template>
  <div class="gd-camera" :class="online ? '--online' : '--offline'">
    <div class="gd-camera-header">
      <div
        class="gd-camera-header-status"
        :class="online ? '--online' : '--offline'"
      ></div>
      <div class="gd-camera-header-information">
        <span class="gd-camera-header-information-name gd-headline-5">{{
          camera.name
        }}</span>
        <span class="gd-camera-header-information-address gd-body-5">{{
          formatAddress(camera.address)
        }}</span>
      </div>
      <div class="gd-camera-header-action">
        <gd-button type="tertiary" :icon="'dots'" @click="emits('click')" />
      </div>
    </div>
    <div class="gd-camera-body">
      <div class="gd-camera-body-evidence">
        <gd-camera-evidence v-if="evidence" :evidence="evidence" />
      </div>
      <div class="gd-camera-body-metrics">
        <div class="gd-camera-body-metrics-item">
          <span class="gd-camera-body-metrics-item-placeholder gd-body-5"
            >FPS</span
          >
          <span class="gd-camera-body-metrics-item-value gd-headline-5">{{
            (reading?.[2] || 0).toFixed(2)
          }}</span>
        </div>
        <div class="gd-camera-body-metrics-item">
          <span class="gd-camera-body-metrics-item-placeholder gd-body-5"
            >Last update</span
          >
          <span class="gd-camera-body-metrics-item-value gd-headline-5">{{
            formatTime(reading?.[1] || 0)
          }}</span>
        </div>
        <div class="gd-camera-body-metrics-item">
          <span class="gd-camera-body-metrics-item-placeholder gd-body-5"
            >Persons</span
          >
          <span class="gd-camera-body-metrics-item-value gd-headline-5">{{
            evidence?.person.length || 0
          }}</span>
        </div>
        <div class="gd-camera-body-metrics-item">
          <span class="gd-camera-body-metrics-item-placeholder gd-body-5"
            >Violator</span
          >
          <span class="gd-camera-body-metrics-item-value gd-headline-5">{{
            evidence?.person.filter((a) => a.violation.length > 0).length || 0
          }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
  import type { Camera } from "~/types/camera";
  import type { Reading } from "~/types/general";

  const props = defineProps<{
    camera: Camera;
    reading?: Reading["camera"][0];
  }>();
  const emits = defineEmits<{
    (event: "click"): void;
  }>();
  const { reading: r } = useReader();
  const {
    public: { processor: api },
  } = useRuntimeConfig();

  const reading = computed(() => {
    if (!r.value) return undefined;
    return r.value.camera[props.camera.id];
  });
  const online = computed(
    () =>
      reading.value?.[1] && new Date().getTime() < reading.value?.[1] + 30000,
  );
  const evidence = computed(() => reading.value?.[0]);

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

  watch(
    () => evidence.value?.id,
    (val) => {
      console.log(val);
    },
  );
</script>

<style lang="scss" scoped>
  .gd-camera {
    position: relative;
    width: 100%;
    height: 100%;
    background: var(--background-depth-one-color);
    padding: 0.75rem;
    border: var(--border);
    border-radius: 0.75rem;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;

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
      gap: 0.5rem;

      &-status {
        position: relative;
        width: 2rem;
        height: 2rem;
        border-radius: 0.5rem;
        background: var(--background-depth-two-color);
        display: flex;
        justify-content: center;
        align-items: center;
        flex-shrink: 0;
        &::before {
          content: "";
          position: absolute;
          width: 1rem;
          height: 1rem;
          border-radius: 0.5rem;
          opacity: 0.2;
        }
        &::after {
          content: "";
          position: absolute;
          width: 0.5rem;
          height: 0.5rem;
          border-radius: 0.25rem;
        }
        &.--online {
          &::before {
            background: var(--success-color);
          }
          &::after {
            background: var(--success-color);
          }
        }
        &.--offline {
          &::before {
            background: var(--error-color);
          }
          &::after {
            background: var(--error-color);
          }
        }
      }

      &-information {
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

      &-action {
        position: absolute;
        top: 0;
        right: 0;
      }
    }

    &-body {
      position: relative;
      flex-grow: 1;
      display: flex;
      gap: 0.75rem;

      &-evidence {
        position: relative;
        width: calc(100% - 6.75rem);
        height: 100%;
        background-color: var(--background-depth-two-color);
        border-radius: 0.5rem;
        display: flex;
        overflow: hidden;
      }
      &-metrics {
        position: relative;
        width: 6rem;
        height: 100%;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        &-item {
          position: relative;
          width: 100%;
          height: 100%;
          background-color: var(--background-depth-two-color);
          border-radius: 0.5rem;
          padding: 0.5rem;
          box-sizing: border-box;
          display: flex;
          flex-direction: column;
          justify-content: space-between;
          &-placeholder {
            color: var(--font-secondary-color);
          }
        }
      }
    }
  }
</style>
