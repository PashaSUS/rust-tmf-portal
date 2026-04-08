import { Routes, Route, Navigate } from "react-router-dom";
import { AppShell } from "@/components/layout/app-shell";
import { lazy, Suspense } from "react";
import { LoadingState } from "@/components/shared/loading-state";

const AdminPage = lazy(() => import("@/pages/admin"));
const ApiBrowserPage = lazy(() => import("@/pages/api/browser"));
const ResourceDetailPage = lazy(() => import("@/pages/api/detail"));
const OrderBuilderPage = lazy(() => import("@/pages/orders/builder"));

function Fallback() {
  return <div className="flex items-center justify-center h-64"><LoadingState /></div>;
}

export default function App() {
  return (
    <Routes>
      <Route element={<AppShell />}>
        <Route index element={<Navigate to="/admin" replace />} />
        <Route path="admin" element={<Suspense fallback={<Fallback />}><AdminPage /></Suspense>} />
        <Route path="orders/new" element={<Suspense fallback={<Fallback />}><OrderBuilderPage /></Suspense>} />
        <Route path="api/:apiId" element={<Suspense fallback={<Fallback />}><ApiBrowserPage /></Suspense>} />
        <Route path="api/:apiId/:resource/:id" element={<Suspense fallback={<Fallback />}><ResourceDetailPage /></Suspense>} />
      </Route>
    </Routes>
  );
}