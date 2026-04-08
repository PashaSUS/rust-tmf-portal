import { Outlet } from "react-router-dom";
import { SidebarProvider, SidebarInset, SidebarTrigger } from "@/components/ui/sidebar";
import { SidebarNav } from "./sidebar-nav";
import { Separator } from "@/components/ui/separator";
import { Toaster } from "@/components/ui/sonner";
import { Sun, Moon } from "lucide-react";
import { useTheme } from "./theme-provider";
import { Button } from "@/components/ui/button";

export function AppShell() {
  const { theme, setTheme } = useTheme();

  return (
    <SidebarProvider>
      <SidebarNav />
      <SidebarInset>
        <header className="flex h-14 items-center gap-2 border-b px-4">
          <SidebarTrigger />
          <Separator orientation="vertical" className="h-6" />
          <div className="flex-1" />
          <Button
            variant="ghost"
            size="icon"
            onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
          >
            {theme === "dark" ? <Sun className="h-4 w-4" /> : <Moon className="h-4 w-4" />}
          </Button>
        </header>
        <main className="flex-1 p-6">
          <Outlet />
        </main>
      </SidebarInset>
      <Toaster />
    </SidebarProvider>
  );
}
