import type { NextConfig } from "next";
require("dotenv").config();

const nextConfig: NextConfig = {
  /* config options here */
  env: {
    NEXT_BASE_URL_API: process.env.NEXT_BASE_URL_API
  }
};

export default nextConfig;
