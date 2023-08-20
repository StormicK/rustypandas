import { JSX } from "solid-js";

export type NavItemProps = {
  link: string;
  text: string;
};

export function NavItem(props: NavItemProps): JSX.Element {
  return (
    <a href={props.link} class="block">
      <li class="px-4 py-2 bg-red-panda-accent-2 hover:bg-red-panda-accent-2-dark border-b border-red-panda-accent-2-dark">
        <span class="text-red-panda-text">{props.text}</span>
      </li>
    </a>
  );
}

export type SideNavigationProps = {
  navItemProps: NavItemProps[];
};

export default function SideNavigation(props: SideNavigationProps): JSX.Element {
  return (
    <nav>
      <ul>
        {props.navItemProps.map((item) => (
          <NavItem link={item.link} text={item.text} />
        ))}
      </ul>
    </nav>
  );
}
