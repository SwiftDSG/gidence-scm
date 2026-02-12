<template>
  <gd-menu :active="active" label="Delete Camera" class="gd-menu">
    <p class="gd-menu-message gd-body-5">
      Warning! deleting this camera will also delete all devices available
      within this camera, also this action cannot be undone
    </p>
    <div class="gd-menu-footer">
      <gd-button
        @click="submit"
        style="width: 100%"
        text="Delete camera"
        type="error"
      />
    </div>
  </gd-menu>
</template>

<script lang="ts" setup>
  import type { Camera } from "~/types/camera";

  const emits = defineEmits(["exit", "shake"]);
  const props = defineProps<{
    active: boolean;
    camera_id: string;
  }>();
  const { closeMenu } = useMain();
  const { cameras, deleteCamera } = useCamera();

  const submitLoading = ref<boolean>(false);

  const camera = computed<Camera | null>(
    () => cameras.value.find((p) => p.id === props.camera_id) || null,
  );

  async function submit(): Promise<void> {
    if (camera.value === null) return;

    submitLoading.value = true;

    const result = await deleteCamera(props.camera_id);
    setTimeout(() => {
      submitLoading.value = false;
      if (result) {
        closeMenu();
      } else {
        emits("shake");
      }
    }, 1000);
  }
</script>

<style lang="scss" scoped>
  .gd-menu {
    &-message {
      position: relative;
      width: 100%;
      color: var(--font-secondary-color);
    }

    &-footer {
      position: fixed;
      left: 0;
      bottom: 0;
      width: 100%;
      height: 4rem;
      padding: 1rem;
      border-top: var(--border);
      box-sizing: border-box;
      background: var(--background-depth-two-color);
    }
  }
</style>
