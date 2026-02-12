<template>
  <div :class="['gd-progress', `--${type}`, { '--labeled': label }]">
    <div v-if="label" class="gd-progress-label gd-body-4">
      {{ label }}
    </div>
    <div class="gd-progress-container">
      <div class="gd-progress-container-track">
        <div
          class="gd-progress-container-track-fill"
          :style="{ width: `${percentage}%` }"
        ></div>
      </div>
      <div
        v-if="showPercentage"
        class="gd-progress-container-text gd-caption-text"
      >
        {{ Math.round(percentage) }}%
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
type ProgressProps = {
  value: number;
  max?: number;
  label?: string;
  type?: "primary" | "secondary" | "success" | "error" | "warning";
  showPercentage?: boolean;
};

const props = withDefaults(defineProps<ProgressProps>(), {
  max: 100,
  type: "primary",
  showPercentage: true,
});

const percentage = computed(() => {
  if (props.max === 0) return 0;
  const percent = (props.value / props.max) * 100;
  return Math.min(Math.max(percent, 0), 100);
});
</script>

<style lang="scss" scoped>
.gd-progress {
  position: relative;
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;

  &-label {
    position: relative;
    color: var(--font-primary-color);
  }

  &-container {
    position: relative;
    display: flex;
    align-items: center;
    gap: 0.5rem;

    &-track {
      position: relative;
      flex: 1;
      height: 0.5rem;
      background-color: var(--background-depth-one-color);
      border: var(--border);
      border-radius: 0.25rem;
      overflow: hidden;

      &-fill {
        position: relative;
        height: 100%;
        border-radius: 0.25rem;
        transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      }
    }

    &-text {
      position: relative;
      flex-shrink: 0;
      color: var(--font-secondary-color);
      min-width: 2rem;
      text-align: right;
    }
  }

  &.--primary {
    .gd-progress-container-track-fill {
      background-color: var(--primary-color);
    }
  }

  &.--secondary {
    .gd-progress-container-track-fill {
      background-color: var(--secondary-color);
    }
  }

  &.--success {
    .gd-progress-container-track-fill {
      background-color: var(--success-color);
    }
  }

  &.--error {
    .gd-progress-container-track-fill {
      background-color: var(--error-color);
    }
  }

  &.--warning {
    .gd-progress-container-track-fill {
      background-color: var(--warning-color);
    }
  }
}
</style>
