/* eslint-env node */

const config = {
  root: true,
  env: {
    browser: true,
    node: true,
    es2022: true,
  },
  extends: [
    "eslint:recommended",
    "plugin:import/recommended",
    "plugin:react/recommended",
    "plugin:react/jsx-runtime",
    "plugin:react-hooks/recommended",
    "plugin:jsx-a11y/recommended",
    "plugin:tailwindcss/recommended",
    "prettier",
  ],
  parserOptions: {
    ecmaVersion: "latest",
    sourceType: "module",
    ecmaFeatures: {
      jsx: true,
    },
  },
  plugins: ["react-refresh"],
  rules: {
    // eslint
    // "arrow-body-style": "warn",
    // "sort-imports": ["warn", { "ignoreDeclarationSort": true }],
    "object-shorthand": "warn",

    // eslint-plugin-import
    "import/order": [
      "warn",
      {
        groups: [
          "builtin",
          "external",
          "internal",
          "parent",
          "sibling",
          "index",
          "object",
          "type",
        ],
        "newlines-between": "always",
        alphabetize: {
          order: "asc",
          caseInsensitive: true,
        },
      },
    ],

    // eslint-plugin-react
    // "react/destructuring-assignment": "warn",
    "react/prop-types": "off",
    "react/no-unstable-nested-components": ["warn", { allowAsProps: true }],

    "react-refresh/only-export-components": [
      "warn",
      { allowConstantExport: true },
    ],

    // tailwindcss
    "tailwindcss/no-custom-classname": "off",
  },
  settings: {
    react: {
      version: "detect",
    },
    "import/parsers": { "@typescript-eslint/parser": [".ts", ".tsx", ".d.ts"] },
    "import/resolver": {
      node: {
        extensions: [".js", ".jsx", ".ts", ".tsx"],
      },
      typescript: {
        alwaysTryTypes: true,
      },
    },
  },
  overrides: [
    {
      files: ["**/*.ts?(x)"],
      extends: [
        // "plugin:@typescript-eslint/recommended",
        "plugin:@typescript-eslint/stylistic",
        // "plugin:@typescript-eslint/strict",
        "plugin:@typescript-eslint/recommended-type-checked",
        // "plugin:@typescript-eslint/strict-type-checked",
        // "plugin:@typescript-eslint/stylistic-type-checked",
        "plugin:import/typescript",
      ],
      parser: "@typescript-eslint/parser",
      parserOptions: {
        ecmaVersion: "latest",
        sourceType: "module",
        project: true,
        tsconfigRootDir: __dirname,
      },
      plugins: ["@typescript-eslint"],
      rules: {
        // @typescript-eslint
        "@typescript-eslint/no-unused-vars": "warn",
        "@typescript-eslint/prefer-optional-chain": "warn",

        // @typescript-eslint with type checking
        "@typescript-eslint/no-floating-promises": "off",
        "@typescript-eslint/no-misused-promises": "off",
        /*"@typescript-eslint/await-thenable": "warn",
        "@typescript-eslint/require-await": "warn",
        "@typescript-eslint/consistent-type-exports": "warn",
        "@typescript-eslint/no-unnecessary-boolean-literal-compare": "warn",
        "@typescript-eslint/no-unnecessary-condition": "warn",
        "@typescript-eslint/no-unnecessary-type-assertion": "warn",
        "@typescript-eslint/no-unsafe-return": "warn",
        "@typescript-eslint/prefer-includes": "warn",
        "@typescript-eslint/prefer-nullish-coalescing": "warn",
        "@typescript-eslint/unbound-method": "warn",
        "@typescript-eslint/no-inferrable-types": "warn",*/
      },
    },
  ],
};

module.exports = config;
