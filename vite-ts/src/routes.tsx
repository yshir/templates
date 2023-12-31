import { createBrowserRouter } from 'react-router-dom';

import { Layout } from './components/layout';
import { OrderListPage } from './modules/order/pages/list.page';
import { ProductListPage } from './modules/product/pages/list.page';
import { HomePage } from './pages/home.page';

export const router = createBrowserRouter([
  {
    path: '/',
    element: <Layout />,
    children: [
      { path: '/', element: <HomePage /> },
      { path: '/products', element: <ProductListPage /> },
      { path: '/orders', element: <OrderListPage /> },
    ],
  },
]);
