import { Link, useLocation } from "react-router-dom";
import {
  Sidebar, SidebarContent, SidebarGroup, SidebarGroupContent,
  SidebarGroupLabel, SidebarHeader, SidebarMenu, SidebarMenuButton,
  SidebarMenuItem, SidebarFooter
} from "@/components/ui/sidebar";
import { Badge } from "@/components/ui/badge";
import { useApiStatus, useNavSections, useConfiguredApis } from "@/hooks/use-api-status";
import { Settings, Activity, ShoppingCart } from "lucide-react";

export function SidebarNav() {
  const location = useLocation();
  const sections = useNavSections();
  const configured = useConfiguredApis();
  const { isLoading, isError } = useApiStatus();

  return (
    <Sidebar>
      <SidebarHeader className="p-4">
        <Link to="/" className="flex items-center gap-2 font-semibold text-lg">
          <Activity className="h-6 w-6 text-primary" />
          <span>TMF Portal</span>
        </Link>
        <p className="text-xs text-muted-foreground mt-1">
          {configured.length} API{configured.length !== 1 ? "s" : ""} configured
        </p>
      </SidebarHeader>

      <SidebarContent>
        {isLoading && (
          <SidebarGroup>
            <SidebarGroupContent>
              <SidebarMenu>
                <SidebarMenuItem>
                  <SidebarMenuButton disabled>
                    <span className="text-muted-foreground text-sm">Discovering APIs…</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        )}
        {isError && (
          <SidebarGroup>
            <SidebarGroupContent>
              <SidebarMenu>
                <SidebarMenuItem>
                  <SidebarMenuButton disabled>
                    <span className="text-destructive text-sm">Could not reach backend</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        )}
        {sections.map((section) => (
          <SidebarGroup key={section.domain}>
            <SidebarGroupLabel>{section.domain}</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                {section.apis.map((api) => {
                  const path = `/api/${api.id}`;
                  const active = location.pathname.startsWith(path);
                  return (
                    <SidebarMenuItem key={api.id}>
                      <SidebarMenuButton render={<Link to={path} />} isActive={active}>
                        <span className="font-mono text-xs opacity-60">{api.id.toUpperCase()}</span>
                        <span className="truncate">{api.name}</span>
                      </SidebarMenuButton>
                    </SidebarMenuItem>
                  );
                })}
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        ))}
      </SidebarContent>

      <SidebarFooter className="p-4">
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton render={<Link to="/orders/new" />} isActive={location.pathname === "/orders/new"}>
              <ShoppingCart className="h-4 w-4" />
              <span>Order Builder</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
          <SidebarMenuItem>
            <SidebarMenuButton render={<Link to="/admin" />} isActive={location.pathname === "/admin"}>
              <Settings className="h-4 w-4" />
              <span>Admin</span>
              <Badge variant="outline" className="ml-auto text-xs">
                {configured.length}
              </Badge>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarFooter>
    </Sidebar>
  );
}
