import {
  Header,
  HeaderLink,
  HeaderNav,
  HeaderButton,
  HeaderTitle,
} from '../../components/header/component';

import { ReactNode } from 'react';
import { Outlet } from 'react-router-dom';

export const Root = () => {
  return (
    <div>
      <Header>
        <HeaderNav>
          <HeaderTitle>Freelance</HeaderTitle>
        </HeaderNav>
        <HeaderNav>
          <HeaderButton href="/sign-in">Sign In</HeaderButton>
          <HeaderButton href="/sign-up">Sign Up</HeaderButton>
        </HeaderNav>
      </Header>

      <Outlet />
    </div>
  );
};
