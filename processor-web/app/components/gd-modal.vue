<template>
  <teleport to="body">
    <div class="gd-modal" ref="gdModal" :data-view="view">
      <div
        class="gd-modal-overlay"
        ref="gdModalOverlay"
        @click="emits('close')"
      ></div>
      <div class="gd-modal-panel" ref="gdModalPanel">
        <div
          v-if="view === 'small'"
          class="gd-modal-panel-dragger"
          ref="gdModalPanelDragger"
        ></div>
        <div class="gd-modal-panel-header">
          <span class="gd-modal-header-name gd-headline-5">{{
            props.name
          }}</span>
          <gd-button
            v-if="view !== 'small'"
            type="secondary"
            icon="close"
            @click="emits('close')"
            class="gd-modal-close-button"
          />
        </div>
        <div class="gd-modal-panel-content" ref="gdModalPanelContent">
          <slot />
        </div>
      </div>
    </div>
  </teleport>
</template>

<script lang="ts" setup>
  import gsap from "gsap";

  type Drag = {
    y: number; // Previous Y touch position
    cy: number; // Current Y touch position
    ay: number; // Actual Y position
    sy: number; // Scroll Y position
    vy: number; // Y velocity
    dy: number; // Y deceleration
    t: number;
    ct: number; // Countdown timer for deceleration
    threshold: number;
    limit: number;
    state:
      | "dragging"
      | "scrolling"
      | "releasing-drag"
      | "releasing-scroll"
      | "idle";
    target: "open" | "close" | null;
    gdModalOverlay: HTMLDivElement | null;
    gdModalPanel: HTMLDivElement | null;
    gdModalPanelContent: HTMLDivElement | null;
  };

  const props = defineProps<{
    name: string;
    visible: boolean;
  }>();
  const emits = defineEmits<{
    (e: "close"): void;
  }>();

  const { rem, view } = useMain();

  const drag: Drag = {
    y: 0,
    cy: 0,
    ay: 0,
    sy: 0,
    vy: 0,
    dy: 0,
    t: 0,
    ct: 0,
    threshold: 0,
    limit: 0,
    state: "idle",
    target: null,
    gdModalOverlay: null,
    gdModalPanel: null,
    gdModalPanelContent: null,
  };

  const gdModal = ref<HTMLDivElement>();
  const gdModalOverlay = ref<HTMLDivElement>();
  const gdModalPanel = ref<HTMLDivElement>();
  const gdModalPanelDragger = ref<HTMLDivElement>();
  const gdModalPanelContent = ref<HTMLDivElement>();

  const modalTl = ref<gsap.core.Timeline>(gsap.timeline({ paused: true }));
  const modalHeight = ref(0);

  function dragStart(e: TouchEvent): void {
    if (e.touches[0] && gdModalPanel.value && gdModalPanelContent.value) {
      if (!drag.gdModalPanel || !drag.gdModalOverlay) {
        drag.gdModalPanel = gdModalPanel.value;
        drag.gdModalOverlay = gdModalPanel.value
          .previousElementSibling as HTMLDivElement;
      }
      drag.state = "dragging";
      drag.target = null;
      drag.ct = 0;
      drag.t = 0;
      drag.cy = e.touches[0].clientY;
      drag.sy =
        gdModalPanelContent.value.getBoundingClientRect().top -
        gdModalPanel.value.getBoundingClientRect().top;
      drag.ay = gdModalPanel.value.getBoundingClientRect().top;
      drag.threshold = window.innerHeight - modalHeight.value;
    }
  }
  function dragMove(e: TouchEvent): void {
    if (e.touches[0]) {
      drag.cy = e.touches[0].clientY;

      if (!drag.y) drag.y = drag.cy;
      let dy = drag.cy - drag.y;

      drag.vy = drag.cy - drag.y;

      if (drag.sy < 0 && drag.limit) {
        drag.state = "scrolling";
        if (drag.sy <= drag.limit) {
          dy *= ((drag.sy + dy) / drag.limit) * 0.125;
        } else if (drag.sy + dy >= 0) {
          drag.sy = 0;
          drag.state = "dragging";
        }
      } else {
        drag.state = "dragging";
        if (drag.ay + dy <= drag.threshold && drag.vy && !drag.limit) {
          dy *= ((drag.ay + dy) / drag.threshold) * 0.125;
        } else if (drag.ay + dy <= drag.threshold && drag.vy <= 0) {
          drag.state = "scrolling";
        }
      }

      if (
        drag.state === "dragging" &&
        drag.gdModalPanel &&
        drag.gdModalOverlay
      ) {
        drag.ay += dy;
        drag.gdModalPanel.style.transform = `translate3d(0, ${drag.ay}px, 0)`;
        drag.gdModalOverlay.style.opacity = `${
          1 - (drag.ay - drag.threshold) / (window.innerHeight - drag.threshold)
        }`;
      } else if (drag.state === "scrolling" && drag.gdModalPanelContent) {
        drag.sy += dy;
        drag.gdModalPanelContent.style.transform = `translate3d(0, ${drag.sy}px, 0)`;
      }

      drag.y = drag.cy;
    }
  }
  function dragEnd(): void {
    drag.cy = 0;
    drag.y = 0;

    if (drag.state === "dragging") {
      const dy = drag.threshold - drag.ay;

      if (dy !== 0) {
        drag.state = "releasing-drag";

        if (drag.ay <= drag.threshold) {
          // Overscrolled
          const dy = drag.threshold - drag.ay;
          drag.vy = dy / 7.5;
          drag.dy = drag.vy / -15;
          drag.t = 15;
          drag.ct = 15;
          drag.target = "open";
        } else if (drag.vy >= 0) {
          // Going down
          if (drag.vy > 30) {
            // If fast
            drag.target = "close";
          } else if (
            // If minimum speed requirement met but not the minimum speed threshold
            (drag.vy <= 30 && drag.vy >= 10) ||
            drag.ay >= window.innerHeight - drag.threshold * 0.5
          ) {
            drag.vy = 30;
            drag.target = "close";
          } else {
            // Going down too slow and actual position near opening point
            drag.vy = dy / 15;
            drag.dy = drag.vy / -30;
            drag.target = "open";
            drag.t = 30;
            drag.ct = 30;
          }
        } else if (calculateDistance(drag.vy) > dy) {
          // Going up too slow
          drag.vy = dy / 15;
          drag.dy = drag.vy / -30;
          drag.t = 30;
          drag.ct = 30;
          drag.target = "open";
        } else {
          // Going up fast
          drag.target = "open";
        }
      } else {
        drag.state = "idle";
      }
    } else {
      drag.state = "releasing-scroll";

      if (drag.sy <= drag.limit) {
        const dy = drag.limit - drag.sy;
        drag.vy = dy / 7.5;
        drag.dy = drag.vy / -15;
        drag.t = 15;
        drag.ct = 15;
      }
    }

    requestAnimationFrame(dragHandler);
  }
  function dragHandler(): void {
    if (drag.state === "releasing-drag") {
      if (drag.target === "open") {
        if (Math.abs(drag.vy) <= 0.01 && drag.ay <= drag.threshold) {
          const dy = drag.threshold - drag.ay;
          drag.vy = dy / 7.5;
          drag.dy = drag.vy / -15;
          drag.t = 15;
          drag.ct = 15;
        }

        if (drag.vy <= 0 && drag.ay <= drag.threshold) {
          // if going up and passed the threshold
          drag.vy *= 0.25;
          drag.ay += drag.vy;
        } else if (drag.vy <= 0 && drag.t) {
          // vy is negative, indicating that it's going up and t exists indicating stopping at opening point
          const vy = drag.dy * drag.t - drag.dy * drag.ct + drag.vy;
          const dy = vy + drag.dy / 2;
          drag.ay += dy;
          drag.ct -= 1;
        } else if (drag.vy <= 0) {
          // vy is negative, indicating that it's going up
          drag.vy += 1;
          drag.ay += drag.vy;
        } else if (drag.ay <= drag.threshold && drag.t) {
          // Going down from overscroll
          const vy = drag.dy * drag.t - drag.dy * drag.ct + drag.vy;
          const dy = vy + drag.dy / 2;
          drag.ay += dy;
          drag.ct -= 1;
        }
      } else {
        drag.ay += drag.vy;
      }

      if (drag.gdModalPanel && drag.gdModalOverlay) {
        drag.gdModalPanel.style.transform = `translate3d(0, ${drag.ay}px, 0)`;
        drag.gdModalOverlay.style.opacity = `${
          1 - (drag.ay - drag.threshold) / (window.innerHeight - drag.threshold)
        }`;
      }

      if (drag.ay >= window.innerHeight || (drag.t && !drag.ct)) {
        drag.t = 0;
        drag.ct = 0;
        drag.state = "idle";
        if (drag.gdModalPanel && drag.target === "close") {
          drag.gdModalPanel.removeAttribute("style");
          emits("close");
        }
        return;
      }

      requestAnimationFrame(dragHandler);
    } else if (drag.state === "releasing-scroll") {
      if (Math.abs(drag.vy) <= 0.01) {
        if (drag.sy <= drag.limit || drag.sy >= 0) {
          const dy = drag.sy <= drag.limit ? drag.limit - drag.sy : 0 - drag.sy;
          drag.vy = dy / 7.5;
          drag.dy = drag.vy / -15;
          drag.t = 15;
          drag.ct = 15;
        } else {
          drag.t = 0;
          drag.ct = 0;
          drag.vy = 0;
          return;
        }
      }

      if (drag.t) {
        const vy = drag.dy * drag.t - drag.dy * drag.ct + drag.vy;
        const dy = vy + drag.dy / 2;
        drag.sy += dy;
        drag.ct -= 1;
      } else {
        drag.vy *= drag.sy <= drag.limit || drag.sy >= 0 ? 0.25 : 0.975;
        drag.sy += drag.vy;
      }

      if (drag.gdModalPanelContent) {
        drag.gdModalPanelContent.style.transform = `translate3d(0, ${drag.sy}px, 0)`;
      }

      if (drag.t && !drag.ct) {
        drag.t = 0;
        drag.ct = 0;
        drag.vy = 0;
        return;
      }

      requestAnimationFrame(dragHandler);
    }
  }
  function calculateDistance(
    vy: number,
    a: number = 1,
    t: number = 30
  ): number {
    return vy * t + 0.5 * a * t * t;
  }

  watch(
    () => props.visible,
    (val) => {
      if (view.value !== "small") {
        if (!gdModal.value || !gdModalOverlay.value || !gdModalPanel.value)
          return;
        if (val) {
          gdModal.value.style.pointerEvents = "auto";
          modalTl.value.play();
        } else {
          modalTl.value.reverse();
        }
      } else if (val) {
        if (!gdModalPanel.value || !gdModalPanelContent.value) return;

        if (!drag.gdModalPanel || !drag.gdModalOverlay) {
          drag.gdModalPanel = gdModalPanel.value;
          drag.gdModalOverlay = gdModalPanel.value
            .previousElementSibling as HTMLDivElement;
        }

        const rem = parseInt(getComputedStyle?.(document.body)?.fontSize) || 24;

        const { top, height } =
          gdModalPanelContent.value.getBoundingClientRect();
        modalHeight.value = height;

        drag.limit =
          gdModalPanelContent.value.scrollHeight >
          gdModalPanel.value.getBoundingClientRect().height
            ? height - rem - gdModalPanelContent.value.scrollHeight
            : 0;

        if (drag.limit) {
          drag.gdModalPanelContent = gdModalPanelContent.value;
          drag.gdModalPanelContent.style.transform = `translate3d(0, 0, 0)`;
          drag.sy = 0;
        } else {
          drag.gdModalPanelContent = null;
        }

        if (gdModalPanel.value.parentElement)
          gdModalPanel.value.parentElement.style.zIndex = "10000";
        gdModalPanel.value.style.pointerEvents = "all";
        gdModalPanel.value.style.opacity = "1";

        drag.threshold = window.innerHeight - modalHeight.value;
        drag.ay = top;
        drag.vy = (drag.threshold - drag.ay) / 10;
        drag.dy = drag.vy / -20;
        drag.t = 20;
        drag.ct = 20;
        drag.target = "open";
        drag.state = "releasing-drag";

        requestAnimationFrame(dragHandler);
      }
    }
  );

  onMounted(() => {
    if (!gdModal.value || !gdModalOverlay.value || !gdModalPanel.value) return;
    const tl = gsap.timeline({
      paused: true,
      defaults: { duration: 0.3, ease: "power2.inOut" },
      onComplete() {
        if (gdModal.value) {
          gdModal.value.style.pointerEvents = "auto";
        }
      },
      onReverseComplete() {
        if (gdModal.value) {
          gdModal.value.style.pointerEvents = "none";
        }
      },
    });

    tl.to(gdModalOverlay.value, {
      opacity: 1,
      duration: 0.35,
      ease: "power2.inOut",
    });

    if (view.value !== "small") {
      tl.to(
        gdModalPanel.value,
        {
          opacity: 1,
          scale: 1,
          duration: 0.35,
          ease: "power2.inOut",
        },
        "<0"
      );
    } else {
      modalHeight.value =
        gdModalPanelContent.value?.getBoundingClientRect().height || 0;
      gdModalPanelDragger.value?.addEventListener("touchstart", dragStart);
      gdModalPanelDragger.value?.addEventListener("touchmove", dragMove);
      gdModalPanelDragger.value?.addEventListener("touchend", dragEnd);
    }

    modalTl.value = tl;
  });
</script>

<style lang="scss" scoped>
  .gd-modal {
    z-index: 999;
    pointer-events: none;
    position: fixed;
    top: 0;
    left: 0;
    bottom: 0;
    right: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    &-overlay {
      position: absolute;
      top: 0;
      left: 0;
      bottom: 0;
      right: 0;
      background: rgba(0, 0, 0, 0.5);
      backdrop-filter: blur(5px);
      opacity: 0;
    }
    &-panel {
      position: relative;
      width: auto;
      height: auto;
      border-radius: 1rem;
      border: var(--border);
      box-sizing: border-box;
      background-color: var(--background-depth-two-color);
      display: flex;
      flex-direction: column;
      opacity: 0;
      transform: scale(0.95);
      &-header {
        position: relative;
        width: 100%;
        padding: 1rem;
        box-sizing: border-box;
        display: flex;
        justify-content: space-between;
        align-items: center;
      }
      &-content {
        position: relative;
        width: 100%;
        padding: 0 1rem 1rem 1rem;
        box-sizing: border-box;
        display: flex;
        flex-direction: column;
        gap: 1rem;
      }
    }
    &[data-view="small"] {
      pointer-events: none;
      bottom: 0;
      right: 0;
      height: auto;
      .gd-modal-overlay {
        position: absolute;
        top: 0;
        left: 0;
        bottom: 0;
        right: 0;
        background: rgba(0, 0, 0, 0.5);
        backdrop-filter: blur(5px);
        opacity: 0;
      }
      .gd-modal-panel {
        touch-action: none;
        position: absolute;
        top: 0;
        right: 0;
        bottom: 0;
        left: 0;
        width: 100%;
        height: 100%;
        max-height: 100%;
        border: none;
        padding: 0;
        border-radius: 0.5rem 0.5rem 0 0;
        box-shadow: none;
        transform: translate3d(0, 100%, 0);
        transform-origin: center;

        &-content {
          position: relative;
          width: 100%;
          height: 70vh;
          padding: 1rem 0;
          box-sizing: border-box;
          display: flex;
          flex-direction: column;
        }

        &-dragger {
          z-index: 1;
          position: absolute;
          top: 0;
          left: 0;
          width: 100%;
          height: 4rem;
          background-color: transparent;
          &::before {
            content: "";
            position: absolute;
            top: 0.5rem;
            left: 50%;
            width: 2rem;
            height: 0.25rem;
            background-color: var(--font-secondary-color);
            border-radius: 0.125rem;
            transform: translateX(-50%);
            opacity: 0.25;
          }
        }

        &-header {
          height: 2rem;
          margin-top: 1rem;
          padding: 0;
          justify-content: center;
        }
        &-content {
          width: 100% !important;
          padding: 1rem;
          box-sizing: border-box;
          > * {
            width: 100% !important;
          }
        }
      }
    }
  }
</style>
