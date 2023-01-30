import styles from './style.module.css';
import { Account, AccountInput } from '../../components/account/component';
import { useState } from 'react';
import { redirect } from 'react-router';

type UserDetails = {
  email: string;
  password: string;
};

export const SignIn = () => {
  const [userDetails, setUserDetails] = useState<UserDetails>({
    email: '',
    password: '',
  });

  const handleEmailChange = (newValue: string) => {
    setUserDetails({
      ...userDetails,
      email: newValue,
    });
  };

  const handlePasswordChange = (newValue: string) => {
    setUserDetails({
      ...userDetails,
      password: newValue,
    });
  };
  const handleSubmit = async () => {
    if (!userDetails.email || !userDetails.password) return;

    // We are going to try and log in now
    const loginAccountResponse = await fetch(
      'https://freelance-api.fly.dev/user/login',
      {
        method: 'POST',
        mode: 'cors',
        headers: {
          'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify(userDetails),
      }
    );

    // If account is successfully logged into
    console.log(await loginAccountResponse.text());
    if (loginAccountResponse.status != 200) return;

    redirect('/');
  };

  return (
    <div className={styles.container}>
      <Account
        label="Sign In"
        onSubmit={handleSubmit}
      >
        <AccountInput
          onChange={handleEmailChange}
          label="Email"
          type="email"
        />
        <AccountInput
          onChange={handlePasswordChange}
          label="Password"
          type="password"
        />
      </Account>
    </div>
  );
};
