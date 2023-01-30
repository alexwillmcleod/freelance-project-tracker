import React from 'react';
import ReactDOM from 'react-dom/client';
import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import './index.css';
import { Root } from './pages/root/page';
import { SignUp } from './pages/sign-up/page';
import { Index } from './pages/index/page';
import { SignIn } from './pages/sign-in/page';

const router = createBrowserRouter([
  {
    path: '/',
    element: <Root />,
    children: [
      {
        index: true,
        path: '',
        element: <Index />,
      },
      {
        index: true,
        path: 'sign-up',
        element: <SignUp />,
      },
      {
        index: true,
        path: 'sign-in',
        element: <SignIn />,
      },
    ],
  },
]);

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
);
