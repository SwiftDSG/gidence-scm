export default defineNuxtPlugin(() => {
  let config: RequestInit = {
    mode: "cors",
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
    },
  };
  return {
    provide: {
      setDefaults: (option: RequestInit): void => {
        config = {
          ...config,
          ...option,
          headers: { ...option.headers, "Content-Type": "application/json" },
        };
      },
      fetch: async (
        url: string,
        method: "get" | "post" | "delete" | "put" | "patch",
        body?: string | FormData,
        headers?: RequestInit["headers"]
      ): Promise<Response> => {
        const option: RequestInit = {
          ...config,
          method,
          body,
        };
        if (body && body instanceof FormData && option.headers) {
          const headers = JSON.parse(JSON.stringify(option.headers));
          if (headers["Content-Type"]) delete headers["Content-Type"];
          option.headers = headers;
        } else if (headers) {
          option.headers = {
            ...option.headers,
            ...headers,
          };
        }

        let response = await fetch(url, option);

        return response;
      },
    },
  };
});
