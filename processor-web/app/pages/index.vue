<template>
  <div class="gd" :data-view="view">
    <div class="gd-processor">
      <div class="gd-processor-header">
        <div class="gd-processor-header-information">
          <span class="gd-processor-header-information-name gd-headline-2">
            {{
              processor && processor.name !== processor.id
                ? processor.name
                : "Optense — SCM"
            }}
          </span>
        </div>
        <div class="gd-processor-header-information">
          <div
            class="gd-processor-header-information-status"
            :class="readerOnline ? '--online' : '--offline'"
          >
            <div class="gd-processor-header-information-status-indicator"></div>
            <span
              class="gd-processor-header-information-status-name gd-headline-5"
              >{{ readerOnline ? "Online" : "Offline" }}</span
            >
          </div>
          <gd-button
            v-if="view !== 'large'"
            icon="dots"
            type="tertiary"
            @click="
              openMenu({
                processor: {},
              })
            "
          />
        </div>
      </div>

      <div class="gd-processor-cameras">
        <div v-for="n in 4" class="gd-processor-cameras-item">
          <gd-camera
            v-if="cameras[n - 1]"
            :key="cameras[n - 1]!.id"
            :camera="cameras[n - 1]!"
            @click="
              openMenu({
                camera: {
                  camera_id: cameras[n - 1]!.id,
                },
              })
            "
          />
          <button
            v-else-if="n - 1 === cameras.length"
            class="gd-processor-cameras-item-button"
            @click="
              openMenu({
                cameraInformation: {},
              })
            "
          >
            <div class="gd-processor-cameras-item-button-icon">
              <gd-svg name="plus" color="tertiary" />
            </div>
            <span class="gd-processor-cameras-item-button-title gd-headline-4">
              Add Camera
            </span>
          </button>
          <div v-else class="gd-processor-cameras-item-empty"></div>
        </div>
      </div>
    </div>
    <gd-menus @shake="emits('shake')" />

    <gd-loader
      type="overlay"
      :state="loading ? 'show' : 'hide'"
      fixed
      immediate
    />
  </div>
</template>

<script lang="ts" setup>
  const emits = defineEmits(["shake"]);

  const { view, openMenu } = useMain();
  const { getDevice } = useDevice();
  const { processor } = useProcessor();
  const { cameras } = useCamera();
  const { reading, readerOnline, readerStart } = useReader();

  const loading = ref(true);

  const cameraList = computed(() => Object.values(cameras.value));

  // const recentViolations = computed(() => {
  //   return evidence.value
  //     .filter((e) => e.person.some((p) => p.violation.length > 0))
  //     .slice(0, 5);
  // });

  onMounted(async () => {
    await getDevice();
    await readerStart();
    loading.value = false;

    if (view.value === "large" && processor.value) {
      openMenu({
        processor: {},
      });
    }
  });
</script>

<style lang="scss" scoped>
  .gd {
    position: relative;
    width: 100%;
    background: var(--background-depth-three-color);
    display: flex;

    &-processor {
      position: relative;
      width: 100%;
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      overflow-y: auto;

      &-header {
        position: relative;
        width: 100%;
        height: 4rem;
        padding: 1rem;
        box-sizing: border-box;
        background: var(--background-depth-three-color);
        display: flex;
        justify-content: space-between;
        align-items: center;

        &-information {
          position: relative;
          display: flex;
          align-items: center;
          gap: 0.5rem;
          &-status {
            position: relative;
            height: 2rem;
            padding: 0.5rem 0.75rem;
            border: var(--border);
            border-radius: 1rem;
            box-sizing: border-box;
            background: var(--background-depth-one-color);
            display: flex;
            align-items: center;
            gap: 0.5rem;

            &-indicator {
              position: relative;
              width: 0.5rem;
              height: 0.5rem;
              border-radius: 0.25rem;
              background: var(--primary-color);
              display: flex;
              justify-content: center;
              align-items: center;
              flex-shrink: 0;

              &::after {
                content: "";
                position: relative;
                width: 1rem;
                height: 1rem;
                border-radius: 0.5rem;
                background: var(--primary-color);
                opacity: 0.2;
                flex-shrink: 0;
              }
            }

            &.--online {
              .gd-processor-header-information-status-indicator {
                background: var(--success-color);

                &::after {
                  background: var(--success-color);
                }
              }
            }

            &.--offline {
              .gd-processor-header-information-status-indicator {
                background: var(--error-color);

                &::after {
                  background: var(--error-color);
                }
              }
            }
          }
        }
      }

      // 2x2 grid
      &-cameras {
        position: relative;
        width: 100%;
        flex-grow: 1;
        padding: 0 1rem 1rem 1rem;
        box-sizing: border-box;
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        grid-template-rows: repeat(2, 1fr);
        gap: 1rem;
        &-item {
          position: relative;
          width: 100%;
          &-button {
            cursor: pointer;
            position: relative;
            width: 100%;
            height: 100%;
            border: 1px dashed var(--primary-color);
            border-radius: 0.75rem;
            background-color: transparent;
            box-sizing: border-box;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            gap: 0.5rem;
            &-icon {
              position: relative;
              width: 2rem;
              height: 2rem;
              background-color: var(--primary-color);
              padding: 0 0.5rem;
              border-radius: 0.5rem;
              box-sizing: border-box;
              display: flex;
              justify-self: center;
              align-items: center;
            }
            &-title {
              position: relative;
              color: var(--font-primary-color);
            }
            &::before {
              content: "";
              position: absolute;
              width: 100%;
              height: 100%;
              border-radius: 0.75rem;
              box-sizing: border-box;
              background-color: var(--primary-color);
              opacity: 0.1;
            }
          }
          &-empty {
            position: relative;
            width: 100%;
            height: 100%;
            background-color: var(--background-depth-two-color);
            border: 1px dashed var(--border-color);
            border-radius: 0.75rem;
            box-sizing: border-box;
            opacity: 0.5;
          }
        }
      }

      &.--inactive {
        pointer-events: none;
        opacity: 0.5;
        filter: blur(10px);
      }
    }

    &[data-view="large"] {
      width: calc(100% - 20rem);
    }

    &[data-view="small"] {
      .gd-processor {
        &-header {
          &-information {
            display: none;
            &-status {
              width: 100%;
              justify-content: space-between;
            }
          }
        }
      }
    }
  }
</style>
