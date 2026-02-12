import type { Camera, CameraAddress } from "~/types/camera";

export default function () {
  const { $fetch } = useNuxtApp();
  const { public: { processor: api } } = useRuntimeConfig();

  const cameras = useState<Camera[]>("cameras", () => []);

  const createCamera = async (payload: { name: string; address: CameraAddress }): Promise<Camera | null> => {
    try {
      const response = await $fetch(
        `${api}/camera`,
        "post",
        JSON.stringify(payload)
      );
      if (response.status !== 200) throw new Error("");

      const result = await response.json();
      cameras.value.push(result);
      return result;
    } catch {
      return null;
    }
  };

  const updateCamera = async (id: string, payload: { name?: string; address?: CameraAddress }): Promise<Camera | null> => {
    try {
      const response = await $fetch(
        `${api}/camera/${id}`,
        "put",
        JSON.stringify(payload)
      );
      if (response.status !== 200) throw new Error("");

      const result = await response.json();
      const index = cameras.value.findIndex(camera => camera.id === id);
      if (index !== -1) {
        cameras.value[index] = result;
      }
      return result;
    } catch {
      return null;
    }
  };

  const deleteCamera = async (id: string): Promise<boolean> => {
    try {
      const response = await $fetch(
        `${api}/camera/${id}`,
        "delete"
      );
      if (response.status !== 204) throw new Error("");

      const index = cameras.value.findIndex(camera => camera.id === id);
      if (index !== -1) {
        cameras.value.splice(index, 1);
      }

      return true;
    } catch {
      return false;
    }
  };

  return {
    cameras,
    createCamera,
    updateCamera,
    deleteCamera,
  };
}
