<template>
  <gd-menu
    :active="active"
    :label="`Evidences (${camera.name})`"
    class="gd-menu"
  >
    <div class="gd-menu-body">
      <gd-evidence
        v-for="evidence in evidences"
        :key="evidence.id"
        :evidence="evidence"
        @click="
          openMenu({
            evidenceInformation: {
              camera_id: camera_id,
              evidence_id: evidence.id,
            },
          })
        "
      />
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
  const { evidences, getEvidences } = useEvidence();

  const camera = computed<Camera>(
    () => cameras.value.find((p) => p.id === props.camera_id) as Camera,
  );

  onMounted(async () => {
    await getEvidences(props.camera_id);
  });
</script>

<style lang="scss" scoped>
  .gd-menu {
    &-body {
      position: relative;
      width: 100%;
      padding-bottom: 1rem;
      box-sizing: border-box;
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
