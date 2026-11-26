<?php

use App\Models\News;
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

it('lists news, newest first', function () {
    $older = News::factory()->create(['created_at' => now()->subDay()]);
    $newer = News::factory()->create();

    $this->get(route('admin.news.index'))
        ->assertSuccessful()
        ->assertInertia(fn (Assert $page) => $page
            ->component('Admin/News/Index')
            ->has('news.data', 2)
            ->where('news.data.0.id', $newer->id)
            ->where('news.data.1.id', $older->id));
});

it('searches news by title and short description', function () {
    News::factory()->create(['title' => 'Хуманитарен концерт']);
    News::factory()->create(['title' => 'Друго', 'short_description' => 'Опис за концерт']);
    News::factory()->create(['title' => 'Нешто', 'short_description' => 'Ништо']);

    $this->get(route('admin.news.index', ['search' => 'концерт']))
        ->assertInertia(fn (Assert $page) => $page
            ->has('news.data', 2)
            ->where('filters.search', 'концерт'));
});

it('shows the create page', function () {
    $this->get(route('admin.news.create'))
        ->assertInertia(fn (Assert $page) => $page->component('Admin/News/Create'));
});

it('creates news with an uploaded image', function () {
    $this->post(route('admin.news.store'), [
        'title' => 'Нова акција',
        'short_description' => 'Краток опис',
        'content' => '<p>Содржина</p>',
        'main_image' => UploadedFile::fake()->image('cover.jpg'),
    ])
        ->assertRedirect(route('admin.news.index'))
        ->assertSessionHas('success');

    $news = News::sole();

    expect($news)
        ->title->toBe('Нова акција')
        ->slug->toBe('nova-akcija')
        ->content->toBe('<p>Содржина</p>')
        ->main_image->toStartWith('news/');

    Storage::assertExists($news->main_image);
});

it('gives news with the same title a unique slug', function () {
    News::factory()->create(['title' => 'Иста вест']);

    $this->post(route('admin.news.store'), [
        'title' => 'Иста вест',
        'short_description' => 'Краток опис',
        'content' => '<p>Содржина</p>',
        'main_image' => UploadedFile::fake()->image('cover.jpg'),
    ])->assertSessionHasNoErrors();

    expect(News::query()->pluck('slug')->all())->toBe(['ista-vest', 'ista-vest-2']);
});

it('validates new news', function (array $data, array $errors) {
    $this->post(route('admin.news.store'), [
        'title' => 'Наслов',
        'short_description' => 'Опис',
        'content' => '<p>Содржина</p>',
        'main_image' => UploadedFile::fake()->image('cover.jpg'),
        ...$data,
    ])->assertSessionHasErrors($errors);

    expect(News::count())->toBe(0);
})->with([
    'missing title' => [['title' => ''], ['title']],
    'title too long' => [['title' => str_repeat('а', 256)], ['title']],
    'short description too long' => [['short_description' => str_repeat('а', 501)], ['short_description']],
    'missing content' => [['content' => ''], ['content']],
    'missing image' => [['main_image' => null], ['main_image']],
    'image is not an image' => [['main_image' => UploadedFile::fake()->create('file.pdf', 10, 'application/pdf')], ['main_image']],
]);

it('shows the edit page by slug', function () {
    $news = News::factory()->create();

    $this->get(route('admin.news.edit', $news))
        ->assertInertia(fn (Assert $page) => $page
            ->component('Admin/News/Edit')
            ->where('news.id', $news->id));
});

it('updates news and keeps the image and slug when no new image is given', function () {
    $news = News::factory()->create(['title' => 'Стар наслов']);
    $originalSlug = $news->slug;
    $originalImage = $news->main_image;

    $this->put(route('admin.news.update', $news), [
        'title' => 'Нов наслов',
        'short_description' => 'Нов опис',
        'content' => '<p>Нова содржина</p>',
    ])
        ->assertRedirect(route('admin.news.edit', $originalSlug))
        ->assertSessionHas('success');

    expect($news->fresh())
        ->title->toBe('Нов наслов')
        ->slug->toBe($originalSlug)
        ->main_image->toBe($originalImage);
});

it('replaces the image when a new one is uploaded', function () {
    $news = News::factory()->create();

    $this->put(route('admin.news.update', $news), [
        'title' => $news->title,
        'short_description' => $news->short_description,
        'content' => $news->content,
        'main_image' => UploadedFile::fake()->image('new.png'),
    ])->assertSessionHasNoErrors();

    $news->refresh();

    expect($news->main_image)->toStartWith('news/');
    Storage::assertExists($news->main_image);
});
