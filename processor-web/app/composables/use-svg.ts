export default () => {
  const svgs = useState<Record<string, string>>("svgs", () => ({}));

  const loadSvg = async (name: string): Promise<string> => {
    if (svgs.value[name]) {
      return svgs.value[name];
    }
    try {
      const response = await fetch(`/icons/${name}.svg`);
      svgs.value[name] = await response.text();
      return svgs.value[name];
    } catch (e) {
      return name;
    }
  };

  return { loadSvg };
};
