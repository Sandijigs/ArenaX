import withPWA from "next-pwa";

const pwaConfig = {
  dest: "public",
  register: true,
  skipWaiting: true,
  disable: process.env.NODE_ENV === "development",
  buildExcludes: [/app-build-manifest\.json$/],
  sw: "sw.js",
};

const nextConfig = withPWA(pwaConfig)({
  // ...other Next.js config options
  fallbacks: {
    document: "/offline", // Fallback for page requests
    output: "export",
  },
});

export default nextConfig;
