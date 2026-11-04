// Root ESLint flat config for Kasuwa monorepo
export default [
  {
    ignores: [
      "**/node_modules/**",
      "**/dist/**",
      "**/.turbo/**",
      "**/.next/**",
      "contracts/target/**",
    ],
  },
];
