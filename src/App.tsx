import { createSignal, createResource } from "solid-js";
//import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

const GifSearch = () => {
  const [searchQuery, setSearchQuery] = createSignal("");

  return <form onSubmit={async () => await invoke("update_gif", { search_query: searchQuery })}>
    <input
      id="search-input"
      placeholder="Enter a name..."
      autocomplete="off"
      onInput={(e) => setSearchQuery(e.target.value)}
      />
    <button type="submit">Refresh</button>
  </form>
}

//create function to fetch color schemes
async function fetchColorSchemes(): Promise<string[]>  {
  return await invoke("get_color_schemes")
}

const ColorSchemePicker = () => {
  const [selectedValue, setSelectedValue] = createSignal<string>("unloaded");

  const [colorSchemesResource, loadColorSchemes] = createResource(fetchColorSchemes);

  const handleChange = async (event: Event) => {
    const target = event.target as HTMLSelectElement;
    const selectedName = target.value;
    setSelectedValue(selectedName);
    await invoke("update_color_scheme", { color_scheme: selectedName });
  };

  loadColorSchemes.refetch();

  return (
    <div>
      <select value={selectedValue()} onChange={handleChange}>
        {colorSchemesResource()?.map((scheme: string) => (
          <option value={scheme}>
            {scheme}
          </option>
        ))}
      </select>
    </div>
  );
}

function App() {
  return (
    <div class="container">
      <h1>Gif Picker</h1>
      <GifSearch></GifSearch>
      <h1>Color Scheme Picker</h1>
      <ColorSchemePicker></ColorSchemePicker>
    </div>
  );
}

export default App;
