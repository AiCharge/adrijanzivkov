<?php

namespace App\Http\Controllers;

use App\Models\DonationNeed;
use App\Models\News;
use Inertia\Inertia;

class HomeController extends Controller
{
    public function __invoke()
    {
        $latestNews = News::latest()->take(3)->get();
        $donationNeeds = DonationNeed::latest()->take(6)->get();

        return Inertia::render('Home', [
            'latestNews' => $latestNews,
            'donationNeeds' => $donationNeeds,
        ]);
    }
}
