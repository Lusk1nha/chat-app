interface EnvironmentVariables {
  NEXT_BASE_URL_API: string;
  NEXT_ENV: string;
}

export function getEnvironments(): EnvironmentVariables {
  return {
    NEXT_BASE_URL_API: process.env.NEXT_BASE_URL_API as string,
    NEXT_ENV: process.env.NEXT_ENV as string
  };
}
