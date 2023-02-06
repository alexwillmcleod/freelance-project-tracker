import { useEffect, useState } from 'react';

export const Index = () => {
  const [response, setResponse] = useState('');
  const [projects, setProjects] = useState('');

  useEffect(() => {
    const getResponse = async () => {
      const value = await fetch('/api/', {
        method: 'GET',
        mode: 'cors',
        headers: { 'Access-Control-Allow-Origin': '*' },
        credentials: 'include',
      });
      const valueText = await value.text();
      setResponse(valueText);
    };

    const getProjects = async () => {
      const value = await fetch('api/project/list', {
        method: 'GET',
        mode: 'cors',
        credentials: 'include',
      });
      console.log(`Value: ${value.status}`);
      const valueText = await value.text();
      console.log(`Value Text: ${valueText}`);
      // const valueJson = await value.json();
      // console.log(`valueJson: ${valueJson}`);
      if (value.status == 200) setProjects(`Your Projects: ${valueText}`);
    };

    getResponse();
    getProjects();
  }, []);

  return (
    <div>
      <p>{response}</p>
      <p>{projects}</p>
    </div>
  );
};
