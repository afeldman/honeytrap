import { Outlet, Link, useLocation } from "react-router-dom";
import { Activity, Shield, Database, Brain } from "lucide-react";

const navigation = [
    { name: "Dashboard", href: "/dashboard", icon: Activity },
    { name: "Connections", href: "/connections", icon: Shield },
    { name: "Sessions", href: "/sessions", icon: Database },
    { name: "ML Metrics", href: "/ml", icon: Brain },
];

export default function Layout() {
    const location = useLocation();

    return (
        <div className="min-h-screen bg-gray-900">
            {/* Sidebar */}
            <div className="fixed inset-y-0 left-0 z-50 w-64 bg-gray-800 border-r border-gray-700">
                <div className="flex h-16 items-center px-6">
                    <h1 className="text-xl font-bold text-white">
                        🍯 HoneyTrap
                    </h1>
                </div>
                <nav className="mt-6 px-3">
                    {navigation.map((item) => {
                        const isActive = location.pathname === item.href;
                        const Icon = item.icon;
                        return (
                            <Link
                                key={item.name}
                                to={item.href}
                                className={`
                  flex items-center px-3 py-2 mt-1 text-sm font-medium rounded-lg
                  ${
                      isActive
                          ? "bg-gray-700 text-white"
                          : "text-gray-300 hover:bg-gray-700 hover:text-white"
                  }
                `}
                            >
                                <Icon className="w-5 h-5 mr-3" />
                                {item.name}
                            </Link>
                        );
                    })}
                </nav>
            </div>

            {/* Main content */}
            <div className="pl-64">
                <main className="py-6">
                    <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
                        <Outlet />
                    </div>
                </main>
            </div>
        </div>
    );
}
