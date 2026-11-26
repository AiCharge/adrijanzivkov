<?php

use App\Enums\StorySubmissionStatus;
use App\Models\StorySubmission;
use Illuminate\Foundation\Testing\RefreshDatabase;

uses(RefreshDatabase::class);

it('stores a story submitted from the public form as new', function () {
    $this->post(route('story-submissions.store'), [
        'name' => 'Ана Петрова',
        'phone' => '+389 70 000 000',
        'message' => 'Ни треба помош.',
        'agreed' => true,
    ])
        ->assertRedirect(route('story-submissions.create'))
        ->assertSessionHas('success');

    expect(StorySubmission::sole())
        ->name->toBe('Ана Петрова')
        ->status->toBe(StorySubmissionStatus::New);
});
