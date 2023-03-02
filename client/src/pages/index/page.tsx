import { useEffect, useState } from 'react';
import { ProjectList } from '../../components/project-list/component';
import { NewProjectButton } from '../../components/project-list/new-project-button/component';
import { Project } from '../../components/project-list/project/component';
import styles from './style.module.css';

export const Index = () => {
  const [response, setResponse] = useState('');
  const [projects, setProjects]: [string[], Function] = useState([]);

  useEffect(() => {
    const getResponse = async () => {
      const value = await fetch('/api/user', {
        method: 'GET',
        mode: 'cors',
        headers: { 'Access-Control-Allow-Origin': '*' },
        credentials: 'include',
      });
      const valueText = await value.text();
      setResponse(valueText);
    };

    const getProjects = async () => {
      const value = await fetch('/api/project/list', {
        method: 'GET',
        mode: 'cors',
        credentials: 'include',
      });
      console.log(`Value: ${value.status}`);
      const valueText = await value.text();
      console.log(`Value Text: ${valueText}`);
      const valueArray: string[] = await JSON.parse(valueText);
      // const valueJson = await value.json();
      // console.log(`valueJson: ${valueJson}`);
      if (value.status == 200) setProjects(valueArray);
    };

    getResponse();
    getProjects();
  }, []);

  return (
    <div className={styles.container}>
      <ProjectList>
        {projects.length == 0 ? (
          <p>You have no projects</p>
        ) : (
          projects.map((projectTitle) => <Project name={projectTitle} />)
        )}
      </ProjectList>
      <NewProjectButton />
    </div>
  );
};
