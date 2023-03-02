import styles from './style.module.css';
import {
  Header,
  HeaderLink,
  HeaderNav,
  HeaderButton,
  HeaderTitle,
} from '../../components/header/component';

import { ReactNode, useEffect, useState } from 'react';
import { Outlet } from 'react-router-dom';
import defaultAvatarLink from '/public/default-user-icon.jpg';

export const Root = () => {
  const [user, setUser] = useState(null);
  const [avatarLink, setAvatarLink] = useState(defaultAvatarLink);

  useEffect(() => {
    const getUser = async () => {
      const value = await fetch('/api/user', {
        method: 'GET',
        mode: 'cors',
        credentials: 'include',
      });
      const valueText = await value.text();
      const valueArray = await JSON.parse(valueText);
      // const valueJson = await value.json();
      // console.log(`valueJson: ${valueJson}`);
      if (value.status == 200) setUser(valueArray);
      if (valueArray.avatar != null) {
        setAvatarLink(valueArray.avatar);
      }
    };
    getUser();
  });

  return (
    <div className={styles.container}>
      <Header>
        <HeaderNav>
          <HeaderTitle>Freelance</HeaderTitle>
        </HeaderNav>
        <HeaderNav>
          {user == null ? (
            <>
              <HeaderButton href="/sign-in">Sign In</HeaderButton>
              <HeaderButton href="/sign-up">Sign Up</HeaderButton>
            </>
          ) : (
            <img
              className={styles.avatar}
              src={avatarLink}
            ></img>
          )}
        </HeaderNav>
      </Header>
      <Outlet />
    </div>
  );
};
