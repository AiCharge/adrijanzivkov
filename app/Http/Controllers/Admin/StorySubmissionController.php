<?php

namespace App\Http\Controllers\Admin;

use App\Enums\StorySubmissionStatus;
use App\Http\Controllers\Controller;
use App\Http\Requests\Admin\UpdateStorySubmissionRequest;
use App\Models\StorySubmission;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Http\RedirectResponse;
use Illuminate\Http\Request;
use Inertia\Inertia;
use Inertia\Response;

class StorySubmissionController extends Controller
{
    public function index(Request $request): Response
    {
        $search = $request->string('search')->trim()->value();
        $status = $request->enum('status', StorySubmissionStatus::class);

        $storySubmissions = StorySubmission::query()
            ->when($search !== '', fn (Builder $query) => $query->whereLike('name', "%{$search}%"))
            ->when($status, fn (Builder $query) => $query->where('status', $status))
            ->latest()
            ->paginate(15)
            ->withQueryString();

        return Inertia::render('Admin/StorySubmissions/Index', [
            'storySubmissions' => $storySubmissions,
            'statuses' => StorySubmissionStatus::options(),
            'counts' => [
                'all' => StorySubmission::count(),
                ...collect(StorySubmissionStatus::cases())->mapWithKeys(fn (StorySubmissionStatus $status): array => [
                    $status->value => StorySubmission::query()->where('status', $status)->count(),
                ]),
            ],
            'filters' => [
                'search' => $search,
                'status' => $status?->value ?? '',
            ],
        ]);
    }

    public function edit(StorySubmission $storySubmission): Response
    {
        return Inertia::render('Admin/StorySubmissions/Edit', [
            'storySubmission' => $storySubmission,
            'statuses' => StorySubmissionStatus::options(),
        ]);
    }

    public function update(UpdateStorySubmissionRequest $request, StorySubmission $storySubmission): RedirectResponse
    {
        $storySubmission->update($request->validated());

        return redirect()->route('admin.story-submissions.edit', $storySubmission)
            ->with('success', 'Приказната е зачувана.');
    }
}
