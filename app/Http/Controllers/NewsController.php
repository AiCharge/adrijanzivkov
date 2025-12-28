<?php

namespace App\Http\Controllers;

use App\Models\News;
use Inertia\Inertia;

class NewsController extends Controller
{
    public function show(News $news): \Inertia\Response
    {
        $imageUrl = $news->main_image_url;

        return Inertia::render('News/Show', [
            'news' => $news,
        ])->withViewData([
            'meta' => [
                'title' => $news->title,
                'description' => $news->short_description,
                'image' => $imageUrl,
                'width' => 1200,
                'height' => 630,
            ],
        ]);
    }
}
