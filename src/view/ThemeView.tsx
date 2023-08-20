import GifSearch from "./GifSearch";
import SchemePicker from "./SchemePicker";
import { useThemeViewModel } from "../viewmodel/ThemeViewModel";

export function ThemeView() {
    const viewModel = useThemeViewModel();

    return (
      <div class="flex-row">
        {/* Gif Search */}
        <div class="px-4 pt-4">
          <h1 class="text-red-panda-text text-2xl font-bold mb-4">Gif Picker</h1>
          {GifSearch(viewModel)}
        </div>
  
        {/* Color Scheme Picker */}
        <div class="px-4 pt-4">
          <h1 class="text-red-panda-text text-2xl font-bold mb-4">Color Scheme Picker</h1>
          {SchemePicker(viewModel)}
        </div>
      </div>
    )
  }