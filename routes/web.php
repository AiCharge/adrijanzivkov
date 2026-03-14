<?php

use App\Http\Controllers\AboutController;
use App\Http\Controllers\DonationNeedController;
use App\Http\Controllers\HomeController;
use App\Http\Controllers\LegalController;
use App\Http\Controllers\NewsController;
use App\Http\Controllers\StorySubmissionController;
use Illuminate\Support\Facades\Route;

Route::get('/', HomeController::class)->name('index');
Route::get('/about', AboutController::class)->name('about');
Route::resource('donation-needs', DonationNeedController::class)->only(['index', 'show']);
Route::get('/news', [NewsController::class, 'index'])->name('news.index');
Route::get('/news/{news:slug}', [NewsController::class, 'show'])->name('news.show');
Route::get('/terms-of-use', [LegalController::class, 'terms'])->name('terms');
Route::get('/privacy-policy', [LegalController::class, 'privacy'])->name('privacy');

Route::resource('story-submissions', StorySubmissionController::class)->only(['create']);

Route::middleware('throttle:4,1')->group(function () {
    Route::resource('story-submissions', StorySubmissionController::class)->only(['store']);
});
