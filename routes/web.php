<?php

use App\Http\Controllers\DonationNeedController;
use App\Http\Controllers\HomeController;
use App\Http\Controllers\NewsController;
use Illuminate\Support\Facades\Route;

Route::get('/', HomeController::class)->name('index');
Route::resource('donation-needs', DonationNeedController::class)->only('show');
Route::get('/news/{news:slug}', [NewsController::class, 'show'])->name('news.show');
