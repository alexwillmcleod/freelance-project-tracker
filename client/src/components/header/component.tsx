import styles from './style.module.css';
import { ReactNode } from 'react';

type HeaderProps = {
  children?: ReactNode;
};

export function Header(props: HeaderProps) {
  return <div className={styles.header}>{props.children}</div>;
}

type HeaderNavProps = {
  children?: ReactNode;
};

export function HeaderNav(props: HeaderNavProps) {
  return <div className={styles['header-nav']}>{props.children}</div>;
}

type HeaderLinkProps = {
  children: string;
  href?: string;
  target?: string;
};

export function HeaderLink(props: HeaderLinkProps) {
  return (
    <a
      className={styles['header-link']}
      href={props.href}
      target={props.target}
    >
      {props.children}
    </a>
  );
}

type HeaderButtonProps = {
  children: string;
  href?: string;
  target?: string;
};

export function HeaderButton(props: HeaderButtonProps) {
  return (
    <a
      className={styles['header-button']}
      href={props.href}
      target={props.target}
    >
      {props.children}
    </a>
  );
}

type HeaderTitleProps = {
  children?: string;
};

export function HeaderTitle(props: HeaderTitleProps) {
  return (
    <a
      className={styles['header-title']}
      href="/"
    >
      {props.children}
    </a>
  );
}
