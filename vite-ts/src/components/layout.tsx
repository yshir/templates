import { Outlet } from 'react-router-dom';

import { Sidebar } from './sidebar';

export const Layout: React.FC = () => {
  return (
    <div>
      <p>Layout</p>
      <aside>
        <Sidebar />
      </aside>
      <main>
        <Outlet />
      </main>
    </div>
  );
};
