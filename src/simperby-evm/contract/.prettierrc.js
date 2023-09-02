module.exports = {
  singleQuote: false,
  semi: true,
  overrides: [
    {
      files: "*.sol",
      options: {
        printWidth: 100,
        tabWidth: 4,
        singleQuote: false,
        bracketSpacing: false,
        endOfLine: "lf",
      },
    },
  ],
};
