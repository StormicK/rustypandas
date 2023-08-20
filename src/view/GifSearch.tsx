import { ThemeViewModel } from "../viewmodel/ThemeViewModel";

export default function GifSearch(viewModel: ThemeViewModel) {
  return (
    <form onSubmit={viewModel.handleGifSearch}>
      <input
        id="search-input"
        class="px-3 py-2 bg-red-panda-accent-3 hover:bg-red-panda-accent-3-dark text-red-panda-text-dark placeholder-red-panda-text-dark mr-2 rounded-md"
        placeholder="Enter a search query..."
        autocomplete="off"
        onInput={(e) => viewModel.setGifSearchQuery(e.target.value)}
      />
      <button
        type="submit"
        class="px-4 py-2 bg-red-panda-accent-2 hover:bg-red-panda-accent-2-dark text-red-panda-text rounded-md">
        Refresh
      </button>
    </form>
  );
}
