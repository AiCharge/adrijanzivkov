<?php

use App\Enums\StorySubmissionStatus;
use App\Models\StorySubmission;
use App\Models\User;
use Inertia\Testing\AssertableInertia as Assert;

beforeEach(function () {
    $this->actingAs(User::factory()->create());
});

function createStorySubmission(array $attributes = []): StorySubmission
{
    return StorySubmission::create([
        'name' => 'Ана Петрова',
        'phone' => '+389 70 000 000',
        'message' => 'Ни треба помош.',
        ...$attributes,
    ]);
}

it('lists story submissions with the status options', function () {
    createStorySubmission();

    $this->get(route('admin.story-submissions.index'))
        ->assertSuccessful()
        ->assertInertia(fn (Assert $page) => $page
            ->component('Admin/StorySubmissions/Index')
            ->has('storySubmissions.data', 1)
            ->where('storySubmissions.data.0.status', 'new')
            ->has('statuses', 4)
            ->where('statuses.0', ['value' => 'new', 'label' => 'Ново', 'color' => 'info']));
});

it('filters story submissions by status', function () {
    createStorySubmission();
    createStorySubmission(['status' => StorySubmissionStatus::Accepted]);

    $this->get(route('admin.story-submissions.index', ['status' => 'accepted']))
        ->assertInertia(fn (Assert $page) => $page
            ->has('storySubmissions.data', 1)
            ->where('filters.status', 'accepted')
            ->where('counts', ['all' => 2, 'new' => 1, 'accepted' => 1, 'rejected' => 0, 'completed' => 0]));
});

it('ignores an unknown status filter', function () {
    createStorySubmission();

    $this->get(route('admin.story-submissions.index', ['status' => 'bogus']))
        ->assertInertia(fn (Assert $page) => $page
            ->has('storySubmissions.data', 1)
            ->where('filters.status', ''));
});

it('searches story submissions by name', function () {
    createStorySubmission();
    createStorySubmission(['name' => 'Марко Марковски']);

    $this->get(route('admin.story-submissions.index', ['search' => 'Марко']))
        ->assertInertia(fn (Assert $page) => $page->has('storySubmissions.data', 1));
});

it('shows a story submission', function () {
    $submission = createStorySubmission();

    $this->get(route('admin.story-submissions.edit', $submission))
        ->assertInertia(fn (Assert $page) => $page
            ->component('Admin/StorySubmissions/Edit')
            ->where('storySubmission.message', 'Ни треба помош.'));
});

it('updates only the status and notes', function () {
    $submission = createStorySubmission();

    $this->put(route('admin.story-submissions.update', $submission), [
        'status' => 'completed',
        'notes' => 'Контактирано.',
        'name' => 'Променето',
        'message' => 'Променето',
    ])
        ->assertRedirect(route('admin.story-submissions.edit', $submission))
        ->assertSessionHas('success');

    expect($submission->fresh())
        ->status->toBe(StorySubmissionStatus::Completed)
        ->notes->toBe('Контактирано.')
        ->name->toBe('Ана Петрова')
        ->message->toBe('Ни треба помош.');
});

it('rejects an invalid status', function () {
    $submission = createStorySubmission();

    $this->put(route('admin.story-submissions.update', $submission), ['status' => 'bogus'])
        ->assertSessionHasErrors('status');

    expect($submission->fresh()->status)->toBe(StorySubmissionStatus::New);
});
