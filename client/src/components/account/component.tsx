import { ChangeEventHandler, ReactNode, useState } from 'react';
import styles from './style.module.css';

type AccountProps = {
  label: string;
  children?: ReactNode;
  onSubmit?: Function;
};

export const Account = (props: AccountProps) => {
  return (
    <div className={styles.container}>
      <h1>{props.label}</h1>
      <div className={styles.form}>{props.children}</div>
      <button
        className={styles['submit-button']}
        onClick={() => {
          if (props.onSubmit) props.onSubmit();
        }}
      >
        Submit
      </button>
    </div>
  );
};

type AccountInputProps = {
  onChange: Function;
  label: string;
  type?: string;
};

export const AccountInput = (props: AccountInputProps) => {
  const [value, setValue] = useState('');

  const handleValueChange = (e: any) => {
    const newValue = e.target.value;
    props.onChange(newValue);
    setValue(newValue);
  };

  return (
    <div className={styles['input-container']}>
      <label
        htmlFor="sign-in-input"
        className={styles.label}
      >
        {' '}
        {props.label}{' '}
      </label>
      <input
        className={styles.input}
        name="sign-in-input"
        value={value}
        type={props.type}
        onChange={handleValueChange}
      />
    </div>
  );
};
