import type { Menu, State, Theme, View } from "~/types/general";

export default function () {
  const view = useState<View>("view", () => "large");
  const rem = useState<number>("rem", () => 24);
  const init = useState<boolean>("init", () => true);
  const theme = useState<Theme>("theme", () => "light");
  const state = useState<State>("state", () => "idle");
  const menus = useState<Menu[]>("menus", () => []);

  const getTheme = (): Theme => {
    const stored = localStorage?.getItem("theme") || "";
    if (stored === "light" || stored === "dark") {
      theme.value = stored;
    } else if (window?.matchMedia("prefers-color-scheme: dark").matches) {
      theme.value = "dark";
    }
    return theme.value;
  };
  const setTheme = (data: Theme): Theme => {
    localStorage.setItem("theme", data);
    theme.value = data;
    return theme.value;
  };

  const openMenu = (menu: Menu): void => {
    menus.value.push(menu);
  };
  const closeMenu = (): void => {
    menus.value.pop();
  }

  return {
    rem,
    view,
    init,
    theme,
    state,
    menus,
    getTheme,
    setTheme,
    openMenu,
    closeMenu,
  };
}
