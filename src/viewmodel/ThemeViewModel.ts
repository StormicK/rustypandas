import { Resource, createResource, createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";

async function fetchColorSchemes(): Promise<string[]> {
  return await invoke("get_color_schemes");
}

async function fetchCurrentColorSchemes(): Promise<string> {
  return await invoke("get_current_color_scheme");
}

async function fetchProfiles(): Promise<string[]> {
  return await invoke("get_profiles");
}

async function fetchCurrentProfile(): Promise<string> {
  return await invoke("get_current_profile");
}

export type ThemeViewModel = {
  gifSearchQuery: () => string;
  setGifSearchQuery: (query: string) => void;
  handleGifSearch: () => Promise<void>;
  selectedColorScheme: () => string;
  handleColorSchemeChange: (event: Event) => Promise<void>;
  colorSchemes: Resource<string[]>;
  refreshColorSchemes: () => void;
  selectedProfile: () => string;
  handleProfileChange: (event: Event) => Promise<void>;
  profiles: Resource<string[]>;
  refreshProfiles: () => void;
};

export function useThemeViewModel(): ThemeViewModel {
  const [gifSearchQuery, setGifSearchQuery] = createSignal("");
  const [selectedColorScheme, setSelectedColorScheme] = createSignal<string>("unloaded");
  const [selectedProfile, setSelectedProfile] = createSignal<string>("unloaded");
  const [colorSchemesResource, loadColorSchemes] = createResource(fetchColorSchemes);
  const [profilesResource, loadProfiles] = createResource(fetchProfiles);

  const handleGifSearch = async () => {
    await invoke("update_gif", { search_query: gifSearchQuery() });
  };

  const fetchAndSetCurrentColorScheme = async () => {
    const currentColorScheme = await fetchCurrentColorSchemes();
    setSelectedColorScheme(currentColorScheme);
  };

  const handleColorSchemeChange = async (event: Event) => {
    const target = event.target as HTMLSelectElement;
    const selectedName = target.value;
    setSelectedColorScheme(selectedName);
    await invoke("update_color_scheme", { color_scheme: selectedName });
  };

  const fetchAndSetCurrentProfile = async () => {
    const currentProfile = await fetchCurrentProfile();
    setSelectedProfile(currentProfile);
  };

  const handleProfileChange = async (event: Event) => {
    const target = event.target as HTMLSelectElement;
    const selectedName = target.value;
    setSelectedProfile(selectedName);
    await invoke("set_current_profile", { profile: selectedName });
    fetchAndSetCurrentColorScheme();
  };

  loadColorSchemes.refetch();
  fetchAndSetCurrentColorScheme();
  loadProfiles.refetch();
  fetchAndSetCurrentProfile();

  return {
    gifSearchQuery,
    setGifSearchQuery,
    handleGifSearch,
    selectedColorScheme,
    handleColorSchemeChange,
    colorSchemes: colorSchemesResource,
    refreshColorSchemes: loadColorSchemes.refetch,
    selectedProfile,
    handleProfileChange,
    profiles: profilesResource,
    refreshProfiles: loadProfiles.refetch,
  };
}
