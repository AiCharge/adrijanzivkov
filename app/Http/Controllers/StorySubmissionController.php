<?php

namespace App\Http\Controllers;

use App\Http\Requests\StoreStorySubmissionRequest;
use App\Models\StorySubmission;
use Inertia\Inertia;
use Inertia\Response;

class StorySubmissionController extends Controller
{
    public function create(): Response
    {
        return Inertia::render('StorySubmission');
    }

    public function store(StoreStorySubmissionRequest $request): \Illuminate\Http\RedirectResponse
    {
        StorySubmission::create($request->safe()->except('agreed'));

        return redirect()->route('story-submissions.create')
            ->with('success', 'Вашата порака е успешно испратена. Ќе ве контактираме наскоро.');
    }
}
