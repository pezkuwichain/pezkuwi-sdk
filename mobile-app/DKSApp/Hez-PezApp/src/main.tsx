import { createRoot } from 'react-dom/client'
import App from './App.tsx'
import { AuthProvider } from './contexts/AuthContext'
import { WalletProvider } from './contexts/WalletContext'
import { WebSocketProvider } from './contexts/WebSocketContext'
import './index.css'
import './i18n/config'

// Add window.ethereum type declaration
declare global {
  interface Window {
    ethereum?: any;
  }
}

// Remove dark mode class addition
createRoot(document.getElementById("root")!).render(
  <AuthProvider>
    <WalletProvider>
      <WebSocketProvider>
        <App />
      </WebSocketProvider>
    </WalletProvider>
  </AuthProvider>
);