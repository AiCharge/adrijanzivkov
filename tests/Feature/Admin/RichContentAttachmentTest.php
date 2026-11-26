<?php

use App\Models\User;
use Illuminate\Http\UploadedFile;
use Illuminate\Support\Facades\Storage;

beforeEach(function () {
    Storage::fake();
});

it('uploads an image for rich content and returns its url', function () {
    $response = $this->actingAs(User::factory()->create())
        ->postJson(route('admin.rich-content-attachments.store'), [
            'file' => UploadedFile::fake()->image('inline.png'),
        ])
        ->assertCreated()
        ->assertJsonStructure(['url']);

    expect(Storage::allFiles('rich-content'))->toHaveCount(1)
        ->and($response->json('url'))->toBe(Storage::url(Storage::allFiles('rich-content')[0]));
});

it('rejects files that are not images', function () {
    $this->actingAs(User::factory()->create())
        ->postJson(route('admin.rich-content-attachments.store'), [
            'file' => UploadedFile::fake()->create('script.php', 1, 'text/x-php'),
        ])
        ->assertUnprocessable()
        ->assertJsonValidationErrors('file');

    expect(Storage::allFiles('rich-content'))->toBeEmpty();
});

it('does not let guests upload', function () {
    $this->postJson(route('admin.rich-content-attachments.store'), [
        'file' => UploadedFile::fake()->image('inline.png'),
    ])->assertUnauthorized();
});
