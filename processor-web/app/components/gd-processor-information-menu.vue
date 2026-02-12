<template>
  <gd-menu label="Edit Processor" :active="active" class="gd-menu">
    <div class="gd-menu-informations">
      <div class="gd-menu-informations-header">
        <span class="gd-menu-informations-header-title gd-headline-5"
          >Processor information</span
        >
      </div>
      <div class="gd-menu-informations-body">
        <div class="gd-menu-informations-body-input">
          <gd-input-text
            v-model="nameInput"
            label="Processor name"
            placeholder="Super Chiller 01"
            name="name"
          />
        </div>
      </div>
    </div>
    <div class="gd-menu-informations">
      <div class="gd-menu-informations-header">
        <span class="gd-menu-informations-header-title gd-headline-5"
          >Webhook</span
        >
        <gd-input-toggle
          class="gd-menu-informations-header-input"
          v-model="webhookEnabledInput"
        />
      </div>
      <div class="gd-menu-informations-body">
        <span class="gd-menu-informations-body-message gd-body-5">
          Add a webhook URL in the processor settings to receive real-time
          updates when a violation occur.
        </span>
      </div>
      <div v-if="webhookEnabled" class="gd-menu-informations-body">
        <div class="gd-menu-informations-body-input">
          <gd-input-text
            class="gd-menu-input"
            v-model="webhookHostInput"
            name="webhook-host"
            label="Server Host"
            placeholder="192.168.50.100"
            style="width: calc((100% - 0.5rem) * 3 / 4)"
          />
          <gd-input-text
            class="gd-menu-input"
            v-model="webhookPortInput"
            name="webhook-port"
            label="Port"
            placeholder="8001"
            style="width: calc((100% - 0.5rem) / 4)"
          />
        </div>
        <div class="gd-menu-informations-body-input">
          <gd-input-text
            class="gd-menu-input"
            v-model="webhookPathEvidenceInput"
            name="webhook-path-evidence"
            label="Upload Evidence Path"
            placeholder="/evidence"
          />
        </div>
        <div class="gd-menu-informations-body-input">
          <gd-input-text
            class="gd-menu-input"
            v-model="webhookPathUpdateInput"
            name="webhook-path-update"
            label="Update Path"
            placeholder="/update"
          />
        </div>
        <div class="gd-menu-informations-body-toggle">
          <span class="gd-menu-informations-body-toggle-title gd-headline-6"
            >Use HTTPS</span
          >
          <gd-input-toggle
            class="gd-menu-informations-body-toggle-input"
            v-model="webhookSecureInput"
          />
        </div>
      </div>
    </div>
    <div class="gd-menu-padding"></div>
    <div class="gd-menu-footer">
      <gd-button
        @click="submit"
        :disabled="!name || submitLoading"
        :loading="submitLoading"
        style="width: 100%"
        text="update processor"
      />
    </div>
  </gd-menu>
</template>

<script lang="ts" setup>
  import type { Processor } from "~/types/processor";

  const { closeMenu } = useMain();
  const { processor } = useProcessor();
  const { updateProcessor } = useProcessor();

  const emits = defineEmits(["shake"]);
  defineProps<{
    active: boolean;
  }>();

  const submitLoading = ref<boolean>(false);

  const nameInput = ref("");
  const lockEnabledInput = ref(false);
  const lockInput = ref("");
  const loggingInput = ref(false);
  const webhookEnabledInput = ref(false);
  const webhookHostInput = ref("");
  const webhookPortInput = ref("");
  const webhookPathEvidenceInput = ref("");
  const webhookPathUpdateInput = ref("");
  const webhookSecureInput = ref(false);

  const webhookEnabled = computed(() => webhookEnabledInput.value);

  const name = computed(() => nameInput.value);
  const logging = computed(() => loggingInput.value);
  const webhook = computed<Processor["webhook"] | undefined>(() => {
    if (!webhookEnabled.value) return undefined;

    let port: number | undefined = parseInt(webhookPortInput.value);
    let host: string | [number, number, number, number] =
      webhookHostInput.value;
    const secure = webhookSecureInput.value;
    const path = {
      evidence: webhookPathEvidenceInput.value,
      update: webhookPathUpdateInput.value,
    };

    if (!host) return undefined;
    if (host.split(".").length === 4) {
      const part = host.split(".");
      const host0 = parseInt(part[0]!);
      const host1 = parseInt(part[1]!);
      const host2 = parseInt(part[2]!);
      const host3 = parseInt(part[3]!);
      host = [host0, host1, host2, host3];
    }
    if (isNaN(port) || port! < 1 || port! > 65535) {
      port = undefined;
    }
    if (!path.evidence && !path.update) {
      return undefined;
    }

    return {
      host,
      port,
      path,
      secure,
    };
  });

  async function submit(): Promise<void> {
    if (!name.value || !processor.value) return;
    submitLoading.value = true;
    const payload = {
      ...processor.value,
      name: name.value,
      logging: logging.value,
      webhook: webhook.value,
    };

    const result = await updateProcessor(payload);
    setTimeout(() => {
      submitLoading.value = false;
      if (result) {
        if (processor.value) processor.value = result;
        closeMenu();
      } else {
        emits("shake");
      }
    }, 1000);
  }

  onMounted(() => {
    if (processor.value) {
      nameInput.value = processor.value.name;
      if (processor.value.webhook) {
        webhookEnabledInput.value = true;
        webhookHostInput.value =
          typeof processor.value.webhook.host === "string"
            ? processor.value.webhook.host
            : processor.value.webhook.host.join(".");
        webhookPortInput.value = processor.value.webhook.port?.toString() || "";
        webhookPathEvidenceInput.value =
          processor.value.webhook.path.evidence || "";
        webhookPathUpdateInput.value =
          processor.value.webhook.path.update || "";
        webhookSecureInput.value = processor.value.webhook.secure || false;
      }
    }
  });
</script>

<style lang="scss" scoped>
  .gd-menu {
    position: relative;
    display: flex;
    flex-direction: column;

    &-informations {
      position: relative;
      width: 100%;
      padding: 0.75rem;
      border-radius: 0.75rem;
      border: var(--border);
      box-sizing: border-box;
      background: var(--background-depth-one-color);
      margin-bottom: 1rem;
      display: flex;
      flex-direction: column;
      gap: 0.75rem;

      &-header {
        position: relative;
        width: 100%;
        display: flex;
        justify-content: space-between;
        align-items: center;
        &-input {
          position: relative;
        }
      }

      &-body {
        position: relative;
        width: 100%;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        &-input {
          position: relative;
          width: 100%;
          display: flex;
          align-items: flex-end;
          flex-wrap: wrap;
          gap: 0.5rem;
          &-label {
            position: relative;
            width: 100%;
            height: 1rem;
            color: var(--font-secondary-color);
            display: flex;
            align-items: center;
          }
        }
        &-message {
          position: relative;
          color: var(--font-secondary-color);
        }
        &-toggle {
          position: relative;
          width: 100%;
          height: 2rem;
          background-color: var(--background-depth-two-color);
          border-radius: 0.5rem;
          padding: 0 0.5rem;
          box-sizing: border-box;
          display: flex;
          justify-content: space-between;
          align-items: center;
          &-input {
            position: relative;
          }
        }
      }
    }

    &-padding {
      position: relative;
      width: 100%;
      height: 4rem;
    }

    &-footer {
      position: fixed;
      left: 0;
      bottom: 0;
      width: 100%;
      height: 4rem;
      padding: 1rem;
      border-top: var(--border);
      box-sizing: border-box;
      background: var(--background-depth-two-color);
    }
  }
</style>
