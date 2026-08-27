import tseslint from '@typescript-eslint/eslint-plugin';
import parser from '@typescript-eslint/parser';

export default [{
  files: ['src/**/*.ts', 'tests/**/*.ts', 'playwright.config.ts'],
  languageOptions: { parser, parserOptions: { ecmaVersion: 2022, sourceType: 'module' } },
  plugins: { '@typescript-eslint': tseslint },
  rules: {
    '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
    '@typescript-eslint/consistent-type-imports': 'error'
  }
}];
