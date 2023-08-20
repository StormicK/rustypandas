import { Resource, createResource, createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";

async function fetchColorSchemes(): Promise<string[]> {
  return await invoke("get_color_schemes");
}

export type ThemeViewModel = {
  gifSearchQuery: () => string;
  setGifSearchQuery: (query: string) => void;
  handleGifSearch: () => Promise<void>;
  selectedColorScheme: () => string;
  handleColorSchemeChange: (event: Event) => Promise<void>;
  colorSchemes: Resource<string[]>;
  refreshColorSchemes: () => void;
};

export function useThemeViewModel(): ThemeViewModel {
  const [gifSearchQuery, setGifSearchQuery] = createSignal("");
  const [selectedColorScheme, setSelectedValue] = createSignal<string>("unloaded");
  const [colorSchemesResource, loadColorSchemes] = createResource(fetchColorSchemes);

  const handleGifSearch = async () => {
    await invoke("update_gif", { search_query: gifSearchQuery() });
  };

  const handleColorSchemeChange = async (event: Event) => {
    const target = event.target as HTMLSelectElement;
    const selectedName = target.value;
    setSelectedValue(selectedName);
    await invoke("update_color_scheme", { color_scheme: selectedName });
  };

  loadColorSchemes.refetch();

  return {
    gifSearchQuery,
    setGifSearchQuery,
    handleGifSearch,
    selectedColorScheme,
    handleColorSchemeChange,
    colorSchemes: colorSchemesResource,
    refreshColorSchemes: loadColorSchemes.refetch,
  };
}
