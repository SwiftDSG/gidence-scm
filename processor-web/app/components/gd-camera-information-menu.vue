<template>
  <gd-menu
    :label="camera_id ? 'Edit Camera' : 'Add Camera'"
    :active="active"
    class="gd-menu"
  >
    <div class="gd-menu-body">
      <div class="gd-menu-input-wrapper">
        <gd-input-text
          class="gd-menu-input"
          v-model="nameInputValue"
          :label="nameInputConfig.label"
          :placeholder="nameInputConfig.placeholder"
          :name="nameInputConfig.name"
        />
      </div>
      <div class="gd-menu-input-wrapper">
        <gd-input-text
          class="gd-menu-input"
          v-model="addressHostInputValue[0]"
          label="Host"
          placeholder="0"
          name="address-host-0"
          style="width: calc((100% - 1.5rem) / 4)"
        />
        <gd-input-text
          class="gd-menu-input"
          v-model="addressHostInputValue[1]"
          placeholder="1"
          name="address-host-1"
          style="width: calc((100% - 1.5rem) / 4)"
        />
        <gd-input-text
          class="gd-menu-input"
          v-model="addressHostInputValue[2]"
          placeholder="2"
          name="address-host-2"
          style="width: calc((100% - 1.5rem) / 4)"
        />
        <gd-input-text
          class="gd-menu-input"
          v-model="addressHostInputValue[3]"
          placeholder="3"
          name="address-host-3"
          style="width: calc((100% - 1.5rem) / 4)"
        />
      </div>
      <div class="gd-menu-input-wrapper">
        <gd-input-text
          class="gd-menu-input"
          v-model="addressCameraInputValue"
          :label="addressCameraInputConfig.label"
          :placeholder="addressCameraInputConfig.placeholder"
          :name="addressCameraInputConfig.name"
        />
      </div>
    </div>
    <div class="gd-menu-footer">
      <gd-button
        @click="submit"
        :disabled="!name || !address"
        style="width: 100%"
        :text="camera_id ? 'update camera' : 'create camera'"
      />
    </div>
  </gd-menu>
</template>

<script lang="ts" setup>
  import type { Camera } from "~/types/camera";

  const emits = defineEmits(["shake"]);
  const props = defineProps<{
    active: boolean;
    camera_id?: string;
  }>();

  const { closeMenu } = useMain();
  const { cameras, createCamera, updateCamera } = useCamera();

  const submitLoading = ref<boolean>(false);

  // Input values
  const nameInputValue = ref<string>("");
  const addressHostInputValue = ref<[string, string, string, string]>([
    "",
    "",
    "",
    "",
  ]);
  const addressCameraInputValue = ref<string>("");

  // Input configs
  const nameInputConfig = {
    name: "name",
    label: "Camera name",
    placeholder: "Camera 01",
  };

  const addressCameraInputConfig = {
    name: "address-camera",
    label: "Camera",
    placeholder: "502",
  };

  const camera = computed<Camera | null>(() => {
    return cameras.value.find((p) => p.id === props.camera_id) || null;
  });
  const name = computed<string>(() => nameInputValue.value);
  const address = computed<Camera["address"] | null>(() => {
    const host0 = parseInt(addressHostInputValue.value[0]);
    const host1 = parseInt(addressHostInputValue.value[1]);
    const host2 = parseInt(addressHostInputValue.value[2]);
    const host3 = parseInt(addressHostInputValue.value[3]);
    const port = parseInt(addressCameraInputValue.value);

    if (
      !isNaN(host0) &&
      !isNaN(host1) &&
      !isNaN(host2) &&
      !isNaN(host3) &&
      !isNaN(port)
    ) {
      return {
        host: [host0, host1, host2, host3],
        port,
      };
    }
    return null;
  });

  async function submit(): Promise<void> {
    if (!name.value || !address.value) return;

    submitLoading.value = true;

    const payload = {
      id: "",
      name: name.value,
      address: address.value,
    };

    let result;
    if (props.camera_id) {
      payload.id = props.camera_id;
      result = await updateCamera(props.camera_id, payload);
    } else {
      result = await createCamera(payload);
    }

    setTimeout(() => {
      submitLoading.value = false;
      if (result) {
        closeMenu();
      } else {
        emits("shake");
      }
    }, 1000);
  }

  watch(
    () => camera.value,
    (val) => {
      if (val) {
        nameInputValue.value = val.name;
        addressHostInputValue.value = [
          val.address.host[0].toString(),
          val.address.host[1].toString(),
          val.address.host[2].toString(),
          val.address.host[3].toString(),
        ];
        addressCameraInputValue.value = val.address.port.toString();
      }
    },
    { once: true, immediate: true },
  );
</script>

<style lang="scss" scoped>
  .gd-menu {
    position: relative;
    display: flex;
    flex-direction: column;

    .gd-menu-message {
      position: relative;
      width: 100%;
    }

    .gd-menu-body {
      position: relative;
      width: 100%;
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      .gd-menu-input-wrapper {
        position: relative;
        width: 100%;
        display: flex;
        gap: 0 0.5rem;
        flex-wrap: wrap;
        align-items: flex-end;

        .gd-menu-input {
          position: relative;
          width: 100%;
        }

        .gd-menu-input-error {
          position: relative;
          width: 100%;
          height: 1rem;
          flex-shrink: 0;
        }
      }
    }

    .gd-menu-footer {
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
