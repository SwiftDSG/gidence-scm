<template>
  <div class="gd-component" ref="gdComponent">
    <div
      v-for="alert in alerts"
      :key="alert.id"
      :data-id="alert.id"
      class="gd-alert"
    >
      <div class="gd-alert-icon">
        <gd-svg :name="alert.type" :color="alert.type" />
      </div>
      <div class="gd-alert-information">
        <span class="gd-alert-information-title gd-headline-5">
          {{ alert.title }}
        </span>
        <span class="gd-alert-information-message gd-body-4">
          {{ alert.message }}
        </span>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
  import type { Alert } from "~/types/alert";

  import gsap from "gsap";

  interface AlertTimeout extends Alert {
    id: number;
    playing: boolean;
    timeout: NodeJS.Timeout;
  }

  const { alert, removeAlert } = useAlert();

  const gdComponent = ref<HTMLDivElement | null>(null);

  const alerts = ref<AlertTimeout[]>([]);
  const alertId = ref<number>(0);

  const animate = {
    show(gdComponent: HTMLElement, id: number, cb?: () => void): void {
      const tl: GSAPTimeline = gsap.timeline({
        onComplete() {
          if (cb) cb();
        },
      });

      const gdAlert: HTMLElement | null = gdComponent.querySelector(
        `.gd-alert[data-id="${id}"]`
      );

      tl.to(gdAlert, {
        y: 0,
        ease: "power2.out",
        duration: 0.5,
      });
    },
    hide(gdComponent: HTMLElement, id: number, cb?: () => void): void {
      const tl: GSAPTimeline = gsap.timeline({
        onComplete() {
          if (cb) cb();
        },
      });

      const gdAlert: HTMLElement | null = gdComponent.querySelector(
        `.gd-alert[data-id="${id}"]`
      );

      tl.to(gdAlert, {
        y: "-4rem",
        ease: "power2.in",
        duration: 0.5,
      });
    },
    remove(gdComponent: HTMLElement, id: number, cb?: () => void): void {
      const tl: GSAPTimeline = gsap.timeline({
        onComplete() {
          if (cb) cb();
        },
      });

      const gdAlert: HTMLElement | null = gdComponent.querySelector(
        `.gd-alert[data-id="${id}"]`
      );

      tl.to(gdAlert, {
        opacity: 0,
        scale: 0.75,
        ease: "power2.in",
        duration: 0.5,
      });
    },
  };

  function alertHandler(data: AlertTimeout): void {
    // Only remove global alert state after we've processed it
    nextTick(() => removeAlert());
    
    alerts.value.push(data);
    
    setTimeout(() => {
      // Find previous alert by ID to avoid index issues
      const prevAlert = alerts.value.find((alert, index) => 
        index === alerts.value.length - 2 && alert.id !== data.id
      );
      
      if (prevAlert && !prevAlert.playing) {
        prevAlert.playing = true;
        if (gdComponent.value) {
          animate.remove(gdComponent.value, prevAlert.id, () => {
            // Clear timeout safely
            if (prevAlert.timeout) {
              clearTimeout(prevAlert.timeout);
            }
            // Remove by ID instead of index to avoid race conditions
            const indexToRemove = alerts.value.findIndex(a => a.id === prevAlert.id);
            if (indexToRemove > -1) {
              alerts.value.splice(indexToRemove, 1);
            }
          });
        }
      }
      
      // Show the new alert after a brief delay
      setTimeout(
        () => {
          if (gdComponent.value) {
            animate.show(gdComponent.value, data.id);
          }
        },
        prevAlert ? 100 : 0
      );
    }, 100);
  }

  watch(
    () => alert.value,
    (val) => {
      if (val) {
        const id: number = alertId.value++;
        alertHandler({
          ...val,
          id,
          playing: false,
          timeout: setTimeout(() => {
            const alertToHide = alerts.value.find((a) => a.id === id);
            if (
              alertToHide &&
              !alertToHide.playing &&
              gdComponent.value
            ) {
              alertToHide.playing = true;
              animate.hide(gdComponent.value, id, () => {
                const indexToRemove = alerts.value.findIndex((a) => a.id === id);
                if (indexToRemove > -1) {
                  alerts.value.splice(indexToRemove, 1);
                }
              });
            }
          }, 2000),
        });
      }
    }
  );
</script>

<style lang="scss" scoped>
  .gd-component {
    pointer-events: none;
    z-index: 3000000000;
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 5rem;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    .gd-alert {
      position: absolute;
      bottom: 1rem;
      left: calc(50% - 10rem);
      width: 20rem;
      height: 3rem;
      padding: 0.5rem;
      box-sizing: border-box;
      background: var(--background-depth-two-color);
      border: var(--border);
      border-radius: 0.75rem;
      display: flex;
      justify-content: flex-start;
      align-items: center;
      transform: translateY(-4rem);
      transform-origin: top center;
      &-icon {
        position: relative;
        width: 2rem;
        height: 2rem;
        background: var(--background-depth-three-color);
        border-radius: 0.5rem;
        padding: 0 0.5rem;
        box-sizing: border-box;
        display: flex;
        justify-content: center;
        align-items: center;
      }
      &-information {
        position: relative;
        width: calc(100% - 2rem);
        height: 100%;
        padding-left: 0.5rem;
        box-sizing: border-box;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: flex-start;
        gap: 0.25rem;
        &-title {
          position: relative;
          line-height: 1;
        }
        &-message {
          position: relative;
          line-height: 1;
          color: var(--font-secondary-color);
        }
      }
    }
    [gd-view="mobile"] {
      .gd-alert {
        left: 1rem;
        width: calc(100vw - 2rem);
      }
    }
  }
</style>
