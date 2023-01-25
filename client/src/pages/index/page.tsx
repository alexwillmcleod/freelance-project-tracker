import { useEffect, useState } from 'react';

export const Index = () => {
  const [response, setResponse] = useState('');

  useEffect(() => {
    const getResponse = async () => {
      const value = await fetch('https://freelance-api.fly.dev/', {
        method: 'GET',
        mode: 'cors',
        headers: { 'Access-Control-Allow-Origin': '*' },
        credentials: 'include',
      });
      const valueText = await value.text();
      setResponse(valueText);
    };
    getResponse();
  }, []);

  return (
    <div>
      <p>{response}</p>
    </div>
  );
};
