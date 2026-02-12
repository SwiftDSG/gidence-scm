<template>
  <div class="gd-component-svg" ref="gdComponent" :class="`--${color}`"></div>
</template>

<script lang="ts" setup>
const props = withDefaults(
  defineProps<{
    name: string;
    color?:
      | "primary"
      | "secondary"
      | "tertiary"
      | "error"
      | "success"
      | "warning";
  }>(),
  {
    color: "secondary",
  },
);
const { loadSvg } = useSvg();

const gdComponent = ref<HTMLDivElement>();

const pendingUpdates = new Set<() => void>();

function batchDOMUpdate(updateFn: () => void) {
  pendingUpdates.add(updateFn);
  
  if (pendingUpdates.size === 1) {
    requestAnimationFrame(() => {
      pendingUpdates.forEach(fn => fn());
      pendingUpdates.clear();
    });
  }
}

async function loadElementText() {
  if (!gdComponent.value) return;
  
  const svgContent = await loadSvg(props.name);
  const element = gdComponent.value;
  
  batchDOMUpdate(() => {
    if (element) {
      element.innerHTML = svgContent;
    }
  });
}

watch(
  () => props.name,
  () => {
    loadElementText();
  },
);

onMounted(() => {
  loadElementText();
});
</script>

<style lang="scss">
.gd-component-svg {
  position: relative;
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  transition: 0.25s transform;
  svg {
    position: relative;
    width: 100%;
    height: 100%;
  }

  &.--primary {
    svg {
      &.gd-svg-filled {
        .gd-svg-filled-main {
          fill: var(--primary-color);
        }
        .gd-svg-filled-secondary {
          fill: var(--font-secondary-color);
        }
      }
      &.gd-svg-outlined {
        .gd-svg-outlined-main {
          stroke: var(--primary-color);
        }
        .gd-svg-outlined-secondary {
          stroke: var(--font-secondary-color);
        }
      }
    }
  }

  &.--secondary {
    svg {
      &.gd-svg-filled {
        .gd-svg-filled-main {
          fill: var(--font-secondary-color);
        }
        .gd-svg-filled-secondary {
          fill: var(--font-secondary-color);
        }
      }
      &.gd-svg-outlined {
        .gd-svg-outlined-main {
          stroke: var(--font-secondary-color);
        }
        .gd-svg-outlined-secondary {
          stroke: var(--font-secondary-color);
        }
      }
    }
  }

  &.--tertiary {
    svg {
      &.gd-svg-filled {
        .gd-svg-filled-main {
          fill: var(--font-tertiary-color);
        }
        .gd-svg-filled-secondary {
          fill: var(--font-tertiary-color);
        }
      }
      &.gd-svg-outlined {
        .gd-svg-outlined-main {
          stroke: var(--font-tertiary-color);
        }
        .gd-svg-outlined-secondary {
          stroke: var(--font-tertiary-color);
        }
      }
    }
  }

  &.--success {
    svg {
      &.gd-svg-filled {
        .gd-svg-filled-main {
          fill: var(--success-color);
        }
        .gd-svg-filled-secondary {
          fill: var(--secondary-color);
        }
      }
      &.gd-svg-outlined {
        .gd-svg-outlined-main {
          stroke: var(--success-color);
        }
        .gd-svg-outlined-secondary {
          stroke: var(--secondary-color);
        }
      }
    }
  }

  &.--error {
    svg {
      &.gd-svg-filled {
        .gd-svg-filled-main {
          fill: var(--error-color);
        }
        .gd-svg-filled-secondary {
          fill: var(--secondary-color);
        }
      }
      &.gd-svg-outlined {
        .gd-svg-outlined-main {
          stroke: var(--error-color);
        }
        .gd-svg-outlined-secondary {
          stroke: var(--secondary-color);
        }
      }
    }
  }

  &.--warning {
    svg {
      &.gd-svg-filled {
        .gd-svg-filled-main {
          fill: var(--warning-color);
        }
        .gd-svg-filled-secondary {
          fill: var(--secondary-color);
        }
      }
      &.gd-svg-outlined {
        .gd-svg-outlined-main {
          stroke: var(--warning-color);
        }
        .gd-svg-outlined-secondary {
          stroke: var(--secondary-color);
        }
      }
    }
  }
}
</style>
