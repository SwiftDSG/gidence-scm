<template>
  <gd-menu title="Add Webhook" @close="emits('close')">
    <div class="gd-webhook-menu">
      <div class="gd-webhook-menu-field">
        <span class="gd-webhook-menu-field-label gd-body-4">Host</span>
        <gd-input-text
          v-model="form.host"
          placeholder="example.com or 192.168.1.100"
        />
      </div>
      <div class="gd-webhook-menu-field">
        <span class="gd-webhook-menu-field-label gd-body-4">Port (optional)</span>
        <gd-input-text
          v-model="form.port"
          placeholder="8080"
          style="width: 6rem"
        />
      </div>
      <div class="gd-webhook-menu-field">
        <span class="gd-webhook-menu-field-label gd-body-4">Path</span>
        <gd-input-text
          v-model="form.path"
          placeholder="/api/webhook"
        />
      </div>
      <div class="gd-webhook-menu-field">
        <span class="gd-webhook-menu-field-label gd-body-4">Protocol</span>
        <div class="gd-webhook-menu-field-row">
          <gd-button
            text="HTTP"
            :type="!form.secure ? 'primary' : 'secondary'"
            @click="form.secure = false"
          />
          <gd-button
            text="HTTPS"
            :type="form.secure ? 'primary' : 'secondary'"
            @click="form.secure = true"
          />
        </div>
      </div>
      <div class="gd-webhook-menu-actions">
        <gd-button
          text="Cancel"
          type="tertiary"
          @click="emits('close')"
        />
        <gd-button
          text="Create"
          type="primary"
          :loading="saving"
          @click="handleSave"
        />
      </div>
    </div>
  </gd-menu>
</template>

<script lang="ts" setup>
import type { ProcessorWebhook, ProcessorWebhookHost } from "~/types/processor";

const emits = defineEmits<{
  (event: "close"): void;
  (event: "saved", webhook: ProcessorWebhook): void;
}>();

const { createWebhook } = useProcessor();

const saving = ref(false);

const form = ref({
  host: "",
  port: "",
  path: "/api/webhook",
  secure: false,
});

const parseHost = (host: string): ProcessorWebhookHost => {
  const parts = host.split(".");
  if (parts.length === 4 && parts.every((p) => !isNaN(parseInt(p)))) {
    return parts.map((p) => parseInt(p)) as [number, number, number, number];
  }
  return host;
};

const handleSave = async () => {
  saving.value = true;

  const payload = {
    host: parseHost(form.value.host),
    port: form.value.port ? parseInt(form.value.port) : undefined,
    path: form.value.path,
    secure: form.value.secure,
  };

  const result = await createWebhook(payload);

  saving.value = false;

  if (result) {
    emits("saved", result);
    emits("close");
  }
};
</script>

<style lang="scss" scoped>
.gd-webhook-menu {
  position: relative;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1rem;

  &-field {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    &-label {
      color: var(--font-secondary-color);
    }

    &-row {
      display: flex;
      gap: 0.5rem;
    }
  }

  &-actions {
    position: relative;
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    padding-top: 0.5rem;
    border-top: var(--border);
  }
}
</style>
