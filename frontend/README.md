# 🎮 ArenaX Frontend

## Overview

The ArenaX frontend is a **Next.js Progressive Web App (PWA)** designed for mobile-first gaming tournament experiences. It provides seamless user interaction with Stellar wallet integration for blockchain-based payouts and rewards.

## Tech Stack

- **Framework**: Next.js (PWA)
- **Styling**: TailwindCSS
- **UI Components**: shadcn/ui
- **Blockchain**: Stellar wallet integration
- **State Management**: React Context/Redux (TBD)
- **Real-time**: WebSocket/Server-Sent Events

## Key Features

### 🏗️ Core Functionality
- **Mobile-First PWA**: Optimized for mobile gaming experiences
- **Stellar Wallet Integration**: Seamless blockchain-based payouts and rewards
- **Real-time Updates**: Live tournament and match updates
- **Offline Support**: PWA capabilities for better user experience

### 🎮 Gaming Features
- **Tournament Browsing**: View and join tournaments with Stellar prize pools
- **Match Management**: Report scores, upload proof, view match history
- **Leaderboards**: Real-time rankings with Stellar Reputation Token balances
- **Wallet Management**: View balances, transaction history, and initiate payouts

### 🔐 Authentication & Security
- **Phone-based OTP**: Secure login/signup with rate limiting
- **Device Fingerprinting**: Prevent multi-account abuse
- **Stellar Account Linking**: Each user linked to a Stellar wallet

## Project Structure

```
frontend/
├── src/
│   ├── components/          # Reusable UI components
│   │   ├── ui/             # shadcn/ui components
│   │   ├── tournament/     # Tournament-specific components
│   │   ├── wallet/         # Wallet and Stellar components
│   │   └── common/         # Shared components
│   ├── pages/              # Next.js pages
│   │   ├── api/            # API routes
│   │   ├── tournaments/    # Tournament pages
│   │   ├── wallet/         # Wallet pages
│   │   └── auth/           # Authentication pages
│   ├── hooks/              # Custom React hooks
│   │   ├── useStellar.ts   # Stellar wallet hooks
│   │   ├── useTournament.ts # Tournament hooks
│   │   └── useWallet.ts    # Wallet hooks
│   ├── context/            # React context providers
│   │   ├── AuthContext.tsx # Authentication context
│   │   ├── WalletContext.tsx # Wallet context
│   │   └── TournamentContext.tsx # Tournament context
│   ├── services/           # API and Stellar integration
│   │   ├── api.ts          # Backend API client
│   │   ├── stellar.ts      # Stellar SDK integration
│   │   └── websocket.ts    # WebSocket connections
│   ├── utils/              # Utility functions
│   │   ├── stellar.ts      # Stellar utilities
│   │   ├── validation.ts   # Input validation
│   │   └── constants.ts    # App constants
│   └── styles/             # Global styles and Tailwind config
│       ├── globals.css     # Global styles
│       └── tailwind.config.js # Tailwind configuration
├── public/                 # Static assets
│   ├── icons/             # PWA icons
│   ├── manifest.json      # PWA manifest
│   └── sw.js              # Service worker
├── package.json           # Dependencies and scripts
├── yarn.lock              # Yarn lock file
├── next.config.js         # Next.js configuration
├── tailwind.config.js     # Tailwind CSS config
├── tsconfig.json          # TypeScript configuration
├── .env.example           # Environment variables template
└── README.md              # This documentation
```

## Setup & Development

### Prerequisites
- Node.js (v18+)
- Yarn (recommended package manager)
- Stellar testnet account (for development)

### Expected Package.json Structure

```json
{
  "name": "arenax-frontend",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "lint": "next lint",
    "lint:fix": "next lint --fix",
    "type-check": "tsc --noEmit",
    "test": "jest",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage",
    "test:e2e": "playwright test",
    "format": "prettier --write .",
    "format:check": "prettier --check .",
    "analyze": "cross-env ANALYZE=true next build",
    "build:pwa": "next build && next export",
    "test:stellar": "jest --testPathPattern=stellar",
    "deploy:testnet": "yarn build && vercel --env=testnet",
    "deploy:mainnet": "yarn build && vercel --env=production"
  },
  "dependencies": {
    "next": "^14.0.0",
    "react": "^18.0.0",
    "react-dom": "^18.0.0",
    "@stellar/stellar-sdk": "^11.0.0",
    "@tanstack/react-query": "^5.0.0",
    "zustand": "^4.0.0",
    "tailwindcss": "^3.0.0",
    "@radix-ui/react-*": "^1.0.0",
    "class-variance-authority": "^0.7.0",
    "clsx": "^2.0.0",
    "tailwind-merge": "^2.0.0"
  },
  "devDependencies": {
    "@types/node": "^20.0.0",
    "@types/react": "^18.0.0",
    "@types/react-dom": "^18.0.0",
    "typescript": "^5.0.0",
    "eslint": "^8.0.0",
    "eslint-config-next": "^14.0.0",
    "prettier": "^3.0.0",
    "jest": "^29.0.0",
    "@testing-library/react": "^14.0.0",
    "@testing-library/jest-dom": "^6.0.0",
    "playwright": "^1.40.0"
  }
}
```

### Installation

```bash
# Clone the repository
git clone https://github.com/arenax/arenax.git
cd frontend

# Install dependencies with yarn
yarn install

# Set up environment variables
cp .env.example .env.local
# Edit .env.local with your configuration

# Start development server
yarn dev
```

### Yarn Commands

```bash
# Development
yarn dev              # Start development server
yarn build            # Build for production
yarn start            # Start production server
yarn lint             # Run ESLint
yarn lint:fix         # Fix ESLint issues
yarn type-check       # Run TypeScript type checking

# Testing
yarn test             # Run tests
yarn test:watch       # Run tests in watch mode
yarn test:coverage    # Run tests with coverage
yarn test:e2e         # Run end-to-end tests

# Code Quality
yarn format           # Format code with Prettier
yarn format:check     # Check code formatting
yarn analyze          # Analyze bundle size

# Dependencies
yarn add <package>    # Add new dependency
yarn add -D <package> # Add dev dependency
yarn remove <package> # Remove dependency
yarn upgrade          # Upgrade all dependencies
yarn upgrade <package> # Upgrade specific package

# PWA & Stellar
yarn build:pwa        # Build PWA-optimized version
yarn test:stellar     # Test Stellar integration
yarn deploy:testnet   # Deploy to testnet
yarn deploy:mainnet   # Deploy to mainnet
```

### Environment Variables

```env
# API Configuration
NEXT_PUBLIC_API_URL=http://localhost:8080
NEXT_PUBLIC_WS_URL=ws://localhost:8080

# Stellar Configuration
NEXT_PUBLIC_STELLAR_NETWORK=testnet
NEXT_PUBLIC_STELLAR_HORIZON_URL=https://horizon-testnet.stellar.org

# Payment Gateways
NEXT_PUBLIC_PAYSTACK_PUBLIC_KEY=pk_test_xxx
NEXT_PUBLIC_FLUTTERWAVE_PUBLIC_KEY=FLWPUBK_TEST-xxx
```

## Key Components

### 🏆 Tournament Components
- `TournamentList`: Browse available tournaments
- `TournamentCard`: Individual tournament display
- `TournamentDetails`: Detailed tournament view with Stellar prize pool
- `JoinTournament`: Tournament registration with payment options

### 🎮 Match Components
- `MatchQueue`: Real-time matchmaking queue
- `MatchInterface`: Live match management
- `ScoreReporting`: Submit scores with proof upload
- `MatchHistory`: Past match results and Stellar transactions

### 💰 Wallet Components
- `WalletDashboard`: Balance overview and transaction history
- `StellarWallet`: Stellar account management and payouts
- `PaymentMethods`: Fiat deposit/withdrawal options
- `TransactionHistory`: Detailed transaction logs

### 🏅 Leaderboard Components
- `Leaderboard`: Real-time player rankings
- `ReputationDisplay`: Stellar Reputation Token balances
- `PlayerProfile`: Individual player statistics

## Stellar Integration

### Wallet Management
- **Account Creation**: Automatic Stellar account creation for new users
- **Balance Display**: Real-time XLM and ArenaX Token balances
- **Transaction History**: Stellar transaction logs and status
- **Payout Processing**: Initiate and track Stellar-based payouts

### Custom Assets
- **ArenaX Tokens**: In-platform rewards and tournament entry fees
- **Reputation Tokens**: Player fairness and skill tracking
- **Prize Pools**: Transparent tournament prize management

## Development Guidelines

### Code Standards
- Use TypeScript for type safety
- Follow Next.js best practices
- Implement responsive design with TailwindCSS
- Use shadcn/ui components for consistency

### Testing
- Unit tests with Jest and React Testing Library
- Integration tests for Stellar wallet functionality
- E2E tests with Playwright (planned)

### Performance
- Optimize for mobile performance
- Implement proper caching strategies
- Use Next.js Image optimization
- Minimize bundle size with code splitting

## Deployment

### Production Build
```bash
# Build for production
yarn build

# Start production server
yarn start

# Build PWA-optimized version
yarn build:pwa
```

### PWA Deployment
- Configure service worker for offline support
- Set up proper caching strategies
- Ensure HTTPS for PWA functionality
- Use `yarn build:pwa` for optimized PWA build

## API Integration

The frontend communicates with the ArenaX backend API for:
- User authentication and profile management
- Tournament and match data
- Wallet and payment operations
- Real-time updates via WebSocket

## Stellar Network Integration

- **Testnet**: Development and testing
- **Mainnet**: Production deployment
- **Horizon API**: Stellar network interaction
- **Soroban Contracts**: Smart contract integration

## Contributing

1. Follow the established code style
2. Write tests for new features
3. Update documentation as needed
4. Test Stellar integration thoroughly
5. Ensure mobile responsiveness

## Support

For questions about the frontend implementation:
- Check the main ArenaX documentation
- Review Stellar integration guides
- Contact the development team

---

**Note**: This frontend is designed to work seamlessly with the ArenaX backend and Stellar blockchain integration. Ensure all three components are properly configured for full functionality.
