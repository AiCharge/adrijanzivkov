<?php

use App\Http\Controllers\Admin\AuthenticatedSessionController;
use App\Http\Controllers\Admin\DonationNeedArchiveController;
use App\Http\Controllers\Admin\DonationNeedController;
use App\Http\Controllers\Admin\NewsController;
use App\Http\Controllers\Admin\ProfileController;
use App\Http\Controllers\Admin\RichContentAttachmentController;
use App\Http\Controllers\Admin\StorySubmissionController;
use Illuminate\Support\Facades\Route;

Route::prefix('admin')->name('admin.')->group(function () {
    Route::middleware('guest')->group(function () {
        Route::get('login', [AuthenticatedSessionController::class, 'create'])->name('login');
        Route::post('login', [AuthenticatedSessionController::class, 'store'])
            ->middleware('throttle:10,1')
            ->name('login.store');
    });

    Route::middleware('auth')->group(function () {
        Route::post('logout', [AuthenticatedSessionController::class, 'destroy'])->name('logout');

        Route::redirect('/', '/admin/donation-needs')->name('index');

        Route::get('profile', [ProfileController::class, 'edit'])->name('profile.edit');
        Route::put('profile', [ProfileController::class, 'update'])->name('profile.update');

        Route::resource('donation-needs', DonationNeedController::class)->except(['show', 'destroy']);
        Route::post('donation-needs/{donation_need}/archive', [DonationNeedArchiveController::class, 'store'])
            ->name('donation-needs.archive');
        Route::delete('donation-needs/{donation_need}/archive', [DonationNeedArchiveController::class, 'destroy'])
            ->name('donation-needs.unarchive');

        Route::resource('news', NewsController::class)->except(['show', 'destroy']);

        Route::resource('story-submissions', StorySubmissionController::class)->only(['index', 'edit', 'update']);

        Route::post('rich-content-attachments', [RichContentAttachmentController::class, 'store'])
            ->name('rich-content-attachments.store');
    });
});
