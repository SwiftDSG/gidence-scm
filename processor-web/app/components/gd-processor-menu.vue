<template>
  <gd-menu
    :active="props.active"
    :label="view !== 'large' ? 'Processor Menu' : ''"
    class="gd-menu"
  >
    <div v-if="processor" class="gd-menu-body">
      <div class="gd-menu-body-informations">
        <div class="gd-menu-body-informations-item" style="z-index: 1">
          <span class="gd-menu-body-informations-item-placeholder gd-body-5"
            >Processor ID</span
          >
          <span class="gd-menu-body-informations-item-value gd-headline-5">{{
            processor.id
          }}</span>
        </div>
        <div class="gd-menu-body-informations-item">
          <span class="gd-menu-body-informations-item-placeholder gd-body-5"
            >Processor address</span
          >
          <span class="gd-menu-body-informations-item-value gd-headline-5">{{
            addressWrite(processor.address)
          }}</span>
        </div>
      </div>
      <div
        class="gd-menu-body-item"
        @click="
          openMenu({
            processorInformation: {},
          })
        "
      >
        <div class="gd-menu-body-item-icon">
          <gd-svg color="secondary" name="information" />
        </div>
        <div class="gd-menu-body-item-information">
          <span class="gd-menu-body-item-information-value gd-headline-5"
            >General information</span
          >
          <span class="gd-menu-body-item-information-placeholder gd-body-5"
            >See or change this processor's information</span
          >
        </div>
      </div>
    </div>
    <div class="gd-menu-footer">
      <div class="gd-menu-footer-item">
        <div class="gd-menu-footer-item-icon">
          <gd-svg
            :name="theme === 'dark' ? 'weather-night' : 'weather-sunny'"
            :color="theme === 'dark' ? 'primary' : 'warning'"
          />
        </div>
        <div class="gd-menu-footer-item-information">
          <span class="gd-menu-footer-item-information-value gd-headline-5">
            Enable dark theme
          </span>
          <span class="gd-menu-footer-item-information-placeholder gd-body-5">
            Change the theme of the application
          </span>
        </div>
        <gd-input-toggle
          v-model="themeInput"
          class="gd-menu-footer-item-information-input"
        />
      </div>
    </div>
  </gd-menu>
</template>

<script lang="ts" setup>
  import type { Theme } from "~/types/general";
  import type { Processor } from "~/types/processor";

  const emits = defineEmits(["shake"]);
  const props = defineProps<{
    active: boolean;
  }>();

  const { getTheme, setTheme, view, openMenu } = useMain();
  const { processor } = useProcessor();

  const themeInput = ref(false);

  const theme = computed<Theme>(() => (themeInput.value ? "dark" : "light"));

  function addressWrite(address: Processor["address"]): string {
    return `${address.host.join(".")}:${address.port}`;
  }

  watch(
    () => theme.value,
    (val) => {
      setTheme(val);
    },
  );

  onMounted(() => {
    themeInput.value = getTheme() === "dark";
  });
</script>

<style lang="scss" scoped>
  .gd-menu {
    &-body {
      position: relative;
      width: 100%;
      display: flex;
      flex-direction: column;
      gap: 1rem;

      &-informations {
        position: relative;
        width: 100%;
        padding: 0.75rem;
        border-radius: 0.75rem;
        border: var(--border);
        box-sizing: border-box;
        background: var(--background-depth-one-color);
        display: flex;
        flex-direction: column;
        gap: 0.75rem;

        &-item {
          position: relative;
          display: flex;
          flex-direction: column;

          &-value {
            position: relative;
            width: 100%;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
          }
          &-placeholder {
            color: var(--font-secondary-color);
          }
        }
      }

      &-item {
        cursor: pointer;
        position: relative;
        width: 100%;
        padding: 0.75rem;
        border-radius: 0.75rem;
        border: var(--border);
        background: var(--background-depth-one-color);
        box-sizing: border-box;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        overflow: hidden;

        * {
          pointer-events: none;
        }

        &-icon {
          position: relative;
          width: 2rem;
          height: 2rem;
          background: var(--background-depth-two-color);
          padding: 0 0.5rem;
          border-radius: 0.5rem;
          box-sizing: border-box;
          display: flex;
          justify-content: center;
          align-items: center;
          flex-shrink: 0;
        }

        &-information {
          position: relative;
          width: calc(100% - 2.5rem);
          display: flex;
          flex-direction: column;

          &-placeholder {
            position: relative;
            width: 100%;
            color: var(--font-secondary-color);
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
          }
        }

        &::after {
          content: "";
          position: absolute;
          top: 0;
          left: 0;
          width: 100%;
          height: 100%;
          background: var(--font-primary-color);
          opacity: 0;
          transition: 0.25s opacity;
        }

        &:hover::after {
          opacity: 0.025;
        }
      }
    }

    &-footer {
      position: fixed;
      left: 0;
      bottom: 0;
      width: 100%;
      height: 5.5rem;
      padding: 1rem;
      border-top: var(--border);
      box-sizing: border-box;
      background: var(--background-depth-two-color);

      &-item {
        position: relative;
        width: 100%;
        padding: 0.75rem;
        border-radius: 0.75rem;
        border: var(--border);
        background: var(--background-depth-one-color);
        box-sizing: border-box;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        overflow: hidden;

        &-icon {
          position: relative;
          width: 2rem;
          height: 2rem;
          background: var(--background-depth-two-color);
          padding: 0 0.5rem;
          border-radius: 0.5rem;
          box-sizing: border-box;
          display: flex;
          justify-content: center;
          align-items: center;
          flex-shrink: 0;
        }

        &-information {
          position: relative;
          width: calc(100% - 4.5rem);
          display: flex;
          flex-direction: column;

          &-placeholder {
            position: relative;
            width: 100%;
            color: var(--font-secondary-color);
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
          }

          &-input {
            flex-shrink: 0;
          }
        }
      }
    }
  }
</style>
