import { Link } from 'react-router-dom';

const NAVIGATION = [
  { name: 'ダッシュボード', path: '/' },
  { name: '商品管理', path: '/products' },
  { name: '注文管理', path: '/orders' },
];

export const Sidebar: React.FC = () => {
  return (
    <aside>
      <ul>
        {NAVIGATION.map(item => (
          <li key={item.path}>
            <Link to={item.path}>{item.name}</Link>
          </li>
        ))}
      </ul>
    </aside>
  );
};
