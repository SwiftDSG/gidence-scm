import type { Evidence } from "~/types/evidence";

export default function () {
  const { $fetch } = useNuxtApp();
  const { public: { processor: api } } = useRuntimeConfig();

  const evidences = useState<Evidence[]>('evidences', () => []);

  const getEvidences = async (camera_id: string): Promise<Evidence[]> => {
    try {
      const response = await $fetch(
        `${api}/camera/${camera_id}/evidences`,
        "get",
      );
      if (response.status !== 200) throw new Error("");

      const result = await response.json();

      evidences.value = result;
      return result;
    } catch {
      return [];
    }
  };


  return {
    evidences,
    getEvidences
  };
}
