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
        $donationNeeds = DonationNeed::notArchived()->latest()->take(6)->get();

        return Inertia::render('Home', [
            'latestNews' => $latestNews,
            'donationNeeds' => $donationNeeds,
        ])->withViewData([
            'meta' => [
                'title' => 'Почетна',
                'description' => 'Фондацијата „Адријан Живков" е посветена на поддршка на семејствата, донации за деца и хуманитарни активности во заедницата.',
                'image' => url('/images/meta/default.jpg'),
            ],
        ]);
    }
}
