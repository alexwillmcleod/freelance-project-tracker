import styles from './style.module.css';
import { ReactNode } from 'react';

type ProjectProps = {
  name: string;
};

export function Project(props: ProjectProps) {
  return (
    <div className={styles['project-div']}>
      <p className={styles['project-title']}>{props.name}</p>
    </div>
  );
}
