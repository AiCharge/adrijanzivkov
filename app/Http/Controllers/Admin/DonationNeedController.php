<?php

namespace App\Http\Controllers\Admin;

use App\Http\Controllers\Controller;
use App\Http\Requests\Admin\StoreDonationNeedRequest;
use App\Http\Requests\Admin\UpdateDonationNeedRequest;
use App\Models\DonationNeed;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Http\RedirectResponse;
use Illuminate\Http\Request;
use Inertia\Inertia;
use Inertia\Response;

class DonationNeedController extends Controller
{
    public function index(Request $request): Response
    {
        $search = $request->string('search')->trim()->value();
        $archived = $request->string('archived')->value();

        $donationNeeds = DonationNeed::query()
            ->when($search !== '', fn (Builder $query) => $query->whereLike('title', "%{$search}%"))
            ->when($archived === 'archived', fn (Builder $query) => $query->whereNotNull('archived_at'))
            ->when($archived === 'active', fn (Builder $query) => $query->whereNull('archived_at'))
            ->latest()
            ->paginate(15)
            ->withQueryString();

        return Inertia::render('Admin/DonationNeeds/Index', [
            'donationNeeds' => $donationNeeds,
            'counts' => [
                'all' => DonationNeed::count(),
                'active' => DonationNeed::query()->whereNull('archived_at')->count(),
                'archived' => DonationNeed::query()->whereNotNull('archived_at')->count(),
            ],
            'filters' => [
                'search' => $search,
                'archived' => in_array($archived, ['archived', 'active'], true) ? $archived : '',
            ],
        ]);
    }

    public function create(): Response
    {
        return Inertia::render('Admin/DonationNeeds/Create');
    }

    public function store(StoreDonationNeedRequest $request): RedirectResponse
    {
        DonationNeed::create([
            ...$request->safe()->except('image'),
            'image' => $request->file('image')->store('donation-needs'),
        ]);

        return redirect()->route('admin.donation-needs.index')
            ->with('success', 'Донациската потреба е креирана.');
    }

    public function edit(DonationNeed $donationNeed): Response
    {
        return Inertia::render('Admin/DonationNeeds/Edit', [
            'donationNeed' => $donationNeed,
        ]);
    }

    public function update(UpdateDonationNeedRequest $request, DonationNeed $donationNeed): RedirectResponse
    {
        $donationNeed->fill($request->safe()->except('image'));

        if ($request->hasFile('image')) {
            $donationNeed->image = $request->file('image')->store('donation-needs');
        }

        $donationNeed->save();

        return redirect()->route('admin.donation-needs.edit', $donationNeed)
            ->with('success', 'Донациската потреба е зачувана.');
    }
}
