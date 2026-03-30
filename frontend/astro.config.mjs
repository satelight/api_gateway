// @ts-check
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

// https://astro.build/config
export default defineConfig({
  integrations: [
    starlight({
      title: "NOK API GateWay Docs",
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/withastro/starlight",
        },
      ],
      sidebar: [
        {
          label: "NOK API GateWay",
          items: [
            // Each item here is one entry in the navigation menu.
            { label: "About", slug: "about/about" },
          ],
        },
        {
          label: "Open AI API",
          items: [
            // Each item here is one entry in the navigation menu.
            { label: "Example Guide", slug: "openai_api/explanation" },
          ],
        },
        {
          label: "Gemini API",
          autogenerate: { directory: "gemini_api" },
        },
      ],
    }),
  ],
});
