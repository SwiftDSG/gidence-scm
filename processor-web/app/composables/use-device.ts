import type { Device } from "~/types/general";

export default function () {
  const { $fetch } = useNuxtApp();
  const { public: { processor: api } } = useRuntimeConfig();

  const { processor } = useProcessor();
  const { cameras } = useCamera();

  const device = useState<Device | null>('device', () => null)

  const getDevice = async (): Promise<Device | null> => {
    try {
      const response = await $fetch(
        `${api}/device`,
        "get"
      );
      if (response.status !== 200) throw new Error("");

      const result: Device = await response.json();

      processor.value = result.processor;
      cameras.value = result.camera;

      return result;
    } catch {
      return null;
    }
  };

  return {
    device,
    getDevice,
  };
}
