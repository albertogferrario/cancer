@import '@appolabs/ui/styles/variables.css';

@tailwind base;
@tailwind components;
@tailwind utilities;

:root {
  /* Primary color - indigo (customize to your brand) */
  --primary: 239 84% 67%;
  --primary-foreground: 0 0% 100%;
  --ring: 239 84% 67%;

  /* Sidebar styling */
  --sidebar-primary: 239 84% 67%;
  --sidebar-primary-foreground: 0 0% 100%;
  --sidebar-accent: 239 30% 95%;
  --sidebar-ring: 239 84% 67%;

  /* Chart colors */
  --chart-1: 239 84% 67%;
  --chart-2: 172 66% 50%;
  --chart-3: 38 92% 50%;
  --chart-4: 350 89% 60%;
  --chart-5: 280 84% 60%;

  /* Mesh gradient colors for decorative backgrounds */
  --mesh-primary: 239 84% 67%;
  --mesh-secondary: 172 66% 50%;
  --mesh-accent: 280 84% 60%;
}

.dark {
  /* Primary color - lighter for dark mode */
  --primary: 239 84% 75%;
  --primary-foreground: 0 0% 100%;
  --ring: 239 84% 75%;

  /* Sidebar styling */
  --sidebar-primary: 239 84% 75%;
  --sidebar-primary-foreground: 240 10% 4%;
  --sidebar-accent: 239 40% 15%;
  --sidebar-ring: 239 84% 75%;

  /* Chart colors */
  --chart-1: 239 84% 75%;
}

@layer base {
  * {
    @apply border-border;
  }

  body {
    @apply bg-background text-foreground antialiased;
  }
}

@layer utilities {
  .text-balance {
    text-wrap: balance;
  }
}
