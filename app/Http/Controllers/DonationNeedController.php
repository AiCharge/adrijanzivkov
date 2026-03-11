<?php

namespace App\Http\Controllers;

use App\Models\DonationNeed;
use Inertia\Inertia;

class DonationNeedController extends Controller
{
    public function index()
    {
        $donationNeeds = DonationNeed::latest()->paginate(9);

        return Inertia::render('DonationNeeds/Index', compact('donationNeeds'))
            ->withViewData([
                'meta' => [
                    'title' => 'Потреба за донации',
                    'description' => 'Погледнете ги тековните потреби за донации на Фондацијата „Адријан Живков" и помогнете таму каде е најпотребно.',
                    'image' => url('/images/meta/default.jpg'),
                ],
            ]);
    }

    public function show(DonationNeed $donationNeed)
    {
        $plainDescription = strip_tags($donationNeed->description);
        $excerpt = mb_strimwidth($plainDescription, 0, 160, '...');

        return Inertia::render('DonationNeeds/Show', compact('donationNeed'))
            ->withViewData([
                'meta' => [
                    'title' => $donationNeed->title,
                    'description' => $excerpt,
                    'image' => $donationNeed->image_url,
                ],
            ]);
    }
}
