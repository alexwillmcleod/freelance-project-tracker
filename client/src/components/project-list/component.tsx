import styles from './style.module.css';
import { ReactNode } from 'react';
import { Project } from './project/component';

type ProjectListProps = {
  children?: ReactNode;
};

export function ProjectList(props: ProjectListProps) {
  return <div className={styles['project-list-div']}>{props.children}</div>;
}
