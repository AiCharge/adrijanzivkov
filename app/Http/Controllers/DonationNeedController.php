<?php

namespace App\Http\Controllers;

use App\Models\DonationNeed;
use Inertia\Inertia;

class DonationNeedController extends Controller
{
    public function index()
    {
        $donationNeeds = DonationNeed::latest()->paginate(9);

        return Inertia::render('DonationNeeds/Index', compact('donationNeeds'));
    }

    public function show(DonationNeed $donationNeed)
    {
        return Inertia::render('DonationNeeds/Show', compact('donationNeed'));
    }
}
