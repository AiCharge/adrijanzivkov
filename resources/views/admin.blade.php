<!DOCTYPE html>
<html lang="mk">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover">
    <meta name="robots" content="noindex, nofollow">

    {{-- Applies the saved light/dark choice (or the system's) before the page draws, so it never flashes the other one. --}}
    <script>
        (function () {
            let mode = 'auto';

            try {
                mode = localStorage.getItem('vueuse-color-scheme') ?? 'auto';
            } catch {
                // Storage can be blocked; the system setting still applies.
            }

            const isDark = mode === 'dark' || (mode === 'auto' && window.matchMedia('(prefers-color-scheme: dark)').matches);

            document.documentElement.classList.toggle('dark', isDark);
        })();
    </script>
    <style>
        html {
            background-color: oklch(0.982 0.004 160);
            color-scheme: light;
        }

        html.dark {
            background-color: oklch(0.165 0.015 160);
            color-scheme: dark;
        }
    </style>

    <title inertia>{{ config('app.name', 'Fondacija Adrijan Zhivkov') }}</title>

    <link rel="icon" href="/favicon.ico" sizes="any">
    <link rel="icon" href="/favicon.svg" type="image/svg+xml">
    <link rel="apple-touch-icon" href="/apple-touch-icon.png">

    @vite(['resources/js/admin.ts', "resources/js/pages/{$page['component']}.vue"])
    @inertiaHead
</head>
<body class="font-sans antialiased">
@inertia
</body>
</html>
