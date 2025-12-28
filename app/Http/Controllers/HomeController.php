<?php

namespace App\Http\Controllers;

use App\Models\News;
use Inertia\Inertia;

class HomeController extends Controller
{
    public function __invoke()
    {
        $latestNews = News::latest()->take(4)->get();

        return Inertia::render('Home', [
            'latestNews' => $latestNews,
        ]);
    }
}
