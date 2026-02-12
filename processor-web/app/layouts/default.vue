<template>
  <client-only>
    <div class="gd" :data-view="view">
      <main class="gd-main">
        <nuxt-page />
      </main>
    </div>
    <gd-alert />
  </client-only>
</template>

<script lang="ts" setup>
  import "~/assets/styles/global.scss";

  const { view, rem, theme, init, getTheme } = useMain();

  const resizeHandler = (): void => {
    if (window.innerWidth > 1280) view.value = "large";
    else if (window.innerWidth > 768) view.value = "medium";
    else view.value = "small";

    rem.value = parseInt(getComputedStyle?.(document.body)?.fontSize) || 24;
    document.documentElement.style.setProperty(
      "--vh",
      `${window.innerHeight * 0.01}px`,
    );

    init.value = true;
  };

  watch(
    () => theme.value,
    (newTheme) => {
      document.documentElement.setAttribute("data-theme", newTheme);
    },
  );

  onBeforeMount(() => {
    getTheme();
  });
  onMounted(() => {
    resizeHandler();

    window.addEventListener("resize", resizeHandler);
  });
  onBeforeUnmount(() => {
    window.removeEventListener("resize", resizeHandler);
  });
</script>

<style lang="scss" scoped>
  .gd {
    position: relative;
    width: 100vw;
    display: flex;
    &-main {
      position: relative;
      width: 100vw;
      height: 100vh;
    }
    &[data-view="small"] {
      .gd-main {
        left: 0;
        width: 100vw;
        height: auto;
      }
    }
  }
</style>
