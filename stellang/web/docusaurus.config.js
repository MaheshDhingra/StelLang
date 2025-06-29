// @ts-check
// `@type` JSDoc annotations allow editor autocompletion and type checking
// (when paired with `@ts-check`).
// There are various equivalent ways to declare your Docusaurus config.
// See: https://docusaurus.io/docs/api/docusaurus-config

import { themes as prismThemes } from "prism-react-renderer";

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: "Stel",
  tagline: "Stel the Programming Language",
  favicon: "img/favicon.ico",

  // @ts-ignore
  title: "Welcome to Stel!",
  // @ts-ignore
  tagline: "The New Programming Language🦀",
  // @ts-ignore
  favicon: "img/Stel.png",

  // Set the production url of your site here
  url: "https://Stel-docs.vercel.app/",
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: "/",

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: "Stel-lang", // Usually your GitHub org/user name.
  projectName: "Stel-docs", // Usually your repo name.

  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },

  themes: ["@docusaurus/theme-live-codeblock"],

  presets: [
    [
      "classic",
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: "./sidebars.js",
          editUrl: "https://github.com/Kazooki123/Stel-docs/tree/main/docs",
        },
        blog: {
          showReadingTime: true,
          editUrl: "https://github.com/Kazooki123/Stel-docs/tree/main/docs",
        },
        theme: {
          customCss: "./src/css/custom.css",
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      // Replace with your project's social card
      image: "img/Stel-social-card.jpg",
      navbar: {
        title: "Stel",
        logo: {
          alt: "Stel Logo",
          src: "img/Stel.png",
          // @ts-ignore
          src: "img/Stel.png",
        },
        items: [
          {
            type: "docSidebar",
            sidebarId: "tutorialSidebar",
            position: "left",
            label: "Tutorial",
          },
          { to: "/blog", label: "Blog", position: "left" },
          { to: "/community", label: "Community", position: "left" },
          {
            href: "https://github.com/Stel-lang/Stel",
            label: "GitHub",
            position: "right",
          },
          {
            href: "https://discord.gg/BX7uDaab",
            label: "Discord",
            position: "right",
          },
        ],
      },
      footer: {
        style: "dark",
        links: [
          {
            title: "Docs",
            items: [
              {
                label: "Introduction",
                to: "/docs/",
              },
            ],
          },
          {
            title: "Community",
            items: [
              {
                label: "Stack Overflow",
                href: "https://stackoverflow.com/questions/tagged/Stel",
              },
              {
                label: "Discord",
                href: "https://discord.gg/BX7uDaab",
              },
              {
                label: "Twitter",
                href: "https://twitter.com/Stelofficial1",
              },
            ],
          },
          {
            title: "More",
            items: [
              {
                label: "Blog",
                to: "/blog",
              },
              {
                label: "GitHub",
                href: "https://github.com/Stel-lang/Stel",
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} Stel. Built with Docusaurus.`,
      },
      prism: {
        theme: prismThemes.github,
        darkTheme: prismThemes.dracula,
      },
    }),

  plugins: [
    function myPlugin(context, options) {
      return {
        name: "custom-webpack-plugin",
        configureWebpack(config, isServer, utils) {
          return {
            resolve: {
              fallback: {
                child_process: false,
                path: require.resolve("path-browserify"),
              },
            },
          };
        },
      };
    },
  ],
};


export default config;
