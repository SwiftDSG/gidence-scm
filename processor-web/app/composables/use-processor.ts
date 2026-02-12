import type { Processor } from "~/types/processor";

export default function () {
  const { $fetch } = useNuxtApp();
  const { public: { processor: api } } = useRuntimeConfig();

  const processor = useState<Processor | null>("processor", () => null);

  const updateProcessor = async (payload: Partial<Processor>): Promise<Processor | null> => {
    try {
      const response = await $fetch(
        `${api}/processor`,
        "put",
        JSON.stringify(payload)
      );
      if (response.status !== 200) throw new Error("");

      const result = await response.json();
      processor.value = result;
      return result;
    } catch {
      return null;
    }
  };


  return {
    processor,
    updateProcessor,
  };
}
