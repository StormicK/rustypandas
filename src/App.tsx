import { Routes, Route } from "@solidjs/router";

import SideNavigation, { NavItemProps } from "./view/SideNavigation";
import { ThemeView } from "./view/ThemeView";

const navigationItems: NavItemProps[] = [
    { link: "/", text: "Theme" },
    //{ link: "/worktime", text: "Work Time" },
];

function WorkTime() {
  return (
    <div>
      ** WORK IN PROGRESS **
    </div>
  )
}

export default function App() {
  return (
    <div class="bg-red-panda-background min-h-screen flex">
      {/* Side Navigation */} 
      <div class="bg-red-panda-background-dark flex flex-col align-top">
        <SideNavigation navItemProps={navigationItems} />
      </div>
      <Routes>
        <Route path="/" element={ <ThemeView></ThemeView> } />
        <Route path="/worktime" element={ <WorkTime></WorkTime> } />
      </Routes>
    </div>
  );
}