<?php

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Models\DonationNeed;
use Illuminate\Http\RedirectResponse;

class DonationNeedArchiveController extends Controller
{
    public function store(DonationNeed $donationNeed): RedirectResponse
    {
        $donationNeed->update(['archived_at' => now()]);

        return back()->with('success', 'Донациската потреба е архивирана.');
    }

    public function destroy(DonationNeed $donationNeed): RedirectResponse
    {
        $donationNeed->update(['archived_at' => null]);

        return back()->with('success', 'Донациската потреба е вратена.');
    }
}
