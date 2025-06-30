// @ts-check
// `@type` JSDoc annotations allow editor autocompletion and type checking
// (when paired with `@ts-check`).
// There are various equivalent ways to declare your Docusaurus config.
// See: https://docusaurus.io/docs/api/docusaurus-config

import { themes as prismThemes } from "prism-react-renderer";

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: "stel",
  tagline: "stel the Programming Language",
  favicon: "img/favicon.ico",

  // @ts-ignore
  title: "Welcome to stel!",
  // @ts-ignore
  tagline: "The New Programming Language🦀",
  // @ts-ignore
  favicon: "img/stel.png",

  // Set the production url of your site here
  url: "https://stel-docs.vercel.app/",
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: "/",

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: "StelLang", // Usually your GitHub org/user name.
  projectName: "stel-docs", // Usually your repo name.

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
          editUrl: "https://github.com/MaheshDhingra/StelLang/tree/main/stellang/web/docs",
        },
        blog: {
          showReadingTime: true,
          editUrl: "https://github.com/MaheshDhingra/StelLang/tree/main/stellang/web/docs",
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
      image: "img/stel-social-card.jpg",
      navbar: {
        title: "stel",
        logo: {
          alt: "stel Logo",
          src: "img/stel.png",
          // @ts-ignore
          src: "img/stel.png",
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
            href: "https://github.com/MaheshDhingra/StelLang",
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
                href: "https://stackoverflow.com/questions/tagged/stellang",
              },
              {
                label: "Discord",
                href: "https://discord.gg/W4vJzEJb2C",
              },
              {
                label: "Twitter",
                href: "https://twitter.com/stelofficial",
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
                href: "https://github.com/MaheshDhingra/StelLang",
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} stel. Built with Docusaurus.`,
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
