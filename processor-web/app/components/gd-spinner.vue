<template>
  <div ref="gdSpinner" class="gd-spinner">
    <div class="gd-spinner-circle">
      <div class="gd-spinner-circle-inner"></div>
    </div>
  </div>
</template>

<script lang="ts" setup>
  import gsap from "gsap";

  const props = defineProps<{
    state: "show" | "hide";
  }>();
  const gdSpinner = ref<HTMLDivElement>();

  const animate = {
    init(gdSpinner: HTMLElement): GSAPTimeline {
      const tl = gsap.timeline();
      tl.to(gdSpinner, {
        pointerEvents: "auto",
        opacity: 0.5,
        scale: 1,
        duration: 0.5,
        ease: "power2.inOut",
      });
      return tl;
    },
    exit(gdSpinner: HTMLElement): void {
      const tl = gsap.timeline();
      tl.to(gdSpinner, {
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
      if (!gdSpinner.value) return;
      if (val === "hide") animate.exit(gdSpinner.value);
      else animate.init(gdSpinner.value);
    }
  );

  onMounted(() => {
    setTimeout(() => {
      if (props.state === "show" && gdSpinner.value)
        animate.init(gdSpinner.value);
    }, 250);
  });
</script>

<style lang="scss" scoped>
  .gd-spinner {
    z-index: 100000;
    pointer-events: none;
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: var(--font-tertiary-color);
    border-radius: inherit;
    background: inherit;
    opacity: 0;
    display: flex;
    justify-content: center !important;
    align-items: center !important;
    transform: scale(1);
    backdrop-filter: blur(4px);

    &-circle {
      width: 1rem;
      height: 1rem;
      border-radius: 50%;
      display: flex;
      justify-content: center;
      align-items: center;
      &-inner {
        width: 1rem;
        height: 1rem;
        border: 0.125rem solid transparent;
        border-top-color: var(--background-depth-three-color);
        border-right-color: var(--background-depth-three-color);
        border-radius: 50%;
        box-sizing: border-box;
        animation: gd-spinner-rotate 1s linear infinite;
      }
    }
  }

  @keyframes gd-spinner-rotate {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }
</style>
