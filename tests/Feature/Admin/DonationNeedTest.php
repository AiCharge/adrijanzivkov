<?php

use App\Models\DonationNeed;
use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Illuminate\Http\UploadedFile;
use Illuminate\Support\Facades\Storage;
use Inertia\Testing\AssertableInertia as Assert;

uses(RefreshDatabase::class);

beforeEach(function () {
    Storage::fake();
    $this->actingAs(User::factory()->create());
});

it('lists active and archived donation needs', function () {
    DonationNeed::factory()->create();
    DonationNeed::factory()->create(['archived_at' => now()]);

    $this->get(route('admin.donation-needs.index'))
        ->assertSuccessful()
        ->assertInertia(fn (Assert $page) => $page
            ->component('Admin/DonationNeeds/Index')
            ->has('donationNeeds.data', 2)
            ->where('counts', ['all' => 2, 'active' => 1, 'archived' => 1]));
});

it('filters donation needs by archived state', function (string $archived, int $expectedCount) {
    DonationNeed::factory()->count(2)->create();
    DonationNeed::factory()->create(['archived_at' => now()]);

    $this->get(route('admin.donation-needs.index', ['archived' => $archived]))
        ->assertInertia(fn (Assert $page) => $page->has('donationNeeds.data', $expectedCount));
})->with([
    'active' => ['active', 2],
    'archived' => ['archived', 1],
    'unknown value shows all' => ['nonsense', 3],
]);

it('searches donation needs by title', function () {
    DonationNeed::factory()->create(['title' => 'Операција за Марко']);
    DonationNeed::factory()->create(['title' => 'Друго']);

    $this->get(route('admin.donation-needs.index', ['search' => 'Марко']))
        ->assertInertia(fn (Assert $page) => $page->has('donationNeeds.data', 1));
});

it('creates a donation need', function () {
    $this->post(route('admin.donation-needs.store'), [
        'title' => 'Помош за семејство',
        'description' => '<p>Опис</p>',
        'sticky_content' => '<p>Сметка</p>',
        'needed_amount' => 50000,
        'image' => UploadedFile::fake()->image('photo.jpg'),
    ])
        ->assertRedirect(route('admin.donation-needs.index'))
        ->assertSessionHas('success');

    $donationNeed = DonationNeed::sole();

    expect($donationNeed)
        ->title->toBe('Помош за семејство')
        ->needed_amount->toBe(50000)
        ->sticky_content->toBe('<p>Сметка</p>')
        ->archived_at->toBeNull()
        ->image->toStartWith('donation-needs/');

    Storage::assertExists($donationNeed->image);
});

it('validates new donation needs', function (array $data, array $errors) {
    $this->post(route('admin.donation-needs.store'), [
        'title' => 'Наслов',
        'description' => '<p>Опис</p>',
        'needed_amount' => 1000,
        'image' => UploadedFile::fake()->image('photo.jpg'),
        ...$data,
    ])->assertSessionHasErrors($errors);

    expect(DonationNeed::count())->toBe(0);
})->with([
    'missing title' => [['title' => ''], ['title']],
    'missing description' => [['description' => ''], ['description']],
    'zero amount' => [['needed_amount' => 0], ['needed_amount']],
    'fractional amount' => [['needed_amount' => 10.5], ['needed_amount']],
    'text amount' => [['needed_amount' => 'много'], ['needed_amount']],
    'missing image' => [['image' => null], ['image']],
]);

it('updates a donation need without replacing the image', function () {
    $donationNeed = DonationNeed::factory()->create();
    $originalImage = $donationNeed->image;

    $this->put(route('admin.donation-needs.update', $donationNeed), [
        'title' => 'Изменет наслов',
        'description' => '<p>Изменет опис</p>',
        'sticky_content' => '',
        'needed_amount' => 1234,
    ])
        ->assertRedirect(route('admin.donation-needs.edit', $donationNeed))
        ->assertSessionHas('success');

    expect($donationNeed->fresh())
        ->title->toBe('Изменет наслов')
        ->needed_amount->toBe(1234)
        ->sticky_content->toBeNull()
        ->image->toBe($originalImage);
});

it('shows the edit page for archived donation needs', function () {
    $donationNeed = DonationNeed::factory()->create(['archived_at' => now()]);

    $this->get(route('admin.donation-needs.edit', $donationNeed))
        ->assertInertia(fn (Assert $page) => $page
            ->component('Admin/DonationNeeds/Edit')
            ->where('donationNeed.id', $donationNeed->id));
});

it('archives and restores a donation need', function () {
    $donationNeed = DonationNeed::factory()->create();

    $this->from(route('admin.donation-needs.index'))
        ->post(route('admin.donation-needs.archive', $donationNeed))
        ->assertRedirect(route('admin.donation-needs.index'))
        ->assertSessionHas('success');

    expect($donationNeed->fresh()->archived_at)->not->toBeNull();

    $this->from(route('admin.donation-needs.edit', $donationNeed))
        ->delete(route('admin.donation-needs.unarchive', $donationNeed))
        ->assertRedirect(route('admin.donation-needs.edit', $donationNeed));

    expect($donationNeed->fresh()->archived_at)->toBeNull();
});
