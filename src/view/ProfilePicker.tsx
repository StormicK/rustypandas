import { ThemeViewModel } from "../viewmodel/ThemeViewModel";

export default function ProfilePicker(viewModel: ThemeViewModel) {
  return (
    <div class="mt-4">
      <select
        value={viewModel.selectedProfile()}
        onChange={viewModel.handleProfileChange}
        class="block w-full px-2 py-2 bg-red-panda-accent-3 hover:bg-red-panda-accent-3-dark rounded-md focus:border-red-panda-link"
      >
        {viewModel.profiles()?.map((profile: string) => (
          <option value={profile} class="text-red-panda-text-dark hover:bg-red-panda-accent-2">
            {profile}
          </option>
        ))}
      </select>
    </div>
  );
}
