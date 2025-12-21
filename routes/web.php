<?php

use App\Http\Controllers\DonationNeedController;
use App\Http\Controllers\HomeController;
use Illuminate\Support\Facades\Route;

Route::get('/', HomeController::class)->name('index');
Route::resource('donation-needs', DonationNeedController::class)->only('show');
