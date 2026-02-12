import type { Reading } from "~/types/general";

export default function () {
  const { $fetch } = useNuxtApp();
  const { public: { processor: api } } = useRuntimeConfig();

  const reading = useState<Reading>("reading", () => ({
    camera: {},
  }));

  const readerOnline = useState<boolean>("readerOnline", () => false);
  const readerFail = useState<number>("readerFail", () => 0);

  const read = async (): Promise<boolean> => {
    try {
      const response = await $fetch(`${api}/reading`, "get");
      if (response.status !== 200) throw new Error("");

      const result = await response.json();
      reading.value = result;

      return true;
    } catch {
      return false;
    }
  };

  const readerStart = async () => {
    if (await read()) {
      readerFail.value = 0;
      readerOnline.value = true;
    } else {
      readerFail.value += 1;
      if (readerFail.value === 5) {
        readerOnline.value = false;
      }
    }

    setInterval(async () => {
      if (!await read()) {
        readerFail.value += 1;
        if (readerFail.value === 5) {
          readerOnline.value = false;
        }
      } else {
        readerFail.value = 0;
        readerOnline.value = true;
      }
    }, 5000);
  }

  return {
    reading,
    readerOnline,
    readerStart
  };
}
