<?php

use App\Http\Controllers\DonationNeedController;
use App\Http\Controllers\HomeController;
use App\Http\Controllers\LegalController;
use App\Http\Controllers\NewsController;
use Illuminate\Support\Facades\Route;

Route::get('/', HomeController::class)->name('index');
Route::resource('donation-needs', DonationNeedController::class)->only(['index', 'show']);
Route::get('/news', [NewsController::class, 'index'])->name('news.index');
Route::get('/news/{news:slug}', [NewsController::class, 'show'])->name('news.show');
Route::get('/uslovi-na-koristenje', [LegalController::class, 'terms'])->name('terms');
Route::get('/politika-za-privatnost', [LegalController::class, 'privacy'])->name('privacy');
