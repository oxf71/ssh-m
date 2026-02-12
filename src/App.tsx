import { Routes, Route } from "react-router-dom";
import { Layout } from "./components/Layout";
import { SshManager } from "./pages/SshManager";
// import { BlockchainAccounts } from "./pages/BlockchainAccounts";
import { SettingsPage } from "./pages/Settings";
import { useTheme } from "./hooks/useTheme";

function App() {
  // Initialize theme on app mount
  useTheme();

  return (
    <Routes>
      <Route element={<Layout />}>
        <Route path="/" element={<SshManager />} />
        {/* <Route path="/blockchain" element={<BlockchainAccounts />} /> */}
        <Route path="/settings" element={<SettingsPage />} />
      </Route>
    </Routes>
  );
}

export default App;
