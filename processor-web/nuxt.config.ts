// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: false },
  ssr: false,
  runtimeConfig: {
    public: {
      processor: process.env.PROCESSOR_URL || "http://localhost:8000",
      base: process.env.BASE_URL || "http://localhost:3000",
    }
  },
  app: {
    head: {
      title: "SCM Processor — Safety Compliance Monitoring",
      link: [
        { rel: 'icon', type: 'image/svg+xml', href: '/favicon.svg' }
      ],
      script: [
        { src: "/theme.js" }
      ]
    }
  }
})
