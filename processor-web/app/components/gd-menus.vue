<template>
  <div
    class="gd-menus"
    :style="menusCopy.length > 0 ? 'pointer-events: all;' : ''"
  >
    <div v-for="(menu, i) in menusCopy" :key="i" class="gd-menus-item">
      <gd-processor-menu
        v-if="menu['processor']"
        :active="i === menusCopy.length - 1"
        @shake="emits('shake')"
      />
      <gd-processor-information-menu
        v-else-if="menu['processorInformation']"
        :active="i === menusCopy.length - 1"
        @shake="emits('shake')"
      />
      <gd-camera-menu
        v-if="menu['camera']"
        :camera_id="menu['camera'].camera_id"
        :active="i === menusCopy.length - 1"
        @shake="emits('shake')"
      />
      <gd-camera-delete-menu
        v-if="menu['cameraDelete']"
        :camera_id="menu['cameraDelete'].camera_id"
        :active="i === menusCopy.length - 1"
        @shake="emits('shake')"
      />
      <gd-camera-information-menu
        v-else-if="menu['cameraInformation']"
        :camera_id="menu['cameraInformation'].camera_id"
        :active="i === menusCopy.length - 1"
        @shake="emits('shake')"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
  import type { Menu } from "~/types/general";

  const emits = defineEmits(["shake"]);

  const { menus } = useMain();

  const menusCopy = ref<Menu[]>([]);

  watch(
    () => menus.value.length,
    (val, oldVal) => {
      setTimeout(
        () => {
          menusCopy.value = JSON.parse(JSON.stringify(menus.value));
        },
        val < (oldVal || 0) ? 500 : 0,
      );
    },
    { deep: true, immediate: true },
  );
</script>

<style lang="scss" scoped>
  .gd-menus {
    pointer-events: none;
    position: fixed;
    top: 0;
    right: 0;
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-shrink: 0;
    &-item {
      position: absolute;
      top: 0;
      right: 0;
    }

    @media only screen and (min-width: 1281px) {
      width: 20rem;
      height: 100vh;
      background-color: var(--background-depth-two-color);

      .gd-menus-menu {
        border-left: var(--border);
        box-sizing: border-box;
      }
    }
  }
</style>
