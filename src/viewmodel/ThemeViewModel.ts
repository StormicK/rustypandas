import { createResource, createSignal } from "solid-js";
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
  colorSchemes: string[] | undefined;
  refreshColorSchemes: () => void;
};

export function useThemeViewModel(): ThemeViewModel {
  const [searchQuery, setSearchQuery] = createSignal("");
  const [selectedScheme, setSelectedValue] = createSignal<string>("unloaded");
  const [schemesResource, loadColorSchemes] = createResource(fetchColorSchemes);

  const handleGifSearch = async () => {
    await invoke("update_gif", { search_query: searchQuery() });
  };

  const handleSchemeChange = async (event: Event) => {
    const target = event.target as HTMLSelectElement;
    const selectedName = target.value;
    setSelectedValue(selectedName);
    await invoke("update_color_scheme", { color_scheme: selectedName });
  };

  loadColorSchemes.refetch();

  return {
    gifSearchQuery: searchQuery,
    setGifSearchQuery: setSearchQuery,
    handleGifSearch,
    selectedColorScheme: selectedScheme,
    handleColorSchemeChange: handleSchemeChange,
    colorSchemes: schemesResource(),
    refreshColorSchemes: loadColorSchemes.refetch,
  };
}
