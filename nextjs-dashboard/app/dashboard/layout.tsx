// Import the sidebar (SideNav)
import SideNav from "@/app/ui/dashboard/sidenav";

// The <Layout /> component receives a `children` prop. This children can be a page or another layout.
export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex h-screen flex-col md:flex-row md:overflow-hidden">
      {/* Divide the body in two columns */}
      {/* Left column is the SideNav */}
      <div className="w-full flex-none md:w-64">
        <SideNav />
      </div>

      {/* The right one is the children */}
      <div className="flex-grow p-6 md:overflow-y-auto md:p-12">{children}</div>
    </div>
  )
}
