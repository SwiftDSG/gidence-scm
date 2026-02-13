<template>
  <gd-menu :active="active" :label="camera.name" class="gd-menu">
    <div class="gd-menu-body">
      <div
        class="gd-menu-body-item"
        @click="
          openMenu({
            cameraInformation: {
              camera_id: camera.id,
            },
          })
        "
      >
        <div class="gd-menu-body-item-icon">
          <gd-svg name="information" />
        </div>
        <div class="gd-menu-body-item-information">
          <span class="gd-menu-body-item-information-value gd-headline-5"
            >General information</span
          >
          <span class="gd-menu-body-item-information-placeholder gd-body-5"
            >See or change this camera's information</span
          >
        </div>
      </div>
      <div
        class="gd-menu-body-item"
        @click="
          openMenu({
            evidence: {
              camera_id: camera.id,
            },
          })
        "
      >
        <div class="gd-menu-body-item-icon">
          <gd-svg name="worker" />
        </div>
        <div class="gd-menu-body-item-information">
          <span class="gd-menu-body-item-information-value gd-headline-5"
            >Evidence list</span
          >
          <span class="gd-menu-body-item-information-placeholder gd-body-5"
            >See evidences captured by this camera</span
          >
        </div>
      </div>
      <div
        class="gd-menu-body-item --error"
        @click="
          openMenu({
            cameraDelete: {
              camera_id: camera.id,
            },
          })
        "
      >
        <div class="gd-menu-body-item-icon">
          <gd-svg name="delete" color="error" />
        </div>
        <div class="gd-menu-body-item-information">
          <span class="gd-menu-body-item-information-value gd-headline-5"
            >Delete camera</span
          >
          <span class="gd-menu-body-item-information-placeholder gd-body-5"
            >Remove this camera from the application</span
          >
        </div>
      </div>
    </div>
  </gd-menu>
</template>

<script lang="ts" setup>
  import type { Camera } from "~/types/camera";

  const emits = defineEmits(["shake"]);
  const props = defineProps<{
    active: boolean;
    camera_id: string;
  }>();

  const { openMenu } = useMain();
  const { cameras } = useCamera();

  const camera = computed<Camera>(
    () => cameras.value.find((p) => p.id === props.camera_id) as Camera,
  );
</script>

<style lang="scss" scoped>
  .gd-menu {
    &-body {
      position: relative;
      width: 100%;
      display: flex;
      flex-direction: column;
      gap: 1rem;

      &-item {
        cursor: pointer;
        position: relative;
        width: 100%;
        padding: 0.75rem;
        border-radius: 0.75rem;
        border: var(--border);
        background: var(--background-depth-one-color);
        box-sizing: border-box;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        overflow: hidden;

        * {
          pointer-events: none;
        }

        &-icon {
          position: relative;
          width: 2rem;
          height: 2rem;
          padding: 0 0.5rem;
          border-radius: 0.5rem;
          box-sizing: border-box;
          display: flex;
          justify-content: center;
          align-items: center;
          overflow: hidden;
          &::before {
            content: "";
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background: var(--background-depth-two-color);
            opacity: 1;
          }
        }

        &-information {
          position: relative;
          display: flex;
          flex-direction: column;
          &-placeholder {
            position: relative;
            color: var(--font-secondary-color);
          }
        }

        &::after {
          content: "";
          position: absolute;
          top: 0;
          left: 0;
          width: 100%;
          height: 100%;
          background: var(--font-primary-color);
          opacity: 0;
          transition: 0.25s opacity;
        }

        &:hover::after {
          opacity: 0.025;
        }
      }
    }
  }
</style>
