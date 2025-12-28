<!DOCTYPE html>
<html lang="{{ str_replace('_', '-', app()->getLocale()) }}">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">

    @php
        $title = ($meta['title'] ?? 'Во вечен спомен на Адријан Живков') . ' | adrijanzivkov.org';
        $description = $meta['description'] ?? 'Фондацијата „Адријан Живков“ е посветена на поддршка на семејствата, донации за деца и хуманитарни активности во заедницата.';
        $url = url()->full();
        if (!isset($meta['image'])) {
            $image = url()->to('/') . '/images/meta/default.jpg';
        } else {
            $image = $meta['image'];
        }
    @endphp

    <title inertia>{{ config('app.name', 'Fondacija Adrijan Zhivkov') }}</title>

    <meta name="description" content="{{ $description }}">
    <meta property="og:type" content="website" />
    <meta property="og:url" content="{{ $url }}" />
    <meta property="og:title" content="{{ $title }}" />
    <meta property="og:description" content="{{ $description }}" />
    <meta property="og:image" content="{{ $image }}" />
    <meta property="og:image:secure_url" content="{{ $image }}" />
    <meta property="og:site_name" content="adrijanzivkov.org" />
    <meta name="twitter:url" content="{{ $url }}" />
    <meta name="twitter:title" content="{{ $title }}" />
    <meta name="twitter:description" content="{{ $description }}" />
    <meta name="twitter:image" content="{{ $image }}" />
    <meta name="twitter:card" content="summary_large_image">

    <link rel="icon" href="/favicon.ico" sizes="any">
    <link rel="icon" href="/favicon.svg" type="image/svg+xml">
    <link rel="apple-touch-icon" href="/apple-touch-icon.png">

    <link rel="preconnect" href="https://fonts.bunny.net">
    <link href="https://fonts.bunny.net/css?family=instrument-sans:400,500,600" rel="stylesheet" />

    @routes
    @vite(['resources/js/app.ts', "resources/js/pages/{$page['component']}.vue"])
    @inertiaHead
</head>
<body class="font-sans antialiased">
@inertia
</body>
</html>
