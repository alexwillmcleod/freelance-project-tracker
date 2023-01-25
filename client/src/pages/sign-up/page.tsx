import styles from './style.module.css';
import { Account, AccountInput } from '../../components/account/component';
import { useState } from 'react';
import { redirect } from 'react-router';

type UserDetails = {
  email: string;
  password: string;
  first_name: string;
  last_name: string;
};

export const SignUp = () => {
  const [userDetails, setUserDetails] = useState<UserDetails>({
    email: '',
    password: '',
    first_name: '',
    last_name: '',
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

  const handleFirstNameChange = (newValue: string) => {
    setUserDetails({
      ...userDetails,
      first_name: newValue,
    });
  };

  const handleLastNameChange = (newValue: string) => {
    setUserDetails({
      ...userDetails,
      last_name: newValue,
    });
  };

  const handleSubmit = async () => {
    if (
      !userDetails.email ||
      !userDetails.first_name ||
      !userDetails.last_name ||
      !userDetails.password
    )
      return;

    const createAccountResponse = await fetch(
      'https://freelance-api.fly.dev/user/create',
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

    // If account is not successfully created

    console.log(await createAccountResponse.text());
    if (createAccountResponse.status != 201) return;

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
        label="Sign Up"
        onSubmit={handleSubmit}
      >
        <AccountInput
          onChange={handleEmailChange}
          label="Email"
          type="email"
        />
        <AccountInput
          onChange={handleFirstNameChange}
          label="First Name"
        />
        <AccountInput
          onChange={handleLastNameChange}
          label="Last Name"
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
