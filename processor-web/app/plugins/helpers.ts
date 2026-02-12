export default defineNuxtPlugin(() => {
  return {
    provide: {
      formatDateTime: (x: string | number): string => {
        const date = new Date(x);
        return `${date.getDate().toString().padStart(2, "0")}-${(
          date.getMonth() + 1
        )
          .toString()
          .padStart(2, "0")}-${date.getFullYear()} ${date
            .getHours()
            .toString()
            .padStart(2, "0")}:${date.getMinutes().toString().padStart(2, "0")}`;
      },
      formatDate: (x: string | number): string => {
        const date = new Date(x);
        return `${date.getDate().toString().padStart(2, "0")}-${(
          date.getMonth() + 1
        )
          .toString()
          .padStart(2, "0")}-${date.getFullYear()}`;
      },
      formatTime: (x: string | number): string => {
        const date = new Date(x);
        return `${date.getHours().toString().padStart(2, "0")}:${date
          .getMinutes()
          .toString()
          .padStart(2, "0")}`;
      },
      copyText: (x: string) => {
        navigator.clipboard.writeText(x);
      }
    },
  };
});
