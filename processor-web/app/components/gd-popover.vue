<template>
  <div ref="triggerRef" class="gd-popover">
    <slot name="trigger" />
    <teleport to="body">
      <div
        ref="overlayRef"
        class="gd-popover-overlay"
        :style="overlayStyles"
        @click="handleOverlayClick"
      >
        <div
          ref="contentRef"
          class="gd-popover-overlay-content"
          :style="contentStyles"
          @click.stop
        >
          <slot name="content" />
        </div>
      </div>
    </teleport>
  </div>
</template>

<script lang="ts" setup>
  import { gsap } from "gsap";
  import type { StyleValue } from "vue";

  // Types
  interface PopoverProps {
    visible?: boolean;
    placement?:
      | "auto"
      | "top"
      | "bottom"
      | "left"
      | "right"
      | "top-start"
      | "top-end"
      | "bottom-start"
      | "bottom-end";
    offset?: number;
    closeOnClickOutside?: boolean;
    closeOnEscape?: boolean;
    contentWidth?: string;
    contentMaxHeight?: string;
  }

  interface PopoverEmits {
    "update:visible": [value: boolean];
    close: [];
    open: [];
  }

  interface Position {
    top: number;
    left: number;
  }

  // Props with defaults
  const props = withDefaults(defineProps<PopoverProps>(), {
    visible: false,
    placement: "auto",
    offset: 0,
    closeOnClickOutside: true,
    closeOnEscape: true,
    contentWidth: "10rem",
    contentMaxHeight: "10rem",
  });

  // Emits
  const emit = defineEmits<PopoverEmits>();

  // Composables
  const { rem } = useMain();

  // Refs
  const triggerRef = ref<HTMLElement>();
  const overlayRef = ref<HTMLElement>();
  const contentRef = ref<HTMLElement>();

  // State
  const isAnimating = ref(false);
  const contentPosition = ref<Position>({ top: 0, left: 0 });

  // Computed
  const overlayStyles = computed<StyleValue>(() => ({
    pointerEvents: props.visible ? "auto" : "none",
  }));

  const contentStyles = computed<StyleValue>(() => ({
    top: `${contentPosition.value.top}px`,
    left: `${contentPosition.value.left}px`,
    width: props.contentWidth,
    maxHeight: props.contentMaxHeight,
  }));

  // Methods
  const calculatePosition = (): Position => {
    if (!triggerRef.value || !contentRef.value) {
      return { top: 0, left: 0 };
    }

    const trigger = triggerRef.value.getBoundingClientRect();
    const content = contentRef.value.getBoundingClientRect();
    const viewport = {
      width: window.innerWidth,
      height: window.innerHeight,
    };

    const offsetPx = (props.offset * (rem?.value || 16)) / 16;
    let position: Position = { top: 0, left: 0 };

    // Calculate position based on placement
    switch (props.placement) {
      case "top":
        position = {
          top: trigger.top - content.height - offsetPx,
          left: trigger.left + (trigger.width - content.width) / 2,
        };
        break;
      case "bottom":
        position = {
          top: trigger.bottom + offsetPx,
          left: trigger.left + (trigger.width - content.width) / 2,
        };
        break;
      case "left":
        position = {
          top: trigger.top + (trigger.height - content.height) / 2,
          left: trigger.left - content.width - offsetPx,
        };
        break;
      case "right":
        position = {
          top: trigger.top + (trigger.height - content.height) / 2,
          left: trigger.right + offsetPx,
        };
        break;
      case "top-start":
        position = {
          top: trigger.top - content.height - offsetPx,
          left: trigger.left,
        };
        break;
      case "top-end":
        position = {
          top: trigger.top - content.height - offsetPx,
          left: trigger.right - content.width,
        };
        break;
      case "bottom-start":
        position = {
          top: trigger.bottom + offsetPx,
          left: trigger.left,
        };
        break;
      case "bottom-end":
        position = {
          top: trigger.bottom + offsetPx,
          left: trigger.right - content.width,
        };
        break;
      case "auto":
      default:
        // Auto placement logic
        position = {
          top: trigger.bottom + offsetPx,
          left: trigger.left,
        };

        // Check horizontal overflow
        if (position.left + content.width > viewport.width) {
          position.left = Math.max(
            0,
            trigger.right - content.width - offsetPx / 2
          );
        } else if (position.left < 0) {
          position.left = 0;
        }

        // Check vertical overflow
        if (position.top + content.height > viewport.height) {
          const topPosition = trigger.top - content.height - offsetPx;
          if (topPosition >= 0) {
            position.top = topPosition;
          } else {
            // If no space above, keep below but adjust
            position.top = Math.max(
              0,
              viewport.height - content.height - offsetPx
            );
          }
        }
        break;
    }

    // Ensure content stays within viewport bounds
    position.top = Math.max(
      0,
      Math.min(position.top, viewport.height - content.height)
    );
    position.left = Math.max(
      0,
      Math.min(position.left, viewport.width - content.width)
    );

    return position;
  };

  const updatePosition = async () => {
    await nextTick();
    if (!triggerRef.value || !contentRef.value) return;

    contentPosition.value = calculatePosition();
  };

  const open = async () => {
    if (isAnimating.value) return;

    isAnimating.value = true;
    await updatePosition();

    if (!contentRef.value || !overlayRef.value) return;

    gsap.set(contentRef.value, {
      scale: 0.95,
      opacity: 0,
    });

    gsap.to(contentRef.value, {
      scale: 1,
      opacity: 1,
      duration: 0.35,
      ease: "power2.out",
      onComplete: () => {
        isAnimating.value = false;
        emit("open");
      },
    });
  };

  const close = () => {
    if (isAnimating.value) return;

    isAnimating.value = true;

    if (!contentRef.value) return;

    gsap.to(contentRef.value, {
      scale: 0.95,
      opacity: 0,
      duration: 0.25,
      ease: "power2.in",
      onComplete: () => {
        isAnimating.value = false;
        emit("update:visible", false);
        emit("close");
      },
    });
  };

  const handleOverlayClick = (event: MouseEvent) => {
    if (props.closeOnClickOutside && !isAnimating.value) {
      close();
    }
  };

  const handleEscape = (event: KeyboardEvent) => {
    if (event.key === "Escape" && props.closeOnEscape && props.visible) {
      close();
    }
  };

  const handleResize = () => {
    if (props.visible) {
      updatePosition();
    }
  };

  const handleScroll = (event: Event) => {
    // Close on scroll if the trigger is scrolled out of view
    if (props.visible && triggerRef.value) {
      const triggerRect = triggerRef.value.getBoundingClientRect();
      if (
        triggerRect.bottom < 0 ||
        triggerRect.top > window.innerHeight ||
        triggerRect.right < 0 ||
        triggerRect.left > window.innerWidth
      ) {
        close();
      } else {
        updatePosition();
      }
    }
  };

  // Watchers
  watch(
    () => props.visible,
    async (newValue, oldValue) => {
      if (newValue && !oldValue) {
        await open();
      } else if (!newValue && oldValue) {
        close();
      }
    }
  );

  // Lifecycle
  onMounted(() => {
    if (props.closeOnEscape) {
      window.addEventListener("keydown", handleEscape);
    }
    window.addEventListener("resize", handleResize);
    window.addEventListener("scroll", handleScroll, true);
  });

  onUnmounted(() => {
    window.removeEventListener("keydown", handleEscape);
    window.removeEventListener("resize", handleResize);
    window.removeEventListener("scroll", handleScroll, true);

    // Clean up any running animations
    if (contentRef.value) {
      gsap.killTweensOf(contentRef.value);
    }
  });

  // Expose methods for parent components
  defineExpose({
    open,
    close,
    updatePosition,
  });
</script>

<style lang="scss" scoped>
  .gd-popover {
    position: relative;
    width: 100%;
    display: flex;
  }
</style>

<style lang="scss">
  .gd-popover {
    &-overlay {
      z-index: 10001;
      pointer-events: none;
      position: fixed;
      top: 0;
      left: 0;
      bottom: 0;
      right: 0;

      &-content {
        position: absolute;
        width: 10rem;
        max-height: 10rem;
        padding: 0.25rem;
        border: var(--border);
        border-radius: 0.75rem;
        box-sizing: border-box;
        background-color: var(--background-depth-one-color);
        display: flex;
        flex-direction: column;
        transform: scale(0.95);
        overflow-y: auto;
        opacity: 0;
      }
    }
  }
</style>
