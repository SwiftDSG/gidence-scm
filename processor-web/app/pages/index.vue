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
        <!-- <gd-camera
          v-for="camera in cameraList"
          :key="camera.id"
          :camera="camera"
          :evidence="reading.camera[camera.id]?.evidence"
          :online="isOnline(camera.id)"
          @click="openCameraMenu(camera)"
        /> -->
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

      &-warning {
        position: relative;
        width: calc(100% - 2rem);
        padding: 0.75rem;
        margin: 0 1rem;
        border-radius: 0.75rem;
        background: var(--warning-color);
        box-sizing: border-box;
        display: flex;
        align-items: center;
        gap: 0.5rem;

        &-icon {
          position: relative;
          width: 2rem;
          height: 2rem;
          background: rgba(0, 0, 0, 0.05);
          padding: 0 0.5rem;
          border-radius: 0.5rem;
          box-sizing: border-box;
          display: flex;
          justify-content: center;
          align-items: center;
          flex-shrink: 0;
        }
        &-title {
          position: relative;
          width: calc(100% - 2.5rem);
          height: 2rem;
          display: flex;
          justify-content: center;
          flex-direction: column;
          &-value {
            position: relative;
            color: var(--font-tertiary-color);
          }
          &-message {
            position: relative;
            width: 100%;
            color: var(--font-tertiary-color);
            opacity: 0.5;
          }
        }
      }

      &-calculations {
        position: relative;
        width: 100%;
        padding: 0 1rem 1rem 1rem;
        box-sizing: border-box;
        display: flex;
        gap: 1rem;
        flex-wrap: wrap;

        .gd-calculation {
          position: relative;
          width: calc((100% - 3rem) / 4);
          flex-shrink: 0;
        }
      }
      &-functions {
        position: relative;
        width: 100%;
        padding: 0 1rem 1rem 1rem;
        box-sizing: border-box;
        display: flex;
        gap: 1rem;
        flex-wrap: wrap;

        .gd-function {
          position: relative;
          width: calc((100% - 3rem) / 4);
          flex-shrink: 0;
        }
      }

      &-template {
        position: relative;
        width: 100%;
        height: calc(100vh - 18rem);
        margin-bottom: 1rem;
        box-sizing: border-box;
        display: flex;
        flex-direction: column;
        &-illustration {
          position: relative;
          display: flex;
          flex: 1 0;
        }
      }

      &-table {
        position: relative;
        width: 100%;
        padding: 0 1rem 1rem 1rem;
        box-sizing: border-box;
        display: flex;
        flex-direction: column;
        gap: 1rem;
        &-section {
          position: relative;
          width: 100%;
          display: flex;
          flex-direction: column;
          gap: 0.5rem;

          &-title {
            color: var(--text-primary-color);
          }

          &-table {
            position: relative;
            width: 100%;
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
  .gd-page {
    position: relative;
    width: 100%;
    min-height: 100vh;
    padding: 1.5rem;
    box-sizing: border-box;
    background: var(--background-depth-three-color);
    display: flex;
    flex-direction: column;
    gap: 1.5rem;

    &-header {
      position: relative;
      display: flex;
      justify-content: space-between;
      align-items: center;

      &-title {
        display: flex;
        align-items: center;
        gap: 1rem;

        &-status {
          display: flex;
          align-items: center;
          gap: 0.5rem;
          padding: 0.25rem 0.75rem;
          border-radius: 1rem;
          background: var(--background-depth-one-color);
          border: var(--border);

          &-indicator {
            width: 0.5rem;
            height: 0.5rem;
            border-radius: 50%;
          }

          &.--online {
            .gd-page-header-title-status-indicator {
              background: var(--success-color);
            }
          }

          &.--offline {
            .gd-page-header-title-status-indicator {
              background: var(--error-color);
            }
          }
        }
      }

      &-actions {
        display: flex;
        gap: 0.5rem;
      }
    }

    &-empty {
      position: relative;
      flex: 1;
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
      gap: 1rem;
      color: var(--font-secondary-color);
    }

    &-cameras {
      position: relative;
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(20rem, 1fr));
      gap: 1rem;
    }

    &-recent {
      position: relative;
      display: flex;
      flex-direction: column;
      gap: 1rem;

      &-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
      }

      &-list {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
      }
    }
  }
</style>
