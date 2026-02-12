<template>
  <div class="gd-menu" :data-view="view">
    <div class="gd-menu-panel" ref="gdMenuPanel">
      <div
        v-if="label"
        class="gd-menu-panel-header"
        :class="headerActive ? '--active' : ''"
      >
        <gd-button
          class="gd-menu-panel-header-button"
          type="tertiary"
          :icon="'arrow-left'"
          @click="closeMenu"
        />
        <h2 v-if="label" class="gd-menu-panel-header-title gd-headline-5">
          {{ label }}
        </h2>
        <gd-button
          size="small"
          v-if="action"
          class="gd-menu-panel-header-button"
          :icon="action.icon"
          :disabled="action.disabled"
          :type="
            action.type === 'default' ? 'secondary' : action.type || 'primary'
          "
          @click="emits('clicked')"
        />
      </div>
      <div
        class="gd-menu-panel-body"
        @scroll="scrollHandler"
        :class="!label ? '--full' : ''"
      >
        <h2 v-if="label" class="gd-menu-panel-body-title gd-headline-3">
          {{ label }}
        </h2>
        <div class="gd-menu-panel-body-content">
          <slot />
        </div>
      </div>
      <gd-loader
        :state="loading ? 'show' : 'hide'"
        class="gd-menu-panel-loading"
        type="overlay"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
  import type { View } from "~/types/general";

  import gsap from "gsap";

  const { view, rem, menus, closeMenu } = useMain();
  const props = defineProps<{
    active: boolean;
    label?: string;
    action?: {
      icon: string;
      type?: "primary" | "secondary" | "default";
      disabled?: boolean;
    };
    loading?: boolean;
  }>();
  const emits = defineEmits(["exit", "clicked"]);

  const gdMenuPanel = ref<HTMLDivElement | null>(null);

  const scrollValue = ref<number>(0);
  const scrollThreshold = ref<number>(0);

  const headerActive: ComputedRef<boolean> = computed(
    (): boolean => scrollValue.value >= scrollThreshold.value,
  );

  const animate = {
    init(gdMenuPanel: HTMLElement) {
      gsap.to(gdMenuPanel, {
        x: 0,
        y: 0,
        duration: 0.5,
        ease: "power4.out",
      });
    },
    exit(view: View, gdMenuPanel: HTMLElement, cb?: () => void) {
      const vars: gsap.TweenVars = {
        y: "125%",
        duration: 0.5,
        ease: "power2.inOut",
        onComplete() {
          if (cb) cb();
        },
      };
      if (view === "large") {
        delete vars.y;
        vars.x = "125%";
      }

      gsap.to(gdMenuPanel, vars);
    },
  };

  function scrollHandler(e: Event): UIEvent {
    if (e.target instanceof HTMLElement) {
      scrollValue.value = e.target.scrollTop;
    }
    return e as UIEvent;
  }

  function exit(): void {
    if (gdMenuPanel.value) {
      animate.exit(view.value, gdMenuPanel.value);
    }
  }

  watch(
    () => menus.value.length,
    (val, oldVal) => {
      if (val < oldVal && gdMenuPanel.value && props.active) {
        exit();
      }
    },
  );

  onMounted(() => {
    scrollThreshold.value = 2 * rem.value;
    if (gdMenuPanel.value) {
      animate.init(gdMenuPanel.value);
    }
  });
</script>

<style lang="scss" scoped>
  .gd-menu {
    position: fixed;
    top: 0;
    right: 0;
    width: 100vw;
    height: 100vh;
    height: calc(var(--vh) * 100);
    display: flex;
    flex-shrink: 0;

    &-panel {
      position: absolute;
      top: 0;
      left: 0;
      width: 100vw;
      height: 100vh;
      height: calc(var(--vh) * 100);
      background: var(--background-depth-two-color);
      overflow-y: hidden;
      overflow-x: visible;
      display: flex;
      flex-direction: column;
      transform: translateY(125%);

      &-header {
        z-index: 2;
        position: relative;
        width: 100%;
        height: 4rem;
        background: var(--background-depth-two-color);
        padding: 1rem;
        box-sizing: border-box;
        box-shadow: 0 0.5rem 0.5rem 0.5rem var(--background-depth-two-color);
        display: flex;
        flex-shrink: 0;
        justify-content: center;
        align-items: center;

        &-button {
          position: absolute;
          top: 1rem;
          left: 1rem;
          &:last-child {
            left: auto;
            right: 1rem;
          }
        }

        &-title {
          position: relative;
          opacity: 0;
          transform: scale(0.875);
          transition:
            0.25s opacity,
            0.25s transform;
        }

        &.--active {
          box-shadow: none;
          .gd-menu-panel-header-title {
            opacity: 1;
            transform: scale(1);
          }
          &::after {
            opacity: 1;
          }
        }

        &::after {
          content: "";
          position: absolute;
          top: 100%;
          left: 0;
          width: 100%;
          height: 1px;
          background: var(--border-color);
          opacity: 0;
          transition: 0.25s opacity;
        }
      }

      &-body {
        z-index: 1;
        position: relative;
        width: 100%;
        height: calc(100% - 4rem);
        padding: 1rem 1rem 0 1rem;
        box-sizing: border-box;
        overflow-y: auto;
        overflow-x: hidden;
        display: flex;
        flex-shrink: 0;
        flex-direction: column;

        &-title {
          position: relative;
          width: 100%;
          height: 1rem;
          margin-bottom: 1rem;
          display: flex;
          flex-shrink: 0;
          align-items: center;
        }

        &-content {
          position: relative;
          width: 100%;
          flex-shrink: 0;
          display: flex;
          flex-direction: column;
        }

        &.--full {
          height: 100%;
        }
      }

      &-loading {
        pointer-events: none;
        z-index: 999999;
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        border-radius: 1rem;
        opacity: 0;
        display: flex;
        justify-content: center;
        align-items: center;
        transition: 0.25s opacity;
        z-index: 2;
      }
    }

    &[data-view="large"] {
      width: 20rem;
      .gd-menu-panel {
        top: 0;
        right: 0;
        width: 100%;
        border-left: var(--border);
        box-sizing: border-box;
        transform: translateX(125%);
      }
    }
  }
</style>
