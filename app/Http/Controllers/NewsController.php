<?php

namespace App\Http\Controllers;

use App\Models\News;
use Inertia\Inertia;
use Inertia\Response;

class NewsController extends Controller
{
    public function index(): Response
    {
        $news = News::latest()
            ->paginate(12);

        return Inertia::render('News/Index', [
            'news' => $news,
        ]);
    }

    public function show(News $news): Response
    {
        $imageUrl = $news->main_image_url;

        return Inertia::render('News/Show', [
            'news' => $news,
        ])->withViewData([
            'meta' => [
                'title' => $news->title,
                'description' => $news->short_description,
                'image' => $imageUrl,
            ],
        ]);
    }
}
