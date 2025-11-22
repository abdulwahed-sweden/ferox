# Ferox Dashboard Frontend

React + TypeScript dashboard for the Ferox C2 framework.

## Tech Stack

- **React 18** - UI framework
- **TypeScript** - Type safety
- **Tailwind CSS** - Styling
- **Zustand** - State management
- **React Query** - Data fetching
- **Cytoscape.js** - Network visualization
- **Lucide React** - Icons
- **react-hot-toast** - Notifications
- **Vite** - Build tool

## Quick Start

```bash
# Install dependencies
npm install

# Start development server
npm run dev
# Opens at http://localhost:5173

# Build for production
npm run build
# Output to dist/
```

## Project Structure

```
src/
├── App.tsx              # Root component
├── main.tsx             # Entry point
├── index.css            # Global styles
├── components/
│   ├── Layout.tsx       # Main layout with sidebar
│   ├── Skeleton.tsx     # Loading components
│   └── ErrorBoundary.tsx
├── pages/
│   ├── Dashboard.tsx    # Overview with stats
│   ├── Sessions.tsx     # Session management
│   ├── Terminal.tsx     # Command execution
│   ├── Network.tsx      # Network graph
│   ├── Credentials.tsx  # Credential vault
│   ├── Mitre.tsx        # ATT&CK matrix
│   └── Reports.tsx      # Report generation
├── hooks/
│   ├── useWebSocket.ts  # WebSocket connection
│   ├── useApi.ts        # API client
│   └── useDebounce.ts   # Utility hooks
├── store/
│   └── index.ts         # Zustand store
└── types/
    └── index.ts         # TypeScript types
```

## Features

- Real-time session management via WebSocket
- Interactive command terminal with history
- Network topology visualization with Cytoscape
- Two-column credential vault with intelligence
- MITRE ATT&CK coverage heat map
- Report generation
- Toast notifications for events
- Dark theme with Ferox green (#00ff88)

## Development

```bash
# Type checking
npx tsc --noEmit

# Lint (if eslint configured)
npm run lint

# Preview production build
npm run preview
```

## Environment

The frontend expects the backend API at the same host:
- API: `http://localhost:8080/api/*`
- WebSocket: `ws://localhost:8080/ws`

In development with Vite proxy:
- Frontend: `http://localhost:5173`
- Proxied to backend: `http://localhost:8080`

## Color Scheme

| Color | Hex | Usage |
|-------|-----|-------|
| Ferox Green | `#00ff88` | Primary, success |
| Dark 900 | `#0a0e27` | Main background |
| Dark 700 | `#1a1f3a` | Cards |
| Danger | `#ff3366` | Errors, critical |
| Warning | `#ffaa00` | Warnings |
| Info | `#00ccff` | Information |

## License

MIT
