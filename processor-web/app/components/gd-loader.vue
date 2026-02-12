<template>
  <div
    ref="gdLoader"
    class="gd-loader"
    :class="{
      '--overlay': type === 'overlay',
      '--immediate': immediate,
      '--fixed': fixed && type === 'overlay',
    }"
  >
    <div class="gd-loader-bar">
      <div class="gd-loader-bar-inner"></div>
    </div>
  </div>
</template>

<script lang="ts" setup>
  import gsap from "gsap";

  const props = defineProps<{
    state: "show" | "hide";
    type?: "default" | "overlay";
    immediate?: boolean;
    fixed?: boolean;
  }>();
  const gdLoader = ref<HTMLDivElement>();

  const animate = {
    init(gdLoader: HTMLElement): GSAPTimeline {
      const tl = gsap.timeline();
      tl.to(gdLoader, {
        pointerEvents: "auto",
        opacity: 1,
        scale: 1,
        duration: 0.5,
        ease: "power2.inOut",
      });
      return tl;
    },
    exit(gdLoader: HTMLElement): void {
      const tl = gsap.timeline();
      tl.to(gdLoader, {
        pointerEvents: "none",
        opacity: 0,
        scale: 1.125,
        duration: 0.5,
        ease: "power2.inOut",
      });
    },
  };

  watch(
    () => props.state,
    (val) => {
      if (!gdLoader.value) return;
      if (val === "hide") animate.exit(gdLoader.value);
      else animate.init(gdLoader.value);
    }
  );

  onMounted(() => {
    setTimeout(() => {
      if (props.state === "show" && gdLoader.value && !props.immediate)
        animate.init(gdLoader.value);
    }, 250);
  });
</script>

<style lang="scss" scoped>
  .gd-loader {
    pointer-events: none;
    position: relative;
    width: 10rem;
    height: 2rem;
    padding: 0.75rem;
    border-radius: 0.75rem;
    box-sizing: border-box;
    background: var(--background-depth-one-color);
    box-shadow: var(--box-shadow);
    opacity: 0;
    display: flex;
    justify-content: center !important;
    align-items: center !important;
    transform: scale(0.875);
    overflow: hidden;
    &-bar {
      position: relative;
      width: 100%;
      height: 0.25rem;
      border-radius: 0.125rem;
      background: var(--background-depth-three-color);
      overflow: hidden;
      display: flex;
      &-inner {
        position: absolute;
        top: 0;
        left: 0;
        right: 100%;
        height: 100%;
        background: var(--primary-color);
        border-radius: 0.125rem;
        animation: gd-loading 2s ease infinite;
      }
    }
    &.--overlay {
      z-index: 100000;
      position: absolute;
      top: 0;
      right: 0;
      bottom: 0;
      left: 0;
      width: 100%;
      height: 100%;
      padding: 0 2rem;
      border-radius: 0;
      background: var(--background-depth-one-color);
      opacity: 0;
      transform: scale(1) !important;
      .gd-loader-bar {
        max-width: 15rem;
      }
      &.--immediate {
        pointer-events: auto;
        opacity: 1;
      }
      &.--fixed {
        position: fixed;
      }
    }
  }
</style>
