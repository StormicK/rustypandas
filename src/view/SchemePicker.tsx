import { ThemeViewModel } from "../viewmodel/ThemeViewModel";

export default function SchemePicker(viewModel: ThemeViewModel) {
  return (
    <div class="mt-4">
      <select
        value={viewModel.selectedColorScheme()}
        onChange={viewModel.handleColorSchemeChange}
        class="block w-full px-2 py-2 bg-red-panda-accent-3 hover:bg-red-panda-accent-3-dark rounded-md focus:border-red-panda-link"
      >
        {viewModel.colorSchemes?.map((scheme: string) => (
          <option value={scheme} class="text-red-panda-text-dark hover:bg-red-panda-accent-2">
            {scheme}
          </option>
        ))}
      </select>
    </div>
  );
}
